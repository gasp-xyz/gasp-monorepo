use crate::chainio::SignerClient;
use crate::chainio::{avs::AvsContracts, build_eth_client, eigen::ElContracts, Client};
use crate::cli::CliArgs;
use crate::crypto::bn254::{BlsKeypair, OperatorId};
use crate::crypto::EthConvert;
use crate::rpc::Rpc;

use bindings::{
    finalizer_task_manager::{
        FinalizerTaskManagerEvents, Operator as TMOperator, OperatorStateInfo, OperatorsAdded,
        OperatorsQuorumCountUpdate, OperatorsStakeUpdate, QuorumsAdded, QuorumsApkUpdate,
        QuorumsStakeUpdate,
    },
    shared_types::{G1Point, G2Point, OpTask, OpTaskResponse, RdTask, RdTaskResponse},
};
use ethers::providers::{Middleware, PendingTransaction, SubscriptionStream};
use ethers::signers::LocalWallet;
use ethers::{
    contract::{stream, LogMeta},
    providers::StreamExt,
    types::{Address, Bytes},
};

use ethers::abi::AbiEncode;
use eyre::{eyre, OptionExt};
use serde::Serialize;
use sp_core::H256;
use sp_runtime::traits::{Hash, Keccak256};
use std::collections::HashMap;
use std::sync::Arc;
use substrate_rpc_client::{rpc_params, ws_client, ClientT};
use tokio::select;
use tracing::{debug, error, info, instrument};

type QuorumNum = u8;

#[derive(Clone)]
pub struct CustomOperatorAvsState {
    pub operator_id: [u8; 32],
    pub stake_per_quorum: HashMap<QuorumNum, u128>,
}

#[derive(Debug, Serialize)]
pub struct OperatorStatus {
    pub eth_address: Address,
    pub registered_with_eigen: bool,
    pub bls_key_registered: bool,
    pub bls_g1: G1Point,
    pub bls_g2: G2Point,
    pub registered_with_avs: bool,
    pub operator_id: Option<OperatorId>,
    // opted_in_salshing_by_avs: bool,
    // frozen: bool,
}

#[derive(Debug)]
pub struct Operator {
    signer: Option<Arc<SignerClient>>,
    client: Arc<Client>,
    address: Address,
    avs_contracts: AvsContracts,
    el_contracts: ElContracts,
    bls_keypair: BlsKeypair,
    substrate_client_uri: String,
    rpc: Rpc,
}
impl Operator {
    #[instrument(name = "create_operator", skip_all)]
    pub async fn from_cli(cfg: &CliArgs) -> eyre::Result<Arc<Self>> {
        let (address, client, signer) = build_eth_client(cfg).await?;
        let avs_contracts = AvsContracts::build(cfg, client.clone()).await?;
        let el_contracts = ElContracts::build(cfg, client.clone()).await?;

        info!("Decrypting BLS keypair...");
        let bls_key = cfg.get_bls_keystore()?.into_bls_keypair()?;
        info!(
            "Bls Keypair decrypted with operator id: {:x}",
            bls_key.operator_id()
        );

        let rpc = Rpc::build(cfg);

        Ok(Arc::new(Self {
            signer,
            client,
            address,
            avs_contracts,
            el_contracts,
            substrate_client_uri: cfg.substrate_rpc_url.to_owned(),
            bls_keypair: bls_key,
            rpc,
        }))
    }

