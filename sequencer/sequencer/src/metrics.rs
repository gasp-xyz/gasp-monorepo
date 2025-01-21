use alloy::providers::{Provider, ProviderBuilder};
use prometheus::{labels, opts, register_gauge, Encoder, Gauge, TextEncoder};
use warp::Filter;

//TODO: collect all metrics to single module
lazy_static::lazy_static! {
    static ref BALANCE: Gauge = register_gauge!(opts!(
        "account_balance",
        "Balance of the sequencer account",
    ))
    .unwrap();
}

pub async fn report_account_balance(uri: String, address:[u8; 20] ) {
    loop {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_builtin(uri.as_ref())
        .await.unwrap();
        let balance = provider.get_balance(address.into()).await.unwrap();
        let division = 1_000_000_000__000_000_000u128.try_into().expect("fits into U256");
        let result: f64 = balance.wrapping_div(division).try_into().expect("can be divided");
        BALANCE.set(result / 1_000_000_f64);
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
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
