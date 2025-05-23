#![cfg_attr(not(feature = "std"), no_std)]
// #![feature(custom_test_frameworks)]

use frame_support::{
	dispatch::DispatchResult,
	ensure,
	pallet_prelude::*,
	storage::bounded_btree_set::BoundedBTreeSet,
	traits::{Get, MultiTokenCurrency, StorageVersion},
	transactional,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use mangata_support::{
	pools::ValuateFor,
	traits::{FeeLockTriggerTrait, XykFunctionsTrait},
};
use orml_tokens::{MultiTokenCurrencyExtended, MultiTokenReservableCurrency};

use sp_runtime::{
	traits::{Bounded, CheckedAdd, Zero},
	Saturating,
};
use sp_std::{convert::TryInto, prelude::*};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

pub(crate) const LOG_TARGET: &'static str = "fee-lock";

// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] 💸 ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

pub use pallet::*;

pub type BalanceOf<T> = <<T as pallet::Config>::Tokens as MultiTokenCurrency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type CurrencyIdOf<T> = <<T as pallet::Config>::Tokens as MultiTokenCurrency<
	<T as frame_system::Config>::AccountId,
>>::CurrencyId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_idle(now: BlockNumberFor<T>, remaining_weight: Weight) -> Weight {
			let mut consumed_weight: Weight = Default::default();

			// process only up to 80% or remaining weight
			let base_cost = T::DbWeight::get().reads(1)   // UnlockQueueBegin
				+ T::DbWeight::get().reads(1)   // UnlockQueueEnd
				+ T::DbWeight::get().reads(1); // FeeLockMetadata

			let cost_of_single_unlock_iteration = T::WeightInfo::unlock_fee() // cost of unlock action
				+ T::DbWeight::get().reads(1)   // FeeLockMetadataQeueuePosition
				+ T::DbWeight::get().reads(1)   // AccountFeeLockData
				+ T::DbWeight::get().reads(1)   // UnlockQueue
				+ T::DbWeight::get().writes(1); // UnlockQueueBegin

			if (base_cost + cost_of_single_unlock_iteration).ref_time() >
				remaining_weight.ref_time()
			{
				return Weight::from_parts(0, 0)
			}

			let metadata = Self::get_fee_lock_metadata();
			let period_length = metadata.map(|meta| meta.period_length);
			let begin = UnlockQueueBegin::<T>::get();
			let end = UnlockQueueEnd::<T>::get();
			consumed_weight += base_cost;

			for i in begin..end {
				consumed_weight += T::DbWeight::get().reads(3); // UnlockQueue, AccountFeeLockData FeeLockMetadataQeueuePosition
				UnlockQueueBegin::<T>::put(i);
				consumed_weight += T::DbWeight::get().writes(1);
				let who = UnlockQueue::<T>::get(i);
				let queue_pos =
					who.as_ref().and_then(|acc| FeeLockMetadataQeueuePosition::<T>::get(acc));

				if matches!(queue_pos, Some(id) if id == i) {
					let lock_info = who.and_then(|who| {
						AccountFeeLockData::<T>::try_get(&who).map(|lock| (who.clone(), lock)).ok()
					});

					match (period_length, lock_info) {
						(Some(period_length), Some((who, lock))) => {
							let unlock_block = lock.last_fee_lock_block.checked_add(&period_length);

							if matches!(unlock_block, Some(unlock) if unlock <= now) {
								UnlockQueueBegin::<T>::put(i + 1);
								consumed_weight += T::WeightInfo::unlock_fee();
								let _ = <Self as FeeLockTriggerTrait<
									T::AccountId,
									BalanceOf<T>,
									CurrencyIdOf<T>,
								>>::unlock_fee(&who);
							} else {
								break
							}
						},
						_ => break,
					};
				} else {
					UnlockQueueBegin::<T>::put(i + 1);
				}

				if cost_of_single_unlock_iteration.ref_time() >
					(remaining_weight.ref_time() - consumed_weight.ref_time())
				{
					break
				}
			}
			consumed_weight
		}
	}

	#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
	#[codec(mel_bound(T: Config))]
	#[scale_info(skip_type_params(T))]
	pub struct FeeLockMetadataInfo<T: Config> {
		pub period_length: BlockNumberFor<T>,
		pub fee_lock_amount: BalanceOf<T>,
		pub swap_value_threshold: BalanceOf<T>,
		pub whitelisted_tokens: BoundedBTreeSet<CurrencyIdOf<T>, T::MaxCuratedTokens>,
	}

	impl<T: Config> FeeLockMetadataInfo<T> {
		pub fn is_whitelisted(&self, token_id: CurrencyIdOf<T>) -> bool {
			if T::NativeTokenId::get() == token_id {
				return true
			}
			self.whitelisted_tokens.contains(&token_id)
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn get_fee_lock_metadata)]
	pub type FeeLockMetadata<T: Config> = StorageValue<_, FeeLockMetadataInfo<T>, OptionQuery>;

	#[pallet::storage]
	pub type FeeLockMetadataQeueuePosition<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, u128, OptionQuery>;

	#[pallet::storage]
	pub type UnlockQueue<T: Config> = StorageMap<_, Twox64Concat, u128, T::AccountId, OptionQuery>;

	#[pallet::storage]
	pub type UnlockQueueBegin<T: Config> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	pub type UnlockQueueEnd<T: Config> = StorageValue<_, u128, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_token_value_threshold)]
	pub type TokenValueThreshold<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery, BalanceMaxValue<T>>;
	#[pallet::type_value]
	pub fn BalanceMaxValue<T: Config>() -> BalanceOf<T> {
		BalanceOf::<T>::max_value()
	}

	#[derive(
		Eq, PartialEq, Clone, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo, Default,
	)]
	pub struct AccountFeeLockDataInfo<BlockNumber, Balance> {
		pub total_fee_lock_amount: Balance,
		pub last_fee_lock_block: BlockNumber,
	}

	#[pallet::storage]
	#[pallet::getter(fn get_account_fee_lock_data)]
	pub type AccountFeeLockData<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		AccountFeeLockDataInfo<BlockNumberFor<T>, BalanceOf<T>>,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		FeeLockMetadataUpdated,
		FeeLockUnlocked(T::AccountId, BalanceOf<T>),
		FeeLocked { who: T::AccountId, lock_amount: BalanceOf<T>, total_locked: BalanceOf<T> },
		TokenValueThresholdsUpdated,
	}

	#[pallet::error]
	/// Errors
	pub enum Error<T> {
		/// Locks were incorrectly initialized
		FeeLocksIncorrectlyInitialzed,
		/// Lock metadata is invalid
		InvalidFeeLockMetadata,
		/// Locks have not been initialzed
		FeeLocksNotInitialized,
		/// No tokens of the user are fee-locked
		NotFeeLocked,
		/// The lock cannot be unlocked yet
		CantUnlockFeeYet,
		/// The limit on the maximum curated tokens for which there is a swap threshold is exceeded
		MaxCuratedTokensLimitExceeded,
		/// An unexpected failure has occured
		UnexpectedFailure,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type MaxCuratedTokens: Get<u32>;
		type Tokens: MultiTokenCurrencyExtended<Self::AccountId>
			+ MultiTokenReservableCurrency<Self::AccountId>;
		type ValuateForNative: ValuateFor<
			Self::NativeTokenId,
			Balance = BalanceOf<Self>,
			CurrencyId = CurrencyIdOf<Self>,
		>;
		#[pallet::constant]
		type NativeTokenId: Get<CurrencyIdOf<Self>>;
		type WeightInfo: WeightInfo;
		#[cfg(all(feature = "runtime-benchmarks", not(test)))]
		type Xyk: XykFunctionsTrait<Self::AccountId, BalanceOf<Self>, CurrencyIdOf<Self>>;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub period_length: Option<BlockNumberFor<T>>,
		pub fee_lock_amount: Option<BalanceOf<T>>,
		pub swap_value_threshold: Option<BalanceOf<T>>,
		pub whitelisted_tokens: Vec<CurrencyIdOf<T>>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				period_length: Default::default(),
				fee_lock_amount: Default::default(),
				swap_value_threshold: Default::default(),
				whitelisted_tokens: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			match (self.period_length, self.fee_lock_amount, self.swap_value_threshold) {
				(Some(period), Some(amount), Some(threshold)) => {
					let mut tokens: BoundedBTreeSet<CurrencyIdOf<T>, T::MaxCuratedTokens> =
						Default::default();
					for t in self.whitelisted_tokens.iter() {
						tokens
							.try_insert(*t)
							.expect("list of tokens is <= than T::MaxCuratedTokens");
					}

					FeeLockMetadata::<T>::put(FeeLockMetadataInfo {
						period_length: period,
						fee_lock_amount: amount,
						swap_value_threshold: threshold,
						whitelisted_tokens: tokens,
					});
				},
				(None, None, None) => {},
				_ => {
					panic!("either all or non config parameters should be set");
				},
			};
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// The weight is calculated using MaxCuratedTokens so it is the worst case weight
		#[pallet::call_index(0)]
		#[transactional]
		#[pallet::weight(T::WeightInfo::update_fee_lock_metadata())]
		pub fn update_fee_lock_metadata(
			origin: OriginFor<T>,
			period_length: Option<BlockNumberFor<T>>,
			fee_lock_amount: Option<BalanceOf<T>>,
			swap_value_threshold: Option<BalanceOf<T>>,
			should_be_whitelisted: Option<Vec<(CurrencyIdOf<T>, bool)>>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let mut fee_lock_metadata =
				Self::get_fee_lock_metadata().unwrap_or(FeeLockMetadataInfo {
					period_length: Default::default(),
					fee_lock_amount: Default::default(),
					swap_value_threshold: Default::default(),
					whitelisted_tokens: Default::default(),
				});

			fee_lock_metadata.period_length =
				period_length.unwrap_or(fee_lock_metadata.period_length);
			fee_lock_metadata.fee_lock_amount =
				fee_lock_amount.unwrap_or(fee_lock_metadata.fee_lock_amount);
			fee_lock_metadata.swap_value_threshold =
				swap_value_threshold.unwrap_or(fee_lock_metadata.swap_value_threshold);

			ensure!(
				!fee_lock_metadata.fee_lock_amount.is_zero(),
				Error::<T>::InvalidFeeLockMetadata
			);
			ensure!(!fee_lock_metadata.period_length.is_zero(), Error::<T>::InvalidFeeLockMetadata);
			ensure!(
				!fee_lock_metadata.swap_value_threshold.is_zero(),
				Error::<T>::InvalidFeeLockMetadata
			);

			if let Some(should_be_whitelisted) = should_be_whitelisted {
				for (token_id, should_be_whitelisted) in should_be_whitelisted.iter() {
					match should_be_whitelisted {
						true => {
							let _ = fee_lock_metadata
								.whitelisted_tokens
								.try_insert(*token_id)
								.map_err(|_| Error::<T>::MaxCuratedTokensLimitExceeded)?;
						},
						false => {
							let _ = fee_lock_metadata.whitelisted_tokens.remove(token_id);
						},
					}
				}
			}

			FeeLockMetadata::<T>::put(fee_lock_metadata);

			Pallet::<T>::deposit_event(Event::FeeLockMetadataUpdated);

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[transactional]
		#[pallet::weight(T::WeightInfo::unlock_fee())]
		pub fn unlock_fee(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Ok(<Self as FeeLockTriggerTrait<T::AccountId, BalanceOf<T>, CurrencyIdOf<T>>>::unlock_fee(&who)?.into())
		}

		// The weight is calculated using MaxCuratedTokens so it is the worst case weight
		#[pallet::call_index(2)]
		#[transactional]
		#[pallet::weight(T::DbWeight::get().writes(token_value_thresholds.len() as u64).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn update_token_value_threshold(
			origin: OriginFor<T>,
			token_value_thresholds: Vec<(CurrencyIdOf<T>, Option<BalanceOf<T>>)>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			for (token_id, maybe_set) in token_value_thresholds.iter() {
				match maybe_set {
					Some(v) => {
						TokenValueThreshold::<T>::insert(token_id, v);
					},
					None => {
						TokenValueThreshold::<T>::remove(token_id);
					},
				}
			}

			Pallet::<T>::deposit_event(Event::TokenValueThresholdsUpdated);

			Ok(().into())
		}
	}
}
impl<T: Config> Pallet<T> {
	pub(crate) fn push_to_the_end_of_unlock_queue(who: &T::AccountId) {
		let mut id = Default::default();
		let id_ref = &mut id;
		UnlockQueueEnd::<T>::mutate(|id| {
			*id_ref = *id;
			*id = *id + 1
		});
		UnlockQueue::<T>::insert(id, who);
		FeeLockMetadataQeueuePosition::<T>::set(who, Some(id));
	}

