
use cli::CliArgs;
use eyre::{eyre, Ok};
use syncer::Syncer;
use std::sync::Arc;
use tracing::{info, instrument, warn};

mod cli;
mod crypto;
mod syncer;
mod chainio;


pub async fn start() -> eyre::Result<()> {
    let cli = CliArgs::build();
    info!(
        "Creating a new Operator from {}",
        serde_json::to_string_pretty(&cli)?
    );
    let syncer = Syncer::from_cli(&cli).await?;

    if cli.reinit {

        syncer.clone().reinit(&cli).await?;
        syncer.sync(&cli).await?;

    } else if cli.only_reinit {

        syncer.reinit(&cli).await?;

    } else {

        syncer.sync(&cli).await?;

    }

    warn!("Eth websocket listener closed, shutting down.");

    Ok(())
}
