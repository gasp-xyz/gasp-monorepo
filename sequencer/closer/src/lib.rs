use clap::Parser;

mod cli;
mod closer;

fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .init();
}

#[derive(thiserror::Error, Debug)]
pub enum Error{
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let args = cli::Cli::parse();
    init_logger();
    Ok(())
}

