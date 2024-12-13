use crate::chainio::{avs::AvsContracts, build_clients, SourceClient, TargetClient};
use crate::cli::CliArgs;

use bindings::{
    finalizer_task_manager::{
        FinalizerTaskManager, FinalizerTaskManagerCalls, FinalizerTaskManagerEvents,
        NewOpTaskCreatedFilter, OpTaskCompletedFilter, Operator as TMOperator, OperatorStateInfo,
        OperatorsAdded, OperatorsQuorumCountUpdate, OperatorsStakeUpdate, QuorumsAdded,
        QuorumsApkUpdate, QuorumsStakeUpdate, RdTaskCompletedFilter,
    },
    gasp_multi_rollup_service::{GaspMultiRollupService, Range},
    shared_types::{OpTask, OpTaskResponse},
};
use ethers::providers::{Middleware, SubscriptionStream};
use ethers::{
    abi::AbiDecode,
    contract::{stream, EthEvent, EthLogDecode, LogMeta},
    providers::StreamExt,
    types::{Bytes, Filter},
};

use ethers::abi::AbiEncode;
use eyre::{eyre, OptionExt};
use sp_core::H256;
use sp_runtime::traits::{Hash, Keccak256, Zero};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::select;
use tracing::{debug, error, info, instrument};

// TODO
// In addition to reinit we could also have a function in the syncer that would cherry pick the task and its response
// to be admin forced on the alt-l1. We could either provide the taskIndex as the input or we can provide number of tasks to
// sync or we can provide the block number (taskCreatedBlock) to sync to...
// But rdTasks and opTasks make this fairly annoying to do...

// TODO!
// Maybe also check logs.removed?

type QuorumNum = u8;

#[derive(Clone)]
pub struct CustomOperatorAvsState {
    pub operator_id: [u8; 32],
    pub stake_per_quorum: HashMap<QuorumNum, u128>,
}

#[derive(Debug)]
pub struct Syncer {
    pub source_client: Arc<SourceClient>,
    #[allow(dead_code)]
    pub target_client: Arc<TargetClient>,
    pub target_chain_index: u8,
    pub root_target_client: Option<Arc<TargetClient>>,
    avs_contracts: AvsContracts,
    gasp_service_contract: GaspMultiRollupService<TargetClient>,
    root_gasp_service_contract: Option<GaspMultiRollupService<TargetClient>>,
    filter_limit: u64,
}
impl Syncer {
    #[instrument(name = "create_syncer", skip_all)]
    pub async fn from_cli(cfg: &CliArgs) -> eyre::Result<Arc<Self>> {
        let (source_client, target_client, maybe_root_target_client) = build_clients(cfg).await?;
        let (source_client, target_client) = (Arc::new(source_client), Arc::new(target_client));
        let avs_contracts = AvsContracts::build(cfg, source_client.clone()).await?;
        let gasp_service_contract =
            GaspMultiRollupService::new(cfg.gasp_service_addr, target_client.clone());
        let maybe_arc_root_target_client = maybe_root_target_client.map(Arc::new);
        let root_gasp_service_contract = if cfg.reinit || cfg.only_reinit {
            let root_target_client = maybe_arc_root_target_client
                .clone()
                .expect("should work here");
            Some(GaspMultiRollupService::new(
                cfg.gasp_service_addr,
                root_target_client.clone(),
            ))
        } else {
            None
        };
        let target_chain_index = cfg.target_chain_index;

        Ok(Arc::new(Self {
            source_client,
            target_client,
            target_chain_index,
            root_target_client: maybe_arc_root_target_client,
            avs_contracts,
            gasp_service_contract,
            root_gasp_service_contract,
            filter_limit: cfg.filter_limit,
        }))
    }

