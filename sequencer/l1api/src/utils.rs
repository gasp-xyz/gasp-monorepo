use alloy::{providers::Provider, transports::TransportError};


pub async fn estimate_gas_in_wei(provider: &impl Provider) -> Result<(u128, u128), TransportError> {
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