    #[instrument(skip_all)]
    pub async fn watch_new_tasks(self: Arc<Self>) -> eyre::Result<()> {
        let evs = self.clone().avs_contracts.new_task_stream();
        let mut stream: stream::EventStream<'_, _, (FinalizerTaskManagerEvents, LogMeta), _> =
            evs.subscribe_with_meta().await?;

        // event stream does not finish with `None` after websocket closure, use block subscription for it
        let mut blocks: SubscriptionStream<'_, _, _> =
            self.avs_contracts.ws_client.subscribe_blocks().await?;

        info!("Subscribed to events - now watching subscription");

        loop {
            select! {
                Some(stream_event) = stream.next() => match stream_event {
                    Ok((stream_event, log)) => {
                        debug!("Got new task at: {:?}", log);
                        PendingTransaction::new(log.transaction_hash, self.clone().client.provider()).await?;
                        let mut op_payload: Option<OpTaskResponse> = None;
                        let mut rd_payload: Option<RdTaskResponse> = None;
                        match stream_event {
                            FinalizerTaskManagerEvents::NewOpTaskCreatedFilter(event) => {
                                let operators_state_info_hash = self.clone().get_operators_state_info_hash(event.task.clone()).await?;
                                debug!("Operators State Info Hash {:?}", operators_state_info_hash);
                                op_payload = Some(OpTaskResponse {
                                    reference_task_index: event.task_index,
                                    reference_task_hash: Keccak256::hash(
                                        vec![0u8;31].into_iter().chain(vec![32u8]).chain(
                                            event.task.clone().encode().into_iter()
                                        ).collect::<Vec<_>>()
                                        .as_ref()).into(),
                                    operators_state_info_hash,
                                });
                            },
                            FinalizerTaskManagerEvents::NewRdTaskCreatedFilter(event) => {
                                let mut rd_payload_tmp = self.clone().get_rd_update(event.task.clone()).await?;
                                rd_payload_tmp.reference_task_index = event.task_index;
                                rd_payload_tmp.reference_task_hash = Keccak256::hash(
                                    vec![0u8;31].into_iter().chain(vec![32u8]).chain(
                                        event.task.clone().encode().into_iter()
                                    ).collect::<Vec<_>>()
                                    .as_ref()).into();
                                debug!("rd_payload_tmp: {:?}", rd_payload_tmp);
                                rd_payload = Some(rd_payload_tmp);
                            },
                            _ => return Err(eyre!("Got unexpected stream event"))
                        }
                        let response = self
                            .rpc
                            .send_task_response(op_payload, rd_payload, &self.bls_keypair)
                            .await;
                        match response {
                            Ok(r) => match r.error_for_status_ref() {
                                Err(e) => error!("{} - {}", e, r.text().await?),
                                Ok(_) => info!("Task finished successfuly and sent to AVS service"),
                            },
                            Err(e) => error!("send_task_response failed with error: {:?}", e),
                        }
                    }
                    Err(e) => return Err(eyre!("EthWs subscription error {:?}", e)),
                },
                block = blocks.next() => {
                    if block.is_none() {
                        break
                    }
                }
            }
        }

        // ws provider has internal reconnect, but if it fails we are done
        Ok(())
    }

