use clap::Parser;
use futures::{stream::BoxStream, FutureExt, StreamExt};
use l1api::{Subscription, L1};
use l2api::Gasp;
use tokio::sync::mpsc::channel;

mod cli;
mod ferry;
mod filter;
mod hunter;
mod metrics;

fn init_logger(with_colors: bool) {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(with_colors)
        .init();
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unsupported chain id {0:?}")]
    InvalidChainId(#[from] gasp_types::ChainParseError),

    #[error("Hunter error `{0}`")]
    HunterError(#[from] hunter::HunterError),

    #[error("ExecutorError error `{0}`")]
    ExecutorError(#[from] ferry::FerryError),

    #[error("L1Error error `{0}`")]
    L1Error(#[from] l1api::L1Error),

    #[error("L2Error error `{0}`")]
    L2Error(#[from] l2api::L2Error),

    #[error("Task error `{0}`")]
    TaskError(#[from] tokio::task::JoinError),
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let args = cli::Cli::parse();
    init_logger(args.colors);
    let chain: gasp_types::Chain = args.chain_id.try_into()?;

    tracing::info!("config: {args:#?}");

    let subscription = if args.polling {
        Subscription::Polling
    } else {
        Subscription::Subscription
    };

    let provider = l1api::create_provider(args.l1_uri, args.private_key.into()).await?;
    let sender = l1api::address(provider.clone());
    let rolldown = l1api::RolldownContract::new(provider.clone(), args.rolldown_contract_address);

    let l1 = L1::new(rolldown.clone(), None, provider.clone(), subscription);
    let l2 = Gasp::new(&args.l2_uri, args.private_key.into()).await?;

    let (hunter_to_filter, filter_input) = channel(1_000_000);
    let stream: BoxStream<'static, Result<u128, l1api::L1Error>> = if args.polling {
        rolldown.subscribe_blocks().await?.boxed()
    } else {
        rolldown
            .subscribe_blocks_polling(tokio::time::Duration::from_secs_f64(60.0))
            .await?
            .boxed()
    };

    let (to_executor, executor, delay_fut) =
        common::delay::create_delay_channel(stream, args.block_delay);

    let delay_task = tokio::spawn(delay_fut).map(|elem| Ok::<_, Error>(elem??));

    let mut hunter = { hunter::FerryHunter::new(chain, l1.clone(), l2.clone(), hunter_to_filter) };

    let mut executor = ferry::Ferry::new(
        l1.clone(),
        l2.clone(),
        sender,
        chain,
        args.tx_cost,
        executor,
    );

    for (addr, _) in args.enabled.iter() {
        if let Err(e) = executor.track_balance(*addr).await {
            tracing::error!("{}", e);
        }
    }
    executor.refresh_balances().await?;

    if let Some(port) = args.prometheus_port {
        let _balance = tokio::spawn(async move {
            common::report_account_balance(provider).await;
        });
        let _metrics = tokio::spawn(async move {
            common::serve_metrics(port).await;
        });
    }

    let hunter_handle =
        tokio::spawn(async move { hunter.run().await }).map(|elem| Ok::<_, Error>(elem??));

    let filter_handle = tokio::spawn(async move {
        filter::filter_deposits(
            filter_input,
            to_executor,
            args.enabled.into_iter().collect(),
        )
        .await
    })
    .map(|elem| Ok::<_, Error>(elem?));

    let executor_handle =
        tokio::spawn(async move { executor.run().await }).map(|elem| Ok::<_, Error>(elem??));

    if let Err(e) = futures::try_join!(executor_handle, filter_handle, hunter_handle, delay_task) {
        tracing::error!("err : {e}");
    }

    Ok(())
}
