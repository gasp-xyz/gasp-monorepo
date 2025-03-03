//! # Market Pallet.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{
		tokens::{
			currency::{MultiTokenCurrency, MultiTokenVestingLocks},
			Balance, CurrencyId,
		},
		Contains, ExistenceRequirement, Get, WithdrawReasons,
	},
};
use frame_system::pallet_prelude::*;
use mangata_support::{
	pools::{ComputeBalances, Inspect, Mutate, PoolPair, SwapResult, TreasuryBurn, Valuate},
	traits::{
		AssetRegistryProviderTrait, FeeLockTriggerTrait, GetMaintenanceStatusTrait,
		ProofOfStakeRewardsApi, Valuate as ValuateXyk, XykFunctionsTrait,
	},
	utils::ConvertError,
};
use mangata_types::multipurpose_liquidity::ActivateKind;

#[cfg(feature = "runtime-benchmarks")]
use mangata_support::traits::ComputeIssuance;

use sp_arithmetic::{helpers_128bit::multiply_by_rational_with_rounding, per_things::Rounding};
use sp_runtime::{
	traits::{CheckedSub, MaybeConvert, MaybeDisplay, MaybeFromStr, Saturating, Zero},
	ModuleError,
};
use sp_std::{
	convert::{TryFrom, TryInto},
	fmt::Debug,
	vec,
	vec::Vec,
};

use orml_tokens::MultiTokenCurrencyExtended;
use orml_traits::asset_registry::Inspect as AssetRegistryInspect;

pub mod weights;
pub use crate::weights::WeightInfo;