    pub(crate) async fn get_rd_update(
        self: Arc<Self>,
        rd_task: RdTask,
    ) -> eyre::Result<RdTaskResponse> {
        use codec::{Decode, Encode};
        #[derive(Debug, codec::Encode, codec::Decode)]
        struct StorageItemKeyType(u8, u128);
        #[derive(Debug, codec::Encode, codec::Decode)]
        struct StorageItemDataType(u32, (u128, u128), [u8; 20]);

        debug!("get_rd_update - rd_task: {:?}", rd_task);

        let rpc = ws_client(self.substrate_client_uri.clone())
            .await
            .map_err(|e| eyre!(e))?;
        let two_x_hash_pallet = sp_io::hashing::twox_128(b"Rolldown");
        let two_x_hash_storage_item = sp_io::hashing::twox_128(b"L2RequestsBatch");

        let storage_item_key: StorageItemKeyType =
            StorageItemKeyType(rd_task.chain_id, rd_task.batch_id.into());

        debug!("get_rd_update - storage_item_key: {:?}", storage_item_key);

        let storage_item_key_encoded = storage_item_key.encode();
        debug!(
            "get_rd_update - storage_item_key_encoded: {:?}",
            array_bytes::bytes2hex("0x", storage_item_key_encoded.clone())
        );

        let mut storage_item_key_hashed =
            sp_io::hashing::blake2_128(&storage_item_key_encoded[..]).to_vec();
        debug!(
            "get_rd_update - storage_item_key_hashed: {:?}",
            array_bytes::bytes2hex("0x", storage_item_key_hashed.clone())
        );
        storage_item_key_hashed.extend_from_slice(&storage_item_key_encoded[..]);
        debug!(
            "get_rd_update - storage_item_key_hashed: {:?}",
            array_bytes::bytes2hex("0x", storage_item_key_hashed.clone())
        );

        let mut storage_key = Vec::<u8>::new();
        storage_key.extend_from_slice(&two_x_hash_pallet[..]);
        storage_key.extend_from_slice(&two_x_hash_storage_item[..]);
        storage_key.extend_from_slice(&storage_item_key_hashed[..]);

        debug!(
            "get_rd_update - storage_key: {:?}",
            array_bytes::bytes2hex("0x", storage_key.clone())
        );

        let params = rpc_params!(array_bytes::bytes2hex("0x", storage_key));

        debug!("get_rd_update - params: {:?}", params);

        let maybe_storage_raw_string: Option<String> =
            rpc.request("state_getStorage", params).await?;

        let storage_raw_string =
            maybe_storage_raw_string.ok_or_eyre("state_getStorage got None")?;

        debug!(
            "get_rd_update - storage_raw_string: {:?}",
            storage_raw_string
        );

        let storage_raw = array_bytes::hex2bytes(storage_raw_string.as_str())
            .map_err(|_| eyre!("Failed to decode storage_raw_string"))?;

        let StorageItemDataType(created_block_number, (range_start, range_end), updater) =
            StorageItemDataType::decode(&mut &storage_raw[..])?;

        debug!(
            "get_rd_update - (created_block_number, (range_start, range_end), updater): {:?}",
            (created_block_number, (range_start, range_end), updater)
        );

        // TODO
        // Properly solve this hack
        let chain_as_string = match rd_task.chain_id {
            0 => String::from("Ethereum"),
            1 => String::from("Arbitrum"),
            2 => String::from("Base"),
            3 => String::from("Monad"),
            4 => String::from("MegaEth"),
            5 => String::from("Sonic"),
            6 => String::from("Berachain"),
            _ => return Err(eyre!("Unexpected chain in task")),
        };
        let params = rpc_params!(chain_as_string, (range_start, range_end));
        let rd_update: H256 = rpc
            .request("rolldown_get_merkle_root", params.clone())
            .await?;

        if rd_update.is_zero() {
            return Err(eyre!(
                "rolldown_get_merkle_root returned zero value, params: {:?}",
                params
            ));
        }

        debug!("get_rd_update - rd_update: {:?}", rd_update);

        let partial_rd_task_response = RdTaskResponse {
            reference_task_index: Default::default(),
            reference_task_hash: Default::default(),
            chain_id: rd_task.chain_id,
            batch_id: rd_task.batch_id,
            rd_update: rd_update.into(),
            range_start: range_start.into(),
            range_end: range_end.into(),
            updater: updater.into(),
        };

        Ok(partial_rd_task_response)
    }

    pub(crate) async fn wait_for_gasp_to_sync(self: Arc<Self>) -> eyre::Result<()> {
        let rpc = ws_client(self.substrate_client_uri.clone())
            .await
            .map_err(|e| eyre!(e))?;

        let params = rpc_params!();
        let health: sc_rpc_api::system::helpers::Health =
            rpc.request("system_health", params).await?;

        debug!("Gasp node is_syncing - {:?}", health.is_syncing);

        if health.is_syncing {
            let retry_delay: tokio::time::Duration = tokio::time::Duration::from_secs(600);
            info!("Waiting for gasp node to finish syncing");

            loop {
                let params = rpc_params!();
                let health: sc_rpc_api::system::helpers::Health =
                    rpc.request("system_health", params).await?;

                info!("Gasp node is_syncing - {:?}", health.is_syncing);
                if !health.is_syncing {
                    break;
                }
                info!("Rechecking sync status after - {:?}", retry_delay);
                tokio::time::sleep(retry_delay).await;
            }
        }
        debug!("Done waiting for gasp node to sync");
        info!("Gasp node is synced!");
        Ok(())
    }

