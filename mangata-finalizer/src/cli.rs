use clap::{ArgGroup, Args, Parser, Subcommand};
use ethers::types::Address;
use eyre::Ok;
use serde::Serialize;
use std::{fmt::Debug, path::PathBuf};

use crate::crypto::keystore::EncodedKeystore;

/// Simple program to greet a person
#[derive(Parser, Serialize)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("ecdsa-key")
        .required(true)
        .args(&["ecdsa_key_file", "ecdsa_key_json"]),
))]
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

    #[command(flatten)]
    pub ecdsa_key: EcdsaKey,
    #[arg(long, env, default_value_t = String::new())]
    #[serde(skip)]
    pub ecdsa_key_password: String,

    #[command(flatten)]
    pub bls_key: BlsKey,
    #[arg(long, env, default_value_t = String::new())]
    #[serde(skip)]
    pub bls_key_password: String,

    #[arg(long, env, default_value_t = false)]
    pub register_at_startup: bool,

    #[command(subcommand)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Commands>,
}

#[derive(Args, Serialize, Debug)]
#[group(required = true, multiple = false)]
pub struct EcdsaKey {
    #[arg(long, env)]
    pub ecdsa_key_file: Option<PathBuf>,
    #[arg(long, env)]
    #[serde(skip)]
    pub ecdsa_key_json: Option<String>,
}

#[derive(Args, Serialize, Debug)]
#[group(required = true, multiple = false)]
pub struct BlsKey {
    #[arg(long, env)]
    pub bls_key_file: Option<PathBuf>,
    #[arg(long, env)]
    #[serde(skip)]
    pub bls_key_json: Option<String>,
}

#[derive(Debug, Subcommand, Serialize)]
pub enum Commands {
    Register,
    OptInAvs,
    OptOutAvs,
    PrintStatus,
}

impl CliArgs {
    pub fn build() -> Self {
        let args = CliArgs::parse();
        match args.command {
            Some(Commands::Register) if args.chain_id != 31337 => {
                panic!("Commands only supported on local testnet")
            }
            _ => (),
        };
        args
    }

    pub fn get_ecdsa_keystore(&self) -> eyre::Result<EncodedKeystore> {
        let keystore = if let Some(path) = &self.ecdsa_key.ecdsa_key_file {
            EncodedKeystore::from_path(path, self.ecdsa_key_password.to_owned())
        } else {
            EncodedKeystore::from_string(
                self.ecdsa_key
                    .ecdsa_key_json
                    .clone()
                    .expect("either one must be set"),
                self.ecdsa_key_password.to_owned(),
            )
        }?;
        Ok(keystore)
    }

    pub fn get_bls_keystore(&self) -> eyre::Result<EncodedKeystore> {
        let keystore = if let Some(path) = &self.bls_key.bls_key_file {
            EncodedKeystore::from_path(path, self.bls_key_password.to_owned())
        } else {
            EncodedKeystore::from_string(
                self.bls_key
                    .bls_key_json
                    .clone()
                    .expect("either one must be set"),
                self.bls_key_password.to_owned(),
            )
        }?;
        Ok(keystore)
    }
}
