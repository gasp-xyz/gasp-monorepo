use alloy::{
    network::{Network, NetworkWallet},
    providers::{Provider, WalletProvider},
    transports::Transport,
};
use prometheus::{opts, register_gauge, Encoder, Gauge, TextEncoder};
use warp::Filter;

//TODO: collect all metrics to single module
lazy_static::lazy_static! {
    static ref BALANCE: Gauge = register_gauge!(opts!(
        "account_balance",
        "Balance of the sequencer account",
    ))
    .unwrap();
}

pub async fn report_account_balance<T, P, N>(provider: P)
where
    T: Transport + Clone,
    P: Provider<T, N> + WalletProvider<N>,
    N: Network,
{
    loop {
        let account = provider.wallet().default_signer_address();
        if let Ok(balance) = provider.get_balance(account).await {
            let balance_to_decimals = u128::pow(10, 18).try_into().expect("balance fits in U256");
            let decimals: f64 = balance.wrapping_div(balance_to_decimals).into();
            tracing::trace!("sequencer account balance {}", decimals);
            BALANCE.set(decimals);
        } else {
            tracing::warn!("could not fetch sequencer account balance");
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
    }
}

pub async fn serve_metrics(port: u16) {
    let metrics = warp::any().map(|| {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = vec![];
        encoder
            .encode(&metric_families, &mut buffer)
            .expect("metrics available");
        warp::reply::with_header(
            buffer,
            warp::http::header::CONTENT_TYPE,
            encoder.format_type(),
        )
    });
    warp::serve(metrics).run(([0, 0, 0, 0], port)).await;
}
