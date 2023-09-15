#![cfg_attr(not(feature = "std"), no_std)]

use frame_benchmarking::Zero;
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	ensure
};
use frame_system::ensure_signed;
use frame_support::storage::bounded_btree_map::BoundedBTreeMap;
use sp_core::U256;
use sp_runtime::traits::AccountIdConversion;
use mangata_support::traits::Valuate;

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

use sp_runtime::{traits::SaturatedConversion, Perbill};
use sp_std::{convert::TryInto, prelude::*};

mod reward_info;
use reward_info::{AsymptoticCurveRewards, ConstCurveRewards, RewardInfo, RewardsCalculator};
mod benchmarking;

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

const PALLET_ID: frame_support::PalletId = frame_support::PalletId(*b"rewards!");

#[frame_support::pallet]
pub mod pallet {
	use frame_support::traits::Currency;
use mangata_support::traits::PoolCreateApi;

use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[cfg(feature = "runtime-benchmarks")]
	pub trait PoSBenchmarkingConrfig: pallet_issuance::Config {}
	#[cfg(feature = "runtime-benchmarks")]
	impl<T: pallet_issuance::Config> PoSBenchmarkingConfig for T {}

	#[cfg(not(feature = "runtime-benchmarks"))]
	pub trait PoSBenchmarkingConfig {}

	#[cfg(not(feature = "runtime-benchmarks"))]
	impl<T> PoSBenchmarkingConfig for T {}

	#[pallet::config]
	pub trait Config: frame_system::Config + PoSBenchmarkingConfig {
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
		type RewardsSchedulesLimit: Get<u32>;
		type MinRewardsPerSession: Get<u128>;
		type MaxRewardTokensPerPool: Get<u32>;
		type WeightInfo: WeightInfo;
		type ValuationApi: Valuate<Balance = mangata_types::Balance, CurrencyId = mangata_types::TokenId>;
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
		CannotScheduleRewardsInPast,
		PoolDoesNotExist,
		TooManySchedules,
		TooLittleRewardsPerSession,
		PoolRewardTokensLimitExceeded,
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
	pub type UserRewards3rdPartyInfo<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		AccountIdOf<T>,
		Twox64Concat,
		(TokenId, TokenId),
		RewardInfo,
		ValueQuery,
	>;

	#[pallet::storage]
	/// Stores information about pool weight and accumulated rewards. The accumulated
	/// rewards amount is the number of rewards that can be claimed per liquidity
	/// token. Here is tracked the number of rewards per liquidity token relationship.
	/// Expect larger values when the number of liquidity tokens are smaller.
	pub type PromotedPoolRewards<T: Config> =
		StorageValue<_, BTreeMap<TokenId, PromotedPools>, ValueQuery>;

	#[pallet::storage]
	pub type PromotedPoolRewards3rdParty<T: Config> = StorageValue<_, BTreeMap<(TokenId, TokenId), U256>, ValueQuery>;

	// pub type PromotedPoolRewards3rdParty<T: Config> = StorageValue<_, BTreeMap<(TokenId), U256>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn schedules)]
	pub type RewardsSchedules<T: Config> =
		StorageValue<_, BoundedBTreeMap<(T::BlockNumber, TokenId, TokenId, Balance, u64), (), T::RewardsSchedulesLimit>, ValueQuery>;

	#[pallet::storage]
	pub type RewardTokensPerPool<T: Config> = StorageMap<_, Twox64Concat, TokenId, BoundedBTreeMap<TokenId, (), T::MaxRewardTokensPerPool>, ValueQuery>;

	#[pallet::storage]
	pub type ScheduleId<T: Config> = StorageValue<_, u64, ValueQuery>;


	// #[pallet::storage]
	// pub type Rewards3rdParty<T: Config> =
	// 	StorageValue<_, BTreeMap<TokenId, (TokenId, Balance)>, ValueQuery>;
	//
	// #[pallet::storage]
	// pub type Rewards3rdParty<T: Config> =
	// 	StorageValue<_, BTreeMap<TokenId, (TokenId, Balance)>, ValueQuery>;

