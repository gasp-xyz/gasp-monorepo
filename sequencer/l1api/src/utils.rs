pub use alloy::network::primitives::ReceiptResponse;
use alloy::{
    contract::{CallBuilder, CallDecoder},
    network::{Network, NetworkWallet},
    providers::{Provider, WalletProvider},
    transports::{Transport, TransportError},
};
use primitive_types::H256;

use crate::L1Error;

pub async fn estimate_gas_in_wei<T, P, N>(provider: &P) -> Result<(u128, u128), TransportError>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    // https://www.blocknative.com/blog/eip-1559-fees
    // We do not want client to estimate we would like to make our own estimate
    // based on this equation: Max Fee = (2 * Base Fee) + Max Priority Fee

    // Max Fee = maxFeePerGas (client)
    // Max Priority Fee = maxPriorityFeePerGas (client)

    let base_fee_in_wei = provider.get_gas_price().await?;

    let max_priority_fee_per_gas_in_wei = provider.get_max_priority_fee_per_gas().await?;

    let max_fee_in_wei = base_fee_in_wei
        .saturating_mul(2)
        .saturating_add(max_priority_fee_per_gas_in_wei);

    Ok((max_fee_in_wei, max_priority_fee_per_gas_in_wei))
}

pub async fn simulate_send_and_wait_for_result<T, P, D, N>(
    provider: &P,
    call: CallBuilder<T, &P, D, N>,
) -> Result<H256, L1Error>
where
    N: Network,
    D: CallDecoder + Send + Sync + Unpin,
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
{
    let account = provider.wallet().default_signer_address();
    let (max_fee_per_gas_in_wei, max_priority_fee_per_gas_in_wei) =
        crate::utils::estimate_gas_in_wei::<T, P, N>(provider).await?;
    let call = call.from(account);

    match call.call().await {
        Ok(_result) => {
            let receipt = call
                .max_fee_per_gas(max_fee_per_gas_in_wei)
                .max_priority_fee_per_gas(max_priority_fee_per_gas_in_wei)
                .send()
                .await?
                .get_receipt()
                .await?;

            let status = receipt.status();
            let hash = receipt.transaction_hash().0.into();
            let sender = receipt.from();
            tracing::trace!("tx: {hash} sender {sender} execution status {status}");
            if status {
                Ok(hash)
            } else {
                Err(L1Error::TxReverted(hash))
            }
        }
        Err(err) => {
            tracing::error!("status nok {:?}", err);
            Err(err.into())
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use crate::erc20::Erc20Token;
    use contract_bindings::gasptesttoken::GaspTestToken::GaspTestTokenInstance;

    #[derive(Clone)]
    pub struct DevToken<T, P, N> {
        contract_handle: GaspTestTokenInstance<T, P, N>,
    }

    impl<T, P, N> DevToken<T, P, N>
    where
        T: Transport + Clone,
        P: Provider<T, N> + WalletProvider<N> + Clone,
        N: Network,
    {
        pub(crate) async fn deploy(provider: P) -> Result<Self, L1Error> {
            let sender = provider.wallet().default_signer_address();
            let contract_handle = GaspTestTokenInstance::<T, P, N>::deploy(provider).await?;
            tracing::info!("contract deployed");

            Ok(DevToken { contract_handle })
        }

        fn new(address: [u8; 20], provider: P) -> Self {
            DevToken {
                contract_handle: GaspTestTokenInstance::<T, P, N>::new(address.into(), provider),
            }
        }

        pub fn address(&self) -> [u8; 20] {
            let addr = self.contract_handle.address().clone();
            addr.0.into()
        }

        pub async fn mint(&self, address: [u8; 20], amount: u128) -> Result<H256, L1Error> {
            let call = self
                .contract_handle
                .mint(address.into(), amount.try_into().expect("can convert"));
            simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await
        }

        pub async fn transfer(&self, address: [u8; 20], amount: u128) -> Result<H256, L1Error> {
            let token = Erc20Token::<T, P, N>::new(
                self.contract_handle.address().0.into(),
                self.contract_handle.provider().clone(),
            );
            token.transfer(address, amount).await
        }

        pub async fn approve(&self, address: [u8; 20], amount: u128) -> Result<H256, L1Error> {
            let token = Erc20Token::<T, P, N>::new(
                self.contract_handle.address().0.into(),
                self.contract_handle.provider().clone(),
            );
            token.approve(address, amount).await
        }

        pub async fn allowance(
            &self,
            contract: [u8; 20],
            account: [u8; 20],
        ) -> Result<u128, L1Error> {
            let token = Erc20Token::<T, P, N>::new(
                self.contract_handle.address().0.into(),
                self.contract_handle.provider().clone(),
            );
            token.allowance(contract, account).await
        }

        pub async fn balance_of(&self, account: [u8; 20]) -> Result<u128, L1Error> {
            let token = Erc20Token::<T, P, N>::new(
                self.contract_handle.address().0.into(),
                self.contract_handle.provider().clone(),
            );
            token.balance_of(account).await
        }
    }
}
