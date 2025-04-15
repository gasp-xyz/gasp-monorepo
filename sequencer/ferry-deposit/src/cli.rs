use clap::arg;
use clap::Parser;
use common::PKeyWrapper;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, env = "L1_URI")]
    pub l1_uri: String,

    #[arg(long, env = "L2_URI")]
    pub l2_uri: String,

    #[arg(long, value_parser = common::parse_pkey, env = "PRIVATE_KEY")]
    pub private_key: PKeyWrapper,

    #[arg(long, env = "CHAIN")]
    pub chain_id: u32,

    #[arg(long, value_parser = common::parse_addr, env = "ROLLDOWN_CONTRACT")]
    pub rolldown_contract_address: [u8; 20],

    #[arg(long, default_value_t = false, env = "FORCE_POLLING")]
    pub polling: bool,

    #[arg(long, default_value_t = 0, env = "OFFSET")]
    pub offset: u32,

    #[arg(long, env = "PROMETHEUS_PORT")]
    pub prometheus_port: Option<u16>,

    #[arg(long, value_parser = common::parse_tokens_and_weight, env = "ENABLED_TOKENS", value_delimiter=',', num_args = 1..)]
    pub enabled: Vec<([u8; 20], u128)>,

    #[arg(long, default_value_t = 1_000_000_000_000_000, env = "TX_COST")]
    pub tx_cost: u128,
}
