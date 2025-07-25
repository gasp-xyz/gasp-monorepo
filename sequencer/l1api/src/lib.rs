use tokio::time::Duration;

use alloy::{
    network::{EthereumWallet, Network, NetworkWallet},
    providers::{PendingTransactionError, Provider, ProviderBuilder, WalletProvider},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
    transports::Transport,
};
use futures::{stream::BoxStream, StreamExt};
use hex::encode as hex_encode;
use hex_literal::hex;
use primitive_types::H256;

pub mod cicka;
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
        pub use contract_bindings::rolldown::IRolldownPrimitives::Deposit;
        pub use contract_bindings::rolldown::IRolldownPrimitives::L1Update;
        pub use contract_bindings::rolldown::IRolldownPrimitives::Origin;
        pub use contract_bindings::rolldown::IRolldownPrimitives::Range;
        pub use contract_bindings::rolldown::IRolldownPrimitives::RequestId;
        pub use contract_bindings::rolldown::IRolldownPrimitives::Withdrawal;
        pub use contract_bindings::rolldown::Rolldown::DepositAcceptedIntoQueue;
        pub use contract_bindings::rolldown::Rolldown::L2UpdateAccepted;
    }

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum RequestStatus {
        Pending,
        Ferried([u8; 20]),
        Closed,
    }

    use std::fmt;
    impl fmt::Display for RequestStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RequestStatus::Pending => write!(f, "Pending"),
                RequestStatus::Closed => write!(f, "Closed"),
                RequestStatus::Ferried(addr) => write!(f, "Ferried({})", hex::encode(addr)),
            }
        }
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
    #[error("alloy error {0:?}")]
    Alloy(#[from] alloy::contract::Error),
    #[error("alloy error {0:?}")]
    TransportAlloy(#[from] alloy::transports::TransportError),
    #[error("transaction error")]
    TxSendError(#[from] PendingTransactionError),
    #[error("Transaction execution failure `{0}`")]
    TxReverted(H256),
    #[error("subscritption failed")]
    SubscriptionFailed,
    #[error("Deserialization error")]
    DeserializationError,
}

pub const NATIVE_TOKEN_ADDRESS: [u8; 20] = hex!("0000000000000000000000000000000000000001");

#[derive(Debug, Clone)]
pub enum Subscription {
    Subscription,
    Polling,
}

#[allow(async_fn_in_trait)]
pub trait L1Interface {
    async fn subscribe_header(&self) -> Result<BoxStream<Result<u128, L1Error>>, L1Error>;
    async fn subscribe_deposit(&self) -> Result<BoxStream<u128>, L1Error>;
    async fn subscribe_new_batch(&self) -> Result<BoxStream<(H256, (u128, u128))>, L1Error>;
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
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error>;

    async fn close_withdrawals_at_once(
        &self,
        withdrawals: Vec<(gasp_types::Withdrawal, H256, Vec<H256>)>,
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

#[derive(Clone)]
pub struct L1<T, P, N> {
    rolldown_contract: RolldownContract<T, P, N>,
    cicka: Option<cicka::Cicka<T, P, N>>,
    provider: P,
    subscription: Subscription,
}

impl<T, P, N> L1<T, P, N> {
    pub fn new(
        rolldown_contract: RolldownContract<T, P, N>,
        cicka: Option<cicka::Cicka<T, P, N>>,
        provider: P,
        subscription: Subscription,
    ) -> Self {
        L1 {
            rolldown_contract,
            cicka,
            provider,
            subscription,
        }
    }
}

impl<T, P, N> L1Interface for L1<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + Clone + WalletProvider<N>,
    N: Network,
{
    async fn subscribe_header(&self) -> Result<BoxStream<Result<u128, L1Error>>, L1Error> {
        match self.subscription {
            Subscription::Polling => Ok(self
                .rolldown_contract
                .subscribe_blocks_polling(tokio::time::Duration::from_secs_f32(30.0))
                .await?
                .boxed()),
            Subscription::Subscription => {
                Ok(self.rolldown_contract.subscribe_blocks().await?.boxed())
            }
        }
    }

    async fn subscribe_deposit(&self) -> Result<BoxStream<u128>, L1Error> {
        match self.subscription {
            Subscription::Polling => Ok(self
                .rolldown_contract
                .subscribe_deposit_polling(tokio::time::Duration::from_secs_f32(30.0))
                .await?
                .boxed()),
            Subscription::Subscription => {
                Ok(self.rolldown_contract.subscribe_deposit().await?.boxed())
            }
        }
    }

    async fn subscribe_new_batch(&self) -> Result<BoxStream<(H256, (u128, u128))>, L1Error> {
        match self.subscription {
            Subscription::Polling => Ok(self
                .rolldown_contract
                .subscribe_new_batch_polling(Duration::from_secs(30))
                .await?
                .boxed()),
            Subscription::Subscription => {
                Ok(self.rolldown_contract.subscribe_new_batch().await?.boxed())
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self), ret)]
    async fn get_deposit(&self, request_id: u128) -> Result<Option<types::Deposit>, L1Error> {
        let deposit = self.rolldown_contract.get_deposit(request_id).await?;
        if deposit.timeStamp.is_zero() {
            Ok(None)
        } else {
            Ok(Some(deposit.into()))
        }
    }

    #[tracing::instrument(level = "debug", skip(self), ret)]
    async fn erc20_balance(&self, token: [u8; 20], account: [u8; 20]) -> Result<u128, L1Error> {
        let token = Erc20Token::new(token, self.provider.clone());
        token.balance_of(account).await
    }

    #[tracing::instrument(level = "debug", skip(self), ret)]
    async fn native_balance(&self, account: [u8; 20]) -> Result<u128, L1Error> {
        let result = self.provider.get_balance(account.into()).await?;
        result.try_into().map_err(|_| L1Error::OverflowError)
    }

    async fn ferry_withdrawal(&self, withdrawal: gasp_types::Withdrawal) -> Result<H256, L1Error> {
        let amount = (withdrawal.amount - withdrawal.ferry_tip)
            .try_into()
            .unwrap();

        if withdrawal.token_address == NATIVE_TOKEN_ADDRESS {
            self.rolldown_contract
                .send_ferry_withdrawal(withdrawal)
                .await
        } else {
            let p = self.provider.clone();
            let me: [u8; 20] = p.wallet().default_signer_address().into();
            let rolldown_address = self.rolldown_contract.address();
            let token = Erc20Token::new(withdrawal.token_address, p);
            let allowance = token.allowance(rolldown_address, me).await?;
            if allowance < amount {
                let missing_allowance = amount - allowance;
                token.approve(rolldown_address, missing_allowance).await?;
            }
            self.rolldown_contract
                .send_ferry_withdrawal(withdrawal)
                .await
        }
    }

    #[allow(clippy::type_complexity)]
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
        self.rolldown_contract.get_latest_reqeust_id().await
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

    #[tracing::instrument(level = "debug", skip(self), ret)]
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

    #[tracing::instrument(level = "debug", skip(self), ret)]
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

    #[tracing::instrument(level = "debug", skip(self), ret)]
    async fn close_withdrawals_at_once(
        &self,
        withdrawals: Vec<(gasp_types::Withdrawal, H256, Vec<H256>)>,
    ) -> Result<H256, L1Error> {
        if let Some(cicka) = &self.cicka {
            cicka.send_close_withdrawals(withdrawals).await
        } else {
            tracing::error!("L1 Interface initialize without cicka contract address");
            panic!("L1 Interface initialize without cicka contract address");
        }
    }

    #[tracing::instrument(level = "debug", skip(self), ret)]
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let pending_update = self.get_update(start, end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }

    #[tracing::instrument(level = "debug", skip(self), ret)]
    async fn get_status(&self, request_hash: H256) -> Result<types::RequestStatus, L1Error> {
        self.rolldown_contract
            .get_request_status(request_hash)
            .await
    }
}
