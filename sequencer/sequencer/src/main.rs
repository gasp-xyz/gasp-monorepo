use envconfig::Envconfig;
use hex::FromHex;
use tracing::level_filters::LevelFilter;

mod metrics;
mod sequencer;
mod watchdog;
use tokio::time::Duration;
use watchdog::Watchdog;

use sequencer::Sequencer;
mod l1;
use l1::{CachedL1Interface, RolldownContract};

mod l2;
use l2::Gasp;

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

    #[envconfig(from = "PORT", default = "8080")]
    pub metrics_port: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Sequencer error")]
    SequencerError(#[from] sequencer::Error),
    #[error("L1 error")]
    L1Error(#[from] l1::L1Error),
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

    let metrics_port = config.metrics_port;
    let _metrics = tokio::spawn(async move {
        metrics::serve_metrics(metrics_port).await;
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
        "ethereum" => Ok(l2::types::Chain::Ethereum),
        "arbitrum" => Ok(l2::types::Chain::Arbitrum),
        "base" => Ok(l2::types::Chain::Base),
        "monad" => Ok(l2::types::Chain::Monad),
        "megaeth" => Ok(l2::types::Chain::MegaEth),
        "sonic" => Ok(l2::types::Chain::Sonic),
        _ => Err(Error::UnsupportedChain(config.chain.clone())),
    }?;

    tracing::trace!("Initiating connection to {}", config.l1_uri);
    let gasp = Gasp::new(&config.l2_uri, gasp_secret_key)
        .await
        .map_err(Into::<sequencer::Error>::into)?;
    tracing::info!("Connected to {}", config.l2_uri);

    let provider = l1::create_provider(config.l1_uri.as_str(), eth_secret_key).await?;
    tracing::info!("Connected to {}", config.l1_uri);

    let rolldown = RolldownContract::from_provider(rolldown_contract_address, provider.clone());

    let _balance_tracker = tokio::spawn(async move {
        metrics::report_account_balance(provider).await;
    });

    let lru = CachedL1Interface::new(rolldown, std::num::NonZeroUsize::new(100).unwrap());
    let seq = Sequencer::new(lru, gasp, chain, update_size_limit, config.tx_cost);
    let sequencer_service = tokio::spawn(async move { seq.run(tx).await });

    watchdog
        .await
        .map_err(|_| Error::WatchdogExpired(duration))?;
    Ok(sequencer_service.await.expect("joined")?)
}
