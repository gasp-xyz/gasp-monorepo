use alloy::network::{Ethereum, EthereumWallet};
use alloy::signers::local::PrivateKeySigner;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use hex::encode as hex_encode;
use alloy::sol_types::SolValue;
use alloy::transports::BoxTransport;
use primitive_types::H256;
use sha3::{Digest, Keccak256};

use alloy::providers::{Identity, PendingTransactionError, ProviderBuilder, RootProvider};

pub mod types {
    pub use bindings::rolldown::IRolldownPrimitives::Cancel;
    pub use bindings::rolldown::IRolldownPrimitives::L1Update;
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

pub type RolldownInstanceType = bindings::rolldown::Rolldown::RolldownInstance<
    BoxTransport,
    FillProvider<
        JoinFill<
            JoinFill<Identity,
                JoinFill<GasFiller, 
                    JoinFill<BlobGasFiller, 
                        JoinFill<NonceFiller, ChainIdFiller>
                    >
                >,
            >,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider<BoxTransport>,
        BoxTransport,
        Ethereum,
    >,
>;


pub struct RolldownContract {
    contract_handle: RolldownInstanceType,
}

impl RolldownContract {
    pub async fn new(uri: &str, address: [u8; 20], private_key: [u8; 32]) -> Result<Self, L1Error> {

        let signer: PrivateKeySigner = hex::encode(private_key).parse().expect("valid private key");
        let wallet = EthereumWallet::new(signer);
        let provider = ProviderBuilder::new()
            // .wallet(hex!("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"))
            .with_recommended_fillers()
            .wallet(wallet)
            .on_builtin(uri)
            .await?;
        Ok(Self {
            contract_handle: bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider.clone(),
            )
        })
    }

    #[cfg(test)]
    #[tracing::instrument(skip(self))]
    pub async fn deposit(&self, amount: u128, ferry_tip: u128) -> Result<(), L1Error> {
        let call = self.contract_handle.deposit_native_1(
                alloy::primitives::U256::from(ferry_tip)
        ).value(alloy::primitives::U256::from(amount));

        let hash = call.send().await?
            .watch().await?;
        tracing::debug!("deposit hash: {}", hex_encode(hash));

        Ok(())
    }

}

