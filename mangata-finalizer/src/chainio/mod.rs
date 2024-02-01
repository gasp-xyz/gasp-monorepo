use ethers::{
    middleware::{NonceManagerMiddleware, SignerMiddleware},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use tracing::{info, instrument};

use crate::cli::CliArgs;

pub mod avs;
pub mod eigen;

type MW = Provider<Http>;
pub type Client = SignerMiddleware<NonceManagerMiddleware<MW>, LocalWallet>;

#[instrument(skip_all)]
pub(crate) async fn build_eth_client(cfg: &CliArgs) -> eyre::Result<Client> {
    let provider = MW::try_from(cfg.eth_rpc_url.clone())?;
    info!("Eth Wallet decryting...");
    let wallet = cfg.get_ecdsa_keystore()?.into_wallet()?;
    info!("Eth Wallet decrytped with address {:x}", wallet.address());
    let nonce = NonceManagerMiddleware::new(provider, wallet.address());
    let client = Client::new_with_provider_chain(nonce, wallet.with_chain_id(cfg.chain_id)).await?;

    Ok(client)
}
