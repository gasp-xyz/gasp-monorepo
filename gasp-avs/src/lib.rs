use chainio::setup_deposits;
use cli::CliArgs;
use ethers::signers::Signer;
use eyre::{eyre, Ok};
use operator::Operator;
use std::sync::Arc;
use tracing::{info, instrument, warn};

mod chainio;
mod cli;
mod crypto;
mod metrics;
mod operator;
mod rpc;

pub async fn start() -> eyre::Result<()> {
    let cli = CliArgs::build();
    info!(
        "Creating a new Operator from {}",
        serde_json::to_string_pretty(&cli)?
    );
    let operator = Operator::from_cli(&cli).await?;
    let address = operator.signer().address();
    let client = operator.client();

    let _http_server = tokio::spawn(async move {
        metrics::serve_metrics(cli.metrics_port).await;
    });

    let _monitor_balance = tokio::spawn(async move {
        metrics::report_account_balance(address, client).await;
    });

    if let Some(cmd) = &cli.command {
        info!("Operator created with command '{:?}'", cmd);
        match cmd {
            cli::Commands::OptInAvs => operator.opt_in_avs().await?,
            cli::Commands::OptOutAvs => operator.opt_out_avs().await?,
            cli::Commands::PrintStatus => print_status(operator).await?,
        }
    } else if cli.testnet {
        info!("Operator created and starting testnet setup");
        ephemeral_testnet(operator, cli.stake, &cli).await?;
    } else {
        info!("Operator created and starting AVS verification");
        run_node(operator, cli.opt_in_at_startup).await?;
    }

    Ok(())
}

pub async fn run_node(operator: Arc<Operator>, opt_in: bool) -> eyre::Result<()> {
    operator.clone().wait_for_gasp_to_sync().await?;
    check_registration(operator.clone(), opt_in).await?;
    operator.watch_new_tasks().await?;

    warn!("Eth websocket listener closed, shutting down.");

    Ok(())
}

#[instrument(skip_all)]
pub(crate) async fn check_registration(operator: Arc<Operator>, opt_in: bool) -> eyre::Result<()> {
    let status = operator.get_status().await?;
    let local_id = operator.operator_id();

    info!("{:#?}", status);

    match (status.registered_with_eigen, status.operator_id, local_id) {
        (false, _, _) => Err(eyre!(
            "Operator not registered with EigenLayer, use eigenlayer cli to register"
        )),
        (true, None, _) => {
            if opt_in {
                operator.opt_in_avs().await?;
                Ok(())
            } else {
                Err(eyre!(
                    "Operator not registered with AVS, run OptInAvs first"
                ))
            }
        }
        (true, Some(id), local) if id == local => Ok(()),
        _ => Err(eyre!(
            "Registered operator id ({:x}) & BlsKeypair.operator_id() ({:x}) mismatch",
            status.operator_id.unwrap_or_default(),
            local_id
        )),
    }
}

#[instrument(skip_all)]
pub(crate) async fn print_status(operator: Arc<Operator>) -> eyre::Result<()> {
    let status = operator.get_status().await?;
    info!("{:#?}", status);
    Ok(())
}

pub(crate) async fn ephemeral_testnet(
    operator: Arc<Operator>,
    stake: Option<u32>,
    cfg: &CliArgs,
) -> eyre::Result<()> {
    operator.clone().wait_for_gasp_to_sync().await?;
    setup_deposits(
        cfg.eth_rpc_url.clone(),
        cfg.avs_registry_coordinator_addr,
        stake.unwrap_or(100),
        operator.signer(),
        cfg.chain_id,
    )
    .await?;

    operator.clone().register().await?;
    operator.clone().opt_in_avs().await?;
    print_status(operator.clone()).await?;

    info!("Testnet setup sucessfully, starting AVS verification");
    operator.watch_new_tasks().await?;

    Ok(())
}