impl L1Interface for RolldownContract {
    #[tracing::instrument(skip(self))]
    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error>{

        let request_id = alloy::primitives::U256::from(request_id);
        let call = self.contract_handle.find_l2_batch(request_id);
        let merkle_root = call.call().await?._0;

        let call = self.contract_handle.merkleRootRange(merkle_root);
        let range = call.call().await?;

        let range_start = range.start.try_into().or_else(|_| Err(L1Error::OverflowError))?;
        let range_end = range.end.try_into().or_else(|_| Err(L1Error::OverflowError))?;
        Ok((merkle_root.0, (range_start, range_end)))
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        let call = self.contract_handle.counter();
        let result = call.call().await?;
        let next_request_id: u128 = result
            ._0
            .try_into()
            .or_else(|_| Err(L1Error::OverflowError))?;
        if next_request_id == 1 {
            Ok(None)
        }else{
            Ok(next_request_id.checked_sub(1u128))
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error> {
        let call = self.contract_handle.getMerkleRootsLength();

        let length = call.call().await?._0;
        tracing::trace!("there are {} merkle roots on the contract", length);
        if let Some(id) = length.checked_sub(alloy::primitives::U256::from(1)) {
            let latest_root = self.contract_handle.roots(id.clone()).call().await?._0;
            tracing::trace!("latest merkle root {}", hex_encode(latest_root));
            let range = self
                .contract_handle
                .merkleRootRange(latest_root)
                .call()
                .await?;
            let latest: u128 =
                range
                    .end
                    .try_into()
                    .or_else(|_| Err(L1Error::OverflowError))?;
            tracing::trace!("latest request in root {}: {}", hex_encode(latest_root), latest);

            Ok(Some(latest))
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

            let range_start = alloy::primitives::U256::from(start);
            let range_end = alloy::primitives::U256::from(end);
            let call = self
                .contract_handle
                .getPendingRequests(range_start, range_end);
            let result = call.call().await?;

            tracing::debug!(
                "deposits: {} cancel_resolutions: {}",
                result._0.pendingDeposits.len(),
                result._0.pendingCancelResolutions.len()
            );

            Ok(result._0)
        } else {
            tracing::warn!("there are no requests in contract yet");
            Err(L1Error::InvalidRange)
        }
    }

    #[tracing::instrument(skip(self, cancel))]
    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let call = self
            .contract_handle
            .close_cancel(cancel, merkle_root.0.into(), proof);
        match call.call().await {
            Ok(_) => {
                tracing::trace!("status ok");
                Ok(call.send().await?.watch().await?.0.into())
            }
            Err(err) => {
                tracing::warn!("status nok");
                Err(err.into())
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let pending_update = self.get_update(start, end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }

    #[tracing::instrument(skip(self))]
    async fn is_closed(&self, request_hash: H256) -> Result<bool, L1Error>{

        let request_hash = request_hash.0.into();
        let request_status = self
            .contract_handle
            .processedL2Requests(request_hash).call().await
            .map(|elem| elem._0)?;

        let closed = self.contract_handle.CLOSED().call().await?._0;
        let is_closed = request_status == closed;

        tracing::trace!("is_closed: {} ({})", is_closed, hex_encode(request_status));
        return Ok(is_closed);

    }
}

#[cfg(test)]
mod test{
    use super::*;
    use hex_literal::hex;
    use serial_test::serial;

    const URI: &'static str = "http://localhost:8545";
    const ROLLDOWN_ADDRESS: [u8; 20] = hex!("1429859428C0aBc9C2C47C8Ee9FBaf82cFA0F20f");
    const ALICE_PKEY: [u8; 32] = hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");
    // const ETHEREUM: l2types::Chain = l2types::Chain::Ethereum;
    // const ARBITRUM: l2types::Chain = l2types::Chain::Arbitrum;

    // async fn get_update(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error>;
    // async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
    // async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
    // async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;
    // async fn close_cancel( &self, cancel: types::Cancel, merkle_root: H256, proof: Vec<H256>) -> Result<H256, L1Error>;

    #[serial]
    #[tokio::test]
    async fn test_can_connect() {
        RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY).await.unwrap();
    }

    #[serial]
    #[tokio::test]
    async fn test_can_latest_request_id() {
        let rolldown = RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY).await.unwrap();
        rolldown.deposit(1000, 10).await.unwrap();
        assert!(matches!(rolldown.get_latest_reqeust_id().await.expect("can fetch request"), Some(latest) if latest > 0));
    }

    #[serial]
    #[tokio::test]
    async fn test_can_fetch_update_and_update_hash() {
        use alloy::sol_types::SolValue;

        let rolldown = RolldownContract::new(URI, ROLLDOWN_ADDRESS, ALICE_PKEY).await.unwrap();
        rolldown.deposit(1000, 10).await.unwrap();
        let latest_update_first= rolldown.get_latest_reqeust_id().await.expect("can fetch request").unwrap();

        rolldown.deposit(1000, 10).await.unwrap();
        let latest_update_second= rolldown.get_latest_reqeust_id().await.expect("can fetch request").unwrap();

        assert!(latest_update_first < latest_update_second);

        let update1 = rolldown.get_update(1u128, latest_update_first).await.unwrap();
        let update2 = rolldown.get_update(1u128, latest_update_second).await.unwrap();
        let hash1 = rolldown.get_update_hash(1u128, latest_update_first).await.unwrap();
        let hash2 = rolldown.get_update_hash(1u128, latest_update_second).await.unwrap();

        assert!(hash1 != hash2);
        assert!(update1.abi_encode() != update2.abi_encode());
    }

}
