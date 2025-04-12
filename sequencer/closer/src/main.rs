use clap::Parser;
use futures::{future::join_all, stream::FuturesUnordered, StreamExt};
use hex_literal::hex;
use l1api::create_provider;

mod cli;
mod closer;

fn init_logger() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(false)
        .init();
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}

const ALICE_PKEY: [u8; 32] =
    hex!("dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97");

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    // let _args = cli::Cli::parse();
    init_logger();

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

    let mut futures: FuturesUnordered<_> = uris.into_iter().map(|(uri, addr)| {
        tokio::spawn(async move {
            let network = uri.split('.').next().unwrap();
            tracing::info!("{uri} subscribing");
            let provider = create_provider(&uri, ALICE_PKEY).await.unwrap();
            let rolldown = l1api::RolldownContract::new(provider, addr);
            let mut subscription = rolldown.subscribe_deposit().await.expect("can subscribe");
            while let Some(deposit) = subscription.next().await {
                tracing::info!("{network} new deposit found {deposit}");
                match rolldown.get_deposit(deposit).await {
                    Ok(deposit) => {
                        let d : gasp_types::Deposit = deposit.into();
                        tracing::info!("deposit found {d}");
                    },
                    Err(e) => {
                        tracing::warn!("deposit not there yet {e}");
                    }
                }
            }
        })
    }).collect();

    let result = futures.next().await;

    tracing::error!("result {:?}", result);

    Ok(())
}
