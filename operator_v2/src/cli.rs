use clap::{Parser, Subcommand};
use ethers::types::Address;
use serde::Serialize;
use std::{fmt::Debug, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser, Serialize)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long, env)]
    pub avs_service_manager_addr: Address,
    #[arg(long, env)]
    pub bls_compendium_addr: Address,
    #[arg(long, env)]
    pub bls_operator_state_retriever_addr: Address,

    #[arg(long, env)]
    pub substrate_rpc_url: String,
    #[arg(long, env)]
    pub eth_rpc_url: String,
    #[arg(long, env)]
    pub eth_ws_url: String,
    #[arg(long, env)]
    pub avs_rpc_url: String,

    #[arg(long, env)]
    pub chain_id: u64,

    #[arg(long, env)]
    pub ecdsa_key_file: PathBuf,
    #[arg(long, env, default_value_t = String::new())]
    #[serde(skip)]
    pub ecdsa_key_password: String,
    #[arg(long, env)]
    pub bls_key_file: PathBuf,
    #[arg(long, env, default_value_t = String::new())]
    #[serde(skip)]
    pub bls_key_password: String,

    #[arg(long, env, default_value_t = false)]
    pub register_at_startup: bool,

    #[command(subcommand)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand, Serialize)]
pub enum Commands {
    Register,
    Deregister,
    OptInAvs,
    OptOutAvs,
    PrintStatus,
}

impl CliArgs {
    pub fn build() -> Self {
        let args = CliArgs::parse();
        match args.command {
            Some(Commands::Register) | Some(Commands::Deregister) if args.chain_id != 31337 => {
                panic!("Commands only supported on local testnet")
            }
            _ => (),
        };
        args
    }
}
