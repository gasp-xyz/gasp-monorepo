#![cfg_attr(not(feature = "std"), no_std)]

use frame_benchmarking::Zero;
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure,
};
use frame_system::ensure_signed;
use sp_core::U256;
// TODO documentation!

use frame_support::{
	pallet_prelude::*,
	traits::{tokens::currency::MultiTokenCurrency, ExistenceRequirement, Get},
	transactional,
};
use frame_system::pallet_prelude::*;
use mangata_support::traits::{
	ActivationReservesProviderTrait, LiquidityMiningApi, ProofOfStakeRewardsApi,
};
use mangata_types::{multipurpose_liquidity::ActivateKind, Balance, TokenId};
use orml_tokens::{MultiTokenCurrencyExtended, MultiTokenReservableCurrency};
use sp_std::collections::btree_map::BTreeMap;

use sp_runtime::{traits::SaturatedConversion, Permill, Perbill};
use sp_std::{convert::TryInto, prelude::*};

mod reward_info;
use reward_info::RewardInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub(crate) const LOG_TARGET: &str = "proof-of-stake";

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

pub mod weights;
pub use weights::WeightInfo;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ActivationReservesProvider: ActivationReservesProviderTrait<
			AccountId = Self::AccountId,
		>;
		type NativeCurrencyId: Get<TokenId>;
		type Currency: MultiTokenCurrencyExtended<Self::AccountId>
			+ MultiTokenReservableCurrency<Self::AccountId>;
		#[pallet::constant]
		/// The account id that holds the liquidity mining issuance
		type LiquidityMiningIssuanceVault: Get<Self::AccountId>;
		#[pallet::constant]
		type RewardsDistributionPeriod: Get<u32>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	/// Errors
	pub enum Error<T> {
		/// Not enought assets
		NotEnoughAssets,
		/// Math overflow
		MathOverflow,
		/// Not enough rewards earned
		NotEnoughRewardsEarned,
		/// Not a promoted pool
		NotAPromotedPool,
		/// Past time calculation
		PastTimeCalculation,
		LiquidityCheckpointMathError,
		CalculateRewardsMathError,
		MathError,
		CalculateRewardsAllMathError,
		MissingRewardsInfoError,
		DeprecatedExtrinsic,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PoolPromotionUpdated(TokenId, Option<u8>),
		LiquidityActivated(T::AccountId, TokenId, Balance),
		LiquidityDeactivated(T::AccountId, TokenId, Balance),
		RewardsClaimed(T::AccountId, TokenId, Balance),
	}

	#[pallet::storage]
	#[pallet::getter(fn get_rewards_info)]
	pub type RewardsInfo<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		AccountIdOf<T>,
		Twox64Concat,
		TokenId,
		RewardInfo,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_cumulative_liquidity_to_rewards_ratio)]
	pub type CumulativeTotalLiquidityToRewardsRatio<T: Config> =
		StorageValue<_, BTreeMap<TokenId, PromotedPoolsRewardsInfo>, ValueQuery>;

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	pub struct PromotedPoolsRewardsInfo {
		pub weight: u8,
		pub rewards: U256,
	}

	#[pallet::storage]
	#[pallet::getter(fn total_activated_amount)]
	// Total activated liquidity
	pub type LiquidityMiningActivePoolV2<T: Config> =
		StorageMap<_, Twox64Concat, TokenId, u128, ValueQuery>;

	// XYK extrinsics.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<<T as Config>::WeightInfo>::proof_of_stake_compound_rewards())]
		#[transactional]
		pub fn compound_rewards(
			origin: OriginFor<T>,
			_liquidity_asset_id: TokenId,
			_amount_permille: Permill,
		) -> DispatchResultWithPostInfo {
			let _sender = ensure_signed(origin)?;

			//TODO: uncomment and maybe move compound logic to PoS
			// <T::Xyk as XykFunctionsTrait<T::AccountId>>::do_compound_rewards(
			// 	sender,
			// 	liquidity_asset_id.into(),
			// 	amount_permille,
			// )?;

			Ok(().into())
		}

		#[transactional]
		#[pallet::call_index(1)]
		#[pallet::weight(<<T as Config>::WeightInfo>::claim_rewards_v2())]
		pub fn claim_rewards_v2(
			origin: OriginFor<T>,
			_liquidity_token_id: TokenId,
			_amount: Balance,
		) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			Err(DispatchError::from(Error::<T>::DeprecatedExtrinsic))
		}

		#[transactional]
		#[pallet::call_index(2)]
		#[pallet::weight(<<T as Config>::WeightInfo>::claim_rewards_v2())]
		pub fn claim_rewards_all_v2(
			origin: OriginFor<T>,
			liquidity_token_id: TokenId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			<Self as ProofOfStakeRewardsApi<T::AccountId>>::claim_rewards_all(
				sender,
				liquidity_token_id,
			)?;

			Ok(())
		}

		// Disabled pool demotion
		#[pallet::call_index(3)]
		#[pallet::weight(<<T as Config>::WeightInfo>::update_pool_promotion())]
		pub fn update_pool_promotion(
			origin: OriginFor<T>,
			liquidity_token_id: TokenId,
			liquidity_mining_issuance_weight: u8,
		) -> DispatchResult {
			ensure_root(origin)?;

			//TODO what about disabling ??
			<Self as ProofOfStakeRewardsApi<T::AccountId>>::enable(
				liquidity_token_id,
				liquidity_mining_issuance_weight,
			);
			Ok(())
		}

		#[transactional]
		#[pallet::call_index(4)]
		#[pallet::weight(<<T as Config>::WeightInfo>::activate_liquidity_v2())]
		pub fn activate_liquidity_v2(
			origin: OriginFor<T>,
			liquidity_token_id: TokenId,
			amount: Balance,
			use_balance_from: Option<ActivateKind>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			<Self as ProofOfStakeRewardsApi<T::AccountId>>::activate_liquidity(
				sender,
				liquidity_token_id,
				amount,
				use_balance_from,
			)
		}

		#[transactional]
		#[pallet::call_index(5)]
		#[pallet::weight(<<T as Config>::WeightInfo>::deactivate_liquidity_v2())]
		pub fn deactivate_liquidity_v2(
			origin: OriginFor<T>,
			liquidity_token_id: TokenId,
			amount: Balance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			<Self as ProofOfStakeRewardsApi<T::AccountId>>::deactivate_liquidity(
				sender,
				liquidity_token_id,
				amount,
			)
		}
	}
}

