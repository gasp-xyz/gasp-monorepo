use std::sync::Arc;

use ethers::{providers::Middleware, types::NameOrAddress};
use prometheus::{opts, register_gauge, Encoder, Gauge, TextEncoder};
use sp_runtime::SaturatedConversion;
use warp::Filter;

lazy_static::lazy_static! {
    static ref BALANCE: Gauge = register_gauge!(opts!(
        "account_balance",
        "Balance of the gasp-avs account",
    ))
    .unwrap();
}

pub async fn report_account_balance(
    account: ethers::types::Address,
    client: Arc<crate::chainio::Client>,
) {
    loop {
        if let Ok(balance) = client
            .get_balance(NameOrAddress::Address(account), None)
            .await
        {
            let balance_f64: f64 = balance.saturated_into::<u128>() as f64;
            let decimals: f64 = balance_f64 / 1_000_000_000_000_000_000_f64;
            tracing::trace!("gasp-avs account balance {}", decimals);
            BALANCE.set(decimals);
        } else {
            tracing::warn!("could not fetch gasp-avs account balance");
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
