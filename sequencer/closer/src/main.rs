use std::convert::Infallible;

use clap::Parser;
use closer::Closer;
use futures::{FutureExt, StreamExt};
use l1api::{Subscription, L1};
use l2api::{Gasp, L2Interface};
use tokio::sync::mpsc;
use tokio::time::Duration;

mod batch_subscription;
mod cli;
mod closer;
mod filter;
mod metrics;
mod past_withdrawals_finder;

fn init_logger(enable_colors: bool) {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(enable_colors)
        .init();
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let args = cli::Cli::parse();

    init_logger(args.colors);

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

    let p = provider.clone();
    if let Some(port) = args.prometheus_port {
        let _balance = tokio::spawn(async move {
            common::report_account_balance(p).await;
        });
        let _metrics = tokio::spawn(async move {
            common::serve_metrics(port).await;
        });
    }

    let cicka = args
        .cicka_address
        .map(|addr| l1api::cicka::Cicka::new(provider.clone(), addr));
    let l1 = L1::new(rolldown, cicka.clone(), provider, subscription);
    let l2 = Gasp::new(&args.l2_uri, args.private_key.into()).await?;

    let (filter_input, filter) = mpsc::channel(1_000_000);
    let header_stream = l2
        .header_stream(l2api::Finalization::Best)
        .await?
        .map(|elem| elem.map(|(nr, _at)| nr as u128))
        .boxed();
    let (closer_input, closer_sink, delay_fut) =
        common::delay::create_delay_channel(header_stream, args.block_delay as u128);

    let delay_task = tokio::spawn(delay_fut).map(|elem| Ok::<_, anyhow::Error>(elem??));

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
    let finder_task =
        tokio::spawn(async move { finder.run().await }).map(|elem| Ok::<_, anyhow::Error>(elem??));

    let mut new_withdrawals_subscriber = batch_subscription::WithdrawalSubscriber::new(
        chain,
        l1.clone(),
        l2.clone(),
        filter_input.clone(),
        args.batch_size,
        args.replica_id,
        args.replica_count,
    );
    let new_withdrawals_subscriber_task =
        tokio::spawn(async move { new_withdrawals_subscriber.run().await })
            .map(|elem| Ok::<_, anyhow::Error>(elem??));

    let filter_task = match (args.stash_uri, args.skip_stash) {
        (Some(stash_uri), false) => {
            tracing::info!("filtering withdrawals created by frontend");
            tokio::spawn(async move {
                let stash = stash_api::Stash::new(stash_uri);
                filter::filter_deposits_created_by_frontend(stash, filter, closer_input).await;
                Ok::<_, Infallible>(())
            })
        }
        (_, true) => {
            tracing::info!("filtering withdrawals with ferry tip > 0");
            tokio::spawn(async move {
                filter::filter_deposits_without_fee(filter, closer_input).await;
                Ok::<_, Infallible>(())
            })
        }
        (None, false) => {
            panic!("!!! stash uri not set !!!");
        }
    }
    .map(|elem| Ok::<_, anyhow::Error>(elem??));

    let closer_task = tokio::spawn(async move {
        let mut closer = Closer::new(chain, l1, l2, closer_sink, cicka.is_some());
        closer.run().await
    })
    .map(|elem| Ok::<_, anyhow::Error>(elem??));

    if let Err(e) = tokio::try_join!(
        closer_task,
        finder_task,
        delay_task,
        new_withdrawals_subscriber_task,
        filter_task,
    ) {
        tracing::error!("err : {e}");
    }

    Ok(())
}
