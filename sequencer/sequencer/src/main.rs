use envconfig::Envconfig;
use hex::FromHex;
use tracing::level_filters::LevelFilter;

mod sequencer;
mod watchdog;
use tokio::time::Duration;
use watchdog::Watchdog;

use sequencer::Sequencer;
mod l1;
use l1::RolldownContract;

mod l2;
use l2::Gasp;

#[derive(Envconfig, Debug)]
struct Config {
    #[envconfig(from = "ETH_CHAIN_URL")]
    pub l1_uri: String,

    #[envconfig(from = "ETH_PRIV_KEY")]
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
    pub update_size_limit: String,

    #[envconfig(from = "WATCHDOG")]
    pub timeout: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Sequencer error")]
    SequencerError(#[from] sequencer::Error),
    #[error("L1 error")]
    DeserializationError(#[from] hex::FromHexError),
    #[error("Unsupported chain `{0}`")]
    UnsupportedChain(String),
}

#[tokio::main]
pub async fn main() {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("sequencer=trace".parse().expect("proper directive"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let config = Config::init_from_env().unwrap();

    tracing::info!("Config {:#?}", config);

    run(config).await.unwrap();
}

fn strip_prefix(str: &String) -> &str {
    if let Some(without_prefix) = str.strip_prefix("0x") {
        without_prefix
    } else {
        str
    }
}

async fn run(config: Config) -> Result<(), Error> {
    let timeout = config.timeout.parse::<u64>().expect("timeout is set");
    let (tx, mut watchdog) = Watchdog::new(Duration::from_secs(timeout));

    let watchdog = tokio::spawn(async move {
        tracing::info!("Starting watchdog");
        watchdog.run().await;
    });

    let update_size_limit = config.update_size_limit.parse::<u128>().unwrap();
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
        _ => Err(Error::UnsupportedChain(config.chain.clone())),
    }?;

    tracing::trace!("Initiating connection to {}", config.l1_uri);
    let gasp = Gasp::new(&config.l2_uri, gasp_secret_key)
        .await
        .map_err(Into::<sequencer::Error>::into)?;
    tracing::info!("Connected to {}", config.l2_uri);

    let rolldown = RolldownContract::new(&config.l1_uri, rolldown_contract_address, eth_secret_key)
        .await
        .map_err(Into::<sequencer::Error>::into)?;
    tracing::info!("Connected to {}", config.l1_uri);

    let seq = Sequencer::new(rolldown, gasp, chain, update_size_limit);
    let sequencer_service = tokio::spawn(async move { seq.run(tx).await });

    tokio::select! {
        seq = sequencer_service => {
            seq.expect("joined").expect("sequencer failed");
        },
        watch = watchdog => {
            watch.expect("watchdog failed");
        }
    };

    Ok(())
}
