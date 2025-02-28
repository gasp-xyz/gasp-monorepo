use alloy::{contract::{CallBuilder, CallDecoder}, network::{Network, NetworkWallet}, providers::{Provider, WalletProvider}, transports::{Transport, TransportError}};
pub use alloy::network::primitives::ReceiptResponse;
use primitive_types::H256;

use crate::L1Error;


pub async fn estimate_gas_in_wei<T,P,N>(provider: &P) -> Result<(u128, u128), TransportError> where 
    T: Transport + Clone,
    P: Provider<T,N> + WalletProvider<N>,
    N: Network
{
    // https://www.blocknative.com/blog/eip-1559-fees
    // We do not want client to estimate we would like to make our own estimate
    // based on this equation: Max Fee = (2 * Base Fee) + Max Priority Fee

    // Max Fee = maxFeePerGas (client)
    // Max Priority Fee = maxPriorityFeePerGas (client)

    let base_fee_in_wei = provider.get_gas_price().await?;

    let max_priority_fee_per_gas_in_wei = provider
        .get_max_priority_fee_per_gas()
        .await?;

    let max_fee_in_wei = base_fee_in_wei
        .saturating_mul(2)
        .saturating_add(max_priority_fee_per_gas_in_wei);

    Ok((max_fee_in_wei, max_priority_fee_per_gas_in_wei))
}

pub async fn simulate_send_and_wait_for_result<T,P,D,N>(
    provider: &P,
    call: CallBuilder<T,&P,D,N>,
) -> Result<H256, L1Error> 
    where 
    N: Network,
    D: CallDecoder + Send + Sync + Unpin,
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>
{
    let account = provider.wallet().default_signer_address();
    let (max_fee_per_gas_in_wei, max_priority_fee_per_gas_in_wei) = crate::utils::estimate_gas_in_wei::<T,P,N>(provider).await?;
    tracing::info!("estimation !!!!!!!!!!!!!!!!!!!!!!!!!!");
    let call = call.from(account);

    tracing::info!("call !!!!!!!!!!!!!!!!!!!!!!!!!!");
    match call.call().await {
        Ok(_result) => {
            tracing::info!("send !!!!!!!!!!!!!!!!!!!!!!!!");
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
            }else{
                Err(L1Error::TxReverted(hash))
            }
        }
        Err(err) => {
            tracing::error!("status nok {:?}", err);
            Err(err.into())
        }
    }
}
