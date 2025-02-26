use super::{types, L1Error, L1Interface};
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


pub struct RolldownContractBuilder {
    pub uri: &'static str,
    pub pkey: [u8; 32],
    pub address: [u8; 20],
}

impl RolldownContractBuilder {
    pub async fn build(
        self,
    ) -> Result<RolldownContract<impl WalletProvider + Provider>, L1Error> {
        let signer: PrivateKeySigner = hex::encode(self.pkey).parse().expect("valid private key");
        let wallet = EthereumWallet::new(signer);
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_builtin(self.uri)
            .await?;

        Ok(RolldownContract {
            contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance::new(
                self.address.into(),
                provider,
            ),
        })
    }
}

pub struct RolldownContract<P> {
    contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance<BoxTransport, P>,
}

impl<P> RolldownContract<P>
where
    P: Provider
{
    pub fn new(provider: P, address: [u8;20]) -> Self {
        RolldownContract {
            contract_handle: contract_bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider,
            ),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn deposit(&self, amount: u128, ferry_tip: u128) -> Result<(), L1Error> {
        let call = self
            .contract_handle
            .deposit_native_1(alloy::primitives::U256::from(ferry_tip))
            .value(alloy::primitives::U256::from(amount));

        let hash = call.send().await?.watch().await?;
        tracing::debug!("deposit hash: {}", hex_encode(hash));

        Ok(())
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

    pub async fn get_update_impl(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error> {
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

    async fn get_native_account_balance(&self, address: [u8; 20]) -> Result<u128, L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["balance"]).inc();
        let provider = self.contract_handle.provider();
        let result = provider.get_balance(address.into()).await?;
        result.try_into().map_err(|_| L1Error::OverflowError)
    }

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

        let (max_fee_per_gas_in_wei, max_priority_fee_per_gas_in_wei) = crate::utils::estimate_gas_in_wei(self.contract_handle.provider()).await?;

        match call.call().await {
            Ok(_) => {
                tracing::trace!("status ok");
                Ok(call
                    .max_fee_per_gas(max_fee_per_gas_in_wei)
                    .max_priority_fee_per_gas(max_priority_fee_per_gas_in_wei)
                    .send()
                    .await?
                    .watch()
                    .await?
                    .0
                    .into())
            }
            Err(err) => {
                tracing::error!("status nok {:?}", err);
                Err(err.into())
            }
        }
    }

}

