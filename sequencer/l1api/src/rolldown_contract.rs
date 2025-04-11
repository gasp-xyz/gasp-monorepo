use super::{types, L1Error};
use crate::types::RequestStatus;
use crate::utils::simulate_send_and_wait_for_result;
use contract_bindings::irolldownprimitives::IRolldownPrimitives::L2UpdateAccepted;
use futures::{Stream, StreamExt};
use std::pin::Pin;
use lazy_static::lazy_static;

use alloy::network::Network;
use alloy::rpc::types::{Filter, Log};
use alloy::providers::{Provider, WalletProvider};
use alloy::transports::Transport;
use primitive_types::H256;

#[cfg(test)]
use alloy::network::NetworkWallet;

use prometheus::{opts, register_counter_vec, CounterVec};

lazy_static! {
    static ref ETH_CALL_COUNTER: CounterVec = register_counter_vec!(
        opts!("eth_call", "Number of HTTP requests made."),
        &["method"]
    )
    .unwrap();
}

#[derive(Clone)]
pub struct RolldownContract<T, P, N> {
    contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance<T, P, N>,
}

pub type BatchStream = Pin<Box<dyn Stream<Item = Result<H256, L1Error>> + Send + 'static>>;


impl<T, P, N> RolldownContract<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    #[tracing::instrument(skip(self))]
    pub async fn subscribe_new_batch(&self) -> Result<impl Stream<Item = (H256, (u128, u128))>, L1Error>{
        let p = self.contract_handle.provider();
        let filter = self.contract_handle.L2UpdateAccepted_filter().filter;
        Ok(p.subscribe_logs(&filter).await.unwrap()
            .into_stream()
            .map(|elem| {
                let log = elem.log_decode::<crate::types::abi::L2UpdateAccepted>().expect("can decode subscribed log");
                let merkle_root = log.data().root.0; 
                let start = gasp_types::from_l1_u256(log.data().range.start);
                let end = gasp_types::from_l1_u256(log.data().range.end);
                (merkle_root.into(), (start.try_into().unwrap(), end.try_into().unwrap()))
            })
        )
    }

    #[tracing::instrument(skip(self, cancel))]
    pub async fn send_close_cancel_tx(
        &self,
        cancel: types::abi::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let call = self
            .contract_handle
            .close_cancel(cancel, merkle_root.0.into(), proof);
        simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn hash(&self, withdrawal: gasp_types::Withdrawal) -> Result<H256, L1Error> {
        let call = self.contract_handle.hashWithdrawal(withdrawal.into());
        Ok(call.call().await?._0.0.into())
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn send_ferry_withdrawal(
        &self,
        withdrawal: gasp_types::Withdrawal,
    ) -> Result<H256, L1Error> {
        let call = self.contract_handle.ferryWithdrawal(withdrawal.into());
        simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await
    }

    #[tracing::instrument(skip(self, request_id))]
    pub async fn get_deposit(
        &self,
        request_id: u128,
    ) -> Result<crate::types::abi::Deposit, L1Error> {
        let call = self
            .contract_handle
            .deposits(gasp_types::into_l1_u256(request_id.into()));
        let deposit = call.call().await?;
        Ok(crate::types::abi::Deposit {
            requestId: deposit.requestId,
            depositRecipient: deposit.depositRecipient,
            tokenAddress: deposit.tokenAddress,
            amount: deposit.amount,
            timeStamp: deposit.timeStamp,
            ferryTip: deposit.ferryTip,
        })
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn send_close_withdrawal_tx(
        &self,
        withdrawal: gasp_types::Withdrawal,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let native_withdrawal: types::abi::Withdrawal = withdrawal.into();
        let call =
            self.contract_handle
                .closeWithdrawal(native_withdrawal, merkle_root.0.into(), proof);
        simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub(crate) async fn deploy(provider: P) -> Result<Self, L1Error> {
        let sender = provider.wallet().default_signer_address();
        let contract_handle =
            contract_bindings::rolldown::Rolldown::RolldownInstance::<T, P, N>::deploy(provider)
                .await?;
        tracing::info!("contract deployed");

        let call = contract_handle.initialize(sender, 0, sender);
        simulate_send_and_wait_for_result(contract_handle.provider(), call).await?;
        tracing::info!("contract initialized");

        Ok(RolldownContract { contract_handle })
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub(crate) async fn deposit_native(
        &self,
        amount: u128,
        ferry_tip: u128,
    ) -> Result<(), L1Error> where {
        let call = self
            .contract_handle
            .deposit_native_1(alloy::primitives::U256::from(ferry_tip))
            .value(alloy::primitives::U256::from(amount));

        simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await?;
        Ok(())
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub(crate) async fn submit_merkle_root(
        &self,
        root: [u8; 32],
        range: (u128, u128),
    ) -> Result<(), L1Error> where {
        let range = contract_bindings::rolldown::IRolldownPrimitives::Range {
            start: gasp_types::into_l1_u256(gasp_types::U256::from(range.0)),
            end: gasp_types::into_l1_u256(gasp_types::U256::from(range.1)),
        };

        let call = self.contract_handle.update_l1_from_l2(root.into(), range);

        simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await?;
        Ok(())
    }
}

impl<T, P, N> RolldownContract<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + Clone,
    N: Network,
{
    pub fn address(&self) -> [u8; 20] {
        (*self.contract_handle.address()).into()
    }

    pub async fn is_admin(&self, address: [u8; 20]) -> Result<bool, L1Error> {
        let admin_role = self.contract_handle.DEFAULT_ADMIN_ROLE().call().await?._0;
        Ok(self
            .contract_handle
            .hasRole(admin_role, address.into())
            .call()
            .await?
            ._0)
    }

    pub async fn is_updater(&self, address: [u8; 20]) -> Result<bool, L1Error> {
        let updater_role = self.contract_handle.UPDATER_ROLE().call().await?._0;
        Ok(self
            .contract_handle
            .hasRole(updater_role, address.into())
            .call()
            .await?
            ._0)
    }

    pub fn new(provider: P, address: [u8; 20]) -> Self {
        RolldownContract {
            contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider,
            ),
        }
    }

    pub async fn find_l2_batch(&self, request_id: u128) -> Result<[u8; 32], L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["find_l2_batch"]).inc();
        let request_id = alloy::primitives::U256::from(request_id);
        let call = self.contract_handle.find_l2_batch(request_id);
        Ok(call.call().await?._0.0)
    }

    pub async fn get_request_range_from_merkle_root(
        &self,
        merkle_root: [u8; 32],
    ) -> Result<(u128, u128), L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["merkleRootRange"])
            .inc();
        let call = self.contract_handle.merkleRootRange(merkle_root.into());
        let range = call.call().await?;

        let range_start = range.start.try_into().map_err(|_| L1Error::OverflowError)?;
        let range_end = range.end.try_into().map_err(|_| L1Error::OverflowError)?;
        Ok((range_start, range_end))
    }

    pub async fn get_next_request_id(&self) -> Result<u128, L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["counter"]).inc();
        let call = self.contract_handle.counter();
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    pub async fn get_amount_of_merkle_roots(&self) -> Result<u128, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["getMerkleRootsLength"])
            .inc();
        let call = self.contract_handle.getMerkleRootsLength();
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    pub async fn get_merkle_root_by_id(&self, merkle_root_id: u128) -> Result<[u8; 32], L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["roots"]).inc();
        let merkle_root_id = alloy::primitives::U256::from(merkle_root_id);
        let latest_root = self.contract_handle.roots(merkle_root_id).call().await?._0;
        Ok(latest_root.0)
    }

    pub async fn get_update_impl(
        &self,
        start: u128,
        end: u128,
    ) -> Result<types::abi::L1Update, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["getPendingRequests"])
            .inc();
        let range_start = alloy::primitives::U256::from(start);
        let range_end = alloy::primitives::U256::from(end);
        let call = self
            .contract_handle
            .getPendingRequests(range_start, range_end);
        Ok(call.call().await?._0)
    }

    pub async fn get_request_status(&self, request_hash: H256) -> Result<RequestStatus, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["processedL2Requests"])
            .inc();
        let request_hash = request_hash.0.into();
        let request_status = self
            .contract_handle
            .processedL2Requests(request_hash)
            .call()
            .await
            .map(|elem| elem._0)?;

        let closed = self.contract_handle.CLOSED().call().await?._0;
        if request_status == closed {
            Ok(RequestStatus::Closed)
        } else if request_status.0 == [0u8; 20] {
            Ok(RequestStatus::Pending)
        } else {
            Ok(RequestStatus::Ferried(request_status.into()))
        }
    }

    pub async fn is_request_closed(&self, request_hash: H256) -> Result<bool, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["processedL2Requests"])
            .inc();
        let request_hash = request_hash.0.into();
        let request_status = self
            .contract_handle
            .processedL2Requests(request_hash)
            .call()
            .await
            .map(|elem| elem._0)?;

        let closed = self.contract_handle.CLOSED().call().await?._0;
        let is_closed = request_status == closed;
        Ok(is_closed)
    }

    pub async fn is_request_ferried(&self, request_hash: H256) -> Result<bool, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["processedL2Requests"])
            .inc();
        let request_hash = request_hash.0.into();
        let request_status = self
            .contract_handle
            .processedL2Requests(request_hash)
            .call()
            .await
            .map(|elem| elem._0)?;

        let closed = self.contract_handle.CLOSED().call().await?._0;
        let is_closed = request_status == closed;
        Ok(is_closed)
    }
}
