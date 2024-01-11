use std::path::PathBuf;

use eyre::WrapErr;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeConfig {
    // used to set the logger level (true = info, false = debug)
    pub production: bool,
    pub operator_address: String,
    pub el_slasher_address: String,
    pub bls_operator_state_retriever_address: String,
    pub bls_public_key_compendium_address: String,
    pub avs_service_manager_address: String,
    pub eth_rpc_url: String,
    pub eth_ws_url: String,
    pub bls_private_key_store_path: String,
    pub ecdsa_private_key_store_path: String,
    pub aggregator_server_ip_port_address: String,
    pub register_operator_on_startup: bool,
    pub eigen_metrics_ip_port_address: String,
    pub enable_metrics: bool,
    pub node_api_ip_port_address: String,
    pub enable_node_api: bool,
    pub substrate_rpc_url: String,
}

impl NodeConfig {
    pub fn from_path(path: &PathBuf) -> eyre::Result<NodeConfig> {
        let f = std::fs::File::open(path)
            .wrap_err_with(|| format!("Failed to read config from {}", path.display()))?;
        let config = serde_yaml::from_reader(f)?;
        Ok(config)
    }
}

#[test]
fn test_parse_config() {
    let config = NodeConfig::new("../../config-files/operator.anvil.yml").unwrap();
}
