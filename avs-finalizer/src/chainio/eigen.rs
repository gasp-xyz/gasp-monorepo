use std::{fmt::Debug, ops::Add, sync::Arc};

use bindings::{
    delegation_manager::DelegationManager, registry_coordinator::RegistryCoordinator,
    shared_types::OperatorDetails, stake_registry::StakeRegistry,
    strategy_manager_storage::SignatureWithSaltAndExpiry,
};
use ethers::{
    providers::Middleware,
    types::{Address, TransactionReceipt},
};
use eyre::{Ok, OptionExt};
use rand::RngCore;
use sp_core::U256;

use crate::cli::CliArgs;

use super::Client;

pub struct ElContracts {
    delegation: DelegationManager<Client>,
    service_manager_address: Address,
    client: Arc<Client>,
}

impl Debug for ElContracts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElContracts")
            .field("delegation", &self.delegation.address())
            .finish()
    }
}

impl ElContracts {
    pub async fn build(cfg: &CliArgs, client: Arc<Client>) -> eyre::Result<Self> {
        let registry = RegistryCoordinator::new(cfg.avs_registry_coordinator_addr, client.clone());
        let service_addr = registry.service_manager().await?;
        let stake_registry_addr = registry.stake_registry().await?;
        let stake_registry = StakeRegistry::new(stake_registry_addr, client.clone());
        let delegation_addr = stake_registry.delegation().await?;
        let delegation = DelegationManager::new(delegation_addr, client.clone());

        Ok(Self {
            delegation,
            service_manager_address: service_addr,
            client,
        })
    }

    pub async fn is_operator_registered(&self, operator_address: Address) -> eyre::Result<bool> {
        Ok(self.delegation.is_operator(operator_address).await?)
    }

    pub async fn register_as_operator_with_el(
        &self,
        operator_address: Address,
    ) -> eyre::Result<TransactionReceipt> {
        let op_details = OperatorDetails {
            earnings_receiver: operator_address,
            ..Default::default()
        };
        let tx = self
            .delegation
            .register_as_operator(op_details, String::new());

        let pending = tx.send().await?;
        let receipt = pending.await?;

        receipt.ok_or_eyre("register_as_operator_with_el trx failed")
    }

    pub async fn get_delegation_signature_params(
        &self,
    ) -> eyre::Result<SignatureWithSaltAndExpiry> {
        let salt = &mut [0_u8; 32];
        rand::thread_rng().fill_bytes(salt);

        let at = self.client.get_block_number().await?;
        let block = self
            .client
            .get_block(at)
            .await?
            .ok_or_eyre("block should be found for known number")?;

        let expiry = block.timestamp.add(U256::from(30));

        let digest = self
            .delegation
            .calculate_operator_avs_registration_digest_hash(
                self.client.address(),
                self.service_manager_address,
                salt.clone(),
                expiry,
            )
            .await?;

        let sig: [u8; 65] = self
            .client
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
