use cli::CliArgs;
use eyre::Ok;
use syncer::Syncer;
use tracing::{info, warn};

mod chainio;
mod cli;
mod crypto;
mod syncer;

pub async fn start() -> eyre::Result<()> {
    let cli = CliArgs::build();
    info!(
        "Creating a new Syncer from {}",
        serde_json::to_string_pretty(&cli)?
    );
    let syncer = Syncer::from_cli(&cli).await?;

    match () {
        _ if cli.reinit => {
            syncer.clone().reinit(&cli).await?;
            syncer.sync(&cli).await?;
        }
        _ if cli.only_reinit => {
            syncer.reinit(&cli).await?;
        }
        _ if cli.only_reinit_eth => {
            syncer.reinit_eth(&cli).await?;
        }
        _ if cli.reinit_eth_only_print_op_task_creation => {
            syncer.reinit_eth_only_print_op_task_creation(&cli).await?;
        }
        _ if cli.reinit_eth_only_print_op_task_response => {
            syncer.reinit_eth_only_print_op_task_response(&cli).await?;
        }
        _ if cli.reinit_only_print => {
            syncer.reinit_only_print(&cli).await?;
        }
        _ => {
            syncer.sync(&cli).await?;
        }
    }

    warn!("Eth websocket listener closed, shutting down.");

    Ok(())
}
