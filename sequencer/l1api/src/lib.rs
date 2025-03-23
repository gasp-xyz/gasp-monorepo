use alloy::{
    network::{EthereumWallet, Network, NetworkWallet},
    providers::{PendingTransactionError, Provider, ProviderBuilder, WalletProvider},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
    transports::Transport,
};
use hex::encode as hex_encode;
use hex_literal::hex;
use primitive_types::H256;

mod erc20;
mod lru;
mod rolldown_contract;
mod utils;
use erc20::Erc20Token;
pub mod mock;
#[cfg(test)]
mod test;

pub use lru::CachedL1Interface;
pub use rolldown_contract::RolldownContract;
use sha3::{Digest, Keccak256};

pub mod types {
    pub mod abi {
        pub use contract_bindings::rolldown::IRolldownPrimitives::Cancel;
        pub use contract_bindings::rolldown::IRolldownPrimitives::ChainId;
        pub use contract_bindings::rolldown::IRolldownPrimitives::L1Update;
        pub use contract_bindings::rolldown::IRolldownPrimitives::Origin;
        pub use contract_bindings::rolldown::IRolldownPrimitives::Range;
        pub use contract_bindings::rolldown::IRolldownPrimitives::RequestId;
        pub use contract_bindings::rolldown::IRolldownPrimitives::Withdrawal;
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum RequestStatus {
        Pending,
        Ferried([u8; 20]),
        Closed,
    }
    pub use gasp_types::Deposit;
    pub use gasp_types::Withdrawal;
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
    #[error("Transaction execution failure `{0}`")]
    TxReverted(H256),
}

pub const NATIVE_TOKEN_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000001");

#[allow(async_fn_in_trait)]
pub trait L1Interface {
    async fn ferry_withdrawal(&self, withdrawal: gasp_types::Withdrawal) -> Result<H256, L1Error>;
    async fn erc20_balance(&self, token: [u8; 20], account: [u8; 20]) -> Result<u128, L1Error>;
    async fn native_balance(&self, account: [u8; 20]) -> Result<u128, L1Error>;
    async fn get_status(&self, request_hash: H256) -> Result<types::RequestStatus, L1Error>;
    async fn get_update(&self, start: u128, end: u128) -> Result<types::abi::L1Update, L1Error>;
    async fn get_deposit(&self, request_id: u128) -> Result<Option<types::Deposit>, L1Error>;
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error>;
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error>;
    async fn get_merkle_root(
        &self,
        request_id: u128,
    ) -> Result<Option<([u8; 32], (u128, u128))>, L1Error>;
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error>;

    async fn close_cancel(
        &self,
        cancel: gasp_types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error>;

    async fn close_withdrawal(
        &self,
        withdrawal: gasp_types::Withdrawal,
        cerkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error>;
}

pub async fn create_provider(
    uri: impl AsRef<str>,
    pkey: [u8; 32],
) -> Result<impl Provider + WalletProvider + Clone, L1Error> {
    let signer: PrivateKeySigner = hex::encode(pkey).parse().expect("valid private key");
    let wallet = EthereumWallet::new(signer);
    Ok(ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_builtin(uri.as_ref())
        .await?)
}

pub fn address(provider: impl Provider + WalletProvider + Clone) -> [u8; 20] {
    provider.wallet().default_signer_address().into()
}

pub struct L1<T, P, N> {
    rolldown_contract: RolldownContract<T, P, N>,
    provider: P,
}

impl<T, P, N> L1<T, P, N> {
    pub fn new(rolldown_contract: RolldownContract<T, P, N>, provider: P) -> Self {
        L1 {
            rolldown_contract,
            provider,
        }
    }
}

impl<T, P, N> L1Interface for L1<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + Clone + WalletProvider<N>,
    N: Network,
{
    #[tracing::instrument(skip(self), ret)]
    async fn get_deposit(&self, request_id: u128) -> Result<Option<types::Deposit>, L1Error> {
        unimplemented!()
    }

    #[tracing::instrument(skip(self), ret)]
    async fn erc20_balance(&self, token: [u8; 20], account: [u8; 20]) -> Result<u128, L1Error> {
        let token = Erc20Token::new(token, self.provider.clone());
        token.balance_of(account).await
    }

    #[tracing::instrument(skip(self), ret)]
    async fn native_balance(&self, account: [u8; 20]) -> Result<u128, L1Error> {
        let result = self.provider.get_balance(account.into()).await?;
        result.try_into().map_err(|_| L1Error::OverflowError)
    }

    async fn ferry_withdrawal(&self, withdrawal: gasp_types::Withdrawal) -> Result<H256, L1Error> {
        self.rolldown_contract
            .send_ferry_withdrawal(withdrawal)
            .await
    }

    #[tracing::instrument(skip(self))]
    async fn get_merkle_root(
        &self,
        request_id: u128,
    ) -> Result<Option<([u8; 32], (u128, u128))>, L1Error> {
        match self.rolldown_contract.find_l2_batch(request_id).await {
            Ok(merkle_root) => {
                let range = self
                    .rolldown_contract
                    .get_request_range_from_merkle_root(merkle_root)
                    .await?;
                Ok(Some((merkle_root, range)))
            }
            Err(L1Error::Alloy(alloy::contract::Error::TransportError(
                alloy::transports::RpcError::ErrorResp(payload),
            ))) if payload.code == 3 => Ok(None),
            Err(e) => Err(e),
        }
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
            let (_, latest_request_id) = self
                .rolldown_contract
                .get_request_range_from_merkle_root(latest_root)
                .await?;

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

    async fn get_update(&self, start: u128, end: u128) -> Result<types::abi::L1Update, L1Error> {
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

    #[tracing::instrument(skip(self), ret)]
    async fn close_cancel(
        &self,
        cancel: gasp_types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        self.rolldown_contract
            .send_close_cancel_tx(cancel.into(), merkle_root, proof)
            .await
    }

    #[tracing::instrument(skip(self), ret)]
    async fn close_withdrawal(
        &self,
        withdrawal: gasp_types::Withdrawal,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        self.rolldown_contract
            .send_close_withdrawal_tx(withdrawal, merkle_root, proof)
            .await
    }

    #[tracing::instrument(skip(self), ret)]
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let pending_update = self.get_update(start, end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }

    #[tracing::instrument(level = "trace", skip(self), ret)]
    async fn get_status(&self, request_hash: H256) -> Result<types::RequestStatus, L1Error> {
        self.rolldown_contract
            .get_request_status(request_hash)
            .await
    }
}