pub(crate) const LOG_TARGET: &str = "market";
// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: $crate::LOG_TARGET,
			concat!("[{:?}] 💸 ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, TypeInfo)]
pub enum PoolKind {
	/// Classic XYK invariant
	Xyk,
	/// StableSwap
	StableSwap,
}

#[derive(Clone, Debug)]
pub struct PoolInfo<CurrencyId> {
	pub pool_id: CurrencyId,
	pub kind: PoolKind,
	pub pool: PoolPair<CurrencyId>,
}

impl<C: PartialEq + Copy> PoolInfo<C> {
	fn same_and_other(&self, same: C) -> Option<(C, C)> {
		if same == self.pool.0 {
			Some(self.pool)
		} else if same == self.pool.1 {
			Some((self.pool.1, self.pool.0))
		} else {
			None
		}
	}
}

#[derive(Encode, Decode, Eq, PartialEq, Debug, Clone, TypeInfo)]
pub struct AtomicSwap<CurrencyId, Balance> {
	pub pool_id: CurrencyId,
	pub kind: PoolKind,
	pub asset_in: CurrencyId,
	pub asset_out: CurrencyId,
	pub amount_in: Balance,
	pub amount_out: Balance,
}

// use LP token as pool id, extra type for readability
pub type PoolIdOf<T> = <T as Config>::CurrencyId;
// pools are composed of a pair of assets
pub type PoolInfoOf<T> = PoolInfo<<T as Config>::CurrencyId>;
pub type AssetPairOf<T> = (<T as Config>::CurrencyId, <T as Config>::CurrencyId);
pub type BalancePairOf<T> = (<T as Config>::Balance, <T as Config>::Balance);
pub type AtomicSwapOf<T> = AtomicSwap<<T as Config>::CurrencyId, <T as Config>::Balance>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[cfg(feature = "runtime-benchmarks")]
	pub trait MarketBenchmarkingConfig: pallet_fee_lock::Config {}

	#[cfg(not(feature = "runtime-benchmarks"))]
	pub trait MarketBenchmarkingConfig {}

	#[pallet::config]
	pub trait Config: frame_system::Config + MarketBenchmarkingConfig {
		/// Overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Currency type that this works on.
		type Currency: MultiTokenCurrencyExtended<
			Self::AccountId,
			Balance = Self::Balance,
			CurrencyId = Self::CurrencyId,
		>;

		/// The `Currency::Balance` type of the currency.
		type Balance: Balance + Into<u128> + TryFrom<u128>;

		/// Identifier for the assets.
		type CurrencyId: CurrencyId;

		/// Native currency
		type NativeCurrencyId: Get<Self::CurrencyId>;

		/// Max swap list length
		type MaxSwapListLength: Get<u32>;

		/// Xyk pools
		type Xyk: XykFunctionsTrait<Self::AccountId, Self::Balance, Self::CurrencyId>
			+ Inspect<CurrencyId = Self::CurrencyId, Balance = Self::Balance>
			+ ValuateXyk<Self::Balance, Self::CurrencyId>
			+ TreasuryBurn
			+ ComputeBalances;

		/// StableSwap pools
		type StableSwap: Mutate<Self::AccountId, CurrencyId = Self::CurrencyId, Balance = Self::Balance>
			+ ComputeBalances;

		/// Reward apis for native asset LP tokens activation
		type Rewards: ProofOfStakeRewardsApi<Self::AccountId, Self::Balance, Self::CurrencyId>;

		/// Vesting apis for providing native vested liquidity
		type Vesting: MultiTokenVestingLocks<
			Self::AccountId,
			Moment = BlockNumberFor<Self>,
			Currency = Self::Currency,
		>;

		/// Apis for LP asset creation in asset registry
		type AssetRegistry: AssetRegistryProviderTrait<Self::CurrencyId>;

		/// List of tokens ids that are not allowed to be used at all
		type DisabledTokens: Contains<Self::CurrencyId>;

		/// List of assets that are not allowed to form a pool
		type DisallowedPools: Contains<AssetPairOf<Self>>;

		/// Disable trading with maintenance mode
		type MaintenanceStatusProvider: GetMaintenanceStatusTrait;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// Tokens which cannot be transfered by extrinsics/user or use in pool, unless foundation override
		type NontransferableTokens: Contains<Self::CurrencyId>;

		/// A list of Foundation members with elevated rights
		type FoundationAccountsProvider: Get<Vec<Self::AccountId>>;

		/// A special account used for nontransferable tokens to allow 'selling' to balance pools
		type ArbitrageBot: Contains<Self::AccountId>;

		type PoolFeePercentage: Get<u128>;
		type TreasuryFeePercentage: Get<u128>;
		type BuyAndBurnFeePercentage: Get<u128>;
		type FeeDenominator: Get<u128>;

		type TreasuryAccountId: Get<Self::AccountId>;
		type BnbAccountId: Get<Self::AccountId>;

		type FeeLock: FeeLockTriggerTrait<Self::AccountId, Self::Balance, Self::CurrencyId>;

		#[cfg(feature = "runtime-benchmarks")]
		type ComputeIssuance: ComputeIssuance;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// No such pool exists
		NoSuchPool,
		/// Asset id is not allowed
		FunctionNotAvailableForThisToken,
		/// Asset ids are not allowed to create a pool
		DisallowedPool,
		/// Insufficient output amount does not meet min requirements
		InsufficientOutputAmount,
		/// Excesive input amount does not meet max requirements
		ExcesiveInputAmount,
		/// Pool is not paired with native currency id
		NotPairedWithNativeAsset,
		/// Not a promoted pool
		NotAPromotedPool,
		/// Asset does not exists
		AssetDoesNotExists,
		/// Operation not available for such pool type
		FunctionNotAvailableForThisPoolKind,
		/// Trading blocked by maintenance mode
		TradingBlockedByMaintenanceMode,
		/// Multi swap path contains repetive pools
		MultiSwapSamePool,
		/// Input asset id is not connected with output asset id for given pools
		MultiSwapPathInvalid,
		/// Asset cannot be used to create or modify a pool
		NontransferableToken,
		/// Math Overflow
		MathOverflow { id: u8 },
		/// Unexpected failure
		UnexpectedFailure { id: u8 },
		/// Swap prevalidation
		SwapPrevalidation,
		/// Not enough assets for fees,
		NotEnoughAssetsForFees,
		/// Not enough assets for fee lock
		NotEnoughAssetsForFeeLock,
	}

	// Pallet's events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Assets were swapped successfully
		AssetsSwapped {
			/// The account that initiated the swap
			who: T::AccountId,
			/// List of the atomic asset swaps
			swaps: Vec<AtomicSwapOf<T>>,
		},

		/// A successful call of the `CretaPool` extrinsic will create this event.
		PoolCreated {
			/// The account that created the pool.
			creator: T::AccountId,
			/// The pool id and the account ID of the pool.
			pool_id: PoolIdOf<T>,
			/// The id of the liquidity tokens that will be minted when assets are added to this
			/// pool.
			lp_token: T::CurrencyId,
			/// The asset ids associated with the pool. Note that the order of the assets may not be
			/// the same as the order specified in the create pool extrinsic.
			assets: AssetPairOf<T>,
		},

		/// A successful call of the `AddLiquidity` extrinsic will create this event.
		LiquidityMinted {
			/// The account that the liquidity was taken from.
			who: T::AccountId,
			/// The id of the pool that the liquidity was added to.
			pool_id: PoolIdOf<T>,
			/// The amounts of the assets that were added to the pool.
			amounts_provided: BalancePairOf<T>,
			/// The id of the LP token that was minted.
			lp_token: T::CurrencyId,
			/// The amount of lp tokens that were minted of that id.
			lp_token_minted: T::Balance,
			/// The new total supply of the associated LP token.
			total_supply: T::Balance,
		},

		/// A successful call of the `RemoveLiquidity` extrinsic will create this event.
		LiquidityBurned {
			/// The account that the liquidity token was taken from.
			who: T::AccountId,
			/// The id of the pool that the liquidity was taken from.
			pool_id: PoolIdOf<T>,
			/// The amount of the asset that was received.
			amounts: BalancePairOf<T>,
			/// The amount of the associated LP token that was burned.
			burned_amount: T::Balance,
			/// The new total supply of the associated LP token.
			total_supply: T::Balance,
		},

		/// Swap failed with error
		SwapFailed { error: ModuleError },

		/// Swap fees falback failed
		SwapFeesFallbackFailed { id: u32, error: ModuleError },
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn integrity_test() {
			assert!(true, "template",);
		}
	}

	/// Pallet's callable functions.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a liquidity pool and an associated new `lp_token` asset
		/// For a StableSwap pool, the "stable" rate is computed from the ratio of input amounts, max rate is 1e18:1
		#[pallet::call_index(0)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::create_pool_xyk().max(
			<T as pallet::Config>::WeightInfo::create_pool_sswap()
		))]
		pub fn create_pool(
			origin: OriginFor<T>,
			kind: PoolKind,
			first_asset_id: T::CurrencyId,
			first_asset_amount: T::Balance,
			second_asset_id: T::CurrencyId,
			second_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check assets id, or the foundation has a veto
			ensure!(
				(!T::NontransferableTokens::contains(&first_asset_id) &&
					!T::NontransferableTokens::contains(&second_asset_id)) ||
					T::FoundationAccountsProvider::get().contains(&sender) ||
					T::ArbitrageBot::contains(&sender),
				Error::<T>::NontransferableToken
			);

			Self::check_assets_allowed((first_asset_id, second_asset_id))?;

			ensure!(
				!T::DisallowedPools::contains(&(first_asset_id, second_asset_id)),
				Error::<T>::DisallowedPool,
			);

			let lp_token = match kind {
				PoolKind::Xyk => {
					let lp_token = <T as pallet::Config>::Xyk::create_pool(
						sender.clone(),
						first_asset_id,
						first_asset_amount,
						second_asset_id,
						second_asset_amount,
					)?;
					lp_token
				},
				PoolKind::StableSwap => {
					let lp_token = T::StableSwap::create_pool(
						&sender,
						first_asset_id,
						first_asset_amount,
						second_asset_id,
						second_asset_amount,
					)?;

					T::AssetRegistry::create_pool_asset(lp_token, first_asset_id, second_asset_id)?;
					lp_token
				},
			};

			Self::deposit_event(Event::PoolCreated {
				creator: sender.clone(),
				pool_id: lp_token,
				lp_token,
				assets: (first_asset_id, second_asset_id),
			});

			let lp_supply = T::Currency::total_issuance(lp_token);
			Self::deposit_event(Event::LiquidityMinted {
				who: sender,
				pool_id: lp_token,
				amounts_provided: (first_asset_amount, second_asset_amount),
				lp_token,
				lp_token_minted: lp_supply,
				total_supply: lp_supply,
			});

			Ok(())
		}

		/// Provide liquidity into the pool of `pool_id`, suitable for Xyk pools.
		/// An optimal amount of the other asset will be calculated on current rate,
		/// a maximum amount should be provided to limit possible rate slippage.
		/// For a StableSwap pool a rate of 1:1 is used.
		/// Liquidity tokens that represent this share of the pool will be sent to origin.
		#[pallet::call_index(1)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::mint_liquidity_xyk().max(
			<T as pallet::Config>::WeightInfo::mint_liquidity_sswap()
		))]
		pub fn mint_liquidity(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			asset_id: T::CurrencyId,
			asset_amount: T::Balance,
			max_other_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			// check assets id, foundation has no veto
			Self::check_assets_allowed(pool_info.pool)?;

			let (lp_amount, other_asset_amount) = Self::do_mint_liquidity(
				&sender,
				pool_info,
				asset_id,
				asset_amount,
				max_other_asset_amount,
				true,
			)?;

			let lp_supply = T::Currency::total_issuance(pool_id);
			Self::deposit_event(Event::LiquidityMinted {
				who: sender,
				pool_id,
				amounts_provided: (asset_amount, other_asset_amount),
				lp_token: pool_id,
				lp_token_minted: lp_amount,
				total_supply: lp_supply,
			});

			Ok(())
		}

		/// Provide fixed liquidity into the pool of `pool_id`, suitable for StableSwap pools.
		/// For Xyk pools, if a single amount is defined, it will swap internally to to match current rate,
		/// setting both values results in error.
		/// Liquidity tokens that represent this share of the pool will be sent to origin.
		#[pallet::call_index(2)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::mint_liquidity_fixed_amounts_xyk().max(
			<T as pallet::Config>::WeightInfo::mint_liquidity_fixed_amounts_sswap()
		))]
		pub fn mint_liquidity_fixed_amounts(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			amounts: (T::Balance, T::Balance),
			min_amount_lp_tokens: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			// check assets id, foundation has no veto
			ensure!(
				!T::NontransferableTokens::contains(&pool_info.pool.0) &&
					!T::NontransferableTokens::contains(&pool_info.pool.1),
				Error::<T>::NontransferableToken
			);
			Self::check_assets_allowed(pool_info.pool)?;

			let lp_amount = match pool_info.kind {
				PoolKind::Xyk => {
					ensure!(
						amounts.0 == Zero::zero() || amounts.1 == Zero::zero(),
						Error::<T>::FunctionNotAvailableForThisPoolKind
					);

					let (id, amount) = if amounts.1 == Zero::zero() {
						(pool_info.pool.0, amounts.0)
					} else {
						(pool_info.pool.1, amounts.1)
					};

					let (_, lp_amount) =
						<T as pallet::Config>::Xyk::provide_liquidity_with_conversion(
							sender.clone(),
							pool_info.pool.0,
							pool_info.pool.1,
							id,
							amount,
							true,
						)?;
					ensure!(lp_amount > min_amount_lp_tokens, Error::<T>::InsufficientOutputAmount);
					lp_amount
				},
				PoolKind::StableSwap => {
					let amount = T::StableSwap::add_liquidity(
						&sender,
						pool_id,
						amounts,
						min_amount_lp_tokens,
					)?;
					if T::Rewards::native_rewards_enabled(pool_info.pool_id) {
						T::Rewards::activate_liquidity(
							sender.clone(),
							pool_id,
							amount,
							Some(ActivateKind::AvailableBalance),
						)?;
					}
					amount
				},
			};

			let lp_supply = T::Currency::total_issuance(pool_id);
			Self::deposit_event(Event::LiquidityMinted {
				who: sender,
				pool_id,
				amounts_provided: amounts,
				lp_token: pool_id,
				lp_token_minted: lp_amount,
				total_supply: lp_supply,
			});

			Ok(())
		}

		/// Provides liquidity from vested native asset. Tokens are added to pool and
		/// minted LP tokens are then vested instead.
		/// Only pools paired with native asset are allowed.
		#[pallet::call_index(3)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::mint_liquidity_using_vesting_native_tokens_by_vesting_index_xyk().max(
			<T as pallet::Config>::WeightInfo::mint_liquidity_using_vesting_native_tokens_by_vesting_index_sswap()
		))]
		pub fn mint_liquidity_using_vesting_native_tokens_by_vesting_index(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			native_asset_vesting_index: u32,
			vesting_native_asset_unlock_some_amount_or_all: Option<T::Balance>,
			max_other_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			// check assets id, foundation has no veto
			ensure!(
				!T::NontransferableTokens::contains(&pool_info.pool.0) &&
					!T::NontransferableTokens::contains(&pool_info.pool.1),
				Error::<T>::NontransferableToken
			);
			Self::check_assets_allowed(pool_info.pool)?;

			let native_id = T::NativeCurrencyId::get();
			ensure!(
				native_id == pool_info.pool.0 || native_id == pool_info.pool.1,
				Error::<T>::NotPairedWithNativeAsset
			);

			ensure!(T::Rewards::native_rewards_enabled(pool_id), Error::<T>::NotAPromotedPool);

			let (unlocked_amount, vesting_starting_block, vesting_ending_block_as_balance) =
				T::Vesting::unlock_tokens_by_vesting_index(
					&sender,
					native_id,
					native_asset_vesting_index,
					vesting_native_asset_unlock_some_amount_or_all,
				)?;

			let (lp_amount, other_asset_amount) = Self::do_mint_liquidity(
				&sender,
				pool_info,
				native_id,
				unlocked_amount,
				max_other_asset_amount,
				false,
			)?;

			T::Vesting::lock_tokens(
				&sender,
				pool_id,
				lp_amount,
				Some(vesting_starting_block),
				vesting_ending_block_as_balance,
			)?;

			let lp_supply = T::Currency::total_issuance(pool_id);
			Self::deposit_event(Event::LiquidityMinted {
				who: sender,
				pool_id,
				amounts_provided: (unlocked_amount, other_asset_amount),
				lp_token: pool_id,
				lp_token_minted: lp_amount,
				total_supply: lp_supply,
			});

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::mint_liquidity_using_vesting_native_tokens_xyk().max(
			<T as pallet::Config>::WeightInfo::mint_liquidity_using_vesting_native_tokens_sswap()
		))]
		pub fn mint_liquidity_using_vesting_native_tokens(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			native_asset_vesting_amount: T::Balance,
			max_other_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			// check assets id, foundation has no veto
			ensure!(
				!T::NontransferableTokens::contains(&pool_info.pool.0) &&
					!T::NontransferableTokens::contains(&pool_info.pool.1),
				Error::<T>::NontransferableToken
			);
			Self::check_assets_allowed(pool_info.pool)?;

			let native_id = T::NativeCurrencyId::get();
			ensure!(
				native_id == pool_info.pool.0 || native_id == pool_info.pool.1,
				Error::<T>::NotPairedWithNativeAsset
			);

			ensure!(T::Rewards::native_rewards_enabled(pool_id), Error::<T>::NotAPromotedPool);

			let (vesting_starting_block, vesting_ending_block_as_balance) =
				T::Vesting::unlock_tokens(&sender, native_id, native_asset_vesting_amount)?;

			let (lp_amount, other_asset_amount) = Self::do_mint_liquidity(
				&sender,
				pool_info,
				native_id,
				native_asset_vesting_amount,
				max_other_asset_amount,
				false,
			)?;

			T::Vesting::lock_tokens(
				&sender,
				pool_id,
				lp_amount,
				Some(vesting_starting_block),
				vesting_ending_block_as_balance,
			)?;

			let lp_supply = T::Currency::total_issuance(pool_id);
			Self::deposit_event(Event::LiquidityMinted {
				who: sender,
				pool_id,
				amounts_provided: (native_asset_vesting_amount, other_asset_amount),
				lp_token: pool_id,
				lp_token_minted: lp_amount,
				total_supply: lp_supply,
			});

			Ok(())
		}

		/// Allows you to remove liquidity by providing the `lp_burn_amount` tokens that will be
		/// burned in the process. The usage of `min_first_asset_amount`/`min_second_asset_amount`
		/// controls the min amount of returned tokens.
		#[pallet::call_index(5)]
		#[pallet::weight(
			<T as pallet::Config>::WeightInfo::burn_liquidity_xyk().max(
			<T as pallet::Config>::WeightInfo::burn_liquidity_sswap()
		))]
		pub fn burn_liquidity(
			origin: OriginFor<T>,
			pool_id: PoolIdOf<T>,
			liquidity_burn_amount: T::Balance,
			min_first_asset_amount: T::Balance,
			min_second_asset_amount: T::Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let pool_info = Self::get_pool_info(pool_id)?;
			// check assets id, or the foundation has a veto
			ensure!(
				(!T::NontransferableTokens::contains(&pool_info.pool.0) &&
					!T::NontransferableTokens::contains(&pool_info.pool.1)) ||
					T::FoundationAccountsProvider::get().contains(&sender),
				Error::<T>::NontransferableToken
			);
			Self::check_assets_allowed(pool_info.pool)?;

			let amounts = match pool_info.kind {
				PoolKind::Xyk => {
					let amounts = <T as pallet::Config>::Xyk::burn_liquidity(
						sender.clone(),
						pool_info.pool.0,
						pool_info.pool.1,
						liquidity_burn_amount,
					)?;
					ensure!(
						amounts.0 >= min_first_asset_amount,
						Error::<T>::InsufficientOutputAmount
					);
					ensure!(
						amounts.1 >= min_second_asset_amount,
						Error::<T>::InsufficientOutputAmount
					);
					amounts
				},
				PoolKind::StableSwap => {
					// deactivate liquidity if low balance
					let balance = T::Currency::available_balance(pool_id, &sender);
					let deactivate = liquidity_burn_amount.saturating_sub(balance);
					// noop on zero amount
					T::Rewards::deactivate_liquidity(sender.clone(), pool_id, deactivate)?;

					let amounts = T::StableSwap::remove_liquidity(
						&sender,
						pool_id,
						liquidity_burn_amount,
						(min_first_asset_amount, min_second_asset_amount),
					)?;
					amounts
				},
			};

			let lp_supply = T::Currency::total_issuance(pool_id);
			Self::deposit_event(Event::LiquidityBurned {
				who: sender,
				pool_id,
				amounts,
				burned_amount: liquidity_burn_amount,
				total_supply: lp_supply,
			});

			Ok(())
		}

		/// Executes a multiswap asset in a series of swap asset atomic swaps.
		///
		/// Multiswaps must fee lock instead of paying transaction fees.
		/// For a single atomic swap, both `asset_amount_in` and `min_amount_out` are considered to allow free execution without locks.
		///
		/// # Args:
		/// - `swap_token_list` - This list of tokens is the route of the atomic swaps, starting with the asset sold and ends with the asset finally bought
		/// - `asset_id_in`: The id of the asset sold
		/// - `asset_amount_in`: The amount of the asset sold
		/// - `asset_id_out`: The id of the asset received
		/// - `min_amount_out` - The minimum amount of requested asset that must be bought in order to not fail on slippage, use RPC calls to calc expected value
		// This call is part of the fee lock mechanism, which allows free execution on success
		// in case of an error & no native asset to cover fees, a fixed % is subtracted from input swap asset to avoid DOS attacks
		// `OnChargeTransaction` impl should check whether the sender has funds to cover such fee
		// or consider transaction invalid
		#[pallet::call_index(6)]
		#[pallet::weight(
			(
				<T as pallet::Config>::WeightInfo::multiswap_asset_xyk(swap_pool_list.len() as u32).max(
				<T as pallet::Config>::WeightInfo::multiswap_asset_sswap(swap_pool_list.len() as u32)
			)
			.saturating_add(
				<T as pallet::Config>::WeightInfo::is_swap_tokens_lockless().saturating_mul((swap_pool_list.len() as u64).saturating_add(1))
			)
		))]
		pub fn multiswap_asset(
			origin: OriginFor<T>,
			swap_pool_list: Vec<PoolIdOf<T>>,
			asset_id_in: T::CurrencyId,
			asset_amount_in: T::Balance,
			asset_id_out: T::CurrencyId,
			min_amount_out: T::Balance,
		) -> DispatchResultWithPostInfo {
			let function_error_id = 0u8;

			let sender = ensure_signed(origin)?;

			// ensure maintenance mode
			ensure!(
				!T::MaintenanceStatusProvider::is_maintenance(),
				Error::<T>::TradingBlockedByMaintenanceMode
			);

			// At this point pre dispatch has already checked that
			// The user has asset_amount_in and that swap_pool_list.len() in not zero
			// and that the first swap_pool_list element pool has asset_id_in as one of its two assets
			// We do it here again anyway to ensure that this function is standalone
			// TODO
			// We should move this to a pre_validation function
			ensure!(!swap_pool_list.len().is_zero(), Error::<T>::SwapPrevalidation);
			ensure!(
				swap_pool_list.len() <= T::MaxSwapListLength::get() as usize,
				Error::<T>::SwapPrevalidation
			);
			let first_pool_info = Self::get_pool_info(swap_pool_list[0])
				.map_err(|_| Error::<T>::SwapPrevalidation)?;
			ensure!(
				first_pool_info.pool.0 == asset_id_in || first_pool_info.pool.1 == asset_id_in,
				Error::<T>::SwapPrevalidation
			);
			let fee_pool_id = swap_pool_list[0];
			T::Currency::ensure_can_withdraw(
				asset_id_in,
				&sender,
				asset_amount_in,
				// Last two args are ignored by orml_tokens
				WithdrawReasons::all(),
				Default::default(),
			)?;

			let mut is_lockless: Option<bool> = None;
			let mut fees: Option<T::Balance> = None;

			let swap_result: Result<(), DispatchError> =
				frame_support::storage::with_storage_layer(|| -> Result<(), DispatchError> {
					fees = Some(Self::calc_fees_pre(asset_amount_in)?);

					let amount_in = asset_amount_in
						.checked_sub(
							&fees.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
						)
						.ok_or(Error::<T>::MathOverflow { id: function_error_id })?;
					let (pools, path) =
						Self::get_valid_path(&swap_pool_list, asset_id_in, asset_id_out)?;

					if swap_pool_list.len() > 1 {
						if !T::FeeLock::is_whitelisted(asset_id_in) {
							is_lockless = Some(false);
						}
					}

					let mut id = asset_id_in;
					let mut amount_out = amount_in;

					is_lockless = match is_lockless {
						Some(b) => Some(b),
						None => T::FeeLock::is_swap_tokens_lockless(asset_id_in, amount_in)
							.then_some(true),
					};

					// calc output amounts for fee lock detemination
					for (pool, swap) in pools.iter().zip(path.iter()) {
						amount_out = Self::calculate_sell_price(pool.pool_id, id, amount_out)
							.ok_or(Error::<T>::ExcesiveInputAmount)?;
						id = if id == swap.0 { swap.1 } else { swap.0 };

						// Check does the swap output (token and amount)
						// qualify for lockless. Input already checked
						is_lockless = match is_lockless {
							Some(b) => Some(b),
							None =>
								T::FeeLock::is_swap_tokens_lockless(id, amount_out).then_some(true),
						};
					}

					// We counldn't find a reason to make it lockless so it will be fee_lock
					if is_lockless.is_none() {
						is_lockless = Some(false)
					};

					ensure!(amount_out >= min_amount_out, Error::<T>::InsufficientOutputAmount);

					let swaps =
						Self::do_swaps(&sender, pools, path.clone(), amount_in, min_amount_out)?;

					Self::deposit_event(Event::AssetsSwapped { who: sender.clone(), swaps });

					let (_pool_fees, trsy_amt, burn_amt) = Self::charge_fees(
						&sender,
						fee_pool_id,
						asset_id_in,
						fees.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
					)?;
					<T as pallet::Config>::Xyk::settle_treasury_and_burn(
						asset_id_in,
						burn_amt,
						trsy_amt,
					)?;
					// TODO - now
					// do_fee_lock should check that fee_lock_metadata is available (init)
					// also check get_fee_lock_amount
					Self::do_fee_lock(
						&sender,
						is_lockless
							.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
					)?;

					Ok(())
				});

			match swap_result {
				Err(e) => {
					Self::deposit_event(Event::SwapFailed {
						error: ConvertError::maybe_convert(e)
							.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
					});

					let r = frame_support::storage::with_storage_layer(
						|| -> Result<(), DispatchError> {
							// Just reassign fees here incase it is none
							fees = Some(Self::calc_fees_pre(asset_amount_in)?);
							let (_pool_fees, trsy_amt, burn_amt) = Self::charge_fees(
								&sender,
								fee_pool_id,
								asset_id_in,
								fees.ok_or(Error::<T>::UnexpectedFailure {
									id: function_error_id,
								})?,
							)?;
							<T as pallet::Config>::Xyk::settle_treasury_and_burn(
								asset_id_in,
								burn_amt,
								trsy_amt,
							)?;
							Ok(())
						},
					);
					// We handle r here so that we can hard fail the txn if there is a non-module error
					// BadOrigin errors caused by faulty code is possible and ideally we'd handle that with having
					// DispatchError inside the event but, unbounded variants in such as Other might cause breaking bloat
					match r {
						Ok(()) => {},
						Err(e) => Self::deposit_event(Event::SwapFeesFallbackFailed {
							id: 2u32,
							error: ConvertError::maybe_convert(e)
								.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
						}),
					}

					let r = frame_support::storage::with_storage_layer(
						|| -> Result<(), DispatchError> {
							// Till the point we checked we counldn't find a reason to make it lockless so it will be fee_lock
							if is_lockless.is_none() {
								is_lockless = Some(false)
							};

							// TODO - now
							// do_fee_lock should check that fee_lock_metadata is available (init)
							// also check get_fee_lock_amount
							Self::do_fee_lock(
								&sender,
								is_lockless.ok_or(Error::<T>::UnexpectedFailure {
									id: function_error_id,
								})?,
							)?;
							Ok(())
						},
					);
					// We handle r here so that we can hard fail the txn if there is a non-module error
					// BadOrigin errors caused by faulty code is possible and ideally we'd handle that with having
					// DispatchError inside the event but, unbounded variants in such as Other might cause breaking bloat
					match r {
						Ok(()) => {},
						Err(e) => Self::deposit_event(Event::SwapFeesFallbackFailed {
							id: 2u32,
							error: ConvertError::maybe_convert(e)
								.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
						}),
					}
				},
				Ok(()) => {},
			}

			Ok(().into())
		}

		/// Executes a multiswap asset in a series of swap asset atomic swaps.
		/// The precise output amount is provided instead.
		///
		/// Multiswaps must fee lock instead of paying transaction fees.
		/// For a single atomic swap, both `asset_amount_out` and `max_amount_in` are considered to allow free execution without locks.
		///
		/// # Args:
		/// - `swap_token_list` - This list of tokens is the route of the atomic swaps, starting with the asset sold and ends with the asset finally bought
		/// - `asset_id_out`: The id of the asset received
		/// - `asset_amount_out`: The amount of the asset received
		/// - `asset_id_in`: The id of the asset sold
		/// - `max_amount_in` - The maximum amount of sold asset in order to not fail on slippage, use RPC calls to calc expected value
		// This call is part of the fee lock mechanism, which allows free execution on success
		// in case of an error & no native asset to cover fees, a fixed % is subtracted from input swap asset to avoid DOS attacks
		// `OnChargeTransaction` impl should check whether the sender has funds to cover such fee
		// or consider transaction invalid
		#[pallet::call_index(7)]
		#[pallet::weight(
			(
				<T as pallet::Config>::WeightInfo::multiswap_asset_buy_xyk(swap_pool_list.len() as u32).max(
				<T as pallet::Config>::WeightInfo::multiswap_asset_buy_sswap(swap_pool_list.len() as u32)
			)
			.saturating_add(
				<T as pallet::Config>::WeightInfo::is_swap_tokens_lockless().saturating_mul((swap_pool_list.len() as u64).saturating_add(1))
			)
		))]
		pub fn multiswap_asset_buy(
			origin: OriginFor<T>,
			swap_pool_list: Vec<PoolIdOf<T>>,
			asset_id_out: T::CurrencyId,
			asset_amount_out: T::Balance,
			asset_id_in: T::CurrencyId,
			max_amount_in: T::Balance,
		) -> DispatchResultWithPostInfo {
			let function_error_id = 1u8;

			let sender = ensure_signed(origin)?;

			// ensure maintenance mode
			ensure!(
				!T::MaintenanceStatusProvider::is_maintenance(),
				Error::<T>::TradingBlockedByMaintenanceMode
			);

			// At this point pre dispatch has already checked that
			// The user has max_amount_in and that swap_pool_list.len() in not zero
			// and that the first swap_pool_list element pool has asset_id_in as one of its two assets
			// We do it here again anyway to ensure that this function is standalone
			// TODO
			// We should move this to a pre_validation function
			ensure!(!swap_pool_list.len().is_zero(), Error::<T>::SwapPrevalidation);
			ensure!(
				swap_pool_list.len() <= T::MaxSwapListLength::get() as usize,
				Error::<T>::SwapPrevalidation
			);
			let first_pool_info = Self::get_pool_info(swap_pool_list[0])
				.map_err(|_| Error::<T>::SwapPrevalidation)?;
			ensure!(
				first_pool_info.pool.0 == asset_id_in || first_pool_info.pool.1 == asset_id_in,
				Error::<T>::SwapPrevalidation
			);
			let fee_pool_id = swap_pool_list[0];
			T::Currency::ensure_can_withdraw(
				asset_id_in,
				&sender,
				max_amount_in,
				// Last two args are ignored by orml_tokens
				WithdrawReasons::all(),
				Default::default(),
			)?;

			let mut is_lockless: Option<bool> = None;
			let mut fees: Option<T::Balance> = None;

			let swap_result: Result<(), DispatchError> =
				frame_support::storage::with_storage_layer(|| -> Result<(), DispatchError> {
					let (pools, path) =
						Self::get_valid_path(&swap_pool_list, asset_id_in, asset_id_out)?;

					if swap_pool_list.len() > 1 {
						if !T::FeeLock::is_whitelisted(asset_id_in) {
							is_lockless = Some(false);
						}
					}

					let mut id = asset_id_out;
					let mut amount_in = asset_amount_out;

					is_lockless = match is_lockless {
						Some(b) => Some(b),
						None => T::FeeLock::is_swap_tokens_lockless(asset_id_out, asset_amount_out)
							.then_some(true),
					};

					for (pool, swap) in pools.iter().rev().zip(path.iter().rev()) {
						amount_in = Self::calculate_buy_price(pool.pool_id, id, amount_in)
							.ok_or(Error::<T>::ExcesiveInputAmount)?;
						id = if id == swap.0 { swap.1 } else { swap.0 };

						// Check does the swap input (token and amount)
						// qualify for lockless. output already checked
						is_lockless = match is_lockless {
							Some(b) => Some(b),
							None =>
								T::FeeLock::is_swap_tokens_lockless(id, amount_in).then_some(true),
						};
					}

					// We counldn't find a reason to make it lockless so it will be fee_lock
					if is_lockless.is_none() {
						is_lockless = Some(false)
					};

					// Since pre_dispatch checks the availability of the input tokens
					// We can be sure here that fees are available too (atleast max_amount_in)
					// (subject to the further below <= max_amount in check)
					// In the limiting case this calc_fees_post(amount_in) should be the same as calc_fees_pre(max_amount_in)
					fees = Some(Self::calc_fees_post(amount_in)?);
					ensure!(
						amount_in.saturating_add(
							fees.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?
						) <= max_amount_in,
						Error::<T>::ExcesiveInputAmount
					);
					let swaps =
						Self::do_swaps(&sender, pools, path.clone(), amount_in, asset_amount_out)?;

					Self::deposit_event(Event::AssetsSwapped { who: sender.clone(), swaps });

					let (_pool_fees, trsy_amt, burn_amt) = Self::charge_fees(
						&sender,
						fee_pool_id,
						asset_id_in,
						fees.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
					)?;
					<T as pallet::Config>::Xyk::settle_treasury_and_burn(
						asset_id_in,
						burn_amt,
						trsy_amt,
					)?;
					// TODO - now
					// do_fee_lock should check that fee_lock_metadata is available (init)
					// also check get_fee_lock_amount
					Self::do_fee_lock(
						&sender,
						is_lockless
							.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
					)?;

					Ok(())
				});

			match swap_result {
				Err(e) => {
					Self::deposit_event(Event::SwapFailed {
						error: ConvertError::maybe_convert(e)
							.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
					});

					let r = frame_support::storage::with_storage_layer(
						|| -> Result<(), DispatchError> {
							let f_v = Self::calc_fees_pre(max_amount_in)?;
							match fees {
								Some(f) => {
									// if fees is some then we have the final "amount_in"
									// and the calc_fees_post based on it
									// but also never wanna charge more than calc_fees_post with max_amount_in
									if f > f_v {
										fees = Some(f_v);
									}
								},
								None => {
									// fees is none, so either we don't have the final "amount_in"
									// or something went wrong with the calc_fees_post with amount_in
									// so we just use the max_amount_in
									fees = Some(f_v);
								},
							}
							let (_pool_fees, trsy_amt, burn_amt) = Self::charge_fees(
								&sender,
								fee_pool_id,
								asset_id_in,
								fees.ok_or(Error::<T>::UnexpectedFailure {
									id: function_error_id,
								})?,
							)?;
							<T as pallet::Config>::Xyk::settle_treasury_and_burn(
								asset_id_in,
								burn_amt,
								trsy_amt,
							)?;
							Ok(())
						},
					);
					// We handle r here so that we can hard fail the txn if there is a non-module error
					// BadOrigin errors caused by faulty code is possible and ideally we'd handle that with having
					// DispatchError inside the event but, unbounded variants in such as Other might cause breaking bloat
					match r {
						Ok(()) => {},
						Err(e) => Self::deposit_event(Event::SwapFeesFallbackFailed {
							id: 1u32,
							error: ConvertError::maybe_convert(e)
								.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
						}),
					}
					let r = frame_support::storage::with_storage_layer(
						|| -> Result<(), DispatchError> {
							// Till the point we checked we counldn't find a reason to make it lockless so it will be fee_lock
							if is_lockless.is_none() {
								is_lockless = Some(false)
							};

							// TODO - now
							// do_fee_lock should check that fee_lock_metadata is available (init)
							// also check get_fee_lock_amount
							Self::do_fee_lock(
								&sender,
								is_lockless.ok_or(Error::<T>::UnexpectedFailure {
									id: function_error_id,
								})?,
							)?;
							Ok(())
						},
					);
					// We handle r here so that we can hard fail the txn if there is a non-module error
					// BadOrigin errors caused by faulty code is possible and ideally we'd handle that with having
					// DispatchError inside the event but, unbounded variants in such as Other might cause breaking bloat
					match r {
						Ok(()) => {},
						Err(e) => Self::deposit_event(Event::SwapFeesFallbackFailed {
							id: 2u32,
							error: ConvertError::maybe_convert(e)
								.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?,
						}),
					}
				},
				Ok(()) => {},
			}

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn calc_fees_pre(amount: T::Balance) -> Result<T::Balance, DispatchError> {
			let function_error_id = 2u8;
			let total_fee_perc = T::PoolFeePercentage::get()
				.checked_add(T::TreasuryFeePercentage::get())
				.ok_or(Error::<T>::MathOverflow { id: function_error_id })?
				.checked_add(T::BuyAndBurnFeePercentage::get())
				.ok_or(Error::<T>::MathOverflow { id: function_error_id })?;
			let fee_denominator = T::FeeDenominator::get();

			let total_fees: T::Balance = multiply_by_rational_with_rounding(
				amount.into(),
				total_fee_perc,
				fee_denominator,
				Rounding::Down,
			)
			.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?
			.try_into()
			.map_err(|_| Error::<T>::MathOverflow { id: function_error_id })?;

			Ok(total_fees)
		}

		pub fn calc_fees_post(amount: T::Balance) -> Result<T::Balance, DispatchError> {
			let function_error_id = 3u8;
			let total_fee_perc = T::PoolFeePercentage::get()
				.checked_add(T::TreasuryFeePercentage::get())
				.ok_or(Error::<T>::MathOverflow { id: function_error_id })?
				.checked_add(T::BuyAndBurnFeePercentage::get())
				.ok_or(Error::<T>::MathOverflow { id: function_error_id })?;
			let fee_denominator = T::FeeDenominator::get();

			let total_fees: T::Balance = multiply_by_rational_with_rounding(
				amount.into(),
				total_fee_perc,
				fee_denominator
					.checked_sub(total_fee_perc)
					.ok_or(Error::<T>::MathOverflow { id: function_error_id })?,
				Rounding::Down,
			)
			.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?
			.try_into()
			.map_err(|_| Error::<T>::MathOverflow { id: function_error_id })?;

			Ok(total_fees)
		}

		fn charge_fees(
			account: &T::AccountId,
			pool_id: PoolIdOf<T>,
			asset_id: T::CurrencyId,
			amount: T::Balance,
		) -> Result<(T::Balance, T::Balance, T::Balance), DispatchError> {
			let function_error_id = 4u8;
			let pool = Self::get_pool_info(pool_id)?;
			// It is immportant that "asset_id" be in fact a part of the "pool"
			// This should be checked in the swap extrinsics pre_dispatch and also
			// in the pre_validation within the extrinsic itself
			// But since we want this function to be standalone we also do it here
			ensure!(
				pool.pool.0 == asset_id || pool.pool.1 == asset_id,
				Error::<T>::UnexpectedFailure { id: function_error_id }
			);

			let total_fee_perc = T::PoolFeePercentage::get()
				.checked_add(T::TreasuryFeePercentage::get())
				.ok_or(Error::<T>::MathOverflow { id: function_error_id })?
				.checked_add(T::BuyAndBurnFeePercentage::get())
				.ok_or(Error::<T>::MathOverflow { id: function_error_id })?;

			if total_fee_perc.is_zero() || amount.is_zero() {
				return Ok((Default::default(), Default::default(), Default::default()));
			}

			let mut pool_fee_amount: T::Balance = multiply_by_rational_with_rounding(
				amount.into(),
				T::PoolFeePercentage::get(),
				total_fee_perc,
				Rounding::Down,
			)
			.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?
			.try_into()
			.map_err(|_| Error::<T>::MathOverflow { id: function_error_id })?;

			let mut treasury_amount: T::Balance = multiply_by_rational_with_rounding(
				amount.into(),
				T::TreasuryFeePercentage::get(),
				total_fee_perc,
				Rounding::Down,
			)
			.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?
			.try_into()
			.map_err(|_| Error::<T>::MathOverflow { id: function_error_id })?;

			let mut buy_and_burn_amount: T::Balance = multiply_by_rational_with_rounding(
				amount.into(),
				T::BuyAndBurnFeePercentage::get(),
				total_fee_perc,
				Rounding::Down,
			)
			.ok_or(Error::<T>::UnexpectedFailure { id: function_error_id })?
			.try_into()
			.map_err(|_| Error::<T>::MathOverflow { id: function_error_id })?;

			pool_fee_amount = pool_fee_amount.min(amount);
			treasury_amount = treasury_amount.min(amount.saturating_sub(pool_fee_amount));
			buy_and_burn_amount =
				amount.saturating_sub(pool_fee_amount).saturating_sub(treasury_amount);

			// Ensure user has enough tokens
			T::Currency::ensure_can_withdraw(
				asset_id.into(),
				account,
				amount,
				WithdrawReasons::all(),
				Default::default(),
			)
			.or(Err(Error::<T>::NotEnoughAssetsForFees))?;

			T::Currency::transfer(
				asset_id,
				account,
				&T::TreasuryAccountId::get(),
				treasury_amount,
				ExistenceRequirement::AllowDeath,
			)?;

			T::Currency::transfer(
				asset_id,
				account,
				&T::BnbAccountId::get(),
				buy_and_burn_amount,
				ExistenceRequirement::AllowDeath,
			)?;

			match pool.kind {
				PoolKind::StableSwap => {
					T::StableSwap::settle_pool_fees(
						&account.clone().into(),
						pool.pool_id,
						asset_id,
						pool_fee_amount,
					)?;
				},
				PoolKind::Xyk => {
					<T as pallet::Config>::Xyk::settle_pool_fees(
						&account.clone().into(),
						pool.pool_id,
						asset_id,
						pool_fee_amount,
					)?;
				},
			}

			Ok((pool_fee_amount, treasury_amount, buy_and_burn_amount))
		}

		pub fn do_fee_lock(who: &T::AccountId, is_lockless: bool) -> DispatchResult {
			let function_error_id = 5u8;

			// if the fee_lock_metadata has not been init
			// we should just return Ok(()) here
			// since we are already charging normal fee
			// when fee_lock_metadata is uninit
			// Also, it would be incorrect to fail all
			// swaps when fee_lock_metadata is uninit
			if !T::FeeLock::is_fee_lock_init() {
				return Ok(());
			}

			// This should only fail if the fee_lock_metadata is uninit
			let fee_lock_amount = match T::FeeLock::get_fee_lock_amount(who) {
				Ok(v) => v,
				Err(_) => return Ok(()),
			};

			T::Currency::ensure_can_withdraw(
				T::NativeCurrencyId::get().into(),
				who,
				fee_lock_amount,
				WithdrawReasons::all(),
				Default::default(),
			)
			.or(Err(Error::<T>::NotEnoughAssetsForFeeLock))?;

			match is_lockless {
				// If unlock_fee fails do not return the error
				// unlock_fee fails if it cannot unlock fee successfully
				// for any reason (no fee locked, can't unlock yet, etc...)
				true => {
					let _ = T::FeeLock::unlock_fee(who);
				},
				// Process fee_lock shouldn't fail at this point but if it
				// does then return the error failing the swap
				false => {
					T::FeeLock::process_fee_lock(who)?;
				},
			}

			Ok(())
		}

		// impl for runtime apis, rather do the composition here with traits then in runtime with pallets
		pub fn calculate_sell_price(
			pool_id: T::CurrencyId,
			sell_asset_id: T::CurrencyId,
			sell_amount: T::Balance,
		) -> Option<T::Balance> {
			let pool_info = Self::get_pool_info(pool_id).ok()?;
			let (_, other) = pool_info.same_and_other(sell_asset_id)?;
			match pool_info.kind {
				PoolKind::Xyk =>
					<T as pallet::Config>::Xyk::get_dy(pool_id, sell_asset_id, other, sell_amount),
				PoolKind::StableSwap =>
					T::StableSwap::get_dy(pool_id, sell_asset_id, other, sell_amount),
			}
		}

		pub fn calculate_sell_price_with_impact(
			pool_id: T::CurrencyId,
			sell_asset_id: T::CurrencyId,
			sell_amount: T::Balance,
		) -> Option<(T::Balance, T::Balance)> {
			let pool_info = Self::get_pool_info(pool_id).ok()?;
			let (_, other) = pool_info.same_and_other(sell_asset_id)?;
			match pool_info.kind {
				PoolKind::Xyk => <T as pallet::Config>::Xyk::get_dy_with_impact(
					pool_id,
					sell_asset_id,
					other,
					sell_amount,
				),
				PoolKind::StableSwap =>
					T::StableSwap::get_dy_with_impact(pool_id, sell_asset_id, other, sell_amount),
			}
		}

		pub fn calculate_buy_price(
			pool_id: T::CurrencyId,
			bought_asset_id: T::CurrencyId,
			buy_amount: T::Balance,
		) -> Option<T::Balance> {
			let pool_info = Self::get_pool_info(pool_id).ok()?;
			let (_, other) = pool_info.same_and_other(bought_asset_id)?;
			match pool_info.kind {
				PoolKind::Xyk =>
					<T as pallet::Config>::Xyk::get_dx(pool_id, other, bought_asset_id, buy_amount),
				PoolKind::StableSwap =>
					T::StableSwap::get_dx(pool_id, other, bought_asset_id, buy_amount),
			}
		}

		pub fn calculate_buy_price_with_impact(
			pool_id: T::CurrencyId,
			bought_asset_id: T::CurrencyId,
			buy_amount: T::Balance,
		) -> Option<(T::Balance, T::Balance)> {
			let pool_info = Self::get_pool_info(pool_id).ok()?;
			let (_, other) = pool_info.same_and_other(bought_asset_id)?;
			match pool_info.kind {
				PoolKind::Xyk => <T as pallet::Config>::Xyk::get_dx_with_impact(
					pool_id,
					other,
					bought_asset_id,
					buy_amount,
				),
				PoolKind::StableSwap =>
					T::StableSwap::get_dx_with_impact(pool_id, other, bought_asset_id, buy_amount),
			}
		}

		pub fn get_burn_amount(
			pool_id: T::CurrencyId,
			lp_burn_amount: T::Balance,
		) -> Option<(T::Balance, T::Balance)> {
			<T as pallet::Config>::Xyk::get_burn_amounts(pool_id, lp_burn_amount)
				.or_else(|| T::StableSwap::get_burn_amounts(pool_id, lp_burn_amount))
		}

		pub fn get_pools_for_trading() -> Vec<T::CurrencyId> {
			let mut assets = vec![];
			if let Some(pools) = <T as pallet::Config>::Xyk::get_non_empty_pools() {
				assets.extend(pools.iter());
			}
			if let Some(pools) = T::StableSwap::get_non_empty_pools() {
				assets.extend(pools.iter());
			}
			assets
		}

		pub fn get_pools(pool_id: Option<T::CurrencyId>) -> Vec<(PoolInfoOf<T>, BalancePairOf<T>)> {
			let pool_ids = if pool_id.is_some() {
				vec![pool_id.unwrap()]
			} else {
				Self::get_pools_for_trading()
			};
			let mut pools = vec![];
			for id in pool_ids.into_iter() {
				if let Some(info) = Self::get_pool_info(id).ok() {
					let balances = match info.kind {
						PoolKind::Xyk =>
							<T as pallet::Config>::Xyk::get_pool_reserves(info.pool_id),
						PoolKind::StableSwap => T::StableSwap::get_pool_reserves(info.pool_id),
					};
					pools.push((info, balances.unwrap_or_default()))
				}
			}
			pools
		}

		pub fn calculate_expected_amount_for_minting(
			pool_id: PoolIdOf<T>,
			asset_id: T::CurrencyId,
			amount: T::Balance,
		) -> Option<T::Balance> {
			let pool_info = Self::get_pool_info(pool_id).ok()?;
			match pool_info.kind {
				PoolKind::Xyk => <T as pallet::Config>::Xyk::get_expected_amount_for_mint(
					pool_id, asset_id, amount,
				),
				PoolKind::StableSwap =>
					T::StableSwap::get_expected_amount_for_mint(pool_id, asset_id, amount),
			}
		}

		pub fn calculate_expected_lp_minted(
			pool_id: PoolIdOf<T>,
			amounts: BalancePairOf<T>,
		) -> Option<T::Balance> {
			let pool_info = Self::get_pool_info(pool_id).ok()?;
			match pool_info.kind {
				PoolKind::Xyk => <T as pallet::Config>::Xyk::get_mint_amount(pool_id, amounts),
				PoolKind::StableSwap => T::StableSwap::get_mint_amount(pool_id, amounts),
			}
		}

		pub fn get_pool_info(pool_id: PoolIdOf<T>) -> Result<PoolInfoOf<T>, Error<T>> {
			if let Some(pool) = <T as pallet::Config>::Xyk::get_pool_info(pool_id) {
				return Ok(PoolInfo { pool_id, kind: PoolKind::Xyk, pool })
			}
			if let Some(pool) = T::StableSwap::get_pool_info(pool_id) {
				return Ok(PoolInfo { pool_id, kind: PoolKind::StableSwap, pool })
			}

			return Err(Error::<T>::NoSuchPool);
		}

		fn check_assets_allowed(assets: AssetPairOf<T>) -> Result<(), Error<T>> {
			ensure!(
				!T::DisabledTokens::contains(&assets.0) && !T::DisabledTokens::contains(&assets.1),
				Error::<T>::FunctionNotAvailableForThisToken
			);
			Ok(())
		}

		fn get_valid_path(
			swap_pool_list: &Vec<PoolIdOf<T>>,
			asset_in: T::CurrencyId,
			asset_out: T::CurrencyId,
		) -> Result<(Vec<PoolInfoOf<T>>, Vec<AssetPairOf<T>>), Error<T>> {
			// at least one swap
			ensure!(swap_pool_list.len() > 0, Error::<T>::NoSuchPool);

			// check pools repetition
			let mut dedup = swap_pool_list.clone();
			dedup.sort();
			dedup.dedup();
			ensure!(dedup.len() == swap_pool_list.len(), Error::<T>::MultiSwapSamePool);

			let mut path: Vec<AssetPairOf<T>> = vec![];
			let mut pools: Vec<PoolInfoOf<T>> = vec![];
			for &pool_id in swap_pool_list.iter() {
				let pool_info = Self::get_pool_info(pool_id)?;
				pools.push(pool_info.clone());
				// function not available for tokens
				Self::check_assets_allowed(pool_info.pool)?;

				// check pools' asset connection
				// first is asset_id_in, last is asset_id_out
				let prev_asset_id = if let Some(&last) = path.last() { last.1 } else { asset_in };

				let pool = pool_info
					.same_and_other(prev_asset_id)
					.ok_or(Error::<T>::MultiSwapPathInvalid)?;
				path.push(pool);
			}

			ensure!(
				path.last().is_some_and(|&l| l.1 == asset_out),
				Error::<T>::MultiSwapPathInvalid
			);

			Ok((pools, path))
		}

		fn do_mint_liquidity(
			sender: &T::AccountId,
			pool_info: PoolInfoOf<T>,
			asset_id: T::CurrencyId,
			amount: T::Balance,
			max_amount: T::Balance,
			activate: bool,
		) -> Result<(T::Balance, T::Balance), DispatchError> {
			let (asset_with_amount, asset_other) =
				pool_info.same_and_other(asset_id).ok_or(Error::<T>::NoSuchPool)?;

			let amounts = match pool_info.kind {
				PoolKind::Xyk => {
					let (_, lp_amount, second_asset_withdrawn) =
						<T as pallet::Config>::Xyk::mint_liquidity(
							sender.clone(),
							asset_with_amount,
							asset_other,
							amount,
							max_amount,
							activate,
						)?;
					(lp_amount, second_asset_withdrawn)
				},
				PoolKind::StableSwap => {
					let expected = T::StableSwap::get_expected_amount_for_mint(
						pool_info.pool_id,
						asset_id,
						amount,
					)
					.unwrap_or_default();

					ensure!(expected <= max_amount, Error::<T>::ExcesiveInputAmount);

					let amounts = if asset_id == pool_info.pool.0 {
						(amount, expected)
					} else {
						(expected, amount)
					};

					let lp_amount = T::StableSwap::add_liquidity(
						&sender,
						pool_info.pool_id,
						amounts,
						Zero::zero(),
					)?;
					if activate && T::Rewards::native_rewards_enabled(pool_info.pool_id) {
						T::Rewards::activate_liquidity(
							sender.clone(),
							pool_info.pool_id,
							lp_amount,
							Some(ActivateKind::AvailableBalance),
						)?;
					}
					(lp_amount, expected)
				},
			};

			Ok(amounts)
		}

		fn do_swaps(
			sender: &T::AccountId,
			pools: Vec<PoolInfoOf<T>>,
			path: Vec<AssetPairOf<T>>,
			amount_in: T::Balance,
			min_amount_out: T::Balance,
		) -> Result<Vec<AtomicSwapOf<T>>, DispatchError> {
			let mut swaps: Vec<AtomicSwapOf<T>> = vec![];
			let mut amount_out = amount_in;
			for (pool, swap) in pools.iter().zip(path.into_iter()) {
				// check input asset id, or the foundation has a veto
				ensure!(
					!T::NontransferableTokens::contains(&swap.0) ||
						T::ArbitrageBot::contains(sender),
					Error::<T>::NontransferableToken
				);

				let amount_in = amount_out;
				amount_out = match pool.kind {
					PoolKind::StableSwap => {
						let SwapResult { amount_out, treasury_fee, bnb_fee, .. } =
							T::StableSwap::swap(
								sender,
								pool.pool_id,
								swap.0,
								swap.1,
								amount_in,
								Zero::zero(),
							)?;

						<T as pallet::Config>::Xyk::settle_treasury_and_burn(
							swap.1,
							bnb_fee,
							treasury_fee,
						)?;

						amount_out
					},
					PoolKind::Xyk => <T as pallet::Config>::Xyk::sell_asset(
						sender.clone(),
						swap.0,
						swap.1,
						amount_in,
						Zero::zero(),
						true,
					)?,
				};

				swaps.push(AtomicSwap {
					pool_id: pool.pool_id,
					kind: pool.kind.clone(),
					asset_in: swap.0,
					asset_out: swap.1,
					amount_in,
					amount_out,
				});
			}

			ensure!(amount_out >= min_amount_out, Error::<T>::InsufficientOutputAmount);

			Ok(swaps)
		}
	}
}