    #[instrument(skip_all, err)]
    pub async fn handle_sync_event(
        self: Arc<Self>,
        event: FinalizerTaskManagerEvents,
        log: LogMeta,
        latest_completed_op_task_created_block: &mut u32,
        latest_completed_op_task_number: &mut u32,
        latest_completed_rd_task_number: &mut u32,
        last_processed_rd_task_number: &mut Option<u32>,
    ) -> eyre::Result<()> {
        match event {
            FinalizerTaskManagerEvents::OpTaskCompletedFilter(_event) => {
                let txn_hash = log.transaction_hash;
                let txn = self
                    .clone()
                    .avs_contracts
                    .ws_client
                    .get_transaction(txn_hash)
                    .await?
                    .ok_or_else(|| eyre!("missing expected txn {:?}", txn_hash))?;
                debug!("{:?}", txn);
                let call = match FinalizerTaskManagerCalls::decode(txn.input)?{
                    FinalizerTaskManagerCalls::RespondToOpTask(c) => c,
                    FinalizerTaskManagerCalls::ForceRespondToOpTask(_) => {
                        return Err(eyre!("The call that resulted in the OpTaskCompleted event was a ForceRespondToOpTask call. This cannot be synced without a admin reinit"))
                    }
                    _ => {
                        return Err(eyre!("wrong call decoded"))
                    }
                };
                debug!("{:?}", call);
                debug!("{:?}", call.task.clone());
                debug!("encoded - {:?}", call.task.clone().encode());
                debug!(
                    "encoded - {:?}",
                    array_bytes::bytes2hex("0x", call.task.clone().encode())
                );
                let task_hash = Keccak256::hash(
                    vec![0u8; 31]
                        .into_iter()
                        .chain(vec![32u8])
                        .chain(call.task.clone().encode().into_iter())
                        .collect::<Vec<_>>()
                        .as_ref(),
                );
                debug!("{:?}", task_hash);
                debug!(
                    "{:?}",
                    task_hash == call.task_response.reference_task_hash.into()
                );

                if task_hash != call.task_response.reference_task_hash.into() {
                    return Err(eyre!("task_hash mismatch {:?}", task_hash));
                }

                let operators_state_info = self
                    .clone()
                    .get_operators_state_info(call.task.clone())
                    .await?;
                debug!("{:?}", operators_state_info);
                let operators_state_info_hash = Keccak256::hash(
                    vec![0u8; 31]
                        .into_iter()
                        .chain(vec![32u8])
                        .chain(operators_state_info.clone().encode().into_iter())
                        .collect::<Vec<_>>()
                        .as_ref(),
                );
                debug!("{:?}", operators_state_info_hash);
                debug!(
                    "{:?}",
                    operators_state_info_hash
                        == call.task_response.operators_state_info_hash.into()
                );

                if operators_state_info_hash != call.task_response.operators_state_info_hash.into()
                {
                    return Err(eyre!(
                        "operators_state_info_hash mismatch {:?}",
                        operators_state_info_hash
                    ));
                }

                // if latest_completed_op_task_created_block == 0, then we are looking for the
                // first task
                if *latest_completed_op_task_created_block == 0 {
                    if call.task.task_created_block
                        != call.task.last_completed_op_task_created_block
                        || call.task.task_num != call.task.last_completed_op_task_num
                    {
                        return Err(eyre!(
                            "missing expected first op task response {:?} {:?} {:?} {:?}",
                            call.task.task_created_block,
                            call.task.last_completed_op_task_created_block,
                            call.task.task_num,
                            call.task.last_completed_op_task_num
                        ));
                    }
                } else {
                    if *latest_completed_op_task_created_block
                        < call.task.last_completed_op_task_created_block
                    {
                        return Err(eyre!(
                            "missing expected task response {:?}",
                            *latest_completed_op_task_created_block
                        ));
                    }

                    if *latest_completed_op_task_created_block
                        > call.task.last_completed_op_task_created_block
                    {
                        return Ok(());
                    }

                    // At this point *latest_completed_op_task_created_block == call.task.last_completed_op_task_created_block

                    if *latest_completed_op_task_number != call.task.last_completed_op_task_num {
                        return Err(eyre!(
                            "missing expected task response {:?}, {:?}",
                            *latest_completed_op_task_number,
                            *latest_completed_op_task_created_block
                        ));
                    }
                }

                let update_txn = self.clone().gasp_service_contract.process_eigen_op_update(
                    call.task.clone(),
                    call.task_response,
                    call.non_signer_stakes_and_signature,
                    operators_state_info,
                );
                debug!("{:?}", update_txn);
                let update_txn_pending = update_txn.send().await;
                debug!("{:?}", update_txn_pending);
                let update_txn_receipt = update_txn_pending?.await?;
                debug!("{:?}", update_txn_receipt);
                match update_txn_receipt
                    .clone()
                    .ok_or_eyre("failed to unwrap update_txn_receipt")?
                    .status
                {
                    Some(status) if status.is_zero() => {
                        return Err(eyre!("update_txn failed {:?}", update_txn_receipt))
                    }
                    _ => {}
                }

                info!("sucessfully synced op task - {:?}", call.task.clone());
                *latest_completed_op_task_created_block = call.task.task_created_block;
                *latest_completed_op_task_number = call.task.task_num;
            }
            FinalizerTaskManagerEvents::RdTaskCompletedFilter(_event) => {
                let txn_hash = log.transaction_hash;
                let txn = self
                    .clone()
                    .avs_contracts
                    .ws_client
                    .get_transaction(txn_hash)
                    .await?
                    .ok_or_else(|| eyre!("missing expected txn {:?}", txn_hash))?;
                debug!("{:?}", txn);
                let call = match FinalizerTaskManagerCalls::decode(txn.input)? {
                    FinalizerTaskManagerCalls::RespondToRdTask(c) => c,
                    _ => return Err(eyre!("wrong call decoded")),
                };
                debug!("{:?}", call);
                debug!("{:?}", call.task.clone());
                debug!("encoded - {:?}", call.task.clone().encode());
                debug!(
                    "encoded - {:?}",
                    array_bytes::bytes2hex("0x", call.task.clone().encode())
                );
                let task_hash = Keccak256::hash(
                    vec![0u8; 31]
                        .into_iter()
                        .chain(vec![32u8])
                        .chain(call.task.clone().encode().into_iter())
                        .collect::<Vec<_>>()
                        .as_ref(),
                );
                debug!("{:?}", task_hash);
                debug!(
                    "{:?}",
                    task_hash == call.task_response.reference_task_hash.into()
                );

                if call.task.chain_id != self.target_chain_index {
                    return Ok(());
                }

                if task_hash != call.task_response.reference_task_hash.into() {
                    return Err(eyre!("task_hash mismatch {:?}", task_hash));
                }

                // We don't really know which rdTask number to expect here
                // It can always happen that the updater just restarted and acccepted
                // an out of order rdTask (avoiding the below check), but when it tries to push this out of order
                // rdTask to the alt-l1 it will fail due to the chain_rd_batch_nonce
                if *latest_completed_rd_task_number != 0
                    && *latest_completed_rd_task_number >= call.task.task_num
                {
                    return Ok(());
                }

                if let Some(n) = *last_processed_rd_task_number {
                    if n >= call.task.task_num {
                        return Err(eyre!(
                            "out of order or duplicate rdTaskCompleted events {:?}, {:?}",
                            n,
                            call.task.task_num
                        ));
                    }
                }

                if *latest_completed_op_task_created_block
                    != call.task.last_completed_op_task_created_block
                {
                    return Err(eyre!(
                        "latest_completed_op_task_created_block mismatch {:?}",
                        *latest_completed_op_task_created_block
                    ));
                }

                let update_txn = self.clone().gasp_service_contract.process_eigen_rd_update(
                    call.task.clone(),
                    call.task_response,
                    call.non_signer_stakes_and_signature,
                );
                debug!("{:?}", update_txn);
                let update_txn_pending = update_txn.send().await;
                debug!("{:?}", update_txn_pending);
                let update_txn_receipt = update_txn_pending?.await?;
                debug!("{:?}", update_txn_receipt);
                match update_txn_receipt
                    .clone()
                    .ok_or_eyre("failed to unwrap update_txn_receipt")?
                    .status
                {
                    Some(status) if status.is_zero() => {
                        return Err(eyre!("update_txn failed {:?}", update_txn_receipt))
                    }
                    _ => {}
                }

                info!("sucessfully synced rd task - {:?}", call.task.clone());
                *latest_completed_rd_task_number = call.task.task_num;
                *last_processed_rd_task_number = Some(call.task.task_num);
            }
            _ => return Err(eyre!("Got unexpected stream event")),
        }
        Ok(())
    }