    pub(crate) async fn get_operators_state_info_hash(
        self: Arc<Self>,
        task: OpTask,
    ) -> eyre::Result<[u8; 32]> {
        // We assume that the quorumNumbers are alteast unique even if not sorted

        let old_quorum_threshold_percentage =
            if task.last_completed_op_task_created_block == task.task_created_block {
                Default::default()
            } else {
                task.last_completed_op_task_quorum_threshold_percentage
            };
        let new_quorum_threshold_percentage = task.quorum_threshold_percentage;

        let mut old_quorum_numbers =
            if task.last_completed_op_task_created_block == task.task_created_block {
                Default::default()
            } else {
                task.last_completed_op_task_quorum_numbers
                    .into_iter()
                    .collect::<Vec<u8>>()
            };
        let mut new_quorum_numbers = task.quorum_numbers.into_iter().collect::<Vec<u8>>();
        old_quorum_numbers.sort();
        new_quorum_numbers.sort();

        let old_task_block = if task.last_completed_op_task_created_block == task.task_created_block
        {
            Default::default()
        } else {
            task.last_completed_op_task_created_block
        };
        let new_task_block = task.task_created_block;

        info!("old_task_block: {:?}", old_task_block);
        info!("new_task_block: {:?}", new_task_block);

        let registry_coordinator_address = &self.avs_contracts.registry_coordinator_address;
        let stake_registry = &self.avs_contracts.stake_registry;
        let bls_apk_registry = &self.avs_contracts.bls_apk_registry;
        let task_manager = &self.avs_contracts.task_manager;

        let mut maybe_i = old_quorum_numbers.iter().peekable();
        let mut maybe_j = new_quorum_numbers.iter().peekable();

        let mut quorums_common: Vec<u8> = Vec::new();
        let mut quorums_removed: Vec<u8> = Vec::new();
        let mut quorums_added: Vec<QuorumsAdded> = Vec::new();
        let mut quorums_apk_update: Vec<QuorumsApkUpdate> = Vec::new();
        let mut quorums_stake_update: Vec<QuorumsStakeUpdate> = Vec::new();

        loop {
            match (maybe_i.peek(), maybe_j.peek()) {
                (Some(&&i), Some(&&j)) if i == j => {
                    // handle potential update
                    let old_quorum_apk = bls_apk_registry
                        .get_apk(i)
                        .block::<u64>(u64::from(old_task_block))
                        .await?;
                    let old_quorum_stake = stake_registry
                        .get_current_total_stake(i)
                        .block(u64::from(old_task_block))
                        .await?;

                    let new_quorum_apk = bls_apk_registry
                        .get_apk(i)
                        .block(u64::from(new_task_block))
                        .await?;
                    let new_quorum_stake = stake_registry
                        .get_current_total_stake(i)
                        .block(u64::from(new_task_block))
                        .await?;

                    if old_quorum_apk != new_quorum_apk {
                        quorums_apk_update.push(QuorumsApkUpdate {
                            quorum_number: i,
                            quorum_apk: new_quorum_apk,
                        });
                    }
                    if old_quorum_stake != new_quorum_stake {
                        quorums_stake_update.push(QuorumsStakeUpdate {
                            quorum_number: i,
                            quorum_stake: new_quorum_stake,
                        });
                    }

                    quorums_common.push(i);

                    maybe_i.next();
                    maybe_j.next();
                }
                (Some(&&i), Some(&&j)) if i < j => {
                    // handle quorum number removed
                    quorums_removed.push(i);
                    maybe_i.next();
                }
                (Some(&&i), Some(&&j)) if i > j => {
                    // handle quorum number added
                    quorums_added.push(QuorumsAdded {
                        quorum_number: j,
                        quorum_stake: stake_registry
                            .get_current_total_stake(j)
                            .block(u64::from(new_task_block))
                            .await?,
                        quorum_apk: bls_apk_registry
                            .get_apk(j)
                            .block(u64::from(new_task_block))
                            .await?,
                    });

                    maybe_j.next();
                }
                (Some(&&i), None) => {
                    // handle quorum number removed
                    quorums_removed.push(i);
                    maybe_i.next();
                }
                (None, Some(&&j)) => {
                    // handle quorum number added
                    quorums_added.push(QuorumsAdded {
                        quorum_number: j,
                        quorum_stake: stake_registry
                            .get_current_total_stake(j)
                            .block(u64::from(new_task_block))
                            .await?,
                        quorum_apk: bls_apk_registry
                            .get_apk(j)
                            .block(u64::from(new_task_block))
                            .await?,
                    });
                    maybe_j.next();
                }
                (None, None) => {
                    break;
                }
                _ => unreachable!(),
            }
        }

        let old_operators_stake = task_manager
            .get_operator_state(
                *registry_coordinator_address,
                old_quorum_numbers.clone().into(),
                old_task_block,
            )
            .await?;
        let new_operators_stake = task_manager
            .get_operator_state(
                *registry_coordinator_address,
                new_quorum_numbers.clone().into(),
                new_task_block,
            )
            .await?;

        let mut old_operators_avs_state = self
            .get_operators_avs_state_at_block(
                old_operators_stake,
                old_quorum_numbers.clone().into(),
            )
            .await?
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let mut new_operators_avs_state = self
            .get_operators_avs_state_at_block(
                new_operators_stake,
                new_quorum_numbers.clone().into(),
            )
            .await?
            .values()
            .cloned()
            .collect::<Vec<_>>();

        old_operators_avs_state.sort_by_key(|v| v.operator_id);
        new_operators_avs_state.sort_by_key(|v| v.operator_id);

        let mut maybe_i = old_operators_avs_state.iter().peekable();
        let mut maybe_j = new_operators_avs_state.iter().peekable();

        let mut operators_removed: Vec<[u8; 32]> = Vec::new();
        let mut operators_added: Vec<OperatorsAdded> = Vec::new(); // Needs to be sorted
        let mut operators_quorum_count_update: Vec<OperatorsQuorumCountUpdate> = Vec::new();
        let mut operators_stake_update: Vec<OperatorsStakeUpdate> = Vec::new();

        loop {
            match (maybe_i.peek(), maybe_j.peek()) {
                (Some(&i), Some(&j)) if i.operator_id == j.operator_id => {
                    // handle potential update

                    if i.stake_per_quorum.len() != j.stake_per_quorum.len() {
                        operators_quorum_count_update.push(OperatorsQuorumCountUpdate {
                            operator_id: j.operator_id,
                            quorum_count: j.stake_per_quorum.len().try_into()?,
                        });
                    }
                    let mut operator_stake_update = OperatorsStakeUpdate {
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                    };
                    for qn in quorums_removed.iter() {
                        operator_stake_update.quorum_for_stakes.push(*qn);
                        operator_stake_update.quorum_stakes.push(Default::default());
                    }
                    for qn in quorums_added.iter().map(|x| x.quorum_number) {
                        operator_stake_update.quorum_for_stakes.push(qn);
                        let stake = j
                            .stake_per_quorum
                            .get(&qn)
                            .map(u128::to_owned)
                            .unwrap_or_else(|| {
                                error!("Failed to get operator quorum stake");
                                Default::default()
                            });
                        operator_stake_update.quorum_stakes.push(stake)
                    }
                    for qn in quorums_common.iter() {
                        let stake_old = i
                            .stake_per_quorum
                            .get(qn)
                            .map(u128::to_owned)
                            .unwrap_or_else(|| {
                                error!("Failed to get operator quorum stake");
                                Default::default()
                            });
                        let stake_new = j
                            .stake_per_quorum
                            .get(qn)
                            .map(u128::to_owned)
                            .unwrap_or_else(|| {
                                error!("Failed to get operator quorum stake");
                                Default::default()
                            });
                        if stake_old != stake_new {
                            operator_stake_update.quorum_for_stakes.push(*qn);
                            operator_stake_update.quorum_stakes.push(stake_new)
                        }
                    }
                    if !operator_stake_update.quorum_for_stakes.is_empty() {
                        operators_stake_update.push(operator_stake_update);
                    }

                    maybe_i.next();
                    maybe_j.next();
                }
                (Some(&i), Some(&j)) if i.operator_id < j.operator_id => {
                    // handle operator removed
                    operators_removed.push(i.operator_id);
                    maybe_i.next();
                }
                (Some(&i), Some(&j)) if i.operator_id > j.operator_id => {
                    // handle quorum number added

                    let mut operator_added = OperatorsAdded {
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                        quorum_count: j.stake_per_quorum.len().try_into()?,
                    };

                    for qn in j.stake_per_quorum.keys() {
                        operator_added.quorum_for_stakes.push(*qn);
                        let stake = j
                            .stake_per_quorum
                            .get(qn)
                            .map(u128::to_owned)
                            .unwrap_or_else(|| {
                                error!("Failed to get operator quorum stake");
                                Default::default()
                            });
                        operator_added.quorum_stakes.push(stake)
                    }

                    operators_added.push(operator_added);

                    maybe_j.next();
                }
                (Some(&i), None) => {
                    // handle quorum number removed
                    operators_removed.push(i.operator_id);
                    maybe_i.next();
                }
                (None, Some(&j)) => {
                    // handle operator added

                    let mut operator_added = OperatorsAdded {
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                        quorum_count: j.stake_per_quorum.len().try_into()?,
                    };

                    for qn in j.stake_per_quorum.keys() {
                        operator_added.quorum_for_stakes.push(*qn);
                        let stake = j
                            .stake_per_quorum
                            .get(qn)
                            .map(u128::to_owned)
                            .unwrap_or_else(|| {
                                error!("Failed to get operator quorum stake");
                                Default::default()
                            });
                        operator_added.quorum_stakes.push(stake)
                    }

                    operators_added.push(operator_added);
                    maybe_j.next();
                }
                (None, None) => {
                    // handle quorum number added
                    break;
                }
                _ => unreachable!(),
            }
        }

        let operators_state_changed = !quorums_removed.is_empty()
            || !quorums_added.is_empty()
            || !quorums_apk_update.is_empty()
            || !quorums_stake_update.is_empty()
            || !operators_removed.is_empty()
            || !operators_added.is_empty()
            || !operators_stake_update.is_empty()
            || !operators_quorum_count_update.is_empty()
            || (old_quorum_threshold_percentage != new_quorum_threshold_percentage)
            || (old_quorum_numbers != new_quorum_numbers);

        let operator_state_info = OperatorStateInfo {
            operators_state_changed,
            quorums_removed,
            quorums_added,
            quorums_stake_update,
            quorums_apk_update,
            operators_removed,
            operators_added,
            operators_stake_update,
            operators_quorum_count_update,
        };

        info!("operator_state_info: {:?}", operator_state_info);

        let operator_state_info_hash = Keccak256::hash(
            vec![0u8; 31]
                .into_iter()
                .chain(vec![32u8])
                .chain(operator_state_info.clone().encode().into_iter())
                .collect::<Vec<_>>()
                .as_ref(),
        );
        Ok(operator_state_info_hash.into())
    }

