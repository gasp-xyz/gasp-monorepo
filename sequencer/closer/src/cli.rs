use clap::arg;
use clap::Parser;
use hex::FromHexError;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, env = "L1_URI")]
    pub l1_uri: String,

    #[arg(long, env = "L2_URI")]
    pub l2_uri: String,

    #[arg(long, env = "STASH_URI")]
    pub stash_uri: Option<String>,

    #[arg(long, default_value_t = false, env = "SKIP_STASH")]
    pub skip_stash: bool,

    #[arg(long, default_value_t = false, env = "FORCE_POLLING")]
    pub polling: bool,

    #[arg(long, value_parser = parse_pkey, env = "PRIVATE_KEY")]
    pub private_key: [u8; 32],

    #[arg(long, env = "CHAIN")]
    pub chain_id: u32,

    #[arg(long, default_value_t = 0u128, env = "OFFSET")]
    pub offset: u128,

    #[arg(long, env = "DRY_RUN")]
    pub dry_run: bool,

    #[arg(long, value_parser = parse_addr, env = "ROLLDOWN_CONTRACT")]
    pub rolldown_contract_address: [u8; 20],

    #[arg(long, env = "BATCH_SIZE")]
    pub batch_size: usize,

    #[arg(long, default_value_t = 1, env = "REPLICA_ID")]
    pub replica_id: u128,

    #[arg(long, default_value_t = 1, env = "REPLICA_COUNT")]
    pub replica_count: u128,

    #[arg(long, default_value_t = 0.25, env = "DELAY_TIME_BETWEEN_QUERIES")]
    pub delay_between_queries: f64,

    #[arg(long, default_value_t = false, env = "CLOSE_WITHDRAWALS_IN_BATCHES")]
    pub close_withdrawals_in_batches: bool,
}

fn parse_addr(s: &str) -> Result<[u8; 20], FromHexError> {
    let mut result = [0u8; 20];
    let parse_result = match (s.starts_with("0x"), s.len()) {
        (true, 42) => hex::decode(&s[2..]),
        (false, 40) => hex::decode(s),
        _ => Err(hex::FromHexError::InvalidStringLength),
    }?;

    result.copy_from_slice(parse_result.as_ref());
    Ok(result)
}

fn parse_pkey(s: &str) -> Result<[u8; 32], FromHexError> {
    let mut result = [0u8; 32];
    let parse_result = match (s.starts_with("0x"), s.len()) {
        (true, 66) => hex::decode(&s[2..]),
        (false, 64) => hex::decode(s),
        _ => Err(hex::FromHexError::InvalidStringLength),
    }?;

    result.copy_from_slice(parse_result.as_ref());
    Ok(result)
}
