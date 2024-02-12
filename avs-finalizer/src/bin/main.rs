use color_eyre::eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish()
        .with(ErrorLayer::default())
        .init();

    color_eyre::install()?;

    avs_finalizer::start().await?;

    Ok(())
}
