use clap::Parser;
use ferry::FerryError;
use hunter::HunterError;
use l1api::L1;
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

    let (hunter_to_filter, filter_input) = channel(usize::MAX);
    let (to_executor, executor) = channel(usize::MAX);

    let provider = l1api::create_provider(args.l1_uri, args.private_key).await?;
    let sender = l1api::address(provider.clone());
    let rolldown = l1api::RolldownContract::new(provider.clone(), args.rolldown_contract_address);

    let mut cleaner = {
        let l1 = L1::new(rolldown.clone(), provider.clone());
        let l2 = Gasp::new(&args.l2_uri, args.private_key).await?;
        cleaner::FerryCleaner::new(chain, l1, l2, sender, to_executor.clone(), args.offset)
    };

    let mut hunter = {
        let l1 = L1::new(rolldown.clone(), provider.clone());
        let l2 = Gasp::new(&args.l2_uri, args.private_key).await?;
        hunter::FerryHunter::new(chain, l1, l2, hunter_to_filter)
    };

    let mut filter = {
        let l1 = L1::new(rolldown.clone(), provider.clone());
        let l2 = Gasp::new(&args.l2_uri, args.private_key).await?;
        filter::Filter::new(l1, l2, filter_input, to_executor, vec![])
    };

    let mut executor = {
        let l1 = L1::new(rolldown, provider);
        let l2 = Gasp::new(&args.l2_uri, args.private_key).await?;
        ferry::Ferry::new(l1, l2, sender, chain, executor)
    };

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
    ).expect("can join");
    hunter?;
    executor?;
    cleaner?;

    Ok(())

    // Since `tokio::spawn` returns `Result<JoinHandle<Result<(), E>>>`, unwrap the join results
    // match result {
    //     Ok((hunter_res, filter_res, executor_res, cleaner_res)) => {
    //         hunter_res?;
    //         filter_res;
    //         executor_res?;
    //         cleaner_res?;
    //         Ok(())
    //     }
    //     Err(join_err) => Err(join_err.into()), // Convert JoinError if any task panicked
    // }

}