impl<T: Config> Pallet<T> {
	fn rewards_period() -> u32 {
		<T as Config>::RewardsDistributionPeriod::get()
	}

	fn native_token_id() -> TokenId {
		<T as Config>::NativeCurrencyId::get()
	}

	fn get_pool_rewards(liquidity_asset_id: TokenId) -> Result<U256, sp_runtime::DispatchError> {
		Ok(CumulativeTotalLiquidityToRewardsRatio::<T>::get()
			.get(&liquidity_asset_id)
			.map(|v| v.rewards)
			.ok_or(Error::<T>::NotAPromotedPool)?)
	}

	fn get_current_rewards_time() -> Result<u32, sp_runtime::DispatchError> {
		<frame_system::Pallet<T>>::block_number()
			.saturated_into::<u32>()
			.checked_add(1)
			.and_then(|v| v.checked_div(T::RewardsDistributionPeriod::get()))
			.ok_or(DispatchError::from(Error::<T>::CalculateRewardsMathError))
	}

	fn ensure_is_promoted_pool(liquidity_asset_id: TokenId) -> Result<(), DispatchError> {
		if Self::get_pool_rewards(liquidity_asset_id).is_ok() {
			Ok(())
		} else {
			Err(DispatchError::from(Error::<T>::NotAPromotedPool))
		}
	}

	fn set_liquidity_minting_checkpoint_v2(
		user: AccountIdOf<T>,
		liquidity_asset_id: TokenId,
		liquidity_assets_added: Balance,
	) -> DispatchResult {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;
		let current_time: u32 = Self::get_current_rewards_time()?;
		let pool_ratio_current = Self::get_pool_rewards(liquidity_asset_id)?;
		let mut rewards_info = RewardsInfo::<T>::try_get(user.clone(), liquidity_asset_id)
			.unwrap_or(RewardInfo {
				activated_amount: 0_u128,
				rewards_not_yet_claimed: 0_u128,
				rewards_already_claimed: 0_u128,
				last_checkpoint: current_time,
				pool_ratio_at_last_checkpoint: pool_ratio_current,
				missing_at_last_checkpoint: U256::from(0u128),
			});
		rewards_info.activate_more::<T>(
			current_time,
			pool_ratio_current,
			liquidity_assets_added,
		)?;

		RewardsInfo::<T>::insert(user.clone(), liquidity_asset_id, rewards_info);

		//TODO: refactor storage name
		LiquidityMiningActivePoolV2::<T>::try_mutate(liquidity_asset_id, |active_amount| {
			if let Some(val) = active_amount.checked_add(liquidity_assets_added) {
				*active_amount = val;
				Ok(())
			} else {
				Err(())
			}
		})
		.map_err(|_| DispatchError::from(Error::<T>::LiquidityCheckpointMathError))?;

		Ok(())
	}