    #[instrument(skip_all, err)]
    pub async fn sync(self: Arc<Self>, cfg: &CliArgs) -> eyre::Result<()> {
        let alt_block_number: u64 = self.target_client.get_block_number().await?.as_u64();
        let mut latest_completed_op_task_number = self
            .gasp_service_contract
            .latest_completed_op_task_number()
            .block(alt_block_number)
            .await?;
        let mut latest_completed_op_task_created_block = self
            .gasp_service_contract
            .latest_completed_op_task_created_block()
            .block(alt_block_number)
            .await?;
        let mut latest_completed_rd_task_number = self
            .gasp_service_contract
            .latest_completed_rd_task_number()
            .block(alt_block_number)
            .await?;

        if latest_completed_op_task_created_block.is_zero() && !cfg.push_first_init {
            return Err(eyre!("target uninit and push_first_init set to false"));
        }

        // TODO
        // To avoid this we should also link new rdTasks with the last completed ones (atleast the ones on the same chain)
        // But requires more info to be stored in either the alt-l1s or eth-l1s
        let mut last_processed_rd_task_number: Option<u32> = None;

        let source_block_number: u64 = self.source_client.get_block_number().await?.as_u64();

        if source_block_number < latest_completed_op_task_created_block.into() {
            return Err(eyre!("invalid latest_completed_op_task_created_block"));
        }

        let target_block_number: u64 = source_block_number.saturating_sub(1u64);
        let mut from_block: u64 = latest_completed_op_task_created_block.into();

        if target_block_number >= from_block {
            let mut to_block = from_block
                .saturating_add(self.filter_limit)
                .saturating_sub(1u64);
            if to_block > target_block_number {
                to_block = target_block_number;
            }

            loop {
                let events: Vec<(FinalizerTaskManagerEvents, LogMeta)> = self
                    .clone()
                    .avs_contracts
                    .task_manager
                    .event_with_filter(
                        Filter::new()
                            .events([
                                OpTaskCompletedFilter::abi_signature().into_owned(),
                                RdTaskCompletedFilter::abi_signature().into_owned(),
                            ])
                            .select(from_block..to_block),
                    )
                    .query_with_meta()
                    .await?;

                for (event, log) in events {
                    self.clone()
                        .handle_sync_event(
                            event,
                            log,
                            &mut latest_completed_op_task_created_block,
                            &mut latest_completed_op_task_number,
                            &mut latest_completed_rd_task_number,
                            &mut last_processed_rd_task_number,
                        )
                        .await?;
                }

                if to_block == target_block_number {
                    break;
                }
                from_block = to_block.saturating_add(1u64);
                to_block = from_block
                    .saturating_add(self.filter_limit)
                    .saturating_sub(1u64);
                if to_block > target_block_number {
                    to_block = target_block_number;
                }
            }
        }

        let evs = self
            .clone()
            .avs_contracts
            .source_response_stream(source_block_number);
        let mut stream: stream::EventStream<'_, _, (FinalizerTaskManagerEvents, LogMeta), _> =
            evs.subscribe_with_meta().await?;

        // event stream does not finish with `None` after websocket closure, use block subscription for it
        let mut blocks: SubscriptionStream<'_, _, _> =
            self.avs_contracts.ws_client.subscribe_blocks().await?;

        loop {
            select! {
                Some(stream_event) = stream.next() => match stream_event {
                    Ok((stream_event, log)) => {
                        self.clone().handle_sync_event(stream_event, log, &mut latest_completed_op_task_created_block, &mut latest_completed_op_task_number, &mut latest_completed_rd_task_number, &mut last_processed_rd_task_number).await?;
                    }
                    Err(e) => error!("EthWs subscription error {:?}", e),
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
    pub async fn reinit_only_print(self: Arc<Self>, _cfg: &CliArgs) -> eyre::Result<()> {
        let (
            last_task,
            operators_state_info,
            merkle_roots,
            ranges,
            expected_batch_id,
            latest_completed_rd_task_number_update,
            latest_completed_rd_task_created_block_update,
        ) = self.clone().get_reinit_info().await?;
        info!(
            "reinit info - {:?}",
            (
                last_task.clone(),
                operators_state_info.clone(),
                merkle_roots.clone(),
                ranges.clone(),
                expected_batch_id,
                latest_completed_rd_task_number_update,
                latest_completed_rd_task_created_block_update
            )
        );
        let reinit_txn = self
            .clone()
            .gasp_service_contract
            .clone()
            .process_eigen_reinit(
                last_task.clone(),
                operators_state_info.clone(),
                merkle_roots.clone(),
                ranges.clone(),
                expected_batch_id,
                latest_completed_rd_task_number_update,
                latest_completed_rd_task_created_block_update,
            );
        info!("reinit_txn: {:?}", reinit_txn);
        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn reinit(self: Arc<Self>, _cfg: &CliArgs) -> eyre::Result<()> {
        let (
            last_task,
            operators_state_info,
            merkle_roots,
            ranges,
            expected_batch_id,
            latest_completed_rd_task_number_update,
            latest_completed_rd_task_created_block_update,
        ) = self.clone().get_reinit_info().await?;
        info!(
            "reinit info - {:?}",
            (
                last_task.clone(),
                operators_state_info.clone(),
                merkle_roots.clone(),
                ranges.clone(),
                expected_batch_id,
                latest_completed_rd_task_number_update,
                latest_completed_rd_task_created_block_update
            )
        );
        let reinit_txn = self
            .clone()
            .root_gasp_service_contract
            .clone()
            .expect("should work in reinit")
            .process_eigen_reinit(
                last_task.clone(),
                operators_state_info.clone(),
                merkle_roots.clone(),
                ranges.clone(),
                expected_batch_id,
                latest_completed_rd_task_number_update,
                latest_completed_rd_task_created_block_update,
            );
        debug!("{:?}", reinit_txn);
        let reinit_txn_pending = reinit_txn.send().await;
        debug!("{:?}", reinit_txn_pending);
        let reinit_txn_receipt = reinit_txn_pending?.await?;
        debug!("{:?}", reinit_txn_receipt);
        match reinit_txn_receipt
            .clone()
            .ok_or_eyre("failed to unwrap reinit_txn_receipt")?
            .status
        {
            Some(status) if status.is_zero() => {
                return Err(eyre!("reinit_txn failed {:?}", reinit_txn_receipt))
            }
            _ => {}
        }

        info!("Sucessfully completed reinit");

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    #[instrument(skip_all)]
    pub async fn get_reinit_info(
        self: Arc<Self>,
    ) -> eyre::Result<(
        OpTask,
        OperatorStateInfo,
        Vec<[u8; 32]>,
        Vec<Range>,
        u32,
        u32,
        u32,
    )> {
        let alt_block_number: u64 = self.target_client.get_block_number().await?.as_u64();
        let latest_completed_op_task_created_block = self
            .gasp_service_contract
            .latest_completed_op_task_created_block()
            .block(alt_block_number)
            .await?;
        let latest_completed_op_task_quorum_numbers = self
            .gasp_service_contract
            .quorum_numbers()
            .block(alt_block_number)
            .await?;
        let latest_completed_op_task_quorum_threshold_percentage = self
            .gasp_service_contract
            .quorum_threshold_percentage()
            .block(alt_block_number)
            .await?;
        let latest_completed_rd_task_number = self
            .gasp_service_contract
            .latest_completed_rd_task_number()
            .block(alt_block_number)
            .await?;
        let latest_completed_rd_task_created_block = self
            .gasp_service_contract
            .latest_completed_rd_task_created_block()
            .block(alt_block_number)
            .await?;
        let chain_rd_batch_nonce = self
            .gasp_service_contract
            .chain_rd_batch_nonce()
            .block(alt_block_number)
            .await?;

        // TODO!!
        // Get the latest block from source chain and query the following three with it!
        let eth_block_number: u64 = self.source_client.get_block_number().await?.as_u64();
        let task_num = self
            .clone()
            .avs_contracts
            .task_manager
            .last_completed_op_task_num()
            .block(eth_block_number)
            .await?;
        let block_num = self
            .clone()
            .avs_contracts
            .task_manager
            .last_completed_op_task_created_block()
            .block(eth_block_number)
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
            // TODO
            // Use a constant for the enum variant index
            if task_status != 4 {
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
                return Err(eyre!("task not in events for reinit {:?}", task_num));
            }
        };

        last_task.last_completed_op_task_created_block = latest_completed_op_task_created_block;
        last_task.last_completed_op_task_quorum_numbers = latest_completed_op_task_quorum_numbers;
        last_task.last_completed_op_task_quorum_threshold_percentage =
            latest_completed_op_task_quorum_threshold_percentage;

        let (
            merkle_roots,
            ranges,
            expected_batch_id,
            latest_completed_rd_task_number_update,
            latest_completed_rd_task_created_block_update,
        ) = self
            .clone()
            .get_cumulative_rd_update(
                chain_rd_batch_nonce,
                latest_completed_rd_task_number,
                latest_completed_rd_task_created_block,
                eth_block_number,
                u64::from(latest_completed_op_task_created_block),
            )
            .await?;

        let operators_state_info = self
            .clone()
            .get_operators_state_info(last_task.clone())
            .await?;

        Ok((
            last_task.clone(),
            operators_state_info.clone(),
            merkle_roots.clone(),
            ranges.clone(),
            expected_batch_id,
            latest_completed_rd_task_number_update,
            latest_completed_rd_task_created_block_update,
        ))
    }

    #[instrument(skip_all)]
    pub async fn reinit_eth_only_print_op_task_creation(
        self: Arc<Self>,
        _cfg: &CliArgs,
    ) -> eyre::Result<()> {
        // Get the root_target_client here, this should exist at this point
        let source_client = self.source_client.clone();

        // Build the root_task_manager here using the address from the source avs_contracts
        let task_manager_addr = self.avs_contracts.task_manager.address();
        let task_manager = FinalizerTaskManager::new(task_manager_addr, source_client.clone());

        // Create a new opTask via forceCreateNewOpTask
        let force_task_txn = task_manager.force_create_new_op_task(66u32, vec![0u8].into());
        info!("{:?}", force_task_txn);

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn reinit_eth_only_print_op_task_response(
        self: Arc<Self>,
        _cfg: &CliArgs,
    ) -> eyre::Result<()> {
        // Get the root_target_client here, this should exist at this point
        let source_client = self.source_client.clone();

        // Build the root_task_manager here using the address from the source avs_contracts
        let task_manager_addr = self.avs_contracts.task_manager.address();
        let task_manager = FinalizerTaskManager::new(task_manager_addr, source_client.clone());

        let source_block_number: u64 = self.source_client.get_block_number().await?.as_u64();

        let is_task_pending = self
            .clone()
            .avs_contracts
            .task_manager
            .is_task_pending()
            .block(source_block_number)
            .await?;
        if !is_task_pending {
            return Err(eyre!("no task is pending"));
        }

        let task_num = self
            .clone()
            .avs_contracts
            .task_manager
            .latest_op_task_num()
            .block(source_block_number)
            .await?;

        let task_status = self
            .clone()
            .avs_contracts
            .task_manager
            .id_to_task_status(0u8, task_num)
            .block(source_block_number)
            .await?;

        // TODO
        // Use a constant for the enum variant index
        if task_status != 1 {
            return Err(eyre!("pending task not is not op task: {:?}", task_num));
        }

        let last_op_task_created_block = self
            .clone()
            .avs_contracts
            .task_manager
            .last_op_task_created_block()
            .block(source_block_number)
            .await?;

        let mut block_events: Vec<NewOpTaskCreatedFilter> = self
            .clone()
            .avs_contracts
            .task_manager
            .event_with_filter(
                Filter::new()
                    .event(&NewOpTaskCreatedFilter::abi_signature())
                    .select(u64::from(last_op_task_created_block)),
            )
            .query()
            .await?;

        let last_task = match block_events.pop() {
            Some(e) if e.task_index == task_num => e.task,
            _ => {
                return Err(eyre!("task not in events for reinit {:?}", task_num));
            }
        };

        let task = last_task;

        let operators_state_info = self.clone().get_operators_state_info(task.clone()).await?;
        debug!("{:?}", operators_state_info);
        let operators_state_info_hash = Keccak256::hash(
            vec![0u8; 31]
                .into_iter()
                .chain(vec![32u8])
                .chain(operators_state_info.clone().encode().into_iter())
                .collect::<Vec<_>>()
                .as_ref(),
        );
        debug!("{:?}", operators_state_info_hash);

        let task_hash = Keccak256::hash(
            vec![0u8; 31]
                .into_iter()
                .chain(vec![32u8])
                .chain(task.clone().encode().into_iter())
                .collect::<Vec<_>>()
                .as_ref(),
        );

        // Respond to the opTask via forceRespondToOpTask

        let force_respond_txn = task_manager.force_respond_to_op_task(
            task.clone(),
            OpTaskResponse {
                reference_task_index: task.task_num,
                reference_task_hash: task_hash.into(),
                operators_state_info_hash: operators_state_info_hash.into(),
            },
        );
        info!("{:?}", force_respond_txn);

        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn reinit_eth(self: Arc<Self>, _cfg: &CliArgs) -> eyre::Result<()> {
        // Get the root_target_client here, this should exist at this point
        let root_target_client = self
            .root_target_client
            .clone()
            .ok_or_eyre("failed to unwrap root_target_client")?;

        // Build the root_task_manager here using the address from the source avs_contracts
        let task_manager_addr = self.avs_contracts.task_manager.address();
        let task_manager = FinalizerTaskManager::new(task_manager_addr, root_target_client.clone());

        // Create a new opTask via forceCreateNewOpTask

        // TODO
        // Put the quorum_threshold_percentage and quorum_numbers into constants properly somewhere
        // Maybe put default values in the TaskManager contract itself
        let force_task_txn = task_manager.force_create_new_op_task(66u32, vec![0u8].into());
        debug!("{:?}", force_task_txn);
        let force_task_txn_pending = force_task_txn.send().await;
        debug!("{:?}", force_task_txn_pending);
        let force_task_txn_receipt = force_task_txn_pending?.await?;
        debug!("{:?}", force_task_txn_receipt);
        match force_task_txn_receipt
            .clone()
            .ok_or_eyre("failed to unwrap force_task_txn_receipt")?
            .status
        {
            Some(status) if status.is_zero() => {
                return Err(eyre!("force_task_txn failed {:?}", force_task_txn_receipt))
            }
            _ => {}
        }

        let force_task_txn_receipt =
            force_task_txn_receipt.ok_or_eyre("force_task_txn_receipt is None")?;
        let new_op_task_created_event = force_task_txn_receipt.logs.into_iter().find_map(|r| {
            if let Ok(FinalizerTaskManagerEvents::NewOpTaskCreatedFilter(decoded)) =
                FinalizerTaskManagerEvents::decode_log(&r.into())
            {
                Some(decoded)
            } else {
                None
            }
        });

        let new_op_task_created_event =
            new_op_task_created_event.ok_or_eyre("new_op_task_created_event is None")?;

        let task = new_op_task_created_event.task;

        let operators_state_info = self.clone().get_operators_state_info(task.clone()).await?;
        debug!("{:?}", operators_state_info);
        let operators_state_info_hash = Keccak256::hash(
            vec![0u8; 31]
                .into_iter()
                .chain(vec![32u8])
                .chain(operators_state_info.clone().encode().into_iter())
                .collect::<Vec<_>>()
                .as_ref(),
        );
        debug!("{:?}", operators_state_info_hash);

        let task_hash = Keccak256::hash(
            vec![0u8; 31]
                .into_iter()
                .chain(vec![32u8])
                .chain(task.clone().encode().into_iter())
                .collect::<Vec<_>>()
                .as_ref(),
        );

        // Respond to the opTask via forceRespondToOpTask

        let force_respond_txn = task_manager.force_respond_to_op_task(
            task.clone(),
            OpTaskResponse {
                reference_task_index: new_op_task_created_event.task_index,
                reference_task_hash: task_hash.into(),
                operators_state_info_hash: operators_state_info_hash.into(),
            },
        );
        debug!("{:?}", force_respond_txn);
        let force_respond_txn_pending = force_respond_txn.send().await;
        debug!("{:?}", force_respond_txn_pending);
        let force_respond_txn_receipt = force_respond_txn_pending?.await?;
        debug!("{:?}", force_respond_txn_receipt);
        match force_respond_txn_receipt
            .clone()
            .ok_or_eyre("failed to unwrap force_respond_txn_receipt")?
            .status
        {
            Some(status) if status.is_zero() => {
                return Err(eyre!(
                    "force_respond_txn failed {:?}",
                    force_respond_txn_receipt
                ))
            }
            _ => {}
        }

        info!(
            "Successfully completed reinit-eth - {:?}",
            (
                task,
                OpTaskResponse {
                    reference_task_index: new_op_task_created_event.task_index,
                    reference_task_hash: task_hash.into(),
                    operators_state_info_hash: operators_state_info_hash.into(),
                },
            )
        );

        Ok(())
    }

    pub(crate) async fn get_cumulative_rd_update(
        self: Arc<Self>,
        chain_rd_batch_nonce: u32,
        latest_completed_rd_task_number: u32,
        latest_completed_rd_task_created_block: u32,
        eth_block_number: u64,
        latest_completed_op_task_created_block: u64,
    ) -> eyre::Result<(Vec<[u8; 32]>, Vec<Range>, u32, u32, u32)> {
        let mut merkle_roots: Vec<[u8; 32]> = Default::default();
        let mut ranges: Vec<Range> = Default::default();
        let mut expected_batch_id = chain_rd_batch_nonce;

        let mut last_log: Option<LogMeta> = None;

        let mut from_block = latest_completed_op_task_created_block;
        let mut to_block = latest_completed_op_task_created_block
            .saturating_add(self.filter_limit)
            .saturating_sub(1u64);
        if to_block > eth_block_number {
            to_block = eth_block_number;
        }

        loop {
            self.clone()
                .get_rd_update(
                    latest_completed_rd_task_number,
                    from_block,
                    to_block,
                    &mut merkle_roots,
                    &mut ranges,
                    &mut expected_batch_id,
                    &mut last_log,
                )
                .await?;
            if to_block == eth_block_number {
                break;
            }
            from_block = to_block.saturating_add(1u64);
            to_block = from_block
                .saturating_add(self.filter_limit)
                .saturating_sub(1u64);
            if to_block > eth_block_number {
                to_block = eth_block_number;
            }
        }

        let (latest_completed_rd_task_number_update, latest_completed_rd_task_created_block_update) =
            if let Some(log) = last_log {
                let txn_hash = log.transaction_hash;
                let txn = self
                    .clone()
                    .avs_contracts
                    .ws_client
                    .get_transaction(txn_hash)
                    .await?
                    .ok_or_else(|| eyre!("missing expected txn {:?}", txn_hash))?;
                debug!("{:?}", txn);
                let call = match FinalizerTaskManagerCalls::decode(txn.input)? {
                    FinalizerTaskManagerCalls::RespondToRdTask(c) => c,
                    _ => return Err(eyre!("wrong call decoded")),
                };
                debug!("{:?}", call);
                debug!("{:?}", call.task.clone());
                (call.task.task_num, call.task.task_created_block)
            } else {
                (
                    latest_completed_rd_task_number,
                    latest_completed_rd_task_created_block,
                )
            };

        Ok((
            merkle_roots,
            ranges,
            expected_batch_id,
            latest_completed_rd_task_number_update,
            latest_completed_rd_task_created_block_update,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn get_rd_update(
        self: Arc<Self>,
        latest_completed_rd_task_number: u32,
        from_block: u64,
        to_block: u64,
        merkle_roots: &mut Vec<[u8; 32]>,
        ranges: &mut Vec<Range>,
        expected_batch_id: &mut u32,
        last_log: &mut Option<LogMeta>,
    ) -> eyre::Result<()> {
        let events: Vec<RdTaskCompletedFilter> = self
            .clone()
            .avs_contracts
            .task_manager
            .event_with_filter(
                Filter::new()
                    .event(&RdTaskCompletedFilter::abi_signature())
                    .from_block(from_block)
                    .to_block(to_block),
            )
            .query_with_meta()
            .await?;

        for (event, log) in events {
            if latest_completed_rd_task_number >= event.task_response.reference_task_index {
                continue;
            }

            // Here expected_rd_task_number == event.task_response.reference_task_index
            if event.task_response.chain_id == self.target_chain_index {
                if *expected_batch_id != event.task_response.batch_id {
                    return Err(eyre!("missing expected_batch_id {:?}", expected_batch_id));
                }
                merkle_roots.push(event.task_response.rd_update);
                ranges.push(Range {
                    start: event.task_response.range_start,
                    end: event.task_response.range_end,
                });
                *expected_batch_id += 1;
                *last_log = Some(log);
            }
        }
        Ok(())
    }

    pub(crate) async fn get_operators_state_info(
        self: Arc<Self>,
        task: OpTask,
    ) -> eyre::Result<OperatorStateInfo> {
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
                        stake_per_quorum,
                    });
                avs_state
                    .stake_per_quorum
                    .insert(*quorum_num, operator.stake);
            }
        }

        operators_avs_state
    }
}