	#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
	/// Information about single token rewards
	pub struct PromotedPools {
		// Weight of the pool, each of the activated tokens has its weight assignedd
		// Liquidityt Mining Rewards are distributed based on that weight
		pub weight: u8,
		/// **Cumulative** number of rewards amount that can be claimed for single
		/// activted liquidity token
		pub rewards: U256,
	}

	#[pallet::storage]
	#[pallet::getter(fn total_activated_amount)]
	pub type TotalActivatedLiquidity<T: Config> =
		StorageMap<_, Twox64Concat, TokenId, u128, ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Claims liquidity mining rewards
		#[transactional]
		#[pallet::call_index(0)]
		#[pallet::weight(<<T as Config>::WeightInfo>::claim_rewards_all())]
		pub fn claim_rewards_all(
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

		/// Enables/disables pool for liquidity mining rewards
		#[pallet::call_index(1)]
		#[pallet::weight(<<T as Config>::WeightInfo>::update_pool_promotion())]
		pub fn update_pool_promotion(
			origin: OriginFor<T>,
			liquidity_token_id: TokenId,
			liquidity_mining_issuance_weight: u8,
		) -> DispatchResult {
			ensure_root(origin)?;

			if liquidity_mining_issuance_weight > 0 {
				<Self as ProofOfStakeRewardsApi<T::AccountId>>::enable(
					liquidity_token_id,
					liquidity_mining_issuance_weight,
				);
			} else {
				<Self as ProofOfStakeRewardsApi<T::AccountId>>::disable(liquidity_token_id);
			}
			Ok(())
		}

		/// Increases number of tokens used for liquidity mining purposes.
		///
		/// Parameters:
		/// - liquidity_token_id - id of the token
		/// - amount - amount of the token
		/// - use_balance_from - where from tokens should be used
		#[transactional]
		#[pallet::call_index(2)]
		#[pallet::weight(<<T as Config>::WeightInfo>::activate_liquidity())]
		pub fn activate_liquidity(
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

		/// Decreases number of tokens used for liquidity mining purposes
		#[transactional]
		#[pallet::call_index(3)]
		#[pallet::weight(<<T as Config>::WeightInfo>::deactivate_liquidity())]
		pub fn deactivate_liquidity(
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

		/// Schedules rewards for selected liquidity token
		/// - tokens - pair of tokens
		/// - amount - amount of the token
		/// - schedule_end - till when the rewards should be distributed, rewards are
		/// distributed starting from the *next* session till
		#[transactional]
		#[pallet::call_index(4)]
		#[pallet::weight(<<T as Config>::WeightInfo>::claim_rewards_all())]
		pub fn reward_pool(
			origin: OriginFor<T>,
			pool: (TokenId, TokenId),
			token_id: TokenId,
			amount: Balance,
			schedule_end: T::BlockNumber,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let rewarded_token = <T as Config>::ValuationApi::get_liquidity_asset(pool.0, pool.1)
				.map_err(|_| Error::<T>::PoolDoesNotExist)?;

			let current_session = Self::session_index();
			ensure!(
				schedule_end.saturated_into::<u32>() > current_session,
				Error::<T>::CannotScheduleRewardsInPast
			);

			let amount_per_session = schedule_end.saturated_into::<u32>()
				.checked_sub(current_session)
				.and_then(|v| amount.checked_div(v.into()))
				.ok_or(Error::<T>::MathOverflow)?;

			// TODO: use valuation instead amount directly
			ensure!(amount_per_session >= T::MinRewardsPerSession::get(), Error::<T>::TooLittleRewardsPerSession);
			println!("hello worldd!!!!");

			RewardTokensPerPool::<T>::try_mutate(rewarded_token, |tokens| {
				println!("{tokens:?}");
				tokens.try_insert(token_id,())
			}).or(Err(Error::<T>::PoolRewardTokensLimitExceeded))?;

			println!("{rewarded_token} =>>> {:?}",RewardTokensPerPool::<T>::get(rewarded_token));

			T::Currency::transfer(
				token_id.into(),
				&sender,
				&Self::pallet_account(),
				amount.into(),
				ExistenceRequirement::KeepAlive,
			)?;

			let current_session = Self::session_index();
			let schedule_id = ScheduleId::<T>::get();

			RewardsSchedules::<T>::try_mutate(|map| {

				let key: Option<(_,_,_,_,_)> = map
					.first_key_value()
					.map(|(x,y)| x.clone());


				if let Some(val) = key {
					if current_session > val.0.saturated_into::<u32>() {
						map.remove_entry(&val);
					}
				}

				map.try_insert((schedule_end, rewarded_token, token_id, amount_per_session, schedule_id), ())
			}).or(Err(Error::<T>::TooManySchedules))?;

			ScheduleId::<T>::mutate(|v| *v += 1);

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {

	fn calculate_rewards_amount_3rdparty(
		user: AccountIdOf<T>,
		liquidity_asset_id: TokenId,
	) -> Result<Vec<(TokenId, Balance)>, DispatchError> {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;

		let mut result: sp_std::vec::Vec<_> = Default::default();

		// TODO: get rid of collect
		let reward_tokens = RewardTokensPerPool::<T>::get(liquidity_asset_id)
			.iter()
			.map(|(x,_)| *x)
			.collect::<Vec<_>>();


		for i in reward_tokens{
			println!(";iittititiitititit");

			let rewards_info = UserRewards3rdPartyInfo::<T>::try_get(user.clone(), (liquidity_asset_id, i));
			println!("REWARDS INFO {rewards_info:?}");
			if let Ok(info) = rewards_info{
				let current_rewards = match info.activated_amount {
					0 => 0u128,
					_ => {
						let calc = RewardsCalculator::<ConstCurveRewards>::new2::<T>(
							user.clone(),
							liquidity_asset_id,
							i
						)?;
						calc.calculate_rewards().map_err(|err| Into::<Error<T>>::into(err))?
					},
				};

				let total_rewards = current_rewards
					.checked_add(info.rewards_not_yet_claimed)
					.and_then(|v| v.checked_sub(info.rewards_already_claimed))
					.ok_or(Error::<T>::CalculateRewardsMathError)?;
				result.push((i, total_rewards));
			}
		}
		Ok(result)

	}
	fn pallet_account() -> T::AccountId {
		PALLET_ID.into_account_truncating()
	}

	pub fn session_index() -> u32 {
		frame_system::Pallet::<T>::block_number()
			.saturated_into::<u32>()
			.checked_div(T::RewardsDistributionPeriod::get())
			.unwrap_or(0)
	}

	pub fn rewards_period() -> u32 {
		<T as Config>::RewardsDistributionPeriod::get()
	}

	fn native_token_id() -> TokenId {
		<T as Config>::NativeCurrencyId::get()
	}

	fn get_pool_rewards(liquidity_asset_id: TokenId) -> Result<U256, sp_runtime::DispatchError> {
		Ok(PromotedPoolRewards::<T>::get()
			.get(&liquidity_asset_id)
			.map(|v| v.rewards)
			.ok_or(Error::<T>::NotAPromotedPool)?)
	}

	fn get_pool_rewards_3rdparty(liquidity_asset_id: TokenId, reward_asset_id: TokenId) -> Result<U256, sp_runtime::DispatchError> {
		println!("ret_pool_rewards_3rdparty {liquidity_asset_id} {reward_asset_id}");
		Ok(*PromotedPoolRewards3rdParty::<T>::get()
			.get(&(liquidity_asset_id, reward_asset_id))
			//TODO: no error or some dedicated error
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
		println!("ensure_is_promoted_pool {liquidity_asset_id}");
		if Self::get_pool_rewards(liquidity_asset_id).is_ok() || !RewardTokensPerPool::<T>::get(liquidity_asset_id).is_empty() {
			println!("ensure_is_promoted_pool ok");
			Ok(())
		} else {
			println!("ensure_is_promoted_pool err");
			Err(DispatchError::from(Error::<T>::NotAPromotedPool))
		}
	}

	fn set_liquidity_minting_checkpoint(
		user: AccountIdOf<T>,
		liquidity_asset_id: TokenId,
		liquidity_assets_added: Balance,
	) -> DispatchResult {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;

		println!("set_liquidity_minting_checkpoint1");

		{
			let reward_tokens = RewardTokensPerPool::<T>::get(liquidity_asset_id)
				.iter()
				.map(|(x,_)| *x)
				// NOTE: get rid of collect
				.collect::<Vec<_>>();


			for i in reward_tokens{
				let calc = RewardsCalculator::<ConstCurveRewards>::new2::<T>(
					user.clone(),
					liquidity_asset_id,
					i
				)?;

				let rewards_info = calc
					.activate_more(liquidity_assets_added)
					.map_err(|err| Into::<Error<T>>::into(err))?;

				UserRewards3rdPartyInfo::<T>::insert(user.clone(), (liquidity_asset_id,i), rewards_info);
			}
		}
		println!("set_liquidity_minting_checkpoint2");

		{
			let calc = RewardsCalculator::<AsymptoticCurveRewards>::new::<T>(
				user.clone(),
				liquidity_asset_id,
			)?;
			let rewards_info = calc
				.activate_more(liquidity_assets_added)
				.map_err(|err| Into::<Error<T>>::into(err))?;

			RewardsInfo::<T>::insert(user.clone(), liquidity_asset_id, rewards_info);
		}


		println!("set_liquidity_minting_checkpoint3");

		TotalActivatedLiquidity::<T>::try_mutate(liquidity_asset_id, |active_amount| {
			if let Some(val) = active_amount.checked_add(liquidity_assets_added) {
				println!("!!!!!!!!!!!!!!!!!!!!!!!!!");
				*active_amount = val;
				Ok(())
			} else {
				println!("$$$$$$$$$$$$$$$$$$$$$$$$$");
				Err(())
			}
		})
		.map_err(|_| DispatchError::from(Error::<T>::LiquidityCheckpointMathError))?;

		Ok(())
	}

	fn set_liquidity_burning_checkpoint(
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

		let calc = RewardsCalculator::<AsymptoticCurveRewards>::new::<T>(
			user.clone(),
			liquidity_asset_id,
		)?;
		let rewards_info = calc
			.activate_less(liquidity_assets_burned)
			.map_err(|err| Into::<Error<T>>::into(err))?;

		RewardsInfo::<T>::insert(user.clone(), liquidity_asset_id, rewards_info);

		TotalActivatedLiquidity::<T>::try_mutate(liquidity_asset_id, |active_amount| {
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
		PromotedPoolRewards::<T>::mutate(|promoted_pools| {
			promoted_pools
				.entry(liquidity_token_id)
				.and_modify(|info| info.weight = weight)
				.or_insert(PromotedPools { weight, rewards: U256::zero() });
		});
		Pallet::<T>::deposit_event(Event::PoolPromotionUpdated(liquidity_token_id, Some(weight)));
	}

	fn disable(liquidity_token_id: TokenId) {
		PromotedPoolRewards::<T>::mutate(|promoted_pools| {
			if let Some(info) = promoted_pools.get_mut(&liquidity_token_id) {
				info.weight = 0;
			}
		});
		Pallet::<T>::deposit_event(Event::PoolPromotionUpdated(liquidity_token_id, None));
	}

	fn is_enabled(liquidity_token_id: TokenId) -> bool {
		PromotedPoolRewards::<T>::get().contains_key(&liquidity_token_id)
	}

	fn claim_rewards_all(
		user: T::AccountId,
		liquidity_asset_id: Self::CurrencyId,
	) -> Result<Self::Balance, DispatchError> {
		Self::ensure_is_promoted_pool(liquidity_asset_id)?;

		let mut rewards_info = RewardsInfo::<T>::try_get(user.clone(), liquidity_asset_id)
			.or(Err(DispatchError::from(Error::<T>::MissingRewardsInfoError)))?;

		let calc = RewardsCalculator::<AsymptoticCurveRewards>::new::<T>(
			user.clone(),
			liquidity_asset_id,
		)?;
		let current_rewards =
			calc.calculate_rewards().map_err(|err| Into::<Error<T>>::into(err))?;

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
		println!("activate_liquidity");
		ensure!(
			<T as Config>::ActivationReservesProvider::can_activate(
				liquidity_asset_id,
				&user,
				amount,
				use_balance_from.clone()
			),
			Error::<T>::NotEnoughAssets
		);

		Self::set_liquidity_minting_checkpoint(user.clone(), liquidity_asset_id, amount)?;

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
			Self::set_liquidity_burning_checkpoint(user.clone(), liquidity_asset_id, amount)?;
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

		let current_rewards = match rewards_info.activated_amount {
			0 => 0u128,
			_ => {
				let calc = RewardsCalculator::<AsymptoticCurveRewards>::new::<T>(
					user.clone(),
					liquidity_asset_id,
				)?;
				calc.calculate_rewards().map_err(|err| Into::<Error<T>>::into(err))?
			},
		};

		Ok(current_rewards
			.checked_add(rewards_info.rewards_not_yet_claimed)
			.and_then(|v| v.checked_sub(rewards_info.rewards_already_claimed))
			.ok_or(Error::<T>::CalculateRewardsMathError)?)
	}
}

impl<T: Config> LiquidityMiningApi for Pallet<T> {
	/// Distributs liquidity mining rewards between all the activated tokens based on their weight
	fn distribute_rewards(liquidity_mining_rewards: Balance) {

		let schedules = RewardsSchedules::<T>::get();
		let mut pools = PromotedPoolRewards3rdParty::<T>::get();

		let it = schedules
			.iter()
			.filter_map(|((session, rewarded_token, tokenid, amount, _),())|{
				if (*session).saturated_into::<u32>() > Self::session_index() {
					Some((rewarded_token, tokenid, amount))
				} else {
					None
				}
		});

		for (staked_token, token, amount) in it {
			println!("STAKED TOKEN: {staked_token} TOKEN: {token} AMOUNT: {amount}");

			let activated_amount = Self::total_activated_amount(staked_token);
			println!("ACTIVATED AMOUNT: {activated_amount}");
			// NOTE: fix
			let rewards = pools.get(&(*staked_token, *token)).cloned().unwrap_or_default();
			let rewards_for_liquidity = U256::from(*amount)
				.checked_mul(U256::from(u128::MAX))
				.and_then(|x| x.checked_div(activated_amount.into()))
				.and_then(|x| x.checked_add(rewards));

				if let Some(val) = rewards_for_liquidity {
					pools.insert((*staked_token,*token), val);
				}
		}
		println!("STORING POOLS: {pools:?}");
		PromotedPoolRewards3rdParty::<T>::put(pools);


		let _ = PromotedPoolRewards::<T>::try_mutate(|promoted_pools| -> DispatchResult {
			// benchmark with max of X prom pools
			let activated_pools: Vec<_> = promoted_pools
				.clone()
				.into_iter()
				.filter_map(|(token_id, info)| {
					let activated_amount = Self::total_activated_amount(token_id);
					if activated_amount > 0 && info.weight > 0 {
						Some((token_id, info.weight, info.rewards, activated_amount))
					} else {
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
					Some(total_weight) if !total_weight.is_zero() =>
						Perbill::from_rational(weight.into(), total_weight)
							.mul_floor(liquidity_mining_rewards),
					_ => Balance::zero(),
				};

				let rewards_for_liquidity: U256 = U256::from(liquidity_mining_issuance_for_pool)
					.checked_mul(U256::from(u128::MAX))
					.and_then(|x| x.checked_div(activated_amount.into()))
					.and_then(|x| x.checked_add(rewards))
					.ok_or(Error::<T>::MathError)?;

				promoted_pools
					.entry(token_id.clone())
					.and_modify(|info| info.rewards = rewards_for_liquidity);
			}
			Ok(())
		});
	}
}