	pub(crate) fn move_to_the_end_of_unlock_queue(who: &T::AccountId) {
		if let Ok(id) = FeeLockMetadataQeueuePosition::<T>::try_get(who) {
			UnlockQueue::<T>::take(id);
			Self::push_to_the_end_of_unlock_queue(who);
		} else {
			Self::push_to_the_end_of_unlock_queue(who);
		}
	}
}

impl<T: Config> FeeLockTriggerTrait<T::AccountId, BalanceOf<T>, CurrencyIdOf<T>> for Pallet<T> {
	fn is_swap_tokens_lockless(token_id: CurrencyIdOf<T>, token_amount: BalanceOf<T>) -> bool {
		if let Some(fee_lock_metadata) = Self::get_fee_lock_metadata() {
			if fee_lock_metadata.is_whitelisted(token_id) {
				if let Some(v) = Self::get_swap_valuation_for_token(token_id, token_amount) {
					return v >= fee_lock_metadata.swap_value_threshold
				}
			}
		}
		// Default return false
		false
	}

	fn is_fee_lock_init() -> bool {
		Self::get_fee_lock_metadata().is_some()
	}

	fn is_whitelisted(token_id: CurrencyIdOf<T>) -> bool {
		if let Some(fee_lock_metadata) = Self::get_fee_lock_metadata() {
			fee_lock_metadata.is_whitelisted(token_id)
		} else {
			false
		}
	}

