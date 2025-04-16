use clap::Parser;
use closer::Closer;
use futures::StreamExt;
use l1api::{Subscription, L1};
use l2api::{Gasp, L2Interface};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::Duration;

mod batch_subscription;
mod cli;
mod closer;
mod closer_metrics;
mod filter;
mod past_withdrawals_finder;

fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .init();
}

pub type Result = anyhow::Result<()>;
pub type TaskHandle = JoinHandle<Result>;

#[tokio::main]
pub async fn main() -> Result {
    init_logger();
    let args = cli::Cli::parse();

    tracing::info!("config: {args:#?}");

    let subscription = if args.polling {
        Subscription::Polling
    } else {
        Subscription::Subscription
    };
    tracing::info!("L1 subscription type {subscription:?}");

    let chain: gasp_types::Chain = args.chain_id.try_into()?;

    let provider = l1api::create_provider(args.l1_uri, args.private_key.into()).await?;
    let rolldown = if args.dry_run {
        l1api::RolldownContract::new_dry_run(provider.clone(), args.rolldown_contract_address)
    } else {
        l1api::RolldownContract::new(provider.clone(), args.rolldown_contract_address)
    };
    let l1 = L1::new(rolldown, provider, subscription);
    let l2 = Gasp::new(&args.l2_uri, args.private_key.into()).await?;

    let (filter_input, filter) = mpsc::channel(1_000_000);
    let header_stream = l2
        .header_stream(l2api::Finalization::Best)
        .await?
        .map(|elem| elem.map(|(nr, _at)| nr as u128))
        .boxed();
    let (closer_input, closer_sink, delay_fut) =
        common::delay::create_delay_channel(header_stream, 1u128);
    let delay_task: TaskHandle = tokio::spawn(async move { Ok(delay_fut.await?) });

    let mut finder = past_withdrawals_finder::FerryHunter::new(
        chain,
        l1.clone(),
        l2.clone(),
        args.batch_size,
        args.offset,
        Duration::from_secs_f64(args.delay_between_queries),
        filter_input.clone(),
        args.replica_id,
        args.replica_count,
    );
    let finder_task: TaskHandle = tokio::spawn(async move { Ok(finder.run().await?) });
    let mut new_withdrawals_subscriber = batch_subscription::WithdrawalSubscriber::new(
        chain,
        l1.clone(),
        l2.clone(),
        filter_input.clone(),
        args.batch_size,
        args.replica_id,
        args.replica_count,
    );
    let new_withdrawals_subscriber_task: TaskHandle =
        tokio::spawn(async move { Ok(new_withdrawals_subscriber.run().await?) });

    let filter_task: TaskHandle = match (args.stash_uri, args.skip_stash) {
        (Some(stash_uri), false) => {
            tracing::info!("filtering withdrawals created by frontend");
            tokio::spawn(async move {
                let stash = stash_api::Stash::new(stash_uri);
                filter::filter_deposits_created_by_frontend(stash, filter, closer_input).await;
                Ok(())
            })
        }
        (_, true) => {
            tracing::info!("filtering withdrawals with ferry tip > 0");
            tokio::spawn(async move {
                filter::filter_deposits_without_fee(filter, closer_input).await;
                Ok(())
            })
        }
        (None, false) => {
            panic!("!!! stash uri not set !!!");
        }
    };

    let closer_task: TaskHandle = tokio::spawn(async move {
        let mut closer = Closer::new(
            chain,
            l1,
            l2,
            closer_sink,
            args.close_withdrawals_in_batches,
        );
        Ok(closer.run().await?)
    });

    if let Err(e) = futures::future::try_join_all([
        finder_task,
        delay_task,
        new_withdrawals_subscriber_task,
        filter_task,
        closer_task,
    ])
    .await
    {
        tracing::error!("err : {e}");
    }

    Ok(())
}
