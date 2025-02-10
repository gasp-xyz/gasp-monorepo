use ethers::providers::{JsonRpcClient, Middleware, Provider};
use ethers::types::NameOrAddress;
use prometheus::{opts, register_gauge, Encoder, Gauge, TextEncoder};
use sp_runtime::SaturatedConversion;
use warp::Filter;

//TODO: collect all metrics to single module
lazy_static::lazy_static! {
    static ref BALANCE: Gauge = register_gauge!(opts!(
        "account_balance",
        "Balance of the updater account",
    ))
    .unwrap();
}

pub async fn report_account_balance<P>(provider: Provider<P>, address: [u8; 20])
where
    P: JsonRpcClient,
{
    loop {
        if let Ok(balance) = provider
            .get_balance(NameOrAddress::Address(address.into()), None)
            .await
        {
            let balance_f64: f64 = balance.saturated_into::<u128>() as f64;
            let decimals: f64 = balance_f64 / 1_000_000_000_000_000_000_f64;
            tracing::trace!("updater account balance {}", decimals);
            BALANCE.set(decimals);
        } else {
            tracing::warn!("could not fetch updater account balance");
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
