use crate::bindings::*;
use crate::config::NodeConfig;
use crate::executor::execute::execute_block;
use std::sync::Arc;
// use bindings::incredible_squaring_task_manager::IncredibleSquaringTaskManagerEvents::NewTaskCreatedFilter;
use bindings::incredible_squaring_task_manager::NewTaskCreatedFilter;
use ethers::prelude::*;
use node_executor::ExecutorDispatch;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::{generic, OpaqueExtrinsic};
pub type Header = generic::HeaderVer<node_primitives::BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

type MW = Provider<Ws>;

pub struct Operator {
    service_manager: ServiceManager<MW>,
    task_manager: TaskManager<MW>,
    client: Arc<MW>,
    substrate_client_uri: String,
}

impl Operator {
    pub async fn from_config(config: &NodeConfig) -> eyre::Result<Self> {
        let provider = MW::connect(&config.eth_ws_url).await?;
        let client = Arc::new(provider);
        let address: Address = config.avs_service_manager_address.parse()?;
        let service_manager = ServiceManager::new(address, client.clone());

        let address = service_manager
            .incredible_squaring_task_manager()
            .call()
            .await?;
        let task_manager = TaskManager::new(address, client.clone());

        Ok(Self {
            service_manager,
            task_manager,
            client,
            substrate_client_uri: config.substrate_rpc_url.clone(),
        })
    }

    pub async fn watch_new_tasks(&self) -> eyre::Result<()> {
        let evs = self.task_manager.new_task_created_filter();
        let mut stream: stream::EventStream<'_, _, NewTaskCreatedFilter, _> =
            evs.subscribe().await?;
        while let Some(Ok(event)) = stream.next().await {
            println!("{:?}", event);
        }
        Ok(())
    }

    pub(crate) async fn execute_block(&self) -> eyre::Result<()> {
        use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
        execute_block::<
            Block,
            ExtendedHostFunctions<
                sp_io::SubstrateHostFunctions,
                <ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
            >,
        >(&self.substrate_client_uri, 3697184_u32)
        .await?;

        Ok(())
    }
}
