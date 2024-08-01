use std::sync::Arc;

use bindings::{
    delegation_manager::DelegationManager, erc20_mock::ERC20Mock, i_strategy::IStrategy,
    registry_coordinator::RegistryCoordinator, stake_registry::StakeRegistry,
    strategy_manager::StrategyManager,
};
use ethers::{
    contract::{ContractError, EthError},
    middleware::{MiddlewareBuilder, NonceManagerMiddleware, SignerMiddleware},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
    utils::parse_ether,
};
use eyre::eyre;
use tracing::{debug, info, instrument};

use crate::cli::CliArgs;

pub mod avs;

type MW = Provider<Http>;
pub type SourceClient = MW;
pub type TargetClient = SignerMiddleware<NonceManagerMiddleware<MW>, LocalWallet>;

#[instrument(skip_all)]
pub(crate) async fn build_clients(
    cfg: &CliArgs,
) -> eyre::Result<(SourceClient, TargetClient, Option<TargetClient>)> {
    let source_client: Provider<Http> = MW::try_from(cfg.source_rpc_url.clone())?;

    let provider: Provider<Http> = MW::try_from(cfg.target_rpc_url.clone())?;
    info!("Eth Wallet decryting...");
    let wallet = cfg.get_ecdsa_keystore()?.into_wallet()?;
    info!("Eth Wallet decrytped with address {:x}", wallet.address());
    let nonce = NonceManagerMiddleware::new(provider, wallet.address());
    let target_client =
        TargetClient::new_with_provider_chain(nonce, wallet.with_chain_id(cfg.target_chain_id))
            .await?;

    let root_target_client = if cfg.reinit || cfg.only_reinit {
        let root_provider: Provider<Http> = MW::try_from(cfg.target_rpc_url.clone())?;
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
