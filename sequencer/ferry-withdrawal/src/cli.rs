use clap::Parser;
use clap::arg;


#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long, env = "L1_URI")]
    pub l1_uri: String,

    #[arg(long, env = "L2_URI")]
    pub l2_uri: String,

    #[arg(long, value_parser = parse_pkey, env = "PRIVATE_KEY")]
    pub private_key: [u8; 32],

    #[arg(long, env = "CHAIN")]
    pub chain_id: u32,

    #[arg(long, value_parser = parse_addr, env = "ROLLDOWN_CONTRACT")]
    pub rolldown_contract_address: [u8; 20],

    #[arg(long, env = "BATCH_SIZE")]
    pub update_size_limit: usize,

    #[arg(long, env = "CLEANER_BLOCK_OFFSET")]
    pub offset: u64,

    #[arg(long, env = "DRY_RUN")]
    pub dry_run: bool,
}


fn parse_addr(s: &str) -> Result<[u8; 20], ::hex::FromHexError> {
    let mut result = [0u8; 20];
    let parse_result = match (s.starts_with("0x"), s.len()) {
        (true, 42) => hex::decode(&s[2..]),
        (false, 40) => hex::decode(&s[..]),
        _ => Err(hex::FromHexError::InvalidStringLength),
    }?;

    result.copy_from_slice(&parse_result.as_ref());
    Ok(result)
}

fn parse_pkey(s: &str) -> Result<[u8; 32], ::hex::FromHexError> {
    let mut result = [0u8; 32];
    let parse_result = match (s.starts_with("0x"), s.len()) {
        (true, 66) => hex::decode(&s[2..]),
        (false, 64) => hex::decode(&s[..]),
        _ => Err(hex::FromHexError::InvalidStringLength),
    }?;

    result.copy_from_slice(&parse_result.as_ref());
    Ok(result)
}
