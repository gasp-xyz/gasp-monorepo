// Copyright (C) 2021 Mangata team

use codec::Codec;
use jsonrpsee::{
	core::{async_trait, RpcResult},
	proc_macros::rpc,
	types::error::ErrorObject,
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::U256;
use sp_rpc::number::NumberOrHex;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay, MaybeFromStr};
use sp_std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use xyk_runtime_api::RpcAssetMetadata;
pub use xyk_runtime_api::XykRuntimeApi;

#[rpc(client, server)]
pub trait XykApi<BlockHash, Balance, TokenId, AccountId> {
	#[method(name = "xyk_calculate_sell_price")]
	fn calculate_sell_price(
		&self,
		input_reserve: NumberOrHex,
		output_reserve: NumberOrHex,
		sell_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_calculate_buy_price")]
	fn calculate_buy_price(
		&self,
		input_reserve: NumberOrHex,
		output_reserve: NumberOrHex,
		buy_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_calculate_sell_price_id")]
	fn calculate_sell_price_id(
		&self,
		sold_token_id: TokenId,
		bought_token_id: TokenId,
		sell_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_calculate_buy_price_id")]
	fn calculate_buy_price_id(
		&self,
		sold_token_id: TokenId,
		bought_token_id: TokenId,
		buy_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_get_burn_amount")]
	fn get_burn_amount(
		&self,
		first_asset_id: TokenId,
		second_asset_id: TokenId,
		liquidity_asset_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<(NumberOrHex, NumberOrHex)>;

	#[method(name = "xyk_get_max_instant_burn_amount")]
	fn get_max_instant_burn_amount(
		&self,
		user: AccountId,
		liquidity_asset_id: TokenId,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_get_max_instant_unreserve_amount")]
	fn get_max_instant_unreserve_amount(
		&self,
		user: AccountId,
		liquidity_asset_id: TokenId,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_calculate_rewards_amount")]
	fn calculate_rewards_amount(
		&self,
		user: AccountId,
		liquidity_asset_id: TokenId,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_calculate_balanced_sell_amount")]
	fn calculate_balanced_sell_amount(
		&self,
		total_amount: NumberOrHex,
		reserve_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<NumberOrHex>;

	#[method(name = "xyk_get_liq_tokens_for_trading")]
	fn get_liq_tokens_for_trading(&self, at: Option<BlockHash>) -> RpcResult<Vec<TokenId>>;

	#[method(name = "xyk_is_buy_asset_lock_free")]
	fn is_buy_asset_lock_free(
		&self,
		path: sp_std::vec::Vec<TokenId>,
		input_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<Option<bool>>;

	#[method(name = "xyk_is_sell_asset_lock_free")]
	fn is_sell_asset_lock_free(
		&self,
		path: sp_std::vec::Vec<TokenId>,
		input_amount: NumberOrHex,
		at: Option<BlockHash>,
	) -> RpcResult<Option<bool>>;

	#[method(name = "xyk_get_tradeable_tokens")]
	fn get_tradeable_tokens(
		&self,
		at: Option<BlockHash>,
	) -> RpcResult<sp_std::vec::Vec<RpcAssetMetadata<TokenId>>>;
}

pub struct Xyk<C, M> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, P> Xyk<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

trait TryIntoBalance<Balance> {
	fn try_into_balance(self) -> RpcResult<Balance>;
}

impl<T: TryFrom<U256>> TryIntoBalance<T> for NumberOrHex {
	fn try_into_balance(self) -> RpcResult<T> {
		self.into_u256().try_into().or(Err(ErrorObject::owned(
			1,
			"Unable to serve the request",
			Some(String::from("input parameter doesnt fit into u128")),
		)))
	}
}

#[async_trait]
impl<C, Block, Balance, TokenId, AccountId>
	XykApiServer<<Block as BlockT>::Hash, Balance, TokenId, AccountId> for Xyk<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: XykRuntimeApi<Block, Balance, TokenId, AccountId>,
	Balance: Codec + MaybeDisplay + MaybeFromStr + TryFrom<U256> + Into<NumberOrHex>,
	TokenId: Codec + MaybeDisplay + MaybeFromStr,
	AccountId: Codec + MaybeDisplay + MaybeFromStr,
{
	fn calculate_sell_price(
		&self,
		input_reserve: NumberOrHex,
		output_reserve: NumberOrHex,
		sell_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.calculate_sell_price(
			at,
			input_reserve.try_into_balance()?,
			output_reserve.try_into_balance()?,
			sell_amount.try_into_balance()?,
		)
		.map(Into::<NumberOrHex>::into)
		.map_err(|e| ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e))))
	}

	fn calculate_buy_price(
		&self,
		input_reserve: NumberOrHex,
		output_reserve: NumberOrHex,
		buy_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.calculate_buy_price(
			at,
			input_reserve.try_into_balance()?,
			output_reserve.try_into_balance()?,
			buy_amount.try_into_balance()?,
		)
		.map(Into::<NumberOrHex>::into)
		.map_err(|e| ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e))))
	}

	fn calculate_sell_price_id(
		&self,
		sold_token_id: TokenId,
		bought_token_id: TokenId,
		sell_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.calculate_sell_price_id(
			at,
			sold_token_id,
			bought_token_id,
			sell_amount.try_into_balance()?,
		)
		.map(Into::<NumberOrHex>::into)
		.map_err(|e| ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e))))
	}

	fn calculate_buy_price_id(
		&self,
		sold_token_id: TokenId,
		bought_token_id: TokenId,
		buy_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.calculate_buy_price_id(
			at,
			sold_token_id,
			bought_token_id,
			buy_amount.try_into_balance()?,
		)
		.map(Into::<NumberOrHex>::into)
		.map_err(|e| ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e))))
	}

	fn get_burn_amount(
		&self,
		first_asset_id: TokenId,
		second_asset_id: TokenId,
		liquidity_asset_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<(NumberOrHex, NumberOrHex)> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.get_burn_amount(
			at,
			first_asset_id,
			second_asset_id,
			liquidity_asset_amount.try_into_balance()?,
		)
		.map(|(val1, val2)| (val1.into(), val2.into()))
		.map_err(|e| ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e))))
	}

	fn get_max_instant_burn_amount(
		&self,
		user: AccountId,
		liquidity_asset_id: TokenId,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.get_max_instant_burn_amount(at, user, liquidity_asset_id)
			.map(Into::<NumberOrHex>::into)
			.map_err(|e| {
				ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e)))
			})
	}

	fn get_max_instant_unreserve_amount(
		&self,
		user: AccountId,
		liquidity_asset_id: TokenId,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.get_max_instant_unreserve_amount(at, user, liquidity_asset_id)
			.map(Into::<NumberOrHex>::into)
			.map_err(|e| {
				ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e)))
			})
	}

	fn calculate_rewards_amount(
		&self,
		user: AccountId,
		liquidity_asset_id: TokenId,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.calculate_rewards_amount(at, user, liquidity_asset_id)
			.map(Into::<NumberOrHex>::into)
			.map_err(|e| {
				ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e)))
			})
	}

	fn calculate_balanced_sell_amount(
		&self,
		total_amount: NumberOrHex,
		reserve_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NumberOrHex> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.calculate_balanced_sell_amount(
			at,
			total_amount.try_into_balance()?,
			reserve_amount.try_into_balance()?,
		)
		.map(Into::<NumberOrHex>::into)
		.map_err(|e| ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e))))
	}

	fn get_liq_tokens_for_trading(
		&self,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<TokenId>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.get_liq_tokens_for_trading(at).map_err(|e| {
			ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e)))
		})
	}

	fn is_buy_asset_lock_free(
		&self,
		path: sp_std::vec::Vec<TokenId>,
		input_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<bool>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.is_buy_asset_lock_free(at, path, input_amount.try_into_balance()?)
			.map_err(|e| {
				ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e)))
			})
	}

	fn is_sell_asset_lock_free(
		&self,
		path: sp_std::vec::Vec<TokenId>,
		input_amount: NumberOrHex,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<bool>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.is_sell_asset_lock_free(at, path, input_amount.try_into_balance()?)
			.map_err(|e| {
				ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e)))
			})
	}

	fn get_tradeable_tokens(
		&self,
		_at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<RpcAssetMetadata<TokenId>>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		api.get_tradeable_tokens(at).map_err(|e| {
			ErrorObject::owned(1, "Unable to serve the request", Some(format!("{:?}", e)))
		})
	}
}
