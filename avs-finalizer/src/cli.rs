use clap::{error::ErrorKind, Args, CommandFactory, Parser, Subcommand};
use ethers::types::{Address, Chain};
use eyre::Ok;
use serde::Serialize;
use std::{fmt::Debug, path::PathBuf};
use tracing::warn;

use crate::crypto::keystore::EncodedKeystore;

#[derive(Parser, Serialize)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[arg(long, env)]
    pub avs_registry_coordinator_addr: Address,

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
    #[arg(long, env)]
    #[serde(skip)]
    pub ecdsa_key_password: Option<String>,

    #[command(flatten)]
    pub bls_key: BlsKey,
    #[arg(long, env)]
    #[serde(skip)]
    pub bls_key_password: Option<String>,

    #[arg(long, env, default_value_t = false)]
    pub opt_in_at_startup: bool,

    #[arg(long, env, default_value_t = false)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub testnet: bool,

    #[arg(long, env, requires("testnet"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stake: Option<u32>,

    #[command(subcommand)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Commands>,
}

#[derive(Args, Serialize, Debug)]
#[group(required = true, multiple = false)]
pub struct EcdsaKey {
    #[arg(long, env)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecdsa_key_file: Option<PathBuf>,
    #[arg(long, env)]
    #[serde(skip)]
    pub ecdsa_key_json: Option<String>,
    #[arg(long, env)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub ecdsa_ephemeral_key: bool,
}

#[derive(Args, Serialize, Debug)]
#[group(required = true, multiple = false)]
pub struct BlsKey {
    #[arg(long, env)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bls_key_file: Option<PathBuf>,
    #[arg(long, env)]
    #[serde(skip)]
    pub bls_key_json: Option<String>,
    #[arg(long, env)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub bls_ephemeral_key: bool,
}

#[derive(Debug, Subcommand, Serialize)]
pub enum Commands {
    OptInAvs,
    OptOutAvs,
    PrintStatus,
}

impl CliArgs {
    pub fn build() -> Self {
        let args = CliArgs::parse();
        if args.chain_id != Chain::AnvilHardhat as u64 {
            let mut cmd = CliArgs::command();
            if args.testnet {
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "testnet is only available with anvil testnet `--chain-id=31337`",
                )
                .exit();
            }
            if args.ecdsa_key.ecdsa_ephemeral_key || args.bls_key.bls_ephemeral_key {
                warn!("!!! Runing operator with epehemeral keys !!!")
            }
        }
        args
    }

    pub fn get_ecdsa_keystore(&self) -> eyre::Result<EncodedKeystore> {
        get_keystore(
            &self.ecdsa_key.ecdsa_key_file,
            &self.ecdsa_key.ecdsa_key_json,
            self.ecdsa_key.ecdsa_ephemeral_key,
            &self.ecdsa_key_password,
        )
    }
    pub fn get_bls_keystore(&self) -> eyre::Result<EncodedKeystore> {
        get_keystore(
            &self.bls_key.bls_key_file,
            &self.bls_key.bls_key_json,
            self.bls_key.bls_ephemeral_key,
            &self.bls_key_password,
        )
    }
}

fn get_keystore(
    path: &Option<PathBuf>,
    content: &Option<String>,
    is_random: bool,
    password: &Option<String>,
) -> eyre::Result<EncodedKeystore> {
    let keystore = match (path, content, is_random) {
        (_, _, true) => EncodedKeystore::random(),
        (Some(path), _, _) => EncodedKeystore::from_path(path, password.clone()),
        (_, Some(content), _) => EncodedKeystore::from_string(content.to_owned(), password.clone()),
        _ => panic!("one of the key args must be set"),
    }?;
    Ok(keystore)
}
