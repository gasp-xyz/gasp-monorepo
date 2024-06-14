use crate::chainio::{avs::AvsContracts, build_eth_client, eigen::ElContracts, Client};
use crate::cli::CliArgs;
use crate::crypto::bn254::{BlsKeypair, OperatorId};
use crate::crypto::EthConvert;
use crate::executor::execute::execute_block;
use crate::rpc::Rpc;

use bindings::{
    finalizer_task_manager::NewTaskCreatedFilter,
    shared_types::{G1Point, G2Point, OperatorStateInfo, Task, TaskResponse, QuorumsAdded, QuorumsStakeUpdate, QuorumsApkUpdate, OperatorsAdded, OperatorsQuorumCountUpdate, OperatorsStakeUpdate},
};
use ethers::providers::{Middleware, PendingTransaction, SubscriptionStream};
use ethers::{
    contract::{stream, LogMeta},
    providers::StreamExt,
    types::Address, abi::AbiDecode
};
use node_executor::ExecutorDispatch;
use node_primitives::BlockNumber;
use eigen_client_avsregistry::{reader::OperatorStateRetriever};

use alloy_primitives::{Bytes, U256, FixedBytes};
use serde::Serialize;
use sp_core::H256;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::{generic, OpaqueExtrinsic};
use std::sync::Arc;
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio::try_join;
use tracing::{debug, error, info, instrument};
use std::collections::HashMap;
use alloy_sol_types::SolValue;
use alloy_primitives::private::alloy_rlp::Decodable;

pub type Header = generic::HeaderVer<node_primitives::BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

type QuorumNum = u8;