	fn get_swap_valuation_for_token(
		valuating_token_id: CurrencyIdOf<T>,
		valuating_token_amount: BalanceOf<T>,
	) -> Option<BalanceOf<T>> {
		if T::NativeTokenId::get() == valuating_token_id {
			return Some(valuating_token_amount)
		}
		let value =
			T::ValuateForNative::find_valuation_for(valuating_token_id, valuating_token_amount)
				.ok()?;
		Some(value)
	}

	// This function is not expected to fail unless fee_lock_metadata is uninit
	fn get_fee_lock_amount(who: &T::AccountId) -> Result<BalanceOf<T>, DispatchError> {
		let fee_lock_metadata =
			Self::get_fee_lock_metadata().ok_or(Error::<T>::FeeLocksNotInitialized)?;
		let mut account_fee_lock_data = Self::get_account_fee_lock_data(who);
		let now = <frame_system::Pallet<T>>::block_number();

		// This is cause now >= last_fee_lock_block
		ensure!(now >= account_fee_lock_data.last_fee_lock_block, Error::<T>::UnexpectedFailure);

		if now <
			account_fee_lock_data
				.last_fee_lock_block
				.saturating_add(fee_lock_metadata.period_length)
		{
			Ok(fee_lock_metadata.fee_lock_amount)
		} else {
			// We must either reserve more or unreserve
			match (fee_lock_metadata.fee_lock_amount, account_fee_lock_data.total_fee_lock_amount) {
				(x, y) if x > y => Ok(x.saturating_sub(y)),
				_ => Ok(Default::default()),
			}
		}
	}

