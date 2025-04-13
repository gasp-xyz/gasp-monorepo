use clap::Parser;
use futures::{future::join_all, stream::FuturesUnordered, FutureExt, StreamExt};
use hex_literal::hex;
use l2api::{Gasp, L2Error};
use tokio::sync::mpsc;
use l1api::{create_provider, L1Error, L1};
use tokio::time::error::Elapsed;
use tokio::time::Duration;
use gasp_types::ChainParseError;

mod cli;
mod closer;
mod past_withdrawals_finder;
mod filter;
mod batch_subscription;

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
    #[error("unsupported chain id")]
    UnknwonL2Chain(#[from] ChainParseError),
    #[error("L1 error")]
    L1Error(#[from] L1Error),
    #[error("L2 error")]
    L2Error(#[from] L2Error)
}

const ALICE_PKEY: [u8; 32] =
    hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    init_logger();
    let args = cli::Cli::parse();

    let (filter_input, receiver) = mpsc::channel(1_000_000);

    let chain: gasp_types::Chain = args.chain_id.try_into()?;

    let provider = l1api::create_provider(args.l1_uri, args.private_key).await?;
    let sender = l1api::address(provider.clone());
    let rolldown = l1api::RolldownContract::new(provider.clone(), args.rolldown_contract_address);
    let l1 = L1::new(rolldown, provider);

    let l2 = Gasp::new(&args.l2_uri, args.private_key).await?;

    let finder = past_withdrawals_finder::FerryHunter::new(chain, l1.clone(), l2.clone(), args.batch_size, args.offset, Duration::from_secs_f64(0.25), filter_input.clone());
    let new_withdrawals_subscriber = batch_subscription::WithdrawalSubscriber::new(chain, l1.clone(), l2.clone(), filter_input, args.batch_size);

    // let filter = 

    // batch_subscription::WithdrawalSubscriber::new(chain, l1.clone(), l2.clone(), filter_input, args.batch_size);



    let uris = vec![
        (
            "wss://mainnet.gateway.tenderly.co/4GnrTfJWhv3LPgLbq2vCzc".to_string(),
            hex!("79d968d9017B96f202aD4673A2c1BBbdc905A4ca"),
        ),
        (
            "wss://arbitrum.gateway.tenderly.co/CjkGKEDIni405SZDICBBi".to_string(),
            hex!("3aDdEb54ddd43Eb40235eC32DfA7928F28A44bb5"),
        ),
        (
            "wss://base-mainnet.g.alchemy.com/v2/k4y4wYlGWpPoONqTRCvdzUV37hu-0qoJ".to_string(),
            hex!("308e483afDD225D6cb7bF4d44B8e4a03DFD9c0De"),
        ),
        (
            "wss://sonic-mainnet.g.alchemy.com/v2/qaIj_tGGQazBPnF_XBB1FPe5SMZT4aFJ".to_string(),
            hex!("F5C9b2ee7bb091E245c92feCb9218Bf952BEA637"),
        ),
        (
            "wss://holesky.gateway.tenderly.co/FD8CGb2oqAKb2aNEZpdZj".to_string(),
            hex!("34B38Fd98aBE38B52258253fb4E1D0F5EFF26343"),
        ),
        (
            "wss://base-sepolia.gateway.tenderly.co/4hIGZKyCd7jWCrDhWEndhf".to_string(),
            hex!("2215C253202504a42723B46fe72D1Bb61B3b1918"),
        ),
        (
            "wss://arbitrum-sepolia.gateway.tenderly.co/6tKF6q6T9CFWlbKB341Qx".to_string(),
            hex!("18Ef4858e5581d20c2145ea0dFEA7F4d81A957A5"),
        ),
        // ("wss://cosmological-delicate-field.monad-testnet.quiknode.pro/bac2a4681e538a0315f21cdadb50e07d37ba401a", hex!("fd6A45621DDfeBF94C082e60E0De92aA305a97a1")),
        // ("wss://carrot.megaeth.com/ws", hex!("998AaF69F731009d4E2d470E974766F1EB8f5142")),
    ];

    let mut futures: FuturesUnordered<_> = uris
        .into_iter()
        .map(|(uri, addr)| {
            tokio::spawn(async move {
                let network = uri.split('.').next().unwrap();
                tracing::info!("{uri} subscribing");
                let provider = create_provider(&uri, ALICE_PKEY).await.unwrap();
                let rolldown = l1api::RolldownContract::new(provider, addr);
                let mut subscription = rolldown.subscribe_deposit().await.expect("can subscribe");
                loop {
                    match tokio::time::timeout(
                        tokio::time::Duration::from_secs_f64(60.0),
                        subscription.next(),
                    ).await {
                        Ok(notification) => {
                            let deposit_id = notification.expect("infinite stream");
                            let deposit= rolldown
                                .get_deposit(deposit_id)
                                .map(|d| d.map(gasp_types::Deposit::from))
                                .await
                                .expect("should exists");
                            tracing::trace!("deposit found {deposit}")
                        }
                        Err(Elapsed) => {
                            tracing::info!("{uri} keep alive");
                        } // match rolldown.get_deposit(deposit).await {
                          //     Ok(deposit) => {
                          //         let d : gasp_types::Deposit = deposit.into();
                          //         tracing::info!("deposit found {d}");
                          //     },
                          //     Err(e) => {
                          //         tracing::warn!("deposit not there yet {e}");
                          //     }
                          // }
                    }
                }
            })
        })
        .collect();

    let result = futures.next().await;

    tracing::error!("result {:?}", result);

    Ok(())
}
