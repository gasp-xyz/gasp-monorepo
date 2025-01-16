use std::str::FromStr;
use std::sync::Arc;

use bindings::{
    delegation_manager::DelegationManager, erc20_mock::ERC20Mock, i_strategy::IStrategy,
    registry_coordinator::RegistryCoordinator, stake_registry::StakeRegistry,
    strategy_manager::StrategyManager,
};
use ethers::{
    contract::{ContractError, EthError},
    middleware::{Middleware, MiddlewareBuilder, NonceManagerMiddleware, SignerMiddleware},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, TransactionRequest},
    utils::parse_ether,
};
use eyre::{eyre, Ok};
use sp_core::hexdisplay::AsBytesRef;
use tracing::{debug, info, instrument};

use crate::cli::CliArgs;

pub mod avs;
pub mod eigen;

pub type Client = Provider<Http>;
pub type SignerClient = SignerMiddleware<NonceManagerMiddleware<Client>, LocalWallet>;

#[instrument(skip_all)]
pub(crate) async fn build_eth_client(
    cfg: &CliArgs,
) -> eyre::Result<(Address, Arc<Client>, Option<Arc<SignerClient>>)> {
    let provider: Provider<Http> = Client::try_from(cfg.eth_rpc_url.clone())?;
    match &cfg.ecdsa_key.ecdsa_address {
        Some(address) => Ok((Address::from_str(address)?, Arc::new(provider), None)),
        _ => {
            info!("Eth Wallet decryting...");
            let wallet = cfg.get_ecdsa_keystore()?.into_wallet()?;
            let address = wallet.address();
            info!("Eth Wallet decrytped with address {:x}", address);
            let nonce = NonceManagerMiddleware::new(provider.clone(), wallet.address());
            let client =
                SignerClient::new_with_provider_chain(nonce, wallet.with_chain_id(cfg.chain_id))
                    .await?;
            Ok((address, Arc::new(provider), Some(Arc::new(client))))
        }
    }
}

pub(crate) fn map_revert(e: ContractError<SignerClient>) -> eyre::Report {
    match e {
        ContractError::Revert(b) => eyre!(
            "Contract call reverted with message: {}",
            String::decode_with_selector(b.as_bytes_ref())
                .unwrap_or("cannot parse message".to_string())
        ),
        _ => eyre::Report::new(e),
    }
}

#[instrument(skip_all)]
pub(crate) async fn setup_deposits(
    eth_rpc_url: String,
    registry_address: Address,
    stake: u32,
    operator: LocalWallet,
    chain_id: u64,
) -> eyre::Result<()> {
    let op_address = operator.address();
    set_balance(chain_id, eth_rpc_url.clone(), op_address, 100).await?;
    debug!("set some ether to operator");

    let provider: Provider<Http> = Client::try_from(eth_rpc_url)?;
    let client = Arc::new(provider.with_signer(operator));
    let registry = RegistryCoordinator::new(registry_address, client.clone());
    let stake_registry_address = registry.stake_registry().await?;
    let stake_reg = StakeRegistry::new(stake_registry_address, client.clone());
    let delegation_address = stake_reg.delegation().await?;
    let delegation = DelegationManager::new(delegation_address, client.clone());
    let strategy_manager_address = delegation.strategy_manager().await?;
    let strategy_params = stake_reg.strategy_params_by_index(0, 0.into()).await?;
    let strategy_manager = StrategyManager::new(strategy_manager_address, client.clone());
    debug!("startegy address {:?}", strategy_params);
    let strategy = IStrategy::new(strategy_params.strategy, client.clone());
    let erc20_address = strategy.underlying_token().call().await?;
    debug!("erc address {:?}", erc20_address);

    let erc20 = ERC20Mock::new(erc20_address, client.clone());
    erc20.mint(op_address, stake.into()).send().await?.await?;
    debug!("sent some erc20 to operator");
    erc20
        .approve(strategy_manager_address, stake.into())
        .send()
        .await?
        .await?;
    debug!("approve startegy manager for erc20 for operator");
    strategy_manager
        .deposit_into_strategy(strategy_params.strategy, erc20_address, stake.into())
        .send()
        .await?
        .await?;
    debug!("deposited into startegy manager for erc20 for operator");
    Ok(())
}

async fn set_balance(
    chain_id: u64,
    eth_rpc_url: String,
    address: Address,
    ether: u128,
) -> eyre::Result<()> {
    // 0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f
    let dev_wallet = "dbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97"
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id);
    let provider: Provider<Http> = Client::try_from(eth_rpc_url)?;
    let client = provider.with_signer(dev_wallet);
    let tx = TransactionRequest::pay(address, parse_ether(ether).unwrap());
    let _ = client.send_transaction(tx, None).await?.await?;
    Ok(())
}