	fn process_fee_lock(who: &T::AccountId) -> DispatchResult {
		let fee_lock_metadata =
			Self::get_fee_lock_metadata().ok_or(Error::<T>::FeeLocksNotInitialized)?;
		let mut account_fee_lock_data = Self::get_account_fee_lock_data(who);
		let now = <frame_system::Pallet<T>>::block_number();

		// This is cause now >= last_fee_lock_block
		ensure!(now >= account_fee_lock_data.last_fee_lock_block, Error::<T>::UnexpectedFailure);

		if now <
			account_fee_lock_data
				.last_fee_lock_block
				.saturating_add(fee_lock_metadata.period_length)
		{
			// First storage edit
			// Cannot fail beyond this point
			// Rerserve additional fee_lock_amount
			<T as pallet::Config>::Tokens::reserve(
				<T as pallet::Config>::NativeTokenId::get().into(),
				who,
				fee_lock_metadata.fee_lock_amount,
			)?;

			// Insert updated account_lock_info into storage
			// This is not expected to fail
			account_fee_lock_data.total_fee_lock_amount = account_fee_lock_data
				.total_fee_lock_amount
				.saturating_add(fee_lock_metadata.fee_lock_amount);
			account_fee_lock_data.last_fee_lock_block = now;
			AccountFeeLockData::<T>::insert(who.clone(), account_fee_lock_data.clone());
			Self::move_to_the_end_of_unlock_queue(who);
			Self::deposit_event(Event::FeeLocked {
				who: who.clone(),
				lock_amount: fee_lock_metadata.fee_lock_amount,
				total_locked: account_fee_lock_data.total_fee_lock_amount,
			});
		} else {
			// We must either reserve more or unreserve
			match (fee_lock_metadata.fee_lock_amount, account_fee_lock_data.total_fee_lock_amount) {
				(x, y) if x > y => <T as pallet::Config>::Tokens::reserve(
					<T as pallet::Config>::NativeTokenId::get().into(),
					who,
					x.saturating_sub(y),
				)?,
				(x, y) if x < y => {
					let unreserve_result = <T as pallet::Config>::Tokens::unreserve(
						<T as pallet::Config>::NativeTokenId::get().into(),
						who,
						y.saturating_sub(x),
					);
					if !unreserve_result.is_zero() {
						log::warn!(
							"Process fee lock unreserve resulted in non-zero unreserve_result {:?}",
							unreserve_result
						);
					}
				},
				_ => {},
			}
			// Insert updated account_lock_info into storage
			// This is not expected to fail
			account_fee_lock_data.total_fee_lock_amount = fee_lock_metadata.fee_lock_amount;
			account_fee_lock_data.last_fee_lock_block = now;
			AccountFeeLockData::<T>::insert(who.clone(), account_fee_lock_data.clone());
			Self::move_to_the_end_of_unlock_queue(who);
			Self::deposit_event(Event::FeeLocked {
				who: who.clone(),
				lock_amount: fee_lock_metadata.fee_lock_amount,
				total_locked: account_fee_lock_data.total_fee_lock_amount,
			});
		}

		Ok(())
	}

