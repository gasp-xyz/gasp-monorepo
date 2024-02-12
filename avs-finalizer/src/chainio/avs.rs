use std::{fmt::Debug, sync::Arc};

use bindings::{
    bls_registry_coordinator_with_indices::BLSRegistryCoordinatorWithIndices,
    mangata_service_manager::MangataServiceManager,
    mangata_task_manager::{MangataTaskManager, NewTaskCreatedFilter},
    shared_types::Operator,
};
use ethers::{
    contract::Event,
    providers::{Provider, Ws},
    types::{Address, TransactionReceipt, H256},
};
use eyre::{Ok, OptionExt};

use crate::{
    cli::CliArgs,
    crypto::{bn254::BlsKeypair, EthConvert},
};

use super::Client;

pub struct AvsContracts {
    service_manager: MangataServiceManager<Client>,
    task_manager: MangataTaskManager<Client>,
    task_manager_sub: MangataTaskManager<Provider<Ws>>,
    registry: BLSRegistryCoordinatorWithIndices<Client>,
    client: Arc<Client>,
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
    const QUORUM: [u8; 1] = [0_u8; 1];

    pub async fn build(config: &CliArgs, client: Arc<Client>) -> eyre::Result<Self> {
        let ws = Arc::new(Provider::connect(config.eth_ws_url.to_owned()).await?);

        let service_manager =
            MangataServiceManager::new(config.avs_service_manager_addr, client.clone());

        let task_manager_addr = service_manager.task_manager().await?;
        let task_manager = MangataTaskManager::new(task_manager_addr, client.clone());
        let task_manager_sub = MangataTaskManager::new(task_manager_addr, ws);

        let registry_addr = service_manager.registry_coordinator().await?;
        let registry = BLSRegistryCoordinatorWithIndices::new(registry_addr, client.clone());

        Ok(Self {
            service_manager,
            task_manager,
            task_manager_sub,
            registry,
            client,
        })
    }

    pub fn new_task_stream(&self) -> Event<Arc<Provider<Ws>>, Provider<Ws>, NewTaskCreatedFilter> {
        self.task_manager_sub.new_task_created_filter()
    }

    pub async fn slasher_address(&self) -> eyre::Result<Address> {
        Ok(self.service_manager.slasher().await?)
    }

    pub async fn operator_id(&self) -> eyre::Result<Option<H256>> {
        let status: Operator = self.registry.get_operator(self.client.address()).await?;
        let id: H256 = status.operator_id.into();
        if id.is_zero() || status.status != 1_u8 {
            Ok(None)
        } else {
            Ok(Some(id))
        }
    }

    pub async fn register_with_avs(
        &self,
        keypair: &BlsKeypair,
    ) -> eyre::Result<TransactionReceipt> {
        let op_address =
            EthConvert::to_g1(keypair.public).ok_or_eyre("cannot convert G1 public")?;
        let trx = self.registry.register_operator_with_coordinator_1(
            AvsContracts::QUORUM.into(),
            op_address,
            String::new(),
        );

        let pending = trx.send().await?;
        let receipt = pending.await?;

        receipt.ok_or_eyre("register_with_avs trx failed")
    }

    pub async fn deregister_with_avs(
        &self,
        keypair: &BlsKeypair,
    ) -> eyre::Result<TransactionReceipt> {
        let op_address =
            EthConvert::to_g1(keypair.public).ok_or_eyre("cannot convert G1 public")?;
        let trx = self
            .registry
            .deregister_operator_with_coordinator(AvsContracts::QUORUM.into(), op_address);

        let pending = trx.send().await?;
        let receipt = pending.await?;

        receipt.ok_or_eyre("register_with_avs trx failed")
    }
}
