use std::path::PathBuf;

use ethers::{
    middleware::{NonceManagerMiddleware, SignerMiddleware},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use tracing::{instrument, debug, info};

use crate::cli::CliArgs;

pub mod avs;
pub mod eigen;

type MW = Provider<Http>;
pub type Client = SignerMiddleware<NonceManagerMiddleware<MW>, LocalWallet>;

#[derive(Debug)]
pub struct Config {
    eth_rpc_url: String,
    keystore_path: PathBuf,
    keystore_password: String,
    chain_id: u64,
}

impl From<&CliArgs> for Config {
    fn from(args: &CliArgs) -> Self {
        Self {
            eth_rpc_url: args.eth_rpc_url.clone(),
            keystore_path: args.ecdsa_key_file.clone(),
            keystore_password: args.ecdsa_key_password.to_owned(),
            chain_id: args.chain_id,
        }
    }
}

#[instrument(skip_all)]
pub(crate) async fn build_client(cfg: &Config) -> eyre::Result<Client> {
    let provider = MW::try_from(cfg.eth_rpc_url.clone())?;
    info!("Decrypting wallet at {:?}", cfg.keystore_path);
    let wallet =
        LocalWallet::decrypt_keystore(cfg.keystore_path.clone(), cfg.keystore_password.clone())?;
    debug!("Eth Wallet decrytped");
    let nonce = NonceManagerMiddleware::new(provider, wallet.address());
    let client =
        Client::new_with_provider_chain(nonce, wallet.with_chain_id(cfg.chain_id)).await?;

    Ok(client)
}
