use cli::CliArgs;
use operator::Operator;
use tracing::info;

mod chainio;
mod cli;
mod crypto;
mod executor;
mod operator;
mod rpc;

pub async fn start() -> eyre::Result<()> {
    let cli = CliArgs::build();
    info!(
        "Creating a new Operator from {}",
        serde_json::to_string_pretty(&cli)?
    );
    let operator = Operator::from_cli(&cli).await?;

    if let Some(cmd) = cli.command {
        info!("Operator created with command '{:?}'", cmd);
        match cmd {
            cli::Commands::Register => register(operator).await?,
            cli::Commands::Deregister => todo!(),
            cli::Commands::OptInAvs => todo!(),
            cli::Commands::OptOutAvs => todo!(),
            cli::Commands::PrintStatus => todo!(),
        }
    } else {
        info!("Operator created and starting AVS verification");
        verify(operator, cli.register_at_startup).await?;
    }

    Ok(())
}

pub async fn verify(operator: Operator, register: bool) -> eyre::Result<()> {
    if register {
        operator.register().await?;
    }

    operator.check_registration().await?;
    operator.watch_new_tasks().await?;

    Ok(())
}

async fn register(operator: Operator) -> eyre::Result<()> {
    operator.register().await?;

    Ok(())
}
