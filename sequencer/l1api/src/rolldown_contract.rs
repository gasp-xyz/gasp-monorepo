use super::{types, L1Error, L1Interface};
use crate::utils::simulate_send_and_wait_for_result;
use crate::{L1Withdrawal, RequestStatus};
use alloy::contract::{CallBuilder, CallDecoder};
use contract_bindings::irolldown::IRolldownPrimitives::Withdrawal;
use lazy_static::lazy_static;

use alloy::network::{EthereumWallet, Network, NetworkWallet};
use alloy::providers::{Provider, ProviderBuilder, WalletProvider};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol_types::SolValue;
use alloy::transports::{BoxTransport, Transport};
use hex::encode as hex_encode;
use primitive_types::H256;
use sha3::{Digest, Keccak256};

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

impl<T, P, N> RolldownContract<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    #[tracing::instrument(skip(self, cancel))]
    pub async fn send_close_cancel_tx(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let call = self
            .contract_handle
            .close_cancel(cancel, merkle_root.0.into(), proof);
        Ok(simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await?)
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn hash(&self, withdrawal: L1Withdrawal) -> Result<H256, L1Error> {
        let call = self.contract_handle.hashWithdrawal(withdrawal.into());
        Ok(call.call().await?._0.0.into())
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn send_ferry_withdrawal(&self, withdrawal: L1Withdrawal) -> Result<H256, L1Error> {
        let call = self.contract_handle.ferryWithdrawal(withdrawal.into());
        Ok(simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await?)
    }

    #[tracing::instrument(skip(self, withdrawal))]
    pub async fn send_close_withdrawal_tx(
        &self,
        withdrawal: L1Withdrawal,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let native_withdrawal: types::Withdrawal = withdrawal.into();
        let call =
            self.contract_handle
                .closeWithdrawal(native_withdrawal, merkle_root.0.into(), proof);
        Ok(simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await?)
    }

    #[cfg(test)]
    #[tracing::instrument(skip_all)]
    pub(crate) async fn deploy(provider: P) -> Result<Self, L1Error> {
        let sender = provider.wallet().default_signer_address();
        let contract_handle =
            contract_bindings::rolldown::Rolldown::RolldownInstance::<T, P, N>::deploy(provider)
                .await?;
        tracing::info!("contract deployed");

        let call = (&contract_handle).initialize(sender, 0, sender);
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
            start: alloy::primitives::U256::from(range.0),
            end: alloy::primitives::U256::from(range.1),
        };

        let call = self.contract_handle.update_l1_from_l2(root.into(), range);

        tracing::info!("hello world!!!!!!");
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
        self.contract_handle.address().clone().into()
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
    ) -> Result<types::L1Update, L1Error> {
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

    // async fn get_native_account_balance(&self, address: [u8; 20]) -> Result<u128, L1Error> {
    //     ETH_CALL_COUNTER.with_label_values(&["balance"]).inc();
    //     let provider = self.contract_handle.provider();
    //     let result = provider.get_balance(address.into()).await?;
    //     result.try_into().map_err(|_| L1Error::OverflowError)
    // }

    // pub async fn simulate_send_and_wait_for_result<T,Pr,D,N>(
    //     &self,
    //     call: CallBuilder<T,Pr,D,N>,
    // ) -> Result<H256, L1Error>
    // where
    //     N: Network,
    //     D: CallDecoder + Send + Sync + Unpin,
    //     T: Transport + Clone,
    //     Pr: Provider<T, N>
    // {
    //
    //     let (max_fee_per_gas_in_wei, max_priority_fee_per_gas_in_wei) =
    //         crate::utils::estimate_gas_in_wei(self.contract_handle.provider()).await?;
    //     let gas = call.estimate_gas().await?;
    //
    //
    //     match call.call().await {
    //         Ok(_result) => {
    //             let receipt = call
    //                 .gas(gas * 12 / 10)
    //                 .max_fee_per_gas(max_fee_per_gas_in_wei)
    //                 .max_priority_fee_per_gas(max_priority_fee_per_gas_in_wei)
    //                 .send()
    //                 .await?
    //                 .get_receipt()
    //                 .await?;
    //
    //             let status = receipt.status();
    //             let hash = receipt.transaction_hash().0;
    //             tracing::trace!("execution status {status}");
    //             if status {
    //                 Ok(hash.into())
    //             }else{
    //                 Err(L1Error::TxReverted(hash.into()))
    //             }
    //         }
    //         Err(err) => {
    //             tracing::error!("status nok {:?}", err);
    //             Err(err.into())
    //         }
    //     }
    // }
}
