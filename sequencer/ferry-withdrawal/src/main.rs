use clap::Parser;

mod cli;
mod hunter;
mod filter;
mod executor;
#[cfg(test)]
mod test;

fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .init();
}


#[tokio::main]
pub async fn main() -> Result<(), hunter::HunterError> {
    let args = cli::Cli::parse();
    init_logger();
    Ok(())
}