    pub async fn get_operators_avs_state_at_block(
        &self,
        operators_stakes_in_quorums: Vec<Vec<TMOperator>>,
        quorum_nums: Bytes,
    ) -> eyre::Result<HashMap<H256, CustomOperatorAvsState>> {
        let mut operators_avs_state: HashMap<H256, CustomOperatorAvsState> = HashMap::new();

        if operators_stakes_in_quorums.len() != quorum_nums.len() {
            return Err(eyre!(
                "operators_stakes_in_quorums.len() != quorum_nums.len()"
            ));
        }

        for (quorum_id, quorum_num) in quorum_nums.iter().enumerate() {
            for operator in &operators_stakes_in_quorums[quorum_id] {
                let stake_per_quorum = HashMap::new();
                let avs_state = operators_avs_state
                    .entry(H256::from(operator.operator_id))
                    .or_insert_with(|| CustomOperatorAvsState {
                        operator_id: operator.operator_id,
                        stake_per_quorum,
                    });
                avs_state
                    .stake_per_quorum
                    .insert(*quorum_num, operator.stake);
            }
        }

        Ok(operators_avs_state)
    }

    pub(crate) fn operator_id(&self) -> OperatorId {
        self.bls_keypair.operator_id()
    }

    pub(crate) fn signer(&self) -> LocalWallet {
        self.signer.as_ref().unwrap().signer().clone()
    }

