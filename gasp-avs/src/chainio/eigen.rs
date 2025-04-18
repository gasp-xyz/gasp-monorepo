use std::{fmt::Debug, ops::Add, sync::Arc};

use bindings::{
    avs_directory::AVSDirectory,
    delegation_manager::DelegationManager,
    finalizer_service_manager::FinalizerServiceManager,
    registry_coordinator::RegistryCoordinator,
    shared_types::{OperatorDetails, SignatureWithSaltAndExpiry},
    stake_registry::StakeRegistry,
};
use ethers::{
    providers::Middleware,
    types::{Address, TransactionReceipt, U64},
};
use eyre::{eyre, Ok, OptionExt};
use rand::RngCore;
use sp_core::U256;

use crate::cli::CliArgs;

use super::{map_revert, Client, SignerClient};

pub struct ElContracts {
    delegation: DelegationManager<Client>,
    avs_directory: AVSDirectory<Client>,
    service_manager_address: Address,
}

impl Debug for ElContracts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElContracts")
            .field("delegation", &self.delegation.address())
            .field("avs_director", &self.avs_directory.address())
            .finish()
    }
}

impl ElContracts {
    pub async fn build(cfg: &CliArgs, client: Arc<Client>) -> eyre::Result<Self> {
        let registry = RegistryCoordinator::new(cfg.avs_registry_coordinator_addr, client.clone());
        let service_addr = registry.service_manager().await?;
        let service = FinalizerServiceManager::new(service_addr, client.clone());
        let stake_registry_addr = registry.stake_registry().await?;
        let stake_registry = StakeRegistry::new(stake_registry_addr, client.clone());
        let delegation_addr = stake_registry.delegation().await?;
        let delegation = DelegationManager::new(delegation_addr, client.clone());
        let avs_directory_addr = service.avs_directory().await?;
        let avs_directory = AVSDirectory::new(avs_directory_addr, client.clone());

        Ok(Self {
            delegation,
            avs_directory,
            service_manager_address: service_addr,
        })
    }

    pub async fn is_operator_registered(&self, operator_address: Address) -> eyre::Result<bool> {
        Ok(self.delegation.is_operator(operator_address).await?)
    }

    pub async fn register_as_operator_with_el(
        &self,
        client: Arc<SignerClient>,
    ) -> eyre::Result<TransactionReceipt> {
        let delegation = DelegationManager::new(self.delegation.address(), client.clone());
        let op_details = OperatorDetails {
            deprecated_earnings_receiver: client.address(),
            ..Default::default()
        };
        let tx = delegation.register_as_operator(op_details, String::new());

        let pending = tx.send().await.map_err(map_revert)?;
        let receipt = pending.await?;

        if Some(U64::zero()) == receipt.as_ref().and_then(|r| r.status) {
            return Err(eyre!("register_as_operator_with_el tx failed"));
        }

        receipt.ok_or_eyre("register_as_operator_with_el trx failed")
    }

    pub async fn get_delegation_signature_params(
        &self,
        client: &Arc<SignerClient>,
    ) -> eyre::Result<SignatureWithSaltAndExpiry> {
        let salt = &mut [0_u8; 32];
        rand::thread_rng().fill_bytes(salt);

        let at = client.get_block_number().await?;
        let block = client
            .get_block(at)
            .await?
            .ok_or_eyre("block should be found for known number")?;

        let expiry = block.timestamp.add(U256::from(60));

        let digest = self
            .avs_directory
            .calculate_operator_avs_registration_digest_hash(
                client.address(),
                self.service_manager_address,
                *salt,
                expiry,
            )
            .await?;

        let sig: [u8; 65] = client
            .signer()
            .sign_hash(sp_core::H256::from(digest))?
            .into();

        Ok(SignatureWithSaltAndExpiry {
            signature: sig.into(),
            salt: *salt,
            expiry,
        })
    }
}