#[derive(Clone)]
pub struct CustomOperatorAvsState {
    pub operator_id: [u8; 32],
    pub stake_per_quorum: HashMap<QuorumNum, U256>,
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
    pub client: Arc<Client>,
    avs_contracts: AvsContracts,
    el_contracts: ElContracts,
    bls_keypair: BlsKeypair,
    substrate_client_uri: String,
    rpc: Rpc,
}
impl Operator {
    #[instrument(name = "create_operator", skip_all)]
    pub async fn from_cli(cfg: &CliArgs) -> eyre::Result<Arc<Self>> {
        let client = Arc::new(build_eth_client(cfg).await?);
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
            avs_contracts,
            el_contracts,
            substrate_client_uri: cfg.substrate_rpc_url.to_owned(),
            client,
            bls_keypair: bls_key,
            rpc,
        }))
    }

    #[instrument(skip_all)]
    pub async fn watch_new_tasks(self: Arc<Self>) -> eyre::Result<()> {
        let evs = self.clone().avs_contracts.new_task_stream();
        let mut stream: stream::EventStream<'_, _, (NewTaskCreatedFilter, LogMeta), _> =
            evs.subscribe_with_meta().await?;

        // event stream does not finish with `None` after websocket closure, use block subscription for it
        let mut blocks: SubscriptionStream<'_, _, _> =
            self.avs_contracts.ws_client.subscribe_blocks().await?;

        loop {
            select! {
                Some(event) = stream.next() => match event {
                    Ok((event, log)) => {
                        debug!("Got new task at: {:?}", log);
                        PendingTransaction::new(log.transaction_hash, self.client.provider()).await?;
                        let event_clone = event.clone();
                        let self_clone = self.clone();
                        let execute_block_join_handle = tokio::spawn(async move {
                            info!("Executing a Block for task: {:?}", event_clone);
                            self_clone.execute_block(event_clone.task.block_number.as_u32()).await

                        });
                        let event_clone = event.clone();
                        let self_clone = self.clone();
                        let get_operators_state_info_handle = tokio::spawn(async move {
                            info!("Get operators state hash: {:?}", event_clone);
                            self_clone.get_operators_state_info(event_clone.task).await
                        });
                        let (proofs, operators_state_info) = try_join!(execute_block_join_handle, get_operators_state_info_handle)?;
                        let (proofs, operators_state_info) = (proofs?, operators_state_info?);
                        debug!("Operators State Info {:?}", operators_state_info);
                        debug!("Block executed successfully {:?}", proofs);
                        let payload = TaskResponse {
                            reference_task_index: event.task_index,
                            reference_task: event.task.clone(),
                            operators_state_info: operators_state_info.into(),
                            block_hash: proofs.0.as_fixed_bytes().to_owned(),
                            storage_proof_hash: proofs.1.as_fixed_bytes().to_owned(),
                            pending_state_hash: proofs.2.as_fixed_bytes().to_owned(),
                        };
                        let response = self
                            .rpc
                            .send_task_response(payload, &self.bls_keypair)
                            .await;
                        match response {
                            Ok(r) => match r.error_for_status_ref() {
                                Err(e) => error!("{} - {}", e, r.text().await?),
                                Ok(_) => info!("Task finished successfuly and sent to AVS service"),
                            },
                            Err(e) => error!("{}", e),
                        }
                    }
                    Err(e) => tracing::error!("EthWs subscription error {:?}", e),
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

    pub(crate) async fn execute_block(
        self: Arc<Self>,
        block_number: BlockNumber,
    ) -> eyre::Result<(H256, H256, H256)> {
        use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
        let res = execute_block::<
            Block,
            ExtendedHostFunctions<
                sp_io::SubstrateHostFunctions,
                <ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
            >,
        >(&self.substrate_client_uri, block_number)
        .await?;

        Ok(res)
    }

    pub(crate) async fn get_operators_state_info(
        self: Arc<Self>,
        task: Task,
    ) -> eyre::Result<OperatorStateInfo> {

        // We assume that the quorumNumbers are alteast unique even if not sorted
        let mut old_quorum_numbers = task.last_completed_task_quorum_numbers;
        let mut new_quorum_numbers = task.quorum_numbers;
        old_quorum_numbers.sort();
        new_quorum_numbers.sort();

        let mut old_task_block = task.last_completed_task_created_block;
        let mut new_task_block = task.task_created_block;

        let chain_reader = self
        .avs_contracts
        .avs_registry_chain_reader;

        let maybe_i = old_quorum_numbers.iter().peekable();
        let maybe_j = new_quorum_numbers.iter().peekable();
        
        let quorums_common: Vec<u8> = Vec::new();
        let quorums_removed: Vec<u8> = Vec::new();
        let quorums_added: Vec<QuorumsAdded> = Vec::new();
        let quorums_apk_update: Vec<QuorumsApkUpdate> = Vec::new();
        let quorums_stake_update: Vec<QuorumsStakeUpdate> = Vec::new();
        
        loop {
            match (maybe_i.peek(), maybe_j.peek()){
                (Some(&&i), Some(&&j)) if i == j => {
                    // handle potential update
                    let old_quorum_apk = chain_reader.get_quorum_apk_at_block(i, old_task_block).await.map_err(|e| Err(e.to_string()).into())?;
                    let old_quorum_stake = chain_reader.get_quorum_stake_at_block(i, old_task_block).await.map_err(|e| Err(e.to_string()).into())?;

                    let new_quorum_apk = chain_reader.get_quorum_apk_at_block(i, new_task_block).await.map_err(|e| Err(e.to_string()).into())?;
                    let new_quorum_stake = chain_reader.get_quorum_stake_at_block(i, new_task_block).await.map_err(|e| Err(e.to_string()).into())?;

                    if (old_quorum_apk.X != new_quorum_apk.X) && (old_quorum_apk.Y != new_quorum_apk.Y) {
                        quorums_apk_update.push(QuorumsApkUpdate{
                            quorum_number: i,
                            quorum_apk: G1Point::decode(new_quorum_apk.abi_encode())?
                        });
                    }
                    if old_quorum_stake != new_quorum_stake{
                        quorums_stake_update.push(QuorumsStakeUpdate{
                        quorum_number: i,
                        quorum_stake: new_quorum_stake,
                        });
                    }
                    
                    quorums_common.push(i);

                    maybe_i.next(); maybe_j.next();
                },
                (Some(&&i), Some(&&j)) if i < j => {
                    // handle quorum number removed
                    quorums_removed.push(i);
                    // quorums_apk_update.push(QuorumsApkUpdate{
                    //     quorum_number: i,
                    //     quorum_apk: Default::default(),
                    // });
                    // quorums_stake_update.push(QuorumsStakeUpdate{
                    //     quorum_number: i,
                    //     quorum_stake: Default::default(),
                    //     });
                    maybe_i.next();
                },
                (Some(&&i), Some(&&j)) if i > j => {
                    // handle quorum number added
                    quorums_added.push(QuorumsAdded{
                        quorum_number: j,
                        quorum_stake: chain_reader.get_quorum_stake_at_block(j, new_task_block).await.map_err(|e| Err(e.to_string()).into())?,
                        quorum_apk: G1Point::decode(chain_reader.get_quorum_apk_at_block(j, new_task_block).await.map_err(|e| Err(e.to_string()).into())?.abi_encode())?,
                    });

                    maybe_j.next();
                },
                (Some(&&i), None) => {
                    // handle quorum number removed
                    quorums_removed.push(i);
                    // quorums_apk_update.push(QuorumsApkUpdate{
                    //     quorum_number: i,
                    //     quorum_apk: Default::default(),
                    // });
                    // quorums_stake_update.push(QuorumsStakeUpdate{
                    //     quorum_number: i,
                    //     quorum_stake: Default::default(),
                    //     });
                    maybe_i.next();
                },
                (None, Some(&&j)) => {
                    // handle quorum number added
                    quorums_added.push(
                        QuorumsAdded{
                            quorum_number: j,
                            quorum_stake: chain_reader.get_quorum_stake_at_block(j, new_task_block).await.map_err(|e| Err(e.to_string()).into())?,
                            quorum_apk: G1Point::decode(chain_reader.get_quorum_apk_at_block(j, new_task_block).await.map_err(|e| Err(e.to_string()).into())?.abi_encode())?,
                        }
                    );
                    maybe_j.next();
                },
            }

        }



        let old_operators_stake = chain_reader
            .get_operators_stake_in_quorums_at_block(
                old_task_block,
                Bytes::decode(&mut &old_quorum_numbers.abi_encode()[..])?,
            ).await.map_err(|e| Err(e.to_string()).into())?;
        let new_operators_stake = chain_reader
            .get_operators_stake_in_quorums_at_block(new_task_block, Bytes::decode(&mut &new_quorum_numbers.abi_encode()[..])?).await.map_err(|e| Err(e.to_string()).into())?;

        let mut old_operators_avs_state = self.get_operators_avs_state_at_block(old_operators_stake, Bytes::decode(&mut &old_quorum_numbers.abi_encode()[..])?).await.values().cloned().collect::<Vec<_>>();
        let mut new_operators_avs_state = self.get_operators_avs_state_at_block(new_operators_stake, Bytes::decode(& mut &new_quorum_numbers.abi_encode()[..])?).await.values().cloned().collect::<Vec<_>>();
        
        old_operators_avs_state.sort_by_key(|v| v.operator_id);
        new_operators_avs_state.sort_by_key(|v| v.operator_id);

        let maybe_i = old_operators_avs_state.iter().peekable();
        let maybe_j = new_operators_avs_state.iter().peekable();

        let operators_removed: Vec<[u8; 32]> = Vec::new();
        let operators_added: Vec<OperatorsAdded> = Vec::new(); // Needs to be sorted
        let operators_quorum_count_update: Vec<OperatorsQuorumCountUpdate> = Vec::new();
        let operators_stake_update: Vec<OperatorsStakeUpdate> = Vec::new();

        loop {
            match (maybe_i.peek(), maybe_j.peek()){
                (Some(&&i), Some(&&j)) if i.operator_id == j.operator_id => {
                    // handle potential update

                    if i.stake_per_quorum.len() != j.stake_per_quorum.len(){
                        operators_quorum_count_update.push(OperatorsQuorumCountUpdate{
                            operator_id: j.operator_id,
                            quorum_count: j.stake_per_quorum.len().try_into().map_err(|e| Err("Only uint quorums").into())?,
                        });
                    }
                    let operator_stake_update = OperatorsStakeUpdate{
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                    };
                    for qn in quorums_removed {
                        operator_stake_update.quorum_for_stakes.push(qn);
                        operator_stake_update.quorum_stakes.push(Default::default());
                    }
                    for qn in quorums_added.iter().map(|x| x.quorum_number) {
                        operator_stake_update.quorum_for_stakes.push(qn);
                        let stake = j.stake_per_quorum.get(&qn).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); &Default::default()});
                        operator_stake_update.quorum_stakes.push(stake.try_into().map_err(|e| Err("Only uint96 stakes").into())?)
                    }
                    for qn in quorums_common{
                        let stake_old = i.stake_per_quorum.get(&qn).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); &Default::default()});
                        let stake_new = j.stake_per_quorum.get(&qn).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); &Default::default()});
                        if stake_old != stake_new {
                            operator_stake_update.quorum_for_stakes.push(qn);
                            operator_stake_update.quorum_stakes.push(stake_new.try_into().map_err(|e| Err("Only uint96 stakes").into())?)
                        }
                    }
                    if !operator_stake_update.quorum_for_stakes.is_empty(){
                    operators_stake_update.push(operator_stake_update);}

                    maybe_i.next(); maybe_j.next();
                },
                (Some(&&i), Some(&&j)) if i.operator_id < j.operator_id => {
                    // handle operator removed
                    operators_removed.push(i.operator_id);
                    maybe_i.next();
                },
                (Some(&&i), Some(&&j)) if i.operator_id > j.operator_id => {
                    // handle quorum number added

                    let operator_added = OperatorsAdded{
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                        quorum_count: j.stake_per_quorum.len().try_into().map_err(|e| Err("Only uint quorums").into())?,
                    };

                    for qn in j.stake_per_quorum.keys(){
                        operator_added.quorum_for_stakes.push(*qn);
                        let stake = j.stake_per_quorum.get(qn).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); &Default::default()});
                        operator_added.quorum_stakes.push(stake.try_into().map_err(|e| Err("Only uint96 stakes").into())?)
                    }

                    operators_added.push(operator_added);

                    maybe_j.next();
                },
                (Some(&&i), None) => {
                    // handle quorum number removed
                    operators_removed.push(i.operator_id);
                    maybe_i.next();
                },
                (None, Some(&&j)) => {
                    // handle operator added
                    
                    let operator_added = OperatorsAdded{
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                        quorum_count: j.stake_per_quorum.len().try_into().map_err(|e| Err("Only uint quorums").into())?,
                    };

                    for qn in j.stake_per_quorum.keys(){
                        operator_added.quorum_for_stakes.push(*qn);
                        let stake = j.stake_per_quorum.get(qn).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); &Default::default()});
                        operator_added.quorum_stakes.push(stake.try_into().map_err(|e| Err("Only uint96 stakes").into())?)
                    }

                    operators_added.push(operator_added);
                    maybe_j.next();
                },
            }

        }

        // quorums removed
        // A vec of qourums removed
        // Removes the corresponding quorum to apk map entry
        // AND Removes the corresponding quorum to stakes map entry
        // Note whenever a quorum is removed, all the operator's stake map entry for that quorum is also removed
        // Note whenever a quorum is removed, all the operator's quorumCount map entry is also updated
        // Note whenever a quorum is removed and an operator that was in it isn't is other quorums anymore either
        // then such operators are also removed from state

        // quorums added
        // A Vec<(, )> of quorums added and their quorum apks

        // The one above and the one below can be merged
        // Nah... Let em be

        // quorums modified
        // A Vec<(, )> of quorums added and their changed quorums apks

        // OperatorIdPubKey can be updated by the aggregator
        // The aggregator just needs to provide the g1PubKeys
        // The corrseponding entries will be checked against the sorted list of "Added OperatorIds" provided from here

        // Provide a sorted list of new operatorIds (to the state)
        // Mentioed below

        // we have QuorumToStakes update - club with QuorumApks updates above

        // From operator state
        // We need the operators that have left the state
        // We need the operators that have been added to the state in sorted order
        // We need the operators whose stakes or quorum count have been modified


        // This contains all operators that must be removed from associated quorum storage
        // If a quorum is removed

        let operators_state_changed = 
        !quorums_removed.is_empty() || !quorums_added.is_empty() || !quorums_apk_update.is_empty() || !quorums_stake_update.is_empty() || !operators_removed.is_empty() || !operators_added.is_empty() || !operators_stake_update.is_empty() || !operators_quorum_count_update.is_empty() || (task.quorum_threshold_percentage != task.last_completed_task_quorum_threshold_percentage);

        let operator_state_info = OperatorStateInfo {
            operators_state_changed: operators_state_changed,
            operators_state_provided: true,
            quorums_removed: quorums_removed,
            quorums_added: quorums_added,
            quorums_stake_update: quorums_stake_update,
            quorums_apk_update: quorums_apk_update,
            operators_removed: operators_removed,
            operators_added: operators_added,
            operators_stake_update: operators_stake_update,
            operators_quorum_count_update: operators_quorum_count_update,
        };
        Ok(operator_state_info)
    }


    pub async fn get_operators_avs_state_at_block(
        &self,
        operators_stakes_in_quorums: Vec<Vec<OperatorStateRetriever::Operator>>,
        quorum_nums: Bytes,
    ) -> HashMap<FixedBytes<32>, CustomOperatorAvsState> {
        let mut operators_avs_state: HashMap<FixedBytes<32>, CustomOperatorAvsState> = HashMap::new();

        if operators_stakes_in_quorums.len() != quorum_nums.len() {
            // throw error
        }

        for (quorum_id, quorum_num) in quorum_nums.iter().enumerate() {
            for operator in &operators_stakes_in_quorums[quorum_id] {
                let stake_per_quorum = HashMap::new();
                let avs_state = operators_avs_state
                    .entry(FixedBytes(*operator.operatorId))
                    .or_insert_with(|| CustomOperatorAvsState {
                        operator_id: *operator.operatorId,
                        stake_per_quorum: stake_per_quorum,
                    });
                avs_state
                    .stake_per_quorum
                    .insert(*quorum_num, U256::from(operator.stake));
            }
        }

        return operators_avs_state;
    }

    pub(crate) fn operator_id(&self) -> OperatorId {
        self.bls_keypair.operator_id()
    }

    #[instrument(skip_all)]
    pub(crate) async fn get_status(&self) -> eyre::Result<OperatorStatus> {
        let el_status = self
            .el_contracts
            .is_operator_registered(self.client.address())
            .await?;

        let id = self.avs_contracts.operator_id().await?;

        Ok(OperatorStatus {
            eth_address: self.client.address(),
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
        let status = self
            .el_contracts
            .is_operator_registered(self.client.address())
            .await?;

        if !status {
            info!(
                "Registering Operator {:x} with EigenLayer",
                self.client.address()
            );

            self.el_contracts
                .register_as_operator_with_el(self.client.address())
                .await?;

            info!("Sucessfully registered with EigenLayer")
        } else {
            info!("Operator already registered in EigenLayer");
        }

        Ok(())
    }

    #[instrument(skip_all)]
    pub(crate) async fn opt_in_avs(&self) -> eyre::Result<()> {
        if self.avs_contracts.operator_id().await?.is_some() {
            info!("Operator already opt-in AVS");
        } else {
            info!("Registering Operator {:x} with AVS", self.client.address());
            let sig_params = self.el_contracts.get_delegation_signature_params().await?;
            self.avs_contracts
                .register_with_avs(&self.bls_keypair, sig_params)
                .await?;
            let id = self
                .avs_contracts
                .operator_id()
                .await?
                .expect("should have operator id after success register trx");
            info!("Sucessfully registered with AVS with id {:x}", id);
        }
        Ok(())
    }

    #[instrument(skip_all)]
    pub(crate) async fn opt_out_avs(&self) -> eyre::Result<()> {
        if self.avs_contracts.operator_id().await?.is_some() {
            self.avs_contracts.deregister_with_avs().await?;
            info!("Operator opted out with AVS sucessfully");
        } else {
            info!("Operator not opt in with AVS");
        }

        Ok(())
    }
}