// for now ignore the stable swap
// native token is not a stablecoin
// so we don't expect a direct stable pool of (native, token_x)
// stables can be used for multipath evals eg. token_x -> usdc -> usdt -> native
impl<T: Config> Valuate for Pallet<T> {
	type CurrencyId = T::CurrencyId;
	type Balance = T::Balance;

	// a pool's pair has to be connected to base
	fn check_can_valuate(
		base_id: Self::CurrencyId,
		pool_id: Self::CurrencyId,
	) -> Result<(), DispatchError> {
		let pair = Self::get_pool_info(pool_id)?.pool;
		if pair.0 == base_id || pair.1 == base_id {
			return Ok(());
		}
		Err(Error::<T>::NoSuchPool.into())
	}

	fn check_pool_exist(pool_id: Self::CurrencyId) -> Result<(), DispatchError> {
		Self::get_pool_info(pool_id)?;
		Ok(())
	}

	fn find_paired_pool(
		base_id: Self::CurrencyId,
		asset_id: Self::CurrencyId,
	) -> Result<mangata_support::pools::PoolInfo<Self::CurrencyId, Self::Balance>, DispatchError> {
		let pool_id = <T as pallet::Config>::Xyk::get_liquidity_asset(base_id, asset_id)?;
		let maybe_pool = Self::get_pools(Some(pool_id));
		let (info, reserves) = maybe_pool.first().ok_or(Error::<T>::NoSuchPool)?;
		Ok((pool_id, info.pool, *reserves))
	}

