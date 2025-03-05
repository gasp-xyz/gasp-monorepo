use crate::utils::simulate_send_and_wait_for_result;
use crate::L1Error;
use alloy::{
    network::{Network, NetworkWallet},
    providers::{Provider, WalletProvider},
    transports::Transport,
};
use contract_bindings::ierc20::IERC20::IERC20Instance;
use primitive_types::H256;

#[derive(Clone)]
pub struct Erc20Token<T, P, N> {
    contract_handle: IERC20Instance<T, P, N>,
}

impl<T, P, N> Erc20Token<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    #[tracing::instrument(skip(self))]
    pub async fn approve(&self, address: [u8; 20], amount: u128) -> Result<H256, L1Error> {
        let call = self
            .contract_handle
            .approve(address.into(), amount.try_into().expect("can convert"));
        simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn transfer(&self, address: [u8; 20], amount: u128) -> Result<H256, L1Error> {
        let call = self
            .contract_handle
            .transfer(address.into(), amount.try_into().expect("can convert"));
        simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await
    }
}

impl<T, P, N> Erc20Token<T, P, N>
where
    T: Transport + Clone,
    P: Provider<T, N>,
    N: Network,
{
    pub fn new(address: [u8; 20], provider: P) -> Self {
        Erc20Token {
            contract_handle: IERC20Instance::<T, P, N>::new(address.into(), provider),
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn balance_of(&self, account: [u8; 20]) -> Result<u128, L1Error> {
        let call = self.contract_handle.balanceOf(account.into());
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    #[tracing::instrument(skip(self))]
    pub async fn allowance(&self, contract: [u8; 20], account: [u8; 20]) -> Result<u128, L1Error> {
        let call = self
            .contract_handle
            .allowance(contract.into(), account.into());
        let result = call.call().await?;
        result._0.try_into().map_err(|_| L1Error::OverflowError)
    }

    //
    //
    // #[tracing::instrument(skip(self, cancel))]
    // pub async fn transfer(
    //     &self,
    //     cancel: types::Cancel,
    //     merkle_root: H256,
    //     proof: Vec<H256>,
    // ) -> Result<H256, L1Error> {
    //     let proof = proof.into_iter().map(|elem| elem.0.into()).collect();
    //     let call = self
    //         .contract_handle
    //         .close_cancel(cancel, merkle_root.0.into(), proof);
    //     Ok(simulate_send_and_wait_for_result(self.contract_handle.provider(), call).await?)
    // }
}
