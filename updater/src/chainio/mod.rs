use ethers::{
    middleware::{NonceManagerMiddleware, SignerMiddleware},
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
};
use tracing::{info, instrument};

use crate::cli::CliArgs;

pub mod avs;
pub mod decode;

type MW = Provider<Ws>;
pub type SourceClient = MW;
pub type TargetClient = SignerMiddleware<NonceManagerMiddleware<MW>, LocalWallet>;

#[instrument(skip_all)]
pub(crate) async fn build_clients(
    cfg: &CliArgs,
) -> eyre::Result<(SourceClient, TargetClient, Option<TargetClient>)> {
    let source_client: Provider<Ws> = MW::connect(cfg.source_ws_url.clone()).await?;

    let provider: Provider<Ws> = MW::connect(cfg.target_ws_url.clone()).await?;
    info!("Eth Wallet decryting...");
    let wallet = cfg.get_ecdsa_keystore()?.into_wallet()?;
    info!("Eth Wallet decrytped with address {:x}", wallet.address());
    let nonce = NonceManagerMiddleware::new(provider, wallet.address());
    let target_client =
        TargetClient::new_with_provider_chain(nonce, wallet.with_chain_id(cfg.target_chain_id))
            .await?;

    let root_target_client = if cfg.reinit || cfg.only_reinit || cfg.only_reinit_eth {
        let root_provider: Provider<Ws> = MW::connect(cfg.target_ws_url.clone()).await?;
        info!("Eth Wallet decryting...");
        let root_wallet = cfg.get_root_ecdsa_keystore()?.into_wallet()?;
        info!(
            "Eth Wallet decrytped with address {:x}",
            root_wallet.address()
        );
        let root_nonce = NonceManagerMiddleware::new(root_provider, root_wallet.address());
        Some(
            TargetClient::new_with_provider_chain(
                root_nonce,
                root_wallet.with_chain_id(cfg.target_chain_id),
            )
            .await?,
        )
    } else {
        None
    };

    Ok((source_client, target_client, root_target_client))
}
