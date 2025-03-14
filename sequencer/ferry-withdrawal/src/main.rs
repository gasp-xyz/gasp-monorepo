use clap::Parser;
use tokio::sync::mpsc::channel;

mod cli;
mod hunter;
mod cleaner;
mod filter;
mod ferry;
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

    // let (hunter_output, filter_input) = channel(usize::MAX);
    // let (filter_output, hunter_input) = channel(usize::MAX);
    // let (action_sender, executor) = channel(usize::MAX);
    //

    Ok(())
}

