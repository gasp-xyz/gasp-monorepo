use clap::Parser;
use ferry::FerryError;
use hunter::HunterError;
use l1api::{Subscription, L1};
use l2api::Gasp;
use tokio::sync::mpsc::channel;

mod cleaner;
mod cli;
mod ferry;
mod filter;
mod hunter;

fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .init();
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unsupported chain id {0:?}")]
    InvalidChainId(#[from] gasp_types::ChainParseError),

    #[error("Hunter error `{0}`")]
    HunterError(#[from] hunter::HunterError),

    #[error("Cleaner error `{0}`")]
    CleanerError(#[from] cleaner::CleanerError),

    #[error("ExecutorError error `{0}`")]
    ExecutorError(#[from] ferry::FerryError),

    #[error("L1Error error `{0}`")]
    L1Error(#[from] l1api::L1Error),

    #[error("L2Error error `{0}`")]
    L2Error(#[from] l2api::L2Error),
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let args = cli::Cli::parse();
    let chain: gasp_types::Chain = args.chain_id.try_into()?;
    init_logger();

    tracing::info!("config: {args:#?}");

    let subscription = if args.polling {
        Subscription::Polling
    } else {
        Subscription::Subscription
    };
    let (hunter_to_filter, filter_input) = channel(1_000_000);
    let (to_executor, executor) = channel(1_000_000);

    let provider = l1api::create_provider(args.l1_uri, args.private_key.into()).await?;
    let sender = l1api::address(provider.clone());
    let rolldown = l1api::RolldownContract::new(provider.clone(), args.rolldown_contract_address);

    let l2 = Gasp::new(&args.l2_uri, args.private_key.into()).await?;
    let l1 = L1::new(rolldown.clone(), provider.clone(), subscription);

    let mut cleaner = {
        cleaner::FerryCleaner::new(
            chain,
            l1.clone(),
            l2.clone(),
            sender,
            to_executor.clone(),
            args.offset as u64,
        )
    };

    let mut hunter = { hunter::FerryHunter::new(chain, l1.clone(), l2.clone(), hunter_to_filter) };

    let mut filter = {
        filter::Filter::new(
            l1.clone(),
            l2.clone(),
            filter_input,
            to_executor,
            args.enabled.into_iter().collect(),
        )
    };

    let mut executor = {
        ferry::Ferry::new(
            l1.clone(),
            l2.clone(),
            sender,
            chain,
            args.tx_cost,
            executor,
        )
    };

    if let Some(port) = args.prometheus_port {
        let _balance = tokio::spawn(async move {
            common::report_account_balance(provider).await;
        });
        let _metrics = tokio::spawn(async move {
            common::serve_metrics(port).await;
        });
    }

    let hunter_handle = tokio::spawn(async move {
        hunter.run().await?;
        Ok::<_, HunterError>(())
    });

    let filter_handle = tokio::spawn(async move {
        filter.run().await;
    });

    let executor_handle = tokio::spawn(async move {
        executor.run().await?;
        Ok::<_, FerryError>(())
    });

    let cleaner_handle = tokio::spawn(async move {
        cleaner.run().await?;
        Ok::<_, cleaner::CleanerError>(())
    });

    let (hunter, _filter, executor, cleaner) = tokio::try_join!(
        hunter_handle,
        filter_handle,
        executor_handle,
        cleaner_handle
    )
    .expect("can join");
    hunter?;
    executor?;
    cleaner?;

    Ok(())
}
