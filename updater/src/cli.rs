use clap::{error::ErrorKind, Args, CommandFactory, Parser};
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
    pub gasp_service_addr: Address,

    #[arg(long, env)]
    pub source_ws_url: String,
    #[arg(long, env)]
    pub target_ws_url: String,

    #[arg(long, env)]
    pub source_chain_id: u64,
    #[arg(long, env)]
    pub target_chain_id: u64,

    #[arg(long, env, default_value_t = 0)]
    pub source_avs_deployment_block: u64,

    #[arg(long, env)]
    pub target_chain_index: u8,

    #[command(flatten)]
    pub ecdsa_key: EcdsaKey,
    #[arg(long, env)]
    #[serde(skip)]
    pub ecdsa_key_password: Option<String>,

    #[arg(long, env, default_value_t = 4000)]
    pub filter_limit: u64,

    #[arg(long, env = "METRICS_PORT", default_value_t = 8080)]
    pub metrics_port: u16,

    #[arg(long, env, default_value_t = false)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub testnet: bool,

    #[arg(long, env, default_value_t = false, conflicts_with_all = &["sync_skips_first_op_task_completed_event"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub push_first_init: bool,

    #[arg(long, env, default_value_t = false, conflicts_with_all = &["push_first_init"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub sync_skips_first_op_task_completed_event: bool,

    /// To use this please set both the source and target chain
    /// to the eth chain. The gasp_service_addr can be ignored
    // This can be improved later on...
    #[arg(long, env, default_value_t = false, requires = "root", conflicts_with_all = &["reinit, only_reinit, push_first_init, reinit_eth_only_print_op_task_response, reinit_only_print, reinit_eth_only_print_op_task_creation"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub only_reinit_eth: bool,

    /// To use this you only need to set the source chain
    /// to the eth chain. The gasp_service_addr can be ignored
    // This can be improved later on...
    #[arg(long, env, default_value_t = false, conflicts_with_all = &["reinit, only_reinit, push_first_init, only_reinit_eth, reinit_only_print, reinit_eth_only_print_op_task_response"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub reinit_eth_only_print_op_task_creation: bool,

    /// To use this you only need to set the source chain
    /// to the eth chain. The gasp_service_addr can be ignored
    // This can be improved later on...
    #[arg(long, env, default_value_t = false, conflicts_with_all = &["reinit, only_reinit, push_first_init, only_reinit_eth, reinit_only_print, reinit_eth_only_print_op_task_creation"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub reinit_eth_only_print_op_task_response: bool,

    #[arg(long, env, default_value_t = false, requires = "root", conflicts_with_all = &["only_reinit, push_first_init, only_reinit_eth, reinit_eth_only_print_op_task_response, reinit_only_print, reinit_eth_only_print_op_task_creation"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub reinit: bool,

    #[arg(long, env, default_value_t = false, requires = "root", conflicts_with_all = &["reinit, push_first_init, only_reinit_eth, reinit_eth_only_print_op_task_response, reinit_only_print, reinit_eth_only_print_op_task_creation"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub only_reinit: bool,

    #[arg(long, env, default_value_t = false, conflicts_with_all = &["reinit, push_first_init, only_reinit, reinit_eth_only_print_op_task_response, reinit_eth_only_print_op_task_creation"])]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub reinit_only_print: bool,

    #[command(flatten)]
    pub root_ecdsa_key: RootEcdsaKey,
    #[arg(long, env)]
    #[serde(skip)]
    pub root_ecdsa_key_password: Option<String>,
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
#[group(id = "root", multiple = false)]
pub struct RootEcdsaKey {
    #[arg(long, env)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_ecdsa_key_file: Option<PathBuf>,
    #[arg(long, env)]
    #[serde(skip)]
    pub root_ecdsa_key_json: Option<String>,
    #[arg(long, env)]
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub root_ecdsa_ephemeral_key: bool,
}

impl CliArgs {
    pub fn build() -> Self {
        let args = CliArgs::parse();
        let testnet_chains = [
            Chain::AnvilHardhat as u64,
            31338,
            31339,
            31340,
            31341,
            Chain::Dev as u64,
        ];
        // Only if both source and target are testnet then it is testnet
        if !(testnet_chains.contains(&args.source_chain_id)
            && testnet_chains.contains(&args.target_chain_id))
        {
            let mut cmd = CliArgs::command();
            if args.testnet {
                cmd.error(
                    ErrorKind::ArgumentConflict,
                    "testnet is only available with anvil testnet `--chain-id=31337` and reth testnet `--chain-id=1337`",
                )
                .exit();
            }
            if args.ecdsa_key.ecdsa_ephemeral_key {
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

    pub fn get_root_ecdsa_keystore(&self) -> eyre::Result<EncodedKeystore> {
        get_keystore(
            &self.root_ecdsa_key.root_ecdsa_key_file,
            &self.root_ecdsa_key.root_ecdsa_key_json,
            self.root_ecdsa_key.root_ecdsa_ephemeral_key,
            &self.root_ecdsa_key_password,
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
