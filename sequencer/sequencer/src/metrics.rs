use prometheus::{Encoder, TextEncoder};
use warp::Filter;

pub async fn serve_metrics(port: u16) {
    let metrics = warp::any().map(|| {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).expect("metrics available");
        warp::reply::with_header(buffer, warp::http::header::CONTENT_TYPE, encoder.format_type())
    });
    warp::serve(metrics).run(([0, 0, 0, 0], port)).await;
}
