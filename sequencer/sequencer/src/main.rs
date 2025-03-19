use alloy::{network::NetworkWallet, providers::WalletProvider};
use envconfig::Envconfig;
use hex::FromHex;
use l1api::CachedL1Interface;
use tracing::level_filters::LevelFilter;

mod sequencer;
mod watchdog;
use gasp_types::Chain;
use l2api::Gasp;
use tokio::time::Duration;
use watchdog::Watchdog;

use sequencer::Sequencer;

#[derive(Envconfig, Debug)]
struct Config {
    #[envconfig(from = "ETH_CHAIN_URL")]
    pub l1_uri: String,

    #[envconfig(from = "PRIVATE_KEY")]
    pub l1_private_key: String,

    #[envconfig(from = "MANGATA_NODE_URL")]
    pub l2_uri: String,

    #[envconfig(from = "MNEMONIC")]
    pub l2_mnemonic: String,

    #[envconfig(from = "L1_CHAIN")]
    pub chain: String,

    #[envconfig(from = "MANGATA_CONTRACT_ADDRESS")]
    pub rolldown_contract_address: String,

    #[envconfig(from = "LIMIT")]
    pub update_size_limit: u128,

    #[envconfig(from = "WATCHDOG")]
    pub timeout: u128,

    #[envconfig(from = "TX_COST")]
    pub tx_cost: Option<u128>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Sequencer error")]
    SequencerError(#[from] sequencer::Error),
    #[error("L1 error")]
    L1Error(#[from] l1api::L1Error),
    #[error("Deserialization error")]
    DeserializationError(#[from] hex::FromHexError),
    #[error("Unsupported chain `{0}`")]
    UnsupportedChain(String),
    #[error("Watchdog expired `{0:?}`")]
    WatchdogExpired(Duration),
}

#[tokio::main]
pub async fn main() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let mut config = Config::init_from_env().unwrap();

    config.tx_cost = match config.tx_cost {
        Some(0u128) => None,
        Some(amount) => Some(amount),
        None => None,
    };

    tracing::info!("Config {:#?}", config);

    if let Err(err) = run(config).await {
        tracing::error!("Error: {:?}", err);
    }
}

fn strip_prefix(str: &String) -> &str {
    if let Some(without_prefix) = str.strip_prefix("0x") {
        without_prefix
    } else {
        str
    }
}

async fn run(config: Config) -> Result<(), Error> {
    let timeout = config.timeout;
    let duration = Duration::from_secs(timeout.try_into().expect("overflow"));
    let (tx, mut watchdog) = Watchdog::new(duration);

    let watchdog = tokio::spawn(async move {
        tracing::info!("Starting watchdog");
        watchdog.run().await;
    });

    let _metrics = tokio::spawn(async move {
        common::serve_metrics(80).await;
    });

    let update_size_limit = config.update_size_limit;
    assert!(
        update_size_limit > 0,
        "Update size limit must be greater than 0"
    );
    let eth_secret_key = <[u8; 32]>::from_hex(strip_prefix(&config.l1_private_key))?;
    let gasp_secret_key = <[u8; 32]>::from_hex(strip_prefix(&config.l2_mnemonic))?;
    let rolldown_contract_address =
        <[u8; 20]>::from_hex(strip_prefix(&config.rolldown_contract_address))?;
    let chain = match config.chain.to_lowercase().as_ref() {
        "ethereum" => Ok(Chain::Ethereum),
        "arbitrum" => Ok(Chain::Arbitrum),
        "base" => Ok(Chain::Base),
        "monad" => Ok(Chain::Monad),
        "megaeth" => Ok(Chain::MegaEth),
        "sonic" => Ok(Chain::Sonic),
        _ => Err(Error::UnsupportedChain(config.chain.clone())),
    }?;

    tracing::trace!("Initiating connection to {}", config.l1_uri);
    let gasp = Gasp::new(&config.l2_uri, gasp_secret_key)
        .await
        .map_err(Into::<sequencer::Error>::into)?;
    tracing::info!("Connected to {}", config.l2_uri);

    let provider = l1api::create_provider(config.l1_uri.clone(), eth_secret_key).await?;
    let rolldown = l1api::RolldownContract::new(provider.clone(), rolldown_contract_address);
    tracing::info!("Connected to {}", config.l1_uri);

    let l1_account_addr = provider.wallet().default_signer_address().0 .0;
    let l2_account_addr = l2api::signer::Keypair::from_secret_key(gasp_secret_key)
        .address()
        .into_inner();

    let rolldown = l1api::L1::new(rolldown, provider.clone());

    let _balance_tracker = tokio::spawn(async move {
        common::report_account_balance(provider).await;
    });

    let lru = CachedL1Interface::new(rolldown, std::num::NonZeroUsize::new(100).unwrap());
    let seq = Sequencer::new(
        lru,
        gasp,
        chain,
        l1_account_addr,
        l2_account_addr,
        update_size_limit,
        config.tx_cost,
    );
    let sequencer_service = tokio::spawn(async move { seq.run(tx).await });

    watchdog
        .await
        .map_err(|_| Error::WatchdogExpired(duration))?;
    Ok(sequencer_service.await.expect("joined")?)
}