	fn set_liquidity_burning_checkpoint_v2(
		user: AccountIdOf<T>,
		liquidity_asset_id: TokenId,
		liquidity_assets_burned: Balance,
	) -> DispatchResult {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;
		let current_time: u32 = Self::get_current_rewards_time()?;
		let pool_ratio_current = Self::get_pool_rewards(liquidity_asset_id)?;

		let mut rewards_info = RewardsInfo::<T>::try_get(user.clone(), liquidity_asset_id)
			.or(Err(DispatchError::from(Error::<T>::MissingRewardsInfoError)))?;
		ensure!(
			rewards_info.activated_amount >= liquidity_assets_burned,
			Error::<T>::NotEnoughAssets
		);

		rewards_info.activate_less::<T>(
			current_time,
			pool_ratio_current,
			liquidity_assets_burned,
		)?;
		RewardsInfo::<T>::insert(user.clone(), liquidity_asset_id, rewards_info);

		LiquidityMiningActivePoolV2::<T>::try_mutate(liquidity_asset_id, |active_amount| {
			if let Some(val) = active_amount.checked_sub(liquidity_assets_burned) {
				*active_amount = val;
				Ok(())
			} else {
				Err(())
			}
		})
		.map_err(|_| DispatchError::from(Error::<T>::LiquidityCheckpointMathError))?;

		<T as Config>::ActivationReservesProvider::deactivate(
			liquidity_asset_id,
			&user,
			liquidity_assets_burned,
		);

		Ok(())
	}
}

impl<T: Config> ProofOfStakeRewardsApi<T::AccountId> for Pallet<T> {
	type Balance = Balance;

	type CurrencyId = TokenId;

	fn enable(liquidity_token_id: TokenId, weight: u8) {
		CumulativeTotalLiquidityToRewardsRatio::<T>::mutate(|promoted_pools| {
			promoted_pools
				.entry(liquidity_token_id)
				.and_modify(|info| info.weight = weight)
				.or_insert(PromotedPoolsRewardsInfo { weight, rewards: U256::zero() });
		});
		Pallet::<T>::deposit_event(Event::PoolPromotionUpdated(liquidity_token_id, Some(weight)));
	}

	fn disable(liquidity_token_id: TokenId) {
		CumulativeTotalLiquidityToRewardsRatio::<T>::mutate(|promoted_pools| {
			let _ = promoted_pools.remove(&liquidity_token_id);
		});
		Pallet::<T>::deposit_event(Event::PoolPromotionUpdated(liquidity_token_id, None));
	}

	fn is_enabled(liquidity_token_id: TokenId) -> bool {
		CumulativeTotalLiquidityToRewardsRatio::<T>::get().contains_key(&liquidity_token_id)
	}

	fn claim_rewards_all(
		user: T::AccountId,
		liquidity_asset_id: Self::CurrencyId,
	) -> Result<Self::Balance, DispatchError> {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;
		let mut rewards_info = RewardsInfo::<T>::try_get(user.clone(), liquidity_asset_id)
			.or(Err(DispatchError::from(Error::<T>::MissingRewardsInfoError)))?;
		let pool_rewards_ratio_current = Self::get_pool_rewards(liquidity_asset_id)?;

		let current_rewards = rewards_info
			.calculate_rewards_v2(Self::get_current_rewards_time()?, pool_rewards_ratio_current)
			.ok_or(Error::<T>::CalculateRewardsMathError)?;

		let total_available_rewards = current_rewards
			.checked_add(rewards_info.rewards_not_yet_claimed)
			.and_then(|v| v.checked_sub(rewards_info.rewards_already_claimed))
			.ok_or(Error::<T>::CalculateRewardsAllMathError)?;

		rewards_info.rewards_not_yet_claimed = 0_u128;
		rewards_info.rewards_already_claimed = current_rewards;

		<T as Config>::Currency::transfer(
			Self::native_token_id().into(),
			&<T as Config>::LiquidityMiningIssuanceVault::get(),
			&user,
			total_available_rewards.into(),
			ExistenceRequirement::KeepAlive,
		)?;

		RewardsInfo::<T>::insert(user.clone(), liquidity_asset_id, rewards_info);

		Pallet::<T>::deposit_event(Event::RewardsClaimed(
			user,
			liquidity_asset_id,
			total_available_rewards,
		));

		Ok(total_available_rewards)
	}