	fn can_unlock_fee(who: &T::AccountId) -> DispatchResult {
		// Check if total_fee_lock_amount is non-zero
		// THEN Check is period is greater than last

		let account_fee_lock_data = Self::get_account_fee_lock_data(&who);

		ensure!(!account_fee_lock_data.total_fee_lock_amount.is_zero(), Error::<T>::NotFeeLocked);

		let fee_lock_metadata =
			Self::get_fee_lock_metadata().ok_or(Error::<T>::FeeLocksNotInitialized)?;

		let now = <frame_system::Pallet<T>>::block_number();

		ensure!(
			now >= account_fee_lock_data
				.last_fee_lock_block
				.saturating_add(fee_lock_metadata.period_length),
			Error::<T>::CantUnlockFeeYet
		);

		Ok(())
	}

	fn unlock_fee(who: &T::AccountId) -> DispatchResult {
		// Check if total_fee_lock_amount is non-zero
		// THEN Check is period is greater than last

		let account_fee_lock_data = Self::get_account_fee_lock_data(&who);

		ensure!(!account_fee_lock_data.total_fee_lock_amount.is_zero(), Error::<T>::NotFeeLocked);

		let fee_lock_metadata =
			Self::get_fee_lock_metadata().ok_or(Error::<T>::FeeLocksNotInitialized)?;

		let now = <frame_system::Pallet<T>>::block_number();

		ensure!(
			now >= account_fee_lock_data
				.last_fee_lock_block
				.saturating_add(fee_lock_metadata.period_length),
			Error::<T>::CantUnlockFeeYet
		);

		let unreserve_result = <T as pallet::Config>::Tokens::unreserve(
			<T as pallet::Config>::NativeTokenId::get().into(),
			&who,
			account_fee_lock_data.total_fee_lock_amount,
		);
		if !unreserve_result.is_zero() {
			log::warn!(
				"Unlock lock unreserve resulted in non-zero unreserve_result {:?}",
				unreserve_result
			);
		}

		if let Some(pos) = FeeLockMetadataQeueuePosition::<T>::take(&who) {
			UnlockQueue::<T>::take(pos);
		}
		AccountFeeLockData::<T>::remove(&who);

		Self::deposit_event(Event::FeeLockUnlocked(
			who.clone(),
			account_fee_lock_data.total_fee_lock_amount,
		));

		Ok(())
	}
}

pub struct FeeLockWeightProvider<T>(PhantomData<T>);

impl<T: Config> Get<Weight> for FeeLockWeightProvider<T> {
	fn get() -> Weight {
		// We assume that process_fee_lock is heavier than unlock_fee
		// The FeeLockMetadata read is not accounted for since it is called no matter the extrinsic and hence would be accounted for in the ExtrinsicBaseWeight
		T::WeightInfo::process_fee_lock()
			.saturating_add(T::WeightInfo::get_swap_valuation_for_token())
			.saturating_add(Weight::from_parts(40_000_000, 0))
	}
}
