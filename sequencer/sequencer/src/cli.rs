use clap::arg;
use clap::Parser;
use common::PKeyWrapper;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, env = "ETH_CHAIN_URL")]
    pub l1_uri: String,

    #[arg(long, value_parser = common::parse_pkey, env = "PRIVATE_KEY")]
    pub l1_private_key: PKeyWrapper,

    #[arg(long, env = "MANGATA_NODE_URL")]
    pub l2_uri: String,

    #[arg(long, value_parser = common::parse_pkey, env = "MNEMONIC")]
    pub l2_mnemonic: PKeyWrapper,

    #[arg(long, env = "L1_CHAIN")]
    pub chain: String,

    #[arg(long, value_parser = common::parse_addr, env = "MANGATA_CONTRACT_ADDRESS")]
    pub rolldown_contract_address: [u8; 20],

    #[arg(long, env = "LIMIT")]
    pub update_size_limit: u128,

    #[arg(long, env = "WATCHDOG")]
    pub timeout: u128,

    #[arg(long, env = "TX_COST")]
    pub tx_cost: Option<u128>,

    #[arg(long, env = "METRICS_PORT", default_value_t = 8080)]
    pub metrics_port: u16,

    #[arg(long, default_value_t = false, env = "FORCE_POLLING")]
    pub polling: bool,
}
