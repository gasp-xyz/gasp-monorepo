use crate::chainio::{avs::AvsContracts, build_clients, SourceClient, TargetClient};
use crate::cli::CliArgs;

use bindings::{
    finalizer_task_manager::{
        FinalizerTaskManagerCalls, FinalizerTaskManagerEvents, NewOpTaskCreatedFilter,
        NewRdTaskCreatedFilter, OpTaskCompletedFilter, Operator as TMOperator, OperatorStateInfo,
        OperatorsAdded, OperatorsQuorumCountUpdate, OperatorsStakeUpdate, QuorumsAdded,
        QuorumsApkUpdate, QuorumsStakeUpdate, RdTaskCompletedFilter,
    },
    gasp_multi_rollup_service::GaspMultiRollupService,
    shared_types::{G1Point, G2Point, OpTask, OpTaskResponse, RdTask, RdTaskResponse},
};
use ethers::providers::{Middleware, PendingTransaction, SubscriptionStream};
use ethers::{
    abi::AbiDecode,
    contract::{stream, EthEvent, LogMeta},
    providers::StreamExt,
    types::{Address, Bytes, Filter, U256},
};

use ethers::abi::AbiEncode;
use eyre::eyre;
use serde::Serialize;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Hash, Keccak256, Zero};
use sp_runtime::{generic, OpaqueExtrinsic};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio::try_join;
use tracing::{debug, error, info, instrument};

type QuorumNum = u8;

#[derive(Clone)]
pub struct CustomOperatorAvsState {
    pub operator_id: [u8; 32],
    pub stake_per_quorum: HashMap<QuorumNum, u128>,
}

#[derive(Debug)]
pub struct Syncer {
    pub source_client: Arc<SourceClient>,
    pub target_client: Arc<TargetClient>,
    avs_contracts: AvsContracts,
    gasp_service_contract: GaspMultiRollupService<TargetClient>,
    root_gasp_service_contract: Option<GaspMultiRollupService<TargetClient>>,
}
impl Syncer {
    #[instrument(name = "create_syncer", skip_all)]
    pub async fn from_cli(cfg: &CliArgs) -> eyre::Result<Arc<Self>> {
        let (source_client, target_client, maybe_root_target_client) = build_clients(cfg).await?;
        let (source_client, target_client) = (Arc::new(source_client), Arc::new(target_client));
        let avs_contracts = AvsContracts::build(cfg, source_client.clone()).await?;
        let gasp_service_contract =
            GaspMultiRollupService::new(cfg.gasp_service_addr, target_client.clone());
        let root_gasp_service_contract = if cfg.reinit || cfg.only_reinit {
            let root_target_client = Arc::new(maybe_root_target_client.expect("should work here"));
            Some(GaspMultiRollupService::new(
                cfg.gasp_service_addr,
                root_target_client.clone(),
            ))
        } else {
            None
        };

        Ok(Arc::new(Self {
            source_client,
            target_client,
            avs_contracts,
            gasp_service_contract,
            root_gasp_service_contract,
        }))
    }

