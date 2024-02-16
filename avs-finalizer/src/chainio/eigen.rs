use std::{fmt::Debug, sync::Arc};

use bindings::{
    bls_public_key_compendium::BLSPublicKeyCompendium, delegation_manager::DelegationManager,
    shared_types::OperatorDetails, slasher::Slasher,
};
use ethers::types::{Address, TransactionReceipt};
use eyre::{Ok, OptionExt};

use crate::{
    cli::CliArgs,
    crypto::{bn254::BlsKeypair, EthConvert},
};

use super::Client;

pub struct ElContracts {
    delegation: DelegationManager<Client>,
    bls_pub_key: BLSPublicKeyCompendium<Client>,
    client: Arc<Client>,
}

impl Debug for ElContracts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ElContracts")
            .field("delegation", &self.delegation.address())
            .field("bls_pub_key", &self.bls_pub_key.address())
            .finish()
    }
}

impl ElContracts {
    pub async fn build(
        cfg: &CliArgs,
        slasher_addr: Address,
        client: Arc<Client>,
    ) -> eyre::Result<Self> {
        let slasher = Slasher::new(slasher_addr, client.clone());
        let delegation_addr = slasher.delegation().await?;
        let delegation = DelegationManager::new(delegation_addr, client.clone());

        let bls_pubkey_compendium =
            BLSPublicKeyCompendium::new(cfg.bls_compendium_addr, client.clone());

        Ok(Self {
            delegation,
            bls_pub_key: bls_pubkey_compendium,
            client,
        })
    }

    pub async fn is_operator_registered(&self, operator_address: Address) -> eyre::Result<bool> {
        Ok(self.delegation.is_operator(operator_address).await?)
    }

    pub async fn has_operator_pubkey(&self, operator_address: Address) -> eyre::Result<bool> {
        Ok(!self
            .bls_pub_key
            .operator_to_pubkey_hash(operator_address)
            .await?
            .is_empty())
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

    pub async fn register_bls_pub_key(
        &self,
        keypair: &BlsKeypair,
        chain_id: u64,
    ) -> eyre::Result<TransactionReceipt> {
        let signed_hash = keypair.make_pubkey_registration_data(
            self.client.address(),
            self.bls_pub_key.address(),
            chain_id,
        )?;
        let (hash, g1, g2) = (
            EthConvert::to_g1(signed_hash).ok_or_eyre("cannot convert signed_hash")?,
            EthConvert::to_g1(keypair.public).ok_or_eyre("cannot convert G1 public")?,
            EthConvert::to_g2(keypair.public_g2()).ok_or_eyre("cannot convert G2 public")?,
        );

        let tx = self.bls_pub_key.register_bls_public_key(hash, g1, g2);
        let pending = tx.send().await?;
        let receipt = pending.await?;

        receipt.ok_or_eyre("register_bls_pub_key trx failed")
    }
}
