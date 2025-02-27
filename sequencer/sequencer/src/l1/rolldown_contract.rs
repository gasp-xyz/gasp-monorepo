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

pub struct Foo<P> {
    handle: bindings::rolldown::Rolldown::RolldownInstance<BoxTransport, P>
}

pub struct FooBuilder{
    pub uri: &'static str,
    pub pkey: [u8; 32],
    pub address: [u8; 20],
}

impl FooBuilder {
    pub async fn build(self) -> Result<Foo<impl WalletProvider + Provider + Clone>, L1Error> {
        let signer: PrivateKeySigner = hex::encode(self.pkey).parse().expect("valid private key");
        let wallet = EthereumWallet::new(signer);
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_builtin(self.uri)
            .await?;
        Ok(Foo { handle: bindings::rolldown::Rolldown::RolldownInstance::new( [0u8; 20].into(), provider,) })
    }
}

// impl<P> Foo<P> {
//     async fn from_provider(provider: P) -> Result<Self, L1Error> {
//         Ok(Foo { handle:  })
//     }
// }


impl<P> Foo<P>
where
    P: Provider + WalletProvider + Clone,
{
    #[tracing::instrument(skip(self))]
    pub async fn deposit(&self, amount: u128, ferry_tip: u128) -> Result<(), L1Error> {

        let call = 
            self.handle
            .deposit_native_1(alloy::primitives::U256::from(ferry_tip))
            .value(alloy::primitives::U256::from(amount));

        let hash = call.send().await?.watch().await?;
        tracing::debug!("deposit hash: {}", hex_encode(hash));

        Ok(())
    }
}


pub struct RolldownContractBuilder{
    pub uri: &'static str,
    pub pkey: [u8; 32],
    pub address: [u8; 20],
}


pub struct RolldownContract<T, P, N> {
    contract_handle: bindings::rolldown::Rolldown::RolldownInstance<T, P, N>,
    account_address: [u8; 20],
}

pub async fn create_provider(
    uri: &str,
    private_key: [u8; 32],
) -> Result<impl Provider + WalletProvider + Clone, L1Error> {
    let signer: PrivateKeySigner = hex::encode(private_key).parse().expect("valid private key");
    let wallet = EthereumWallet::new(signer);
    Ok(ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_builtin(uri)
        .await?)
}


impl<T, P, N> RolldownContract<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    pub fn from_provider(address: [u8; 20], provider: P) -> Self {
        let account = provider.wallet().default_signer_address();

        Self {
            contract_handle: bindings::rolldown::Rolldown::RolldownInstance::new(
                address.into(),
                provider,
            ),
            account_address: account.into(),
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

    async fn find_l2_batch(&self, request_id: u128) -> Result<[u8; 32], L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["find_l2_batch"]).inc();
        let request_id = alloy::primitives::U256::from(request_id);
        let call = self.contract_handle.find_l2_batch(request_id);
        Ok(call.call().await?._0.0)
    }

    async fn get_request_range_from_merkle_root(
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

    async fn get_next_request_id(&self) -> Result<u128, L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["counter"]).inc();
        let call = self.contract_handle.counter();
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    async fn get_amount_of_merkle_roots(&self) -> Result<u128, L1Error> {
        ETH_CALL_COUNTER
            .with_label_values(&["getMerkleRootsLength"])
            .inc();
        let call = self.contract_handle.getMerkleRootsLength();
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    async fn get_merkle_root_by_id(&self, merkle_root_id: u128) -> Result<[u8; 32], L1Error> {
        ETH_CALL_COUNTER.with_label_values(&["roots"]).inc();
        let merkle_root_id = alloy::primitives::U256::from(merkle_root_id);
        let latest_root = self.contract_handle.roots(merkle_root_id).call().await?._0;
        Ok(latest_root.0)
    }

    async fn get_update_impl(&self, start: u128, end: u128) -> Result<types::L1Update, L1Error> {
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

    async fn is_request_closed(&self, request_hash: H256) -> Result<bool, L1Error> {
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
}

impl<T, P, N> L1Interface for RolldownContract<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    fn account_address(&self) -> [u8; 20] {
        self.account_address
    }

    #[tracing::instrument(skip(self))]
    async fn get_merkle_root(&self, request_id: u128) -> Result<([u8; 32], (u128, u128)), L1Error> {
        let merkle_root = self.find_l2_batch(request_id).await?;
        let range = self.get_request_range_from_merkle_root(merkle_root).await?;

        Ok((merkle_root, range))
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_reqeust_id(&self) -> Result<Option<u128>, L1Error> {
        let next_request_id = self.get_next_request_id().await?;

        if next_request_id == 1 {
            Ok(None)
        } else {
            Ok(next_request_id.checked_sub(1u128))
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_latest_finalized_request_id(&self) -> Result<Option<u128>, L1Error> {
        let length = self.get_amount_of_merkle_roots().await?;

        tracing::trace!("there are {} merkle roots on the contract", length);
        if let Some(id) = length.checked_sub(1) {
            let latest_root = self.get_merkle_root_by_id(id).await?;
            tracing::trace!("latest merkle root {}", hex_encode(latest_root));
            let (_, latest_request_id) =
                self.get_request_range_from_merkle_root(latest_root).await?;

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

            let update = self.get_update_impl(start, end).await?;

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

    #[tracing::instrument(skip(self))]
    async fn estimate_gas_in_wei(&self) -> Result<(u128, u128), L1Error> {
        // https://www.blocknative.com/blog/eip-1559-fees
        // We do not want client to estimate we would like to make our own estimate
        // based on this equation: Max Fee = (2 * Base Fee) + Max Priority Fee

        // Max Fee = maxFeePerGas (client)
        // Max Priority Fee = maxPriorityFeePerGas (client)

        let base_fee_in_wei = self.contract_handle.provider().get_gas_price().await?;
        let max_priority_fee_per_gas_in_wei = self
            .contract_handle
            .provider()
            .get_max_priority_fee_per_gas()
            .await?;
        let max_fee_in_wei = base_fee_in_wei
            .saturating_mul(2)
            .saturating_add(max_priority_fee_per_gas_in_wei);
        Ok((max_fee_in_wei, max_priority_fee_per_gas_in_wei))
    }

    #[tracing::instrument(skip(self, cancel))]
    async fn close_cancel(
        &self,
        cancel: types::Cancel,
        merkle_root: H256,
        proof: Vec<H256>,
    ) -> Result<H256, L1Error> {
        let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
        let (max_fee_per_gas_in_wei, max_priority_fee_per_gas_in_wei) =
            self.estimate_gas_in_wei().await?;
        let call = self
            .contract_handle
            .close_cancel(cancel, merkle_root.0.into(), proof);
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

    #[tracing::instrument(skip(self))]
    async fn get_update_hash(&self, start: u128, end: u128) -> Result<H256, L1Error> {
        let pending_update = self.get_update(start, end).await?;
        let x: [u8; 32] = Keccak256::digest(&pending_update.abi_encode()[..]).into();
        Ok(x.into())
    }

    #[tracing::instrument(skip(self))]
    async fn is_closed(&self, request_hash: H256) -> Result<bool, L1Error> {
        let is_closed = self.is_request_closed(request_hash).await?;
        tracing::trace!(is_closed);
        return Ok(is_closed);
    }

    #[tracing::instrument(skip(self))]
    async fn get_native_balance(&self, address: [u8; 20]) -> Result<u128, L1Error> {
        self.get_native_account_balance(address).await
    }
}