	fn activate_liquidity(
		user: T::AccountId,
		liquidity_asset_id: Self::CurrencyId,
		amount: Self::Balance,
		use_balance_from: Option<ActivateKind>,
	) -> DispatchResult {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;
		ensure!(
			<T as Config>::ActivationReservesProvider::can_activate(
				liquidity_asset_id,
				&user,
				amount,
				use_balance_from.clone()
			),
			Error::<T>::NotEnoughAssets
		);

		Self::set_liquidity_minting_checkpoint_v2(user.clone(), liquidity_asset_id, amount)?;

		// This must not fail due storage edits above
		<T as Config>::ActivationReservesProvider::activate(
			liquidity_asset_id,
			&user,
			amount,
			use_balance_from,
		)?;
		Pallet::<T>::deposit_event(Event::LiquidityActivated(user, liquidity_asset_id, amount));

		Ok(())
	}

	fn deactivate_liquidity(
		user: T::AccountId,
		liquidity_asset_id: Self::CurrencyId,
		amount: Self::Balance,
	) -> DispatchResult {
		if amount > 0 {
			Self::set_liquidity_burning_checkpoint_v2(user.clone(), liquidity_asset_id, amount)?;
			Pallet::<T>::deposit_event(Event::LiquidityDeactivated(
				user,
				liquidity_asset_id,
				amount,
			));
		}

		Ok(())
	}

	fn calculate_rewards_amount(
		user: AccountIdOf<T>,
		liquidity_asset_id: TokenId,
	) -> Result<Balance, DispatchError> {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;
		let rewards_info = RewardsInfo::<T>::try_get(user.clone(), liquidity_asset_id)
			.or(Err(DispatchError::from(Error::<T>::MissingRewardsInfoError)))?;
		println!("RATIO: {}", Self::get_pool_rewards(liquidity_asset_id)?);
		let current_rewards = match rewards_info.activated_amount {
			0 => 0u128,
			_ => rewards_info
				.calculate_rewards_v2(
					Self::get_current_rewards_time()?,
					Self::get_pool_rewards(liquidity_asset_id)?,
				)
				.ok_or(Error::<T>::CalculateRewardsMathError)?,
		};

		Ok(current_rewards
			.checked_add(rewards_info.rewards_not_yet_claimed)
			.and_then(|v| v.checked_sub(rewards_info.rewards_already_claimed))
			.ok_or(Error::<T>::CalculateRewardsMathError)?)
	}
}

//TODO: add unit test and migrate impl from issuance
impl<T: Config> LiquidityMiningApi for Pallet<T> {
	fn distribute_rewards(liquidity_mining_rewards: Balance) {
		//TODO: is that correct ?
		let _ = CumulativeTotalLiquidityToRewardsRatio::<T>::try_mutate(|promoted_pools| -> DispatchResult {
			// benchmark with max of X prom pools
			let activated_pools: Vec<_> = promoted_pools
				.clone()
				.into_iter()
				.filter_map(|(token_id, info)| {
					let activated_amount = Self::total_activated_amount(token_id);
					if activated_amount > 0 {
						Some((token_id, info.weight, info.rewards, activated_amount))
					}else{
						None
					}
				})
				.collect();

			let maybe_total_weight = activated_pools.iter().try_fold(
				0u64,
				|acc, &(_token_id, weight, _rewards, _activated_amount)| {
					acc.checked_add(weight.into())
				},
			);

			for (token_id, weight, rewards, activated_amount) in activated_pools {
				let liquidity_mining_issuance_for_pool = match maybe_total_weight {


					Some(total_weight) if !total_weight.is_zero() =>{
						println!("weight {}", weight);
						println!("total_weight {}", total_weight);
						println!("liquidity_mining_rewards {}", liquidity_mining_rewards);
						println!("liquidity_mining_rewards {}", Perbill::from_rational(1u32, 1u32).mul_floor(liquidity_mining_rewards));
						Perbill::from_rational(weight.into(), total_weight)
							.mul_floor(liquidity_mining_rewards)
					},
					_ => Balance::zero(),
				};
				println!("liquidity_mining_issuance_for_pool {}", liquidity_mining_issuance_for_pool);
				println!("activated_amount {}", activated_amount);

				let rewards_for_liquidity: U256 = U256::from(liquidity_mining_issuance_for_pool)
					.checked_mul(U256::from(u128::MAX))
					.and_then(|x| x.checked_div(activated_amount.into()))
					.and_then(|x| {
						println!("THIS ROUND TOKENS PER 1 LIQ {}", x / U256::from(u128::MAX));
						x.checked_add(rewards)
					})
					.ok_or(Error::<T>::MathError)?;

				println!("liquidity_mining_issuance_for_pool {}", rewards_for_liquidity);

				promoted_pools
					.entry(token_id.clone())
					.and_modify(|info| info.rewards = rewards_for_liquidity);
			}
			Ok(())
		});
	}
}