    #[instrument(skip_all)]
    pub async fn sync(self: Arc<Self>, cfg: &CliArgs) -> eyre::Result<()> {
        let latest_completed_op_task_number = self
            .gasp_service_contract
            .latest_completed_op_task_number()
            .await?;
        let mut latest_completed_op_task_created_block = self
            .gasp_service_contract
            .latest_completed_op_task_created_block()
            .await?;
        // let mut task_number_expected = latest_completed_task_number + 1;

        if latest_completed_op_task_created_block.is_zero() && !cfg.push_first_init {
            tracing::error!("target uninit and push_first_init set to false");
            return Err(eyre!("target uninit and push_first_init set to false"));
        }

        let evs = self
            .clone()
            .avs_contracts
            .source_response_stream(latest_completed_op_task_created_block);
        let mut stream: stream::EventStream<'_, _, (FinalizerTaskManagerEvents, LogMeta), _> =
            evs.subscribe_with_meta().await?;

        // event stream does not finish with `None` after websocket closure, use block subscription for it
        let mut blocks: SubscriptionStream<'_, _, _> =
            self.avs_contracts.ws_client.subscribe_blocks().await?;

        loop {
            select! {
                Some(stream_event) = stream.next() => match stream_event {
                    Ok((stream_event, log)) => {
                        match stream_event {
                            FinalizerTaskManagerEvents::OpTaskCompletedFilter(event) => {
                                let txn_hash = log.transaction_hash;
                                let txn = self.clone().avs_contracts.ws_client.get_transaction(txn_hash).await?
                                .ok_or_else(
                                    || {tracing::error!("missing expected txn {:?}", txn_hash);
                                    eyre!("missing expected txn {:?}", txn_hash)}
                                )?
                                ;
                                println!("{:?}", txn);
                                let call = match FinalizerTaskManagerCalls::decode(txn.input)?{
                                    FinalizerTaskManagerCalls::RespondToOpTask(c) => c,
                                    _ => {
                                        tracing::error!("wrong call decoded");
                                        return Err(eyre!("wrong call decoded"))
                                    }
                                };
                                println!("{:?}", call);
                                println!("{:?}", call.task.clone());
                                println!("encoded - {:?}", call.task.clone().encode());
                                println!("encoded - {:?}", array_bytes::bytes2hex("0x",call.task.clone().encode()));
                                let task_hash = Keccak256::hash(vec![0u8;31].into_iter().chain(vec![32u8]).chain(
                                    call.task.clone().encode().into_iter()
                                ).collect::<Vec<_>>().as_ref());
                                println!("{:?}", task_hash);
                                println!("{:?}", task_hash == call.task_response.reference_task_hash.into());

                                if task_hash != call.task_response.reference_task_hash.into() {
                                    tracing::error!("task_hash mismatch {:?}", task_hash);
                                    return Err(eyre!("task_hash mismatch {:?}", task_hash))
                                }

                                let operators_state_info = self.clone().get_operators_state_info(call.task.clone()).await?;
                                println!("{:?}", operators_state_info);
                                let operators_state_info_hash = Keccak256::hash(vec![0u8;31].into_iter().chain(vec![32u8]).chain(
                                    operators_state_info.clone().encode().into_iter()
                                ).collect::<Vec<_>>().as_ref());
                                println!("{:?}", operators_state_info_hash);
                                println!("{:?}", operators_state_info_hash == call.task_response.operators_state_info_hash.into());

                                if operators_state_info_hash != call.task_response.operators_state_info_hash.into() {
                                    tracing::error!("operators_state_info_hash mismatch {:?}", operators_state_info_hash);
                                    return Err(eyre!("operators_state_info_hash mismatch {:?}", operators_state_info_hash))
                                }

                                if latest_completed_op_task_created_block !=0 && latest_completed_op_task_created_block < call.task.last_completed_op_task_created_block {
                                    tracing::error!("missing expected task response {:?}", latest_completed_op_task_created_block);
                                    return Err(eyre!("missing expected task response {:?}", latest_completed_op_task_created_block))
                                }

                                // This branch is to account for the case where
                                // a task is completed in a block and another task is created
                                // in the same block and then that one is also completed in the same block
                                if latest_completed_op_task_created_block > call.task.last_completed_op_task_created_block {
                                    continue;
                                }

                                if call.task.last_completed_op_task_created_block + 14400 <= call.task.task_created_block {
                                    tracing::error!("stale old state {:?}", latest_completed_op_task_created_block);
                                    return Err(eyre!("stale old state {:?}", latest_completed_op_task_created_block))
                                }

                                let update_txn = self.clone().gasp_service_contract.process_eigen_op_update(call.task.clone(), call.task_response, call.non_signer_stakes_and_signature, operators_state_info);
                                println!("{:?}", update_txn);
                                let update_txn_pending = update_txn.send().await;
                                println!("{:?}", update_txn_pending);
                                let update_txn_receipt = update_txn_pending?.await?;
                                println!("{:?}", update_txn_receipt);

                                latest_completed_op_task_created_block = call.task.task_created_block;
                            },
                            FinalizerTaskManagerEvents::RdTaskCompletedFilter(event) => {
                                let txn_hash = log.transaction_hash;
                                let txn = self.clone().avs_contracts.ws_client.get_transaction(txn_hash).await?
                                .ok_or_else(
                                    || {tracing::error!("missing expected txn {:?}", txn_hash);
                                    eyre!("missing expected txn {:?}", txn_hash)}
                                )?
                                ;
                                println!("{:?}", txn);
                                let call = match FinalizerTaskManagerCalls::decode(txn.input)?{
                                    FinalizerTaskManagerCalls::RespondToRdTask(c) => c,
                                    _ => {
                                        tracing::error!("wrong call decoded");
                                        return Err(eyre!("wrong call decoded"))
                                    }
                                };
                                println!("{:?}", call);
                                println!("{:?}", call.task.clone());
                                println!("encoded - {:?}", call.task.clone().encode());
                                println!("encoded - {:?}", array_bytes::bytes2hex("0x",call.task.clone().encode()));
                                let task_hash = Keccak256::hash(vec![0u8;31].into_iter().chain(vec![32u8]).chain(
                                    call.task.clone().encode().into_iter()
                                ).collect::<Vec<_>>().as_ref());
                                println!("{:?}", task_hash);
                                println!("{:?}", task_hash == call.task_response.reference_task_hash.into());

                                if task_hash != call.task_response.reference_task_hash.into() {
                                    tracing::error!("task_hash mismatch {:?}", task_hash);
                                    return Err(eyre!("task_hash mismatch {:?}", task_hash))
                                }

                                if latest_completed_op_task_created_block != call.task.last_completed_op_task_created_block {
                                    tracing::error!("latest_completed_op_task_created_block mismatch {:?}", latest_completed_op_task_created_block);
                                    return Err(eyre!("latest_completed_op_task_created_block mismatch {:?}", latest_completed_op_task_created_block))
                                }

                                if call.task.last_completed_op_task_created_block + 14400 <= call.task.task_created_block {
                                    tracing::error!("stale old state {:?}", latest_completed_op_task_created_block);
                                    return Err(eyre!("stale old state {:?}", latest_completed_op_task_created_block))
                                }

                                let update_txn = self.clone().gasp_service_contract.process_eigen_rd_update(call.task.clone(), call.task_response, call.non_signer_stakes_and_signature);
                                println!("{:?}", update_txn);
                                let update_txn_pending = update_txn.send().await;
                                println!("{:?}", update_txn_pending);
                                let update_txn_receipt = update_txn_pending?.await?;
                                println!("{:?}", update_txn_receipt);

                            },
                            _ => return Err(eyre!("Got unexpected stream event"))
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

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn reinit(self: Arc<Self>, cfg: &CliArgs) -> eyre::Result<()> {
        let latest_completed_op_task_created_block = self
            .gasp_service_contract
            .latest_completed_op_task_created_block()
            .await?;
        let latest_completed_op_task_quorum_numbers =
            self.gasp_service_contract.quorum_numbers().await?;
        let latest_completed_op_task_quorum_threshold_percentage = self
            .gasp_service_contract
            .quorum_threshold_percentage()
            .await?;

        // TODO!!
        // Get the latest block from source chain and query the following three with it!
        let task_num = self
            .clone()
            .avs_contracts
            .task_manager
            .last_completed_op_task_num()
            .await?;
        let block_num = self
            .clone()
            .avs_contracts
            .task_manager
            .last_completed_op_task_created_block()
            .await?;
        let latest_pending_state_hash = self
            .clone()
            .avs_contracts
            .task_manager
            .latest_pending_state_hash()
            .await?;

        // Unfortunately latestTaskNum and LastCompletedTaskNum both start at 0
        // So if lastCompletedTaskNum is 0 then check if it was infact completed
        if task_num.is_zero() {
            let task_status = self
                .clone()
                .avs_contracts
                .task_manager
                .id_to_task_status(0u8, task_num)
                .await?;
            if task_status != 4 {
                tracing::error!(
                    "no completed tasks for reinit {:?}",
                    latest_completed_op_task_created_block
                );
                return Err(eyre!(
                    "no completed tasks for reinit {:?}",
                    latest_completed_op_task_created_block
                ));
            }
        }

        let mut block_events: Vec<NewOpTaskCreatedFilter> = self
            .clone()
            .avs_contracts
            .task_manager
            .event_with_filter(
                Filter::new()
                    .event(&NewOpTaskCreatedFilter::abi_signature())
                    .select(u64::from(block_num)),
            )
            .query()
            .await?;

        let mut last_task = match block_events.pop() {
            Some(e) if e.task_index == task_num => e.task,
            _ => {
                tracing::error!("task not in events for reinit {:?}", task_num);
                return Err(eyre!("task not in events for reinit {:?}", task_num));
            }
        };

        last_task.last_completed_op_task_created_block = latest_completed_op_task_created_block;
        last_task.last_completed_op_task_quorum_numbers = latest_completed_op_task_quorum_numbers;
        last_task.last_completed_op_task_quorum_threshold_percentage =
            latest_completed_op_task_quorum_threshold_percentage;

        let block_events: Vec<OpTaskCompletedFilter> = self
            .clone()
            .avs_contracts
            .task_manager
            .event_with_filter(
                Filter::new()
                    .event(&OpTaskCompletedFilter::abi_signature())
                    .from_block(u64::from(block_num)),
            )
            .query()
            .await?;

        if block_events.len().is_zero() {
            tracing::error!("missing expected task response {:?}", task_num);
            return Err(eyre!("missing expected task response {:?}", task_num));
        }

        if block_events[0].task_index != task_num {
            tracing::error!("missing expected task response {:?}", task_num);
            return Err(eyre!("missing expected task response {:?}", task_num));
        }

        let task_response = block_events[0].task_response.clone();

        let operators_state_info = self
            .clone()
            .get_operators_state_info(last_task.clone())
            .await?;

        let reinit_txn = self
            .clone()
            .root_gasp_service_contract
            .clone()
            .expect("should work in reinit")
            .process_eigen_reinit(last_task, operators_state_info, latest_pending_state_hash);
        println!("{:?}", reinit_txn);
        let reinit_txn_pending = reinit_txn.send().await;
        println!("{:?}", reinit_txn_pending);
        let reinit_txn_receipt = reinit_txn_pending?.await?;
        println!("{:?}", reinit_txn_receipt);

        Ok(())
    }

    pub(crate) async fn get_operators_state_info(
        self: Arc<Self>,
        task: OpTask,
    ) -> eyre::Result<OperatorStateInfo> {
        // We assume that the quorumNumbers are alteast unique even if not sorted

        let old_quorum_threshold_percentage = if task.last_completed_op_task_created_block == task.task_created_block{
            Default::default()
        }else{
            task.last_completed_op_task_quorum_threshold_percentage
        };
        let new_quorum_threshold_percentage = task.quorum_threshold_percentage;
        
        let mut old_quorum_numbers = if task.last_completed_op_task_created_block == task.task_created_block{
            Default::default()
        }else{
            task
            .last_completed_op_task_quorum_numbers
            .into_iter()
            .collect::<Vec<u8>>()
        };
        let mut new_quorum_numbers = task.quorum_numbers.into_iter().collect::<Vec<u8>>();
        old_quorum_numbers.sort();
        new_quorum_numbers.sort();

        let old_task_block = if task.last_completed_op_task_created_block == task.task_created_block{
            Default::default()
        }else{
            task.last_completed_op_task_created_block
        };
        let new_task_block = task.task_created_block;

        info!("old_task_block: {:?}", old_task_block);
        info!("new_task_block: {:?}", new_task_block);

        let registry_coordinator_address = &self.avs_contracts.registry_coordinator_address;
        let registry_coordinator = &self.avs_contracts.registry;
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
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();
        let mut new_operators_avs_state = self
            .get_operators_avs_state_at_block(
                new_operators_stake,
                new_quorum_numbers.clone().into(),
            )
            .await
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
                            .get(&qn)
                            .map(u128::to_owned)
                            .unwrap_or_else(|| {
                                error!("Failed to get operator quorum stake");
                                Default::default()
                            });
                        let stake_new = j
                            .stake_per_quorum
                            .get(&qn)
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
            || (old_quorum_threshold_percentage
                != new_quorum_threshold_percentage)
            || (old_quorum_numbers != new_quorum_numbers);

        let operator_state_info = OperatorStateInfo {
            operators_state_changed: operators_state_changed,
            quorums_removed: quorums_removed,
            quorums_added: quorums_added,
            quorums_stake_update: quorums_stake_update,
            quorums_apk_update: quorums_apk_update,
            operators_removed: operators_removed,
            operators_added: operators_added,
            operators_stake_update: operators_stake_update,
            operators_quorum_count_update: operators_quorum_count_update,
        };

        info!("operator_state_info: {:?}", operator_state_info);
        Ok(operator_state_info)
    }

    pub async fn get_operators_avs_state_at_block(
        &self,
        operators_stakes_in_quorums: Vec<Vec<TMOperator>>,
        quorum_nums: Bytes,
    ) -> HashMap<H256, CustomOperatorAvsState> {
        let mut operators_avs_state: HashMap<H256, CustomOperatorAvsState> = HashMap::new();

        if operators_stakes_in_quorums.len() != quorum_nums.len() {
            // throw error
        }

        for (quorum_id, quorum_num) in quorum_nums.iter().enumerate() {
            for operator in &operators_stakes_in_quorums[quorum_id] {
                let stake_per_quorum = HashMap::new();
                let avs_state = operators_avs_state
                    .entry(H256::from(operator.operator_id))
                    .or_insert_with(|| CustomOperatorAvsState {
                        operator_id: operator.operator_id,
                        stake_per_quorum: stake_per_quorum,
                    });
                avs_state
                    .stake_per_quorum
                    .insert(*quorum_num, u128::from(operator.stake));
            }
        }

        operators_avs_state
    }
}
