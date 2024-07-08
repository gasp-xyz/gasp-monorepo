use crate::chainio::{avs::AvsContracts, build_clients, SourceClient, TargetClient};
use crate::cli::CliArgs;

use bindings::{
    finalizer_task_manager::{NewTaskCreatedFilter, TaskCompletedFilter, Operator as TMOperator,
        OperatorStateInfo, QuorumsAdded, QuorumsStakeUpdate, QuorumsApkUpdate, OperatorsAdded, OperatorsQuorumCountUpdate, OperatorsStakeUpdate, FinalizerTaskManagerCalls},
    shared_types::{G1Point, G2Point, Task, TaskResponse},
    gasp_multi_rollup_service::GaspMultiRollupService
    
};
use ethers::providers::{Middleware, PendingTransaction, SubscriptionStream};
use ethers::{
    contract::{EthEvent, stream, LogMeta},
    providers::StreamExt,
    types::{Address, U256, Bytes, Filter}, abi::AbiDecode
};

use serde::Serialize;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Keccak256, Hash, Zero};
use sp_runtime::{generic, OpaqueExtrinsic};
use std::sync::Arc;
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio::try_join;
use tracing::{debug, error, info, instrument};
use std::collections::HashMap;
use ethers::abi::AbiEncode;
use eyre::eyre;

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
        let gasp_service_contract = GaspMultiRollupService::new(cfg.gasp_service_addr, target_client.clone());
        let root_gasp_service_contract = if cfg.reinit || cfg.only_reinit{
        let root_target_client = Arc::new(maybe_root_target_client.expect("should work here"));
        Some(GaspMultiRollupService::new(cfg.gasp_service_addr, root_target_client.clone()))
        } else {None};

        Ok(Arc::new(Self {
            source_client,
            target_client,
            avs_contracts,
            gasp_service_contract,
            root_gasp_service_contract
        }))
    }

    #[instrument(skip_all)]
    pub async fn sync(self: Arc<Self>, cfg: &CliArgs) -> eyre::Result<()> {

        let latest_completed_task_number = self.gasp_service_contract.latest_completed_task_number().await?;
        let mut latest_completed_task_created_block = self.gasp_service_contract.latest_completed_task_created_block().await?;
        // let mut task_number_expected = latest_completed_task_number + 1;

        if latest_completed_task_created_block.is_zero() && !cfg.push_first_init{
            tracing::error!("target uninit and push_first_init set to false");
            return Err(eyre!("target uninit and push_first_init set to false"))
        }

        let evs = self.clone().avs_contracts.source_response_stream(latest_completed_task_created_block);
        let mut stream: stream::EventStream<'_, _, (TaskCompletedFilter, LogMeta), _> =
            evs.subscribe_with_meta().await?;

        // event stream does not finish with `None` after websocket closure, use block subscription for it
        let mut blocks: SubscriptionStream<'_, _, _> =
            self.avs_contracts.ws_client.subscribe_blocks().await?;

        loop {
            select! {
                Some(event) = stream.next() => match event {
                    Ok((event, log)) => {
                        let txn_hash = log.transaction_hash;
                        let txn = self.clone().avs_contracts.ws_client.get_transaction(txn_hash).await?
                        .ok_or_else(
                            || {tracing::error!("missing expected txn {:?}", txn_hash);
                            eyre!("missing expected txn {:?}", txn_hash)}
                        )?
                        ;
                        println!("{:?}", txn);
                        let call = match FinalizerTaskManagerCalls::decode(txn.input)?{
                            FinalizerTaskManagerCalls::RespondToTask(c) => c,
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

                        let operators_state_info = self.clone().get_operators_state_info(call.task.clone()).await?;
                        println!("{:?}", operators_state_info);
                        let operators_state_info_hash = Keccak256::hash(vec![0u8;31].into_iter().chain(vec![32u8]).chain(
                            operators_state_info.clone().encode().into_iter()
                        ).collect::<Vec<_>>().as_ref());
                        println!("{:?}", operators_state_info_hash);
                        println!("{:?}", operators_state_info_hash == call.task_response.operators_state_info_hash.into());
                        
                        if latest_completed_task_created_block < call.task.last_completed_task_created_block {
                            tracing::error!("missing expected task response {:?}", latest_completed_task_created_block);
                            return Err(eyre!("missing expected task response {:?}", latest_completed_task_created_block))
                        }

                        // This branch is to account for the case where 
                        // a task is completed in a block and another task is created
                        // in the same block and then that one is also completed in the same block
                        if latest_completed_task_created_block > call.task.last_completed_task_created_block {
                            continue;
                        }

                        if call.task.last_completed_task_created_block + 14400 <= call.task.task_created_block {
                            tracing::error!("stale old state {:?}", latest_completed_task_created_block);
                            return Err(eyre!("stale old state {:?}", latest_completed_task_created_block))
                        }
                        
                        let update_txn = self.clone().gasp_service_contract.process_eigen_update(call.task.clone(), call.task_response, call.non_signer_stakes_and_signature_for_old_state, operators_state_info);
                        println!("{:?}", update_txn);
                        let update_txn_pending = update_txn.send().await;
                        println!("{:?}", update_txn_pending);
                        let update_txn_receipt = update_txn_pending?.await?;
                        println!("{:?}", update_txn_receipt);

                        latest_completed_task_created_block = call.task.task_created_block;
                        // task_number_expected = call.task.task_created_block

                        // return Ok(())
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

        let latest_completed_task_created_block = self.gasp_service_contract.latest_completed_task_created_block().await?;
        let latest_completed_task_quorum_numbers = self.gasp_service_contract.quorum_numbers().await?;
        let latest_completed_task_quorum_threshold_percentage = self.gasp_service_contract.quorum_threshold_percentage().await?;

        let task_num = self.clone().avs_contracts.task_manager.last_completed_task_num().await?;
        let block_num = self.clone().avs_contracts.task_manager.last_completed_task_created_block().await?;

        // Unfortunately latestTaskNum and LastCompletedTaskNum both start at 0
        // So if lastCompletedTaskNum is 0 then check if it was infact completed 
        if task_num.is_zero() {
            let task_status = self.clone().avs_contracts.task_manager.index_to_task_status(task_num).await?;
            if task_status != 3 {
                tracing::error!("no completed tasks for reinit {:?}", latest_completed_task_created_block);
                return Err(eyre!("no completed tasks for reinit {:?}", latest_completed_task_created_block))
            }
        }

        let mut block_events: Vec<NewTaskCreatedFilter> = self.clone().avs_contracts.task_manager.event_with_filter(Filter::new().event(&NewTaskCreatedFilter::abi_signature()).select(u64::from(block_num))).query().await?;

        let mut last_task = match block_events.pop(){
            Some(e) if e.task_index == task_num => {
                e.task
            }
            _ => {
                tracing::error!("task not in events for reinit {:?}", task_num);
                return Err(eyre!("task not in events for reinit {:?}", task_num))
            }
        };

        last_task.last_completed_task_created_block = latest_completed_task_created_block;
        last_task.last_completed_task_quorum_numbers = latest_completed_task_quorum_numbers;
        last_task.last_completed_task_quorum_threshold_percentage = latest_completed_task_quorum_threshold_percentage;

        let block_events: Vec<TaskCompletedFilter>  = self.clone().avs_contracts.task_manager.event_with_filter(Filter::new().event(&TaskCompletedFilter::abi_signature()).from_block(u64::from(block_num))).query().await?;

        if block_events.len().is_zero(){
            tracing::error!("missing expected task response {:?}", task_num);
            return Err(eyre!("missing expected task response {:?}", task_num))
        }

        if block_events[0].task_index != task_num{
            tracing::error!("missing expected task response {:?}", task_num);
            return Err(eyre!("missing expected task response {:?}", task_num))
        }

        let task_response = block_events[0].task_response.clone();

        // let evs = self.clone().avs_contracts.source_response_stream(block_num);
        // let mut stream: stream::EventStream<'_, _, (TaskRespondedFilter, LogMeta), _> =
        //     evs.subscribe_with_meta().await?;

        // // event stream does not finish with `None` after websocket closure, use block subscription for it
        // let mut blocks: SubscriptionStream<'_, _, _> =
        //     self.avs_contracts.ws_client.subscribe_blocks().await?;

        // let task_response = TaskResponse::default();

        // loop {
        //     select! {
        //         Some(event) = stream.next() => match event {
        //             Ok((event, log)) => {
        //                 if task_num != event.taskIndex => {
        //                     tracing::error!("missing expected task response {:?}", task_num);
        //                     return Err(eyre!("missing expected task response {:?}", task_num))
        //                 }
        //                 task_response = event.taskResponse;
        //                 break
        //             }
        //             Err(e) => tracing::error!("EthWs subscription error {:?}", e),
        //         },
        //         block = blocks.next() => {
        //             if block.is_none() {
        //                 tracing::error!("missing expected task response no more blocks {:?}", task_num);
        //                 return Err(eyre!("missing expected task response no more blocks {:?}", task_num))
        //             }
        //         }
        //     }
        // }

        let operators_state_info = self.clone().get_operators_state_info(last_task.clone()).await?;

        let reinit_txn = self.clone().root_gasp_service_contract.clone().expect("should work in reinit").process_eigen_reinit(last_task, task_response, operators_state_info);
        println!("{:?}", reinit_txn);
        let reinit_txn_pending = reinit_txn.send().await;
        println!("{:?}", reinit_txn_pending);
        let reinit_txn_receipt = reinit_txn_pending?.await?;
        println!("{:?}", reinit_txn_receipt);

        Ok(())
    }

    // let latest_completed_task_number = self.gasp_service_contract.get_latest_completed_task_number().await?;
    //     let latest_completed_task_created_block = self.gasp_service_contract.get_latest_completed_task_created_block().await?;

    //     let is_stalled = self.gasp_service_contract.get_is_stalled().await?;

    //     if is_stalled{
    //         // get the Operator_State_Info from latest_completed_task_created_block on target to the lastCompletedTaskCreatedBlock on source
    //         // Use root keys to force the state update and unstall
    //         handle_stall().await?;
    //     }

    //     // If the syncer passes an update and it fails... stop the syncer
    //     // Maybe even pause syncing on contract... But this is only necessary if the syncer is decentralized... But the syncer doesn't need to be decentralized
    //     // So the only expected reason for a stall is update is too big
    //     continue_sync().await?;

    //     let evs = self.clone().avs_contracts.source_stream();
    //     let mut stream: stream::EventStream<'_, _, (NewTaskCreatedFilter, LogMeta), _> =
    //         evs.subscribe_with_meta().await?;

    //     // event stream does not finish with `None` after websocket closure, use block subscription for it
    //     let mut blocks: SubscriptionStream<'_, _, _> =
    //         self.avs_contracts.ws_client.subscribe_blocks().await?;

    //     loop {
    //         select! {
    //             Some(event) = stream.next() => match event {
    //                 Ok((event, log)) => {
    //                     debug!("Got new task at: {:?}", log);
    //                     PendingTransaction::new(log.transaction_hash, self.client.provider()).await?;
    //                     let event_clone = event.clone();
    //                     let self_clone = self.clone();
    //                     let execute_block_join_handle = tokio::spawn(async move {
    //                         info!("Executing a Block for task: {:?}", event_clone);
    //                         self_clone.execute_block(event_clone.task.block_number.as_u32()).await

    //                     });
    //                     let event_clone = event.clone();
    //                     let self_clone = self.clone();
    //                     let get_operators_state_info_hash_handle = tokio::spawn(async move {
    //                         info!("Get operators state hash: {:?}", event_clone);
    //                         self_clone.get_operators_state_info_hash(event_clone.task).await
    //                     });
    //                     let (proofs, operators_state_info_hash) = try_join!(execute_block_join_handle, get_operators_state_info_hash_handle)?;
    //                     let (proofs, operators_state_info_hash) = (proofs?, operators_state_info_hash?);
    //                     debug!("Operators State Info Hash {:?}", operators_state_info_hash);
    //                     debug!("Block executed successfully {:?}", proofs);
    //                     let payload = TaskResponse {
    //                         reference_task_index: event.task_index,
    //                         reference_task_hash: Keccak256::hash(event.task.clone().encode().as_ref()).into(),
    //                         operators_state_info_hash: operators_state_info_hash,
    //                         block_hash: proofs.0.as_fixed_bytes().to_owned(),
    //                         storage_proof_hash: proofs.1.as_fixed_bytes().to_owned(),
    //                         pending_state_hash: proofs.2.as_fixed_bytes().to_owned(),
    //                     };
    //                     let response = self
    //                         .rpc
    //                         .send_task_response(payload, &self.bls_keypair)
    //                         .await;
    //                     match response {
    //                         Ok(r) => match r.error_for_status_ref() {
    //                             Err(e) => error!("{} - {}", e, r.text().await?),
    //                             Ok(_) => info!("Task finished successfuly and sent to AVS service"),
    //                         },
    //                         Err(e) => error!("{}", e),
    //                     }
    //                 }
    //                 Err(e) => tracing::error!("EthWs subscription error {:?}", e),
    //             },
    //             block = blocks.next() => {
    //                 if block.is_none() {
    //                     break
    //                 }
    //             }
    //         }
    //     }

    //     // ws provider has internal reconnect, but if it fails we are done
    //     Ok(())
    // }


    pub(crate) async fn get_operators_state_info(
        self: Arc<Self>,
        task: Task,
    ) -> eyre::Result<OperatorStateInfo> {

        // We assume that the quorumNumbers are alteast unique even if not sorted
        let mut old_quorum_numbers = task.last_completed_task_quorum_numbers.into_iter().collect::<Vec<u8>>();
        let mut new_quorum_numbers = task.quorum_numbers.into_iter().collect::<Vec<u8>>();
        old_quorum_numbers.sort();
        new_quorum_numbers.sort();

        let old_task_block = task.last_completed_task_created_block;
        let new_task_block = task.task_created_block;

        let registry_coordinator_address = &self
        .avs_contracts
        .registry_coordinator_address;
        let registry_coordinator = &self
        .avs_contracts
        .registry;
        let stake_registry = &self
        .avs_contracts
        .stake_registry;
        let bls_apk_registry = &self
        .avs_contracts
        .bls_apk_registry;
        let task_manager = &self
        .avs_contracts
        .task_manager;

        let mut maybe_i = old_quorum_numbers.iter().peekable();
        let mut maybe_j = new_quorum_numbers.iter().peekable();
        
        let mut quorums_common: Vec<u8> = Vec::new();
        let mut quorums_removed: Vec<u8> = Vec::new();
        let mut quorums_added: Vec<QuorumsAdded> = Vec::new();
        let mut quorums_apk_update: Vec<QuorumsApkUpdate> = Vec::new();
        let mut quorums_stake_update: Vec<QuorumsStakeUpdate> = Vec::new();
        
        loop {
            match (maybe_i.peek(), maybe_j.peek()){
                (Some(&&i), Some(&&j)) if i == j => {
                    // handle potential update
                    let old_quorum_apk = bls_apk_registry.get_apk(i).block::<u64>(u64::from(old_task_block)).await?;
                    let old_quorum_stake = stake_registry.get_current_total_stake(i).block(u64::from(old_task_block)).await?;

                    let new_quorum_apk = bls_apk_registry.get_apk(i).block(u64::from(new_task_block)).await?;
                    let new_quorum_stake = stake_registry.get_current_total_stake(i).block(u64::from(new_task_block)).await?;

                    if old_quorum_apk != new_quorum_apk {
                        quorums_apk_update.push(QuorumsApkUpdate{
                            quorum_number: i,
                            quorum_apk: new_quorum_apk
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
                    maybe_i.next();
                },
                (Some(&&i), Some(&&j)) if i > j => {
                    // handle quorum number added
                    quorums_added.push(QuorumsAdded{
                        quorum_number: j,
                        quorum_stake: stake_registry.get_current_total_stake(j).block(u64::from(new_task_block)).await?,
                        quorum_apk: bls_apk_registry.get_apk(j).block(u64::from(new_task_block)).await?,
                    });

                    maybe_j.next();
                },
                (Some(&&i), None) => {
                    // handle quorum number removed
                    quorums_removed.push(i);
                    maybe_i.next();
                },
                (None, Some(&&j)) => {
                    // handle quorum number added
                    quorums_added.push(
                        QuorumsAdded{
                            quorum_number: j,
                            quorum_stake: stake_registry.get_current_total_stake(j).block(u64::from(new_task_block)).await?,
                            quorum_apk: bls_apk_registry.get_apk(j).block(u64::from(new_task_block)).await?,
                        }
                    );
                    maybe_j.next();
                },
                (None, None) => {
                    // handle quorum number added
                    break;
                },
                _ => unreachable!()

            }

        }



        let old_operators_stake = task_manager
            .get_operator_state(
                *registry_coordinator_address,
                old_quorum_numbers.clone().into(),
                old_task_block,
            ).await?;
        let new_operators_stake = task_manager
            .get_operator_state(
                *registry_coordinator_address,
                new_quorum_numbers.clone().into(),
                new_task_block,
            ).await?;

        let mut old_operators_avs_state = self.get_operators_avs_state_at_block(old_operators_stake, old_quorum_numbers.clone().into()).await.values().cloned().collect::<Vec<_>>();
        let mut new_operators_avs_state = self.get_operators_avs_state_at_block(new_operators_stake, new_quorum_numbers.clone().into()).await.values().cloned().collect::<Vec<_>>();
        
        old_operators_avs_state.sort_by_key(|v| v.operator_id);
        new_operators_avs_state.sort_by_key(|v| v.operator_id);

        let mut maybe_i = old_operators_avs_state.iter().peekable();
        let mut maybe_j = new_operators_avs_state.iter().peekable();

        let mut operators_removed: Vec<[u8; 32]> = Vec::new();
        let mut operators_added: Vec<OperatorsAdded> = Vec::new(); // Needs to be sorted
        let mut operators_quorum_count_update: Vec<OperatorsQuorumCountUpdate> = Vec::new();
        let mut operators_stake_update: Vec<OperatorsStakeUpdate> = Vec::new();

        loop {
            match (maybe_i.peek(), maybe_j.peek()){
                (Some(&&ref i), Some(&&ref j)) if i.operator_id == j.operator_id => {
                    // handle potential update

                    if i.stake_per_quorum.len() != j.stake_per_quorum.len(){
                        operators_quorum_count_update.push(OperatorsQuorumCountUpdate{
                            operator_id: j.operator_id,
                            quorum_count: j.stake_per_quorum.len().try_into()?,
                        });
                    }
                    let mut operator_stake_update = OperatorsStakeUpdate{
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
                        let stake = j.stake_per_quorum.get(&qn).map(u128::to_owned).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); Default::default()});
                        operator_stake_update.quorum_stakes.push(stake)
                    }
                    for qn in quorums_common.iter(){
                        let stake_old = i.stake_per_quorum.get(&qn).map(u128::to_owned).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); Default::default()});
                        let stake_new = j.stake_per_quorum.get(&qn).map(u128::to_owned).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); Default::default()});
                        if stake_old != stake_new {
                            operator_stake_update.quorum_for_stakes.push(*qn);
                            operator_stake_update.quorum_stakes.push(stake_new)
                        }
                    }
                    if !operator_stake_update.quorum_for_stakes.is_empty(){
                    operators_stake_update.push(operator_stake_update);}

                    maybe_i.next(); maybe_j.next();
                },
                (Some(&&ref i), Some(&&ref j)) if i.operator_id < j.operator_id => {
                    // handle operator removed
                    operators_removed.push(i.operator_id);
                    maybe_i.next();
                },
                (Some(&&ref i), Some(&&ref j)) if i.operator_id > j.operator_id => {
                    // handle quorum number added

                    let mut operator_added = OperatorsAdded{
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                        quorum_count: j.stake_per_quorum.len().try_into()?,
                    };

                    for qn in j.stake_per_quorum.keys(){
                        operator_added.quorum_for_stakes.push(*qn);
                        let stake = j.stake_per_quorum.get(qn).map(u128::to_owned).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); Default::default()});
                        operator_added.quorum_stakes.push(stake)
                    }

                    operators_added.push(operator_added);

                    maybe_j.next();
                },
                (Some(&&ref i), None) => {
                    // handle quorum number removed
                    operators_removed.push(i.operator_id);
                    maybe_i.next();
                },
                (None, Some(&&ref j)) => {
                    // handle operator added
                    
                    let mut operator_added = OperatorsAdded{
                        operator_id: j.operator_id,
                        quorum_for_stakes: Default::default(),
                        quorum_stakes: Default::default(),
                        quorum_count: j.stake_per_quorum.len().try_into()?,
                    };

                    for qn in j.stake_per_quorum.keys(){
                        operator_added.quorum_for_stakes.push(*qn);
                        let stake = j.stake_per_quorum.get(qn).map(u128::to_owned).unwrap_or_else(|| {error!("Failed to get operator quorum stake"); Default::default()});
                        operator_added.quorum_stakes.push(stake)
                    }

                    operators_added.push(operator_added);
                    maybe_j.next();
                },
                (None, None) => {
                    // handle quorum number added
                    break;
                },
                _ => unreachable!()
            }

        }

        let operators_state_changed = 
        !quorums_removed.is_empty() || !quorums_added.is_empty() || !quorums_apk_update.is_empty() || !quorums_stake_update.is_empty() || !operators_removed.is_empty() || !operators_added.is_empty() || !operators_stake_update.is_empty() || !operators_quorum_count_update.is_empty() || (task.quorum_threshold_percentage != task.last_completed_task_quorum_threshold_percentage);

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

        return operators_avs_state;
    }

}