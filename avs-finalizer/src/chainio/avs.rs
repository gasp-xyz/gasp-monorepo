use std::{fmt::Debug, sync::Arc};

use bindings::{
    finalizer_service_manager::FinalizerServiceManager,
    finalizer_task_manager::{FinalizerTaskManager, NewTaskCreatedFilter},
    registry_coordinator::RegistryCoordinator,
    shared_types::OperatorInfo,
    strategy_manager_storage::{PubkeyRegistrationParams, SignatureWithSaltAndExpiry},
};
use ethers::{
    contract::{EthEvent, Event},
    providers::{Provider, Ws},
    types::{Filter, TransactionReceipt, H256, U64},
};
use eyre::{eyre, Ok, OptionExt};

use crate::{
    cli::CliArgs,
    crypto::{bn254::BlsKeypair, EthConvert},
};

use super::{map_revert, Client};

pub struct AvsContracts {
    service_manager: FinalizerServiceManager<Client>,
    task_manager: FinalizerTaskManager<Client>,
    task_manager_sub: FinalizerTaskManager<Provider<Ws>>,
    registry: RegistryCoordinator<Client>,
    avs_registry_chain_reader: AvsRegistryChainReader,
    avs_registry_subscriber: AvsRegistryChainSubscriber,
    operator_info: OperatorInfoServiceInMemory,
    avs_registry_service_chain_caller: AvsRegistryServiceChainCaller,
    client: Arc<Client>,
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
    const QUORUM: [u8; 1] = [0_u8; 1];

    pub async fn build(config: &CliArgs, client: Arc<Client>) -> eyre::Result<Self> {
        let ws_client = Arc::new(Provider::connect(config.eth_ws_url.to_owned()).await?);

        let registry =
            RegistryCoordinator::new(config.avs_registry_coordinator_addr, client.clone());

        let service_manager_addr = registry.service_manager().await?;
        let service_manager = FinalizerServiceManager::new(service_manager_addr, client.clone());

        let task_manager_addr = service_manager.task_manager().await?;
        let task_manager = FinalizerTaskManager::new(task_manager_addr, client.clone());
        let task_manager_sub = FinalizerTaskManager::new(task_manager_addr, ws_client.clone());

        let bls_apk_registry_addr = registry.bls_apk_registry().await?;
        let stake_registry_addr = registry.stake_registry().await?;
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            bls_apk_registry_addr,
            config.avs_registry_coordinator_addr,
            task_manager_addr,
            stake_registry_addr,
            config.eth_rpc_url.to_string(),
        );

        let avs_registry_subscriber = AvsRegistryChainSubscriber::new(config.eth_ws_url.to_string());

        let operators_info = Arc::new(Mutex::new(
            OperatorInfoServiceInMemory::new(
                avs_registry_subscriber,
                avs_registry_chain_reader,
                config.eth_ws_url.to_string(),
            )
            .await,
        ));
        let operators_info_clone = Arc::clone(&operators_info);
        // start the service with a particular block range
        // from block : 0
        // to block : 0 means current block , else normal
        task::spawn(async move {
            let operators_info = operators_info_clone.lock().await;
            operators_info.start_service(0, 0).await
        });
        
        let avs_registry_service_chain_caller = AvsRegistryServiceChainCaller::new(avs_registry_chain_reader, operators_info);

        Ok(Self {
            service_manager,
            task_manager,
            task_manager_sub,
            registry,
            avs_registry_chain_reader,
            avs_registry_subscriber,
            operator_info,
            avs_registry_service_chain_caller,
            client,
            ws_client,
        })
    }

    pub fn new_task_stream(&self) -> Event<Arc<Provider<Ws>>, Provider<Ws>, NewTaskCreatedFilter> {
        self.task_manager_sub
            .event_with_filter(Filter::new().event(&NewTaskCreatedFilter::abi_signature()))
    }

    pub async fn operator_id(&self) -> eyre::Result<Option<H256>> {
        let status: OperatorInfo = self.registry.get_operator(self.client.address()).await?;
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
        sig_params: SignatureWithSaltAndExpiry,
    ) -> eyre::Result<TransactionReceipt> {
        let message = self
            .registry
            .pubkey_registration_message_hash(self.client.address())
            .await?;
        let signed_hash = keypair.sign_hashed(EthConvert::from_g1(message))?;
        let (sig, g1, g2) = (
            EthConvert::to_g1(signed_hash).ok_or_eyre("cannot convert signed_hash")?,
            EthConvert::to_g1(keypair.public).ok_or_eyre("cannot convert G1 public")?,
            EthConvert::to_g2(keypair.public_g2()).ok_or_eyre("cannot convert G2 public")?,
        );

        let trx = self.registry.register_operator(
            AvsContracts::QUORUM.into(),
            String::new(),
            PubkeyRegistrationParams {
                pubkey_registration_signature: sig,
                pubkey_g1: g1,
                pubkey_g2: g2,
            },
            sig_params,
        );

        let pending = trx.send().await.map_err(map_revert)?;
        let receipt = pending.await?;

        if Some(U64::zero()) == receipt.as_ref().and_then(|r| r.status) {
            return Err(eyre!(
                "trx failed, possibly expired registration signature, check contract call on block explorer for details"
            ));
        }

        receipt.ok_or_eyre("register_with_avs trx failed")
    }

    pub async fn deregister_with_avs(&self) -> eyre::Result<TransactionReceipt> {
        let trx = self
            .registry
            .deregister_operator(AvsContracts::QUORUM.into());

        let pending = trx.send().await.map_err(map_revert)?;
        let receipt = pending.await?;

        receipt.ok_or_eyre("register_with_avs trx failed")
    }
}