	fn find_valuation(
		_: Self::CurrencyId,
		asset_id: Self::CurrencyId,
		amount: Self::Balance,
	) -> Result<Self::Balance, DispatchError> {
		Ok(<T as pallet::Config>::Xyk::valuate_non_liquidity_token(asset_id, amount))
	}

	fn get_reserve_and_lp_supply(
		base_id: Self::CurrencyId,
		pool_id: Self::CurrencyId,
	) -> Option<(Self::Balance, Self::Balance)> {
		let maybe_pool = Self::get_pools(Some(pool_id));
		let (info, reserves) = maybe_pool.first()?;
		let reserve = match info.pool {
			(id, _) if id == base_id => reserves.0,
			(_, id) if id == base_id => reserves.1,
			_ => Zero::zero(),
		};
		let issuance = T::Currency::total_issuance(pool_id);
		if reserve.is_zero() || issuance.is_zero() {
			return None;
		}
		Some((reserve, issuance))
	}

	fn get_valuation_for_paired(
		_: Self::CurrencyId,
		pool_id: Self::CurrencyId,
		amount: Self::Balance,
	) -> Self::Balance {
		<T as pallet::Config>::Xyk::valuate_liquidity_token(pool_id, amount)
	}
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct RpcAssetMetadata<TokenId> {
	pub token_id: TokenId,
	pub decimals: u32,
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct RpcPoolInfo<TokenId, Balance> {
	pub pool_id: TokenId,
	pub kind: PoolKind,
	pub lp_token_id: TokenId,
	pub assets: Vec<TokenId>,
	pub reserves: Vec<Balance>,
}

sp_api::decl_runtime_apis! {
	 /// This runtime api allows people to query the size of the liquidity pools
	 /// and quote prices for swaps.
	pub trait MarketRuntimeApi<Balance, AssetId>
	where
		Balance: Codec + MaybeDisplay + MaybeFromStr,
		AssetId: Codec + MaybeDisplay + MaybeFromStr,
	{
		fn calculate_sell_price(
			pool_id: AssetId,
			sell_asset_id: AssetId,
			sell_amount: Balance
		) -> Option<Balance>;

		fn calculate_sell_price_with_impact(
			pool_id: AssetId,
			sell_asset_id: AssetId,
			sell_amount: Balance
		) -> Option<(Balance, Balance)>;

		fn calculate_buy_price(
			pool_id: AssetId,
			buy_asset_id: AssetId,
			buy_amount: Balance
		) -> Option<Balance>;

		fn calculate_buy_price_with_impact(
			pool_id: AssetId,
			buy_asset_id: AssetId,
			buy_amount: Balance
		) -> Option<(Balance, Balance)>;

		fn get_burn_amount(
			pool_id: AssetId,
			lp_burn_amount: Balance,
		) -> Option<(Balance, Balance)>;

		fn calculate_expected_amount_for_minting(
			pool_id: AssetId,
			asset_id: AssetId,
			amount: Balance,
		) -> Option<Balance>;

		fn calculate_expected_lp_minted(
			pool_id: AssetId,
			amounts: (Balance, Balance),
		) -> Option<Balance>;

// 		fn get_max_instant_burn_amount(
// 			user: AccountId,
// 			liquidity_asset_id: AssetId,
// 		) -> Balance;

// 		fn get_max_instant_unreserve_amount(
// 			user: AccountId,
// 			liquidity_asset_id: AssetId,
// 		) -> Balance;

// 		fn calculate_rewards_amount(
// 			user: AccountId,
// 			liquidity_asset_id: AssetId,
// 		) -> Balance;

// 		fn calculate_balanced_sell_amount(
// 			total_amount: Balance,
// 			reserve_amount: Balance,
// 		) -> Balance;

// 		fn is_buy_asset_lock_free(
// 			path: sp_std::vec::Vec<AssetId>,
// 			input_amount: Balance,
// 		) -> Option<bool>;

// 		fn is_sell_asset_lock_free(
// 			path: sp_std::vec::Vec<AssetId>,
// 			input_amount: Balance,
// 		) -> Option<bool>;

		fn get_tradeable_tokens() -> Vec<RpcAssetMetadata<AssetId>>;

		fn get_pools_for_trading() -> Vec<AssetId>;

		fn get_pools(pool_id: Option<AssetId>) -> Vec<RpcPoolInfo<AssetId, Balance>>;

// 		fn get_total_number_of_swaps() -> u128;
	}
}
