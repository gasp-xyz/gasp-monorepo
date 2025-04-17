use clap::arg;
use clap::Parser;
use common::PKeyWrapper;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, env = "L1_URI")]
    pub l1_uri: String,

    #[arg(long, env = "L2_URI")]
    pub l2_uri: String,

    #[arg(long, env = "STASH_URI")]
    pub stash_uri: Option<String>,

    #[arg(long, value_parser = common::parse_pkey, env = "PRIVATE_KEY")]
    pub private_key: PKeyWrapper,

    #[arg(long, env = "CHAIN")]
    pub chain_id: u32,

    #[arg(long, env = "DRY_RUN")]
    pub dry_run: bool,

    #[arg(long, value_parser = common::parse_addr, env = "ROLLDOWN_CONTRACT")]
    pub rolldown_contract_address: [u8; 20],

    #[arg(long, default_value_t = false, env = "SKIP_STASH")]
    pub skip_stash: bool,

    #[arg(long, default_value_t = false, env = "FORCE_POLLING")]
    pub polling: bool,

    #[arg(long, default_value_t = 0u128, env = "OFFSET")]
    pub offset: u128,

    #[arg(long, default_value_t = 25, env = "BATCH_SIZE")]
    pub batch_size: usize,

    #[arg(long, default_value_t = 1, env = "REPLICA_ID")]
    pub replica_id: u128,

    #[arg(long, default_value_t = 1, env = "REPLICA_COUNT")]
    pub replica_count: u128,

    #[arg(long, default_value_t = 0.25, env = "DELAY_TIME_BETWEEN_QUERIES")]
    pub delay_between_queries: f64,

    #[arg(long, value_parser= common::parse_addr, env = "CICKA_CONTRACT_ADDRESS")]
    pub cicka_address: Option<[u8; 20]>,

    #[arg(long, default_value_t = 0, env = "BLOCK_DELAY")]
    pub block_delay: usize,
}
