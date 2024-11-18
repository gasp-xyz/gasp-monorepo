use tracing::level_filters::LevelFilter;
use envconfig::Envconfig;
use hex::FromHex;

mod sequencer;

use sequencer::Sequencer;
mod l1;
use l1::RolldownContract;

mod l2;
use l2::Gasp;

// ROLLUP_SEQUENCER_MANGATA_NODE_URL=ws://node-alice:9944
// ROLLUP_SEQUENCER_ETH_CHAIN_URL_ETH=ws://eth-stub:8545
// ROLLUP_SEQUENCER_MNEMONIC_ETH="0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b"
// ROLLUP_SEQUENCER_L1_CHAIN_ETH="Ethereum"
// ROLLUP_SEQUENCER_ETH_CHAIN_URL_ARB=ws://arbitrum-stub:8546
// ROLLUP_SEQUENCER_L1_CHAIN_ARB="Arbitrum"
// ROLLUP_SEQUENCER_MNEMONIC_ARB="0x0b6e18cafb6ed99687ec547bd28139cafdd2bffe70e6b688025de6b445aa5c5b"
// ROLLUP_SEQUENCER_BLOCK_NUMBER_DELAY=0

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

    // if let Err(err) = run(config).await {
    //     tracing::error!("{err:?}");
    //     // tracing::stacktrace::stacktrace().unwrap();
    // }
}

fn strip_prefix(str: &String) -> &str {
    if let Some(without_prefix) = str.strip_prefix("0x") {
        without_prefix
    }else{
        &str
    }
}

async fn run(config: Config) -> Result<(), Error> {

    let update_size_limit = config.update_size_limit.parse::<u128>().unwrap();
    assert!(update_size_limit > 0, "Update size limit must be greater than 0");
    let eth_secret_key = <[u8; 32]>::from_hex(strip_prefix(&config.l1_private_key))?;
    let gasp_secret_key = <[u8; 32]>::from_hex(strip_prefix(&config.l2_mnemonic))?;
    let rolldown_contract_address = <[u8; 20]>::from_hex(strip_prefix(&config.rolldown_contract_address))?;
    let chain = match config.chain.to_lowercase().as_ref() {
        "ethereum" => Ok(l2::types::Chain::Ethereum),
        "arbitrum" => Ok(l2::types::Chain::Arbitrum),
        _ => Err(Error::UnsupportedChain(config.chain.clone()))
    }?;

    tracing::trace!("Initiating connection to {}", config.l1_uri);
    let gasp = Gasp::new(
        &config.l2_uri,
        gasp_secret_key,
    )
    .await.map_err(Into::<sequencer::Error>::into)?;
    tracing::info!("Connected to {}", config.l2_uri);

    let rolldown = RolldownContract::new(
        &config.l1_uri,
        rolldown_contract_address,
        eth_secret_key
    )
    .await.map_err(Into::<sequencer::Error>::into)?;
    tracing::info!("Connected to {}", config.l1_uri);

    let seq = Sequencer::new(rolldown, gasp, chain, update_size_limit);
    seq.run().await?;
    Ok(())
}