    pub(crate) fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    #[instrument(skip_all)]
    pub(crate) async fn get_status(&self) -> eyre::Result<OperatorStatus> {
        let el_status = self
            .el_contracts
            .is_operator_registered(self.address)
            .await?;

        let id = self.avs_contracts.operator_id(self.address).await?;

        Ok(OperatorStatus {
            eth_address: self.address,
            registered_with_eigen: el_status,
            bls_key_registered: id.is_some(),
            bls_g1: EthConvert::to_g1(self.bls_keypair.public).unwrap_or_default(),
            bls_g2: EthConvert::to_g2(self.bls_keypair.public_g2()).unwrap_or_default(),
            operator_id: id,
            registered_with_avs: id.is_some(),
        })
    }

    #[instrument(skip_all)]
    pub(crate) async fn register(&self) -> eyre::Result<()> {
        let client = self
            .signer
            .clone()
            .ok_or(eyre!("missing signer ecdsa key"))?;

        let status = self
            .el_contracts
            .is_operator_registered(self.address)
            .await?;

        if !status {
            info!("Registering Operator {:x} with EigenLayer", self.address);

            self.el_contracts
                .register_as_operator_with_el(client)
                .await?;

            info!("Sucessfully registered with EigenLayer")
        } else {
            info!("Operator already registered in EigenLayer");
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub(crate) async fn opt_in_avs(&self) -> eyre::Result<()> {
        let client = self
            .signer
            .clone()
            .ok_or(eyre!("missing signer ecdsa key"))?;

        if self
            .avs_contracts
            .operator_id(self.address)
            .await?
            .is_some()
        {
            info!("Operator already opt-in AVS");
        } else {
            info!("Registering Operator {:x} with AVS", self.address);
            let sig_params = self
                .el_contracts
                .get_delegation_signature_params(&client)
                .await?;
            self.avs_contracts
                .register_with_avs(&self.bls_keypair, sig_params, client)
                .await?;
            let id = self
                .avs_contracts
                .operator_id(self.address)
                .await?
                .expect("should have operator id after success register trx");
            info!("Sucessfully registered with AVS with id {:x}", id);
        }
        Ok(())
    }

    #[instrument(skip_all)]
    pub(crate) async fn opt_out_avs(&self) -> eyre::Result<()> {
        let client = self
            .signer
            .clone()
            .ok_or(eyre!("missing signer ecdsa key"))?;

        if self
            .avs_contracts
            .operator_id(self.address)
            .await?
            .is_some()
        {
            self.avs_contracts.deregister_with_avs(client).await?;
            info!("Operator opted out with AVS sucessfully");
        } else {
            info!("Operator not opt in with AVS");
        }

        Ok(())
    }
}
