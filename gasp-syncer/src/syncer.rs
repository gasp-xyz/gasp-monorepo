use crate::chainio::{avs::AvsContracts, build_clients, SourceClient, TargetClient};
use crate::cli::CliArgs;

use bindings::{
    finalizer_task_manager::{NewTaskCreatedFilter, TaskRespondedFilter, Operator as TMOperator,
        OperatorStateInfo, QuorumsAdded, QuorumsStakeUpdate, QuorumsApkUpdate, OperatorsAdded, OperatorsQuorumCountUpdate, OperatorsStakeUpdate},
    shared_types::{G1Point, G2Point, Task, TaskResponse},
    gasp_multi_rollup_service::GaspMultiRollupService
    
};
use ethers::providers::{Middleware, PendingTransaction, SubscriptionStream};
use ethers::{
    contract::{stream, LogMeta},
    providers::StreamExt,
    types::{Address, U256, Bytes}, abi::AbiDecode
};

use serde::Serialize;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, Keccak256, Hash};
use sp_runtime::{generic, OpaqueExtrinsic};
use std::sync::Arc;
use tokio::select;
use tokio::time::{sleep, Duration};
use tokio::try_join;
use tracing::{debug, error, info, instrument};
use std::collections::HashMap;
use ethers::abi::AbiEncode;
use eyre::eyre;


#[derive(Debug)]
pub struct Syncer {
    pub source_client: Arc<SourceClient>,
    pub target_client: Arc<TargetClient>,
    avs_contracts: AvsContracts,
    gasp_service_contract: GaspMultiRollupService<TargetClient>,
}
impl Syncer {
    #[instrument(name = "create_syncer", skip_all)]
    pub async fn from_cli(cfg: &CliArgs) -> eyre::Result<Arc<Self>> {
        let (source_client, target_client) = build_clients(cfg).await?;
        let (source_client, target_client) = (Arc::new(source_client), Arc::new(target_client));
        let avs_contracts = AvsContracts::build(cfg, source_client.clone()).await?;
        let gasp_service_contract = GaspMultiRollupService::new(cfg.gasp_service_addr, target_client.clone());

        Ok(Arc::new(Self {
            source_client,
            target_client,
            avs_contracts,
            gasp_service_contract
        }))
    }

    #[instrument(skip_all)]
    pub async fn sync(self: Arc<Self>) -> eyre::Result<()> {

        let latest_completed_task_number = self.gasp_service_contract.latest_completed_task_number().await?;
        let latest_completed_task_created_block = self.gasp_service_contract.latest_completed_task_created_block().await?;

        let evs = self.clone().avs_contracts.source_response_stream(latest_completed_task_created_block);
        let mut stream: stream::EventStream<'_, _, (TaskRespondedFilter, LogMeta), _> =
            evs.subscribe_with_meta().await?;

        // event stream does not finish with `None` after websocket closure, use block subscription for it
        let mut blocks: SubscriptionStream<'_, _, _> =
            self.avs_contracts.ws_client.subscribe_blocks().await?;

        loop {
            select! {
                Some(event) = stream.next() => match event {
                    Ok((event, log)) if event.task.taskNum == latest_completed_task_number => {
                        let txn_hash = log.transaction_hash;
                        let txn = self.avs_contracts.ws_client.get_transaction(txn_hash).await.map_err(
                            |e| {tracing::error!("missing expected txn {:?}", txn_hash);
                            Err(eyre!("missing expected txn {:?}", txn_hash))}
                        )?;
                        println!("{:?}", txn);
                        return Ok(())
                    }
                    Ok((event, log)) if event.task.taskNum > latest_completed_task_number => {
                        tracing::error!("missing expected task response {:?}", latest_completed_task_number);
                        return Err(eyre!("missing expected task response {:?}", latest_completed_task_number))
                    }
                    Err(e) => tracing::error!("EthWs subscription error {:?}", e),
                    _ => {
                        // No task can be responded to after another task has been created
                        tracing::error!("missing expected task response {:?}", latest_completed_task_number);
                        return Err(eyre!("missing expected task response {:?}", latest_completed_task_number))
                    }
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


    pub async fn handle_stall(
    ) -> eyre::Result<()> {
        Ok(())
    }

}
