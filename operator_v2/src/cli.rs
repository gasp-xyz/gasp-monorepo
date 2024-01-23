use clap::{Parser, Subcommand};
use ethers::types::Address;
use std::{fmt::Debug, path::PathBuf};

/// Simple program to greet a person
#[derive(Parser)]
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
    pub chain_id: u64,

    #[arg(long, env)]
    pub ecdsa_key_file: PathBuf,
    #[arg(long, env, default_value_t = String::new())]
    pub ecdsa_key_password: String,
    #[arg(long, env)]
    pub bls_key_file: PathBuf,
    #[arg(long, env, default_value_t = String::new())]
    pub bls_key_password: String,

    #[arg(long, env, default_value_t = false)]
    pub register_at_startup: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Debug for CliArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CliArgs")
            .field("avs_service_manager_addr", &self.avs_service_manager_addr)
            .field("bls_compendium_addr", &self.bls_compendium_addr)
            .field(
                "bls_operator_state_retriever_addr",
                &self.bls_operator_state_retriever_addr,
            )
            .field("substrate_rpc_url", &self.substrate_rpc_url)
            .field("eth_rpc_url", &self.eth_rpc_url)
            .field("eth_ws_url", &self.eth_ws_url)
            .field("chain_id", &self.chain_id)
            .field("ecdsa_key_file", &self.ecdsa_key_file)
            .field("bls_key_file", &self.bls_key_file)
            .field("register_at_startup", &self.register_at_startup)
            .field("command", &self.command)
            .finish()
    }
}

#[derive(Debug, Subcommand)]
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
