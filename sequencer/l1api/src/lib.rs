use alloy::{network::EthereumWallet, providers::{PendingTransactionError, Provider, ProviderBuilder, WalletProvider}, signers::local::PrivateKeySigner, sol_types::SolValue};
use primitive_types::H256;
use hex::encode as hex_encode;

mod lru;
mod rolldown_contract;

pub use lru::CachedL1Interface;
pub use rolldown_contract::RolldownContract;
use sha3::{Digest, Keccak256};

pub mod types {
    pub use contract_bindings::rolldown::IRolldownPrimitives::Cancel;
    pub use contract_bindings::rolldown::IRolldownPrimitives::L1Update;
}

#[derive(Debug, thiserror::Error)]
pub enum L1Error {
    #[error("Invalid range")]
    InvalidRange,
    #[error("Overflow error")]
    OverflowError,
    #[error("alloy error")]
    Alloy(#[from] alloy::contract::Error),
    #[error("alloy error")]
    TransportAlloy(#[from] alloy::transports::TransportError),
    #[error("transaction error")]
    TxSendError(#[from] PendingTransactionError),
}

pub trait L1Interface {
    // fn account_address(&self) -> [u8; 20];
    async fn is_closed(&self, request_hash: H256) -> Result<bool, L1Error>;
    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error>;
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error>;
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;
    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error>;
}

    // async fn estimate_gas_in_wei(&self) -> Result<(u128, u128), L1Error>;
    // async fn get_native_balance(&self, address: [u8; 20]) -> Result<u128, L1Error>;


pub struct L1Builder {
    pub uri: &'static str,
    pub pkey: [u8; 32],
    pub rolldown_address: [u8; 20],
}

impl L1Builder {
    pub async fn build(
        self,
    ) -> Result<L1<impl WalletProvider + Provider + Clone>, L1Error> {
        let signer: PrivateKeySigner = hex::encode(self.pkey).parse().expect("valid private key");
        let wallet = EthereumWallet::new(signer);
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_builtin(self.uri)
            .await?;
        let rolldown = RolldownContract::new(provider.clone(), self.rolldown_address);
        Ok(L1{
            rolldown_contract: rolldown,
        })
    }
}

pub struct L1<P> {
    rolldown_contract: RolldownContract<P>
}

impl<P> L1Interface for L1<P>
where
    P: Provider + WalletProvider
{
    // fn account_address(&self) -> [u8; 20] {
    //     self.account_address
    // }

    #[tracing::instrument(skip(self))]
    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error> {
        let merkle_root = self.rolldown_contract.find_l2_batch(request_id).await?;
        let range = self.rolldown_contract.get_request_range_from_merkle_root(merkle_root).await?;
        Ok((merkle_root, range))
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        let next_request_id = self.rolldown_contract.get_next_request_id().await?;

        if next_request_id == 1 {
            Ok(None)
        } else {
            Ok(next_request_id.checked_sub(1u128))
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error> {
        let length = self.rolldown_contract.get_amount_of_merkle_roots().await?;

        tracing::trace!("there are {} merkle roots on the contract", length);
        if let Some(id) = length.checked_sub(1) {
            let latest_root = self.rolldown_contract.get_merkle_root_by_id(id).await?;
            tracing::trace!("latest merkle root {}", hex_encode(latest_root));
            let (_, latest_request_id) =
                self.rolldown_contract.get_request_range_from_merkle_root(latest_root).await?;

            tracing::trace!(
                "latest request in root {}: {}",
                hex_encode(latest_root),
                latest_request_id
            );

            Ok(Some(latest_request_id))
        } else {
            tracing::trace!("latest request: None");
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error> {
        let latest = self.get_latest_reqeust_id().await?;

        if let Some(latest) = latest {
            if start < 1u128 || start > latest || end > latest || end < start {
                tracing::warn!(
                    "latest :{} range.start:{} range.end:{} ",
                    latest,
                    start,
                    end
                );
                return Err(L1Error::InvalidRange);
            }

            let update = self.rolldown_contract.get_update_impl(start, end).await?;

            tracing::debug!(
                "deposits: {} cancel_resolutions: {}",
                update.pendingDeposits.len(),
                update.pendingCancelResolutions.len()
            );

            Ok(update)
        } else {
            tracing::warn!("there are no requests in contract yet");
            Err(L1Error::InvalidRange)
        }
    }

    // #[tracing::instrument(skip(self))]
    // async fn estimate_gas_in_wei(&self) -> Result<(u128, u128), L1Error> {
    //     // https://www.blocknative.com/blog/eip-1559-fees
    //     // We do not want client to estimate we would like to make our own estimate
    //     // based on this equation: Max Fee = (2 * Base Fee) + Max Priority Fee
    //
    //     // Max Fee = maxFeePerGas (client)
    //     // Max Priority Fee = maxPriorityFeePerGas (client)
    //
    //     let base_fee_in_wei = self.contract_handle.provider().get_gas_price().await?;
    //     let max_priority_fee_per_gas_in_wei = self
    //         .contract_handle
    //         .provider()
    //         .get_max_priority_fee_per_gas()
    //         .await?;
    //     let max_fee_in_wei = base_fee_in_wei
    //         .saturating_mul(2)
    //         .saturating_add(max_priority_fee_per_gas_in_wei);
    //     Ok((max_fee_in_wei, max_priority_fee_per_gas_in_wei))
    // }

    #[tracing::instrument(skip(self, cancel))]
    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let (max_fee_per_gas_in_wei, max_priority_fee_per_gas_in_wei) = todo!();
        self.rolldown_contract.send_close_cancel_tx(cancel, merkle_root, proof, max_fee_per_gas_in_wei, max_priority_fee_per_gas_in_wei).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let pending_update = self.get_update(start, end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }

    #[tracing::instrument(skip(self))]
    async fn is_closed(&self, request_hash: H256) -> Result<bool, L1Error> {
        let is_closed = self.rolldown_contract.is_request_closed(request_hash).await?;
        tracing::trace!(is_closed);
        return Ok(is_closed);
    }

    // #[tracing::instrument(skip(self))]
    // async fn get_native_balance(&self, address: [u8; 20]) -> Result<u128, L1Error> {
    //     self.get_native_account_balance(address).await
    // }
}

// impl<P> L1Interface for L1<P> where 
// P: Provider{
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use hex_literal::hex;
//     use serial_test::serial;
//
//     const URI: &str = "http://localhost:8545";
//     const ROLLDOWN_ADDRESS: [u8; 20] = hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f");
//     const ALICE_PKEY: [u8; 32] =
//         hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");
//
//     #[serial]
//     #[tokio::test]
//     async fn test_can_connect() {
//         let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
//         RolldownContract::from_provider(ROLLDOWN_ADDRESS, provider);
//     }
//
//     #[serial]
//     #[tokio::test]
//     async fn test_can_latest_request_id() {
//         let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
//         let rolldown = RolldownContract::from_provider(ROLLDOWN_ADDRESS, provider);
//         rolldown.deposit(1000, 10).await.unwrap();
//         rolldown
//             .get_latest_reqeust_id()
//             .await
//             .expect("can fetch request");
//     }
//
//     #[serial]
//     #[tokio::test]
//     async fn test_can_fetch_balance() {
//         let provider = create_provider(URI, ALICE_PKEY).await.unwrap();
//         let rolldown = RolldownContract::from_provider(ROLLDOWN_ADDRESS, provider);
//
//         let balance = rolldown
//             .get_native_balance(hex!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266"))
//             .await
//             .unwrap();
//         assert!(balance > 0u128);
//     }
//
//     #[serial]
//     #[tokio::test]
//     async fn test_builder() {
//         let foo = FooBuilder {
//             uri: URI,
//             pkey: ALICE_PKEY,
//             address: ROLLDOWN_ADDRESS,
//         }.build().await.unwrap();
//
//         let d = foo.deposit(100u128, 1u128).await.unwrap();
//
//     }
// }
