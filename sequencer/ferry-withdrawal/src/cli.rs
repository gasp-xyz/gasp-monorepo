use clap::arg;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, env = "L1_URI")]
    pub l1_uri: String,

    #[arg(long, env = "L2_URI")]
    pub l2_uri: String,

    #[arg(long, value_parser = common::parse_pkey, env = "PRIVATE_KEY")]
    pub private_key: [u8; 32],

    #[arg(long, env = "CHAIN")]
    pub chain_id: u32,

    #[arg(long, value_parser = common::parse_addr, env = "ROLLDOWN_CONTRACT")]
    pub rolldown_contract_address: [u8; 20],

    #[arg(long, default_value_t = 0, env = "OFFSET")]
    pub offset: u32,

    #[arg(long, env = "PROMETHEUS_PORT")]
    pub prometheus_port: Option<u16>,

    #[arg(long, value_parser = common::parse_addr, env = "ENABLED_TOKENS", num_args(0..))]
    enabled: Vec<[u8; 20]>,

    #[arg(long, default_value_t = 1_000_000_000_000_000, env = "TX_COST")]
    pub tx_cost: u128,
}
