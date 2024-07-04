use std::{fmt::Debug, sync::Arc};

use bindings::{
    finalizer_service_manager::FinalizerServiceManager,
    finalizer_task_manager::{FinalizerTaskManager, NewTaskCreatedFilter, TaskCompletedFilter},
    registry_coordinator::RegistryCoordinator,
    shared_types::OperatorInfo,
    stake_registry::StakeRegistry,
    bls_apk_registry::BLSApkRegistry,
    strategy_manager_storage::{PubkeyRegistrationParams, SignatureWithSaltAndExpiry},
};
use ethers::{
    contract::{EthEvent, Event},
    providers::{Provider, Ws},
    types::{Address, Filter, TransactionReceipt, H256, U64},
};
use eyre::{eyre, Ok, OptionExt};

use crate::{
    cli::CliArgs,
};

use super::{SourceClient as Client};

pub struct AvsContracts {
    pub service_manager: FinalizerServiceManager<Client>,
    pub task_manager: FinalizerTaskManager<Client>,
    pub task_manager_sub: FinalizerTaskManager<Provider<Ws>>,
    pub registry_coordinator_address: Address,
    pub registry: RegistryCoordinator<Client>,
    pub stake_registry: StakeRegistry<Client>,
    pub bls_apk_registry: BLSApkRegistry<Client>,
    pub client: Arc<Client>,
    pub ws_client: Arc<Provider<Ws>>,
}

impl Debug for AvsContracts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AvsContracts")
            .field("service_manager", &self.service_manager.address())
            .field("task_manager", &self.task_manager.address())
            .field("task_manager_sub", &self.task_manager_sub.address())
            .field("registry", &self.registry.address())
            .finish()
    }
}

impl AvsContracts {

    pub async fn build(config: &CliArgs, client: Arc<Client>) -> eyre::Result<Self> {
        let ws_client = Arc::new(Provider::connect(config.source_ws_url.to_owned()).await?);

        let registry =
            RegistryCoordinator::new(config.avs_registry_coordinator_addr, client.clone());

        let service_manager_addr = registry.service_manager().await?;
        let service_manager = FinalizerServiceManager::new(service_manager_addr, client.clone());

        let task_manager_addr = service_manager.task_manager().await?;
        let task_manager = FinalizerTaskManager::new(task_manager_addr, client.clone());
        let task_manager_sub = FinalizerTaskManager::new(task_manager_addr, ws_client.clone());

        let bls_apk_registry_addr = registry.bls_apk_registry().await?;
        let bls_apk_registry = BLSApkRegistry::new(bls_apk_registry_addr, client.clone());
        let stake_registry_addr = registry.stake_registry().await?;
        let stake_registry = StakeRegistry::new(stake_registry_addr, client.clone());

        Ok(Self {
            service_manager,
            task_manager,
            task_manager_sub,
            registry_coordinator_address: config.avs_registry_coordinator_addr.into(),
            registry,
            stake_registry,
            bls_apk_registry,
            client,
            ws_client,
        })
    }

    // TODO
    // Maybe add the task cancel event stream to check against that for early  exit
    pub fn source_response_stream(&self, from_block: u32) -> Event<Arc<Provider<Ws>>, Provider<Ws>, TaskCompletedFilter> {
        self.task_manager_sub
            .event_with_filter(Filter::new().event(&TaskCompletedFilter::abi_signature()).from_block(u64::from(from_block)))
    }
}
