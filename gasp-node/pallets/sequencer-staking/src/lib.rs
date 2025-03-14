#![allow(non_camel_case_types)]
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, Get, ReservableCurrency, StorageVersion},
	transactional,
};
use frame_system::{ensure_signed, pallet_prelude::*};
use sp_std::collections::btree_map::BTreeMap;

use sp_std::{collections::btree_set::BTreeSet, convert::TryInto, prelude::*};

pub use mangata_support::traits::{
	ComputeIssuance, GetIssuance, RolldownProviderTrait, SequencerStakingProviderTrait,
	SequencerStakingRewardsTrait,
};
use sp_runtime::{
	traits::{CheckedAdd, One, Zero},
	Saturating,
};
pub use sp_runtime::{BoundedBTreeMap, BoundedVec, Perbill};

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type SequencerIndex = u32;
type RoundIndex = u32;
type RewardPoint = u32;
type RoundRefCount = RoundIndex;

pub type BalanceOf<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type ChainIdOf<T> = <T as pallet::Config>::ChainId;

pub(crate) const LOG_TARGET: &'static str = "sequencer-staking";

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

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

pub use pallet::*;

#[derive(Eq, PartialEq, Encode, Decode, TypeInfo, Debug, Clone)]
pub enum PayoutRounds {
	All,
	Partial(Vec<RoundIndex>),
}

#[derive(Eq, PartialEq, RuntimeDebug, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum StakeAction {
	StakeOnly,
	StakeAndJoinActiveSet,
}

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub minimal_stake_amount: BalanceOf<T>,
		pub slash_fine_amount: BalanceOf<T>,
		pub sequencers_stake: Vec<(T::AccountId, T::ChainId, BalanceOf<T>)>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig {
				minimal_stake_amount: Default::default(),
				slash_fine_amount: Default::default(),
				sequencers_stake: vec![],
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		// Since this pallet is to be configured above session in construct_runtime!
		// We can't check that these sequencers are eligible
		// We can have that check in initialize_genesis_eligible_sequencers
		// but for now it is not implemented. This is fine and doesn't really
		// break anything. However, if these sequencer join the eligible set and
		// then leave then ofcourse they will be removed from the active set too.
		fn build(&self) {
			MinimalStakeAmount::<T>::put(self.minimal_stake_amount);
			SlashFineAmount::<T>::put(self.slash_fine_amount);

			for (sender, chain, stake_amount) in self.sequencers_stake.iter() {
				assert!(!Pallet::<T>::is_active_sequencer(*chain, &sender));
				assert!(stake_amount >= &MinimalStakeAmount::<T>::get());

				<SequencerStake<T>>::insert((sender, &chain), stake_amount);

				ActiveSequencers::<T>::try_mutate(|active_sequencers| {
					active_sequencers.entry(*chain).or_default().try_push(sender.clone())
				})
				.expect("setting active sequencers is successfull");

				Pallet::<T>::announce_sequencer_joined_active_set(*chain, sender.clone());

				// add full rights to sequencer (create whole entry in SequencersRights @ rolldown)
				// add +1 cancel right to all other sequencers (non active are deleted from SequencersRights @ rolldown)
				assert!(T::Currency::reserve(&sender, *stake_amount).is_ok());
			}

			for chain in
				self.sequencers_stake.iter().map(|(_, chain, _)| chain).collect::<BTreeSet<_>>()
			{
				if let Some(seq) = ActiveSequencers::<T>::get()
					.get(chain)
					.and_then(|sequencers| sequencers.first())
				{
					SelectedSequencer::<T>::mutate(|selected| selected.insert(*chain, seq.clone()));
				}
			}

			<CurrentRound<T>>::put(0);
		}
	}

	// The pallet needs to be configured above session in construct_runtime!
	// to work correctly with the on_initialize hook and the NextSequencerIndex updates
	// This requirement may have changed due later edits (we now are doing all processing in on_finalize)
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			T::DbWeight::get().reads_writes(4, 3)
		}

		fn on_finalize(n: BlockNumberFor<T>) -> () {
			let active_sequencers = ActiveSequencers::<T>::get();
			// let active_chains = active_sequencers.keys().collect()::<BTreeSet<_>>();
			if !(n % T::BlocksForSequencerUpdate::get().into()).is_zero() {
				return
			}

			NextSequencerIndex::<T>::mutate(|idxs| {
				for (chain, set) in active_sequencers.iter() {
					idxs.entry(*chain)
						.and_modify(|v| {
							if *v > set.len() as u32 {
								*v = SequencerIndex::zero();
							}
						})
						.or_insert(SequencerIndex::zero());
				}
				idxs.retain(|chain, _seq| active_sequencers.contains_key(chain));
			});

			SelectedSequencer::<T>::mutate(|selected| {
				for (chain, set) in active_sequencers.iter() {
					if let Some(index) = NextSequencerIndex::<T>::get().get(chain) {
						if let Some(selected_seq) = set.iter().skip(*index as usize).next() {
							selected.insert(*chain, selected_seq.clone());
						}
					}
				}
				selected.retain(|chain, _seq| active_sequencers.contains_key(chain));
			});

			NextSequencerIndex::<T>::mutate(|indexes_set| {
				indexes_set.iter_mut().for_each(|(_chain, idx)| {
					*idx = idx.saturating_add(One::one());
				})
			});
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn get_sequencer_stake)]
	pub type SequencerStake<T: Config> =
		StorageMap<_, Blake2_128Concat, (AccountIdOf<T>, T::ChainId), BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	pub type AliasAccount<T: Config> =
		StorageMap<_, Blake2_128Concat, (AccountIdOf<T>, T::ChainId), AccountIdOf<T>, OptionQuery>;

	#[pallet::storage]
	pub type AliasAccountInUse<T: Config> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, (), OptionQuery>;

	// #[pallet::storage]
	// #[pallet::unbounded]
	// #[pallet::getter(fn get_eligible_to_be_sequencers)]
	// pub type EligibleToBeSequencers<T: Config> =
	// 	StorageValue<_, BTreeMap<AccountIdOf<T>, RoundRefCount>, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_active_sequencers)]
	pub type ActiveSequencers<T: Config> = StorageValue<
		_,
		BTreeMap<T::ChainId, BoundedVec<AccountIdOf<T>, T::MaxSequencers>>,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_selected_sequencer)]
	#[pallet::unbounded]
	pub type SelectedSequencer<T: Config> =
		StorageValue<_, BTreeMap<T::ChainId, T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_current_round)]
	pub type CurrentRound<T: Config> = StorageValue<_, RoundIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_next_sequencer_index)]
	pub type NextSequencerIndex<T: Config> =
		StorageValue<_, BTreeMap<T::ChainId, SequencerIndex>, ValueQuery>;

	// #[pallet::storage]
	// #[pallet::unbounded]
	// #[pallet::getter(fn get_round_collators)]
	// pub type RoundCollators<T: Config> =
	// 	StorageMap<_, Blake2_128Concat, RoundIndex, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_slash_fine_amount)]
	pub type SlashFineAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_minimal_stake_amount)]
	pub type MinimalStakeAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_round_collator_reward_info)]
	/// Stores information about rewards per each session
	pub type RoundSequencerRewardInfo<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Twox64Concat,
		RoundIndex,
		BalanceOf<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn points)]
	/// Total points awarded to collators for block production in the round
	pub type Points<T: Config> = StorageMap<_, Twox64Concat, RoundIndex, RewardPoint, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn awarded_pts)]
	/// Points for each collator per round
	pub type AwardedPts<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		RoundIndex,
		Twox64Concat,
		T::AccountId,
		RewardPoint,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SequencersRemovedFromActiveSet(T::ChainId, Vec<T::AccountId>),
		SequencerJoinedActiveSet(T::ChainId, T::AccountId),
		StakeProvided {
			chain: T::ChainId,
			added_stake: BalanceOf<T>,
			total_stake: BalanceOf<T>,
		},
		StakeRemoved {
			chain: T::ChainId,
			removed_stake: BalanceOf<T>,
		},
		/// Notify about reward periods that has been paid (sequencer, payout rounds, any rewards left)
		SequencerRewardsDistributed(T::AccountId, PayoutRounds),
		/// Paid the account the balance as liquid rewards
		Rewarded(RoundIndex, T::AccountId, BalanceOf<T>),
	}

	#[pallet::error]
	/// Errors
	pub enum Error<T> {
		OperationFailed,
		MathOverflow,
		SequencerIsNotInActiveSet,
		SequencerAlreadyInActiveSet,
		CantUnstakeWhileInActiveSet,
		// NotEligibleToBeSequencer,
		NotEnoughSequencerStake,
		MaxSequencersLimitReached,
		TestUnstakingError,
		UnknownChainId,
		NoStakeToUnStake,
		AddressInUse,
		AliasAccountIsActiveSequencer,
		SequencerAccountIsActiveSequencerAlias,
		SequencerRoundRewardsDNE,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The currency type
		type Currency: ReservableCurrency<Self::AccountId>;
		#[pallet::constant]
		type MinimumSequencers: Get<u32>;
		type RolldownProvider: RolldownProviderTrait<Self::ChainId, Self::AccountId>;
		type ChainId: Parameter
			+ Member
			+ Default
			+ TypeInfo
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ PartialOrd
			+ codec::Decode
			+ codec::Encode
			+ Ord
			+ Copy;
		#[pallet::constant]
		type NoOfPastSessionsForEligibility: Get<u32>;
		#[pallet::constant]
		// TODO: rename 'PerChain'
		type MaxSequencers: Get<u32>;
		#[pallet::constant]
		type BlocksForSequencerUpdate: Get<u32>;
		#[pallet::constant]
		type CancellerRewardPercentage: Get<sp_runtime::Permill>;
		#[pallet::constant]
		type DefaultPayoutLimit: Get<u32>;
		/// The account id that holds the sequencer issuance
		type SequencerIssuanceVault: Get<Self::AccountId>;
		/// Number of rounds after which block authors are rewarded
		#[pallet::constant]
		type RewardPaymentDelay: Get<RoundIndex>;
		/// The module used for computing and getting issuance
		type Issuance: ComputeIssuance + GetIssuance<BalanceOf<Self>>;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::DbWeight::get().reads_writes(5, 5.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		/// provides stake for the purpose of becoming sequencers
		///
		/// - `chain` - chain for which to assign stake_amount
		/// - `stake_amont` - amount of stake
		/// - `alias_account` - optional parameter, alias account is eligible to create manual bataches
		///                     of updates in pallet-rolldown. Alias account can not be set to another
		///                     active sequencer or to some account that is already used as
		///                     alias_account for another sequencer
		/// - `stake_action` - determines what are candidate expectations regarding joining active set,
		/// 	* 'StakeOnly' - sequencer only provides stake, but does not join active set.
		/// 	* 'StakeAndJoinActiveSet' - sequencer provides stake and joins active set. Fails if
		/// 								candidate didnt join active set or if candidate is already in active set.
		///		Candiate can also choose to call `rejoin_active_sequencers` later when there are free seats to
		///		join active set
		pub fn provide_sequencer_stake(
			origin: OriginFor<T>,
			chain: T::ChainId,
			stake_amount: BalanceOf<T>,
			alias_account: Option<T::AccountId>,
			stake_action: StakeAction,
			sender: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;

			ensure!(
				!AliasAccountInUse::<T>::contains_key(sender.clone()),
				Error::<T>::SequencerAccountIsActiveSequencerAlias
			);

			let sequencer_stake = <SequencerStake<T>>::try_mutate(
				(&sender, &chain),
				|stake| -> Result<BalanceOf<T>, Error<T>> {
					*stake = stake.checked_add(&stake_amount).ok_or(Error::<T>::MathOverflow)?;
					Ok(*stake)
				},
			)?;

			if let StakeAction::StakeAndJoinActiveSet = stake_action {
				ensure!(
					!Self::is_active_sequencer(chain, &sender),
					Error::<T>::SequencerAlreadyInActiveSet
				);
				ensure!(
					sequencer_stake >= MinimalStakeAmount::<T>::get(),
					Error::<T>::NotEnoughSequencerStake
				);
				ActiveSequencers::<T>::try_mutate(|active_sequencers| {
					active_sequencers
						.entry(chain)
						.or_default()
						.try_push(sender.clone())
						.or(Err(Error::<T>::MaxSequencersLimitReached))
				})?;
				Self::announce_sequencer_joined_active_set(chain, sender.clone());
			};

			if let Some(alias_account) = alias_account {
				ensure!(
					!AliasAccountInUse::<T>::contains_key(alias_account.clone()),
					Error::<T>::AddressInUse
				);
				ensure!(
					!Self::is_active_sequencer(chain, &alias_account),
					Error::<T>::AliasAccountIsActiveSequencer
				);

				if let Some(prev_alias) = AliasAccount::<T>::take((sender.clone(), chain)) {
					AliasAccountInUse::<T>::remove(prev_alias);
				}

				AliasAccount::<T>::insert((sender.clone(), chain), alias_account.clone());
				AliasAccountInUse::<T>::insert(alias_account.clone(), ());
			}

			// add full rights to sequencer (create whole entry in SequencersRights @ rolldown)
			// add +1 cancel right to all other sequencers (non active are deleted from SequencersRights @ rolldown)
			T::Currency::reserve(&sender, stake_amount)?;

			Self::deposit_event(Event::StakeProvided {
				chain,
				added_stake: stake_amount,
				total_stake: sequencer_stake,
			});

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::DbWeight::get().reads_writes(2, 3.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn leave_active_sequencers(
			origin: OriginFor<T>,
			chain: T::ChainId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(
				Self::is_active_sequencer(chain, &sender),
				Error::<T>::SequencerIsNotInActiveSet
			);

			Self::remove_sequencers_from_active_set(chain, sp_std::iter::once(sender).collect());

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::DbWeight::get().reads_writes(3, 3.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn rejoin_active_sequencers(
			origin: OriginFor<T>,
			chain: T::ChainId,
			sender: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;
			ensure!(
				!Self::is_active_sequencer(chain, &sender),
				Error::<T>::SequencerAlreadyInActiveSet
			);
			ensure!(
				ActiveSequencers::<T>::get().get(&chain).unwrap_or(&Default::default()).len() <
					T::MaxSequencers::get() as usize,
				Error::<T>::MaxSequencersLimitReached
			);
			ensure!(
				SequencerStake::<T>::get((&sender, &chain)) >= MinimalStakeAmount::<T>::get(),
				Error::<T>::NotEnoughSequencerStake
			);

			ActiveSequencers::<T>::try_mutate(|active_sequencers| {
				active_sequencers.entry(chain).or_default().try_push(sender.clone())
			})
			.or(Err(Error::<T>::MaxSequencersLimitReached))?;

			Pallet::<T>::deposit_event(Event::SequencerJoinedActiveSet(chain, sender.clone()));
			Self::announce_sequencer_joined_active_set(chain, sender.clone());

			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::DbWeight::get().reads_writes(6, 6).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn unstake(origin: OriginFor<T>, chain: T::ChainId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(
				!Self::is_active_sequencer(chain, &sender),
				Error::<T>::CantUnstakeWhileInActiveSet
			);
			let sequencer_stake = SequencerStake::<T>::get((&sender, &chain));
			ensure!(!sequencer_stake.is_zero(), Error::<T>::NoStakeToUnStake);

			T::RolldownProvider::sequencer_unstaking(chain, &sender)?;

			let unreserve_remaining = T::Currency::unreserve(&sender, sequencer_stake);
			if !unreserve_remaining.is_zero() {
				log!(error, "unstake unreserve_remaining is non-zero - sender {:?}, sequencer {:?}, unreserve_remaining {:?}", &sender, sequencer_stake, unreserve_remaining);
			}

			SequencerStake::<T>::remove((&sender, &chain));

			Self::deposit_event(Event::StakeRemoved { chain, removed_stake: sequencer_stake });
			Ok(().into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::DbWeight::get().reads_writes(2.saturating_add(T::MaxSequencers::get().into()), 5.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn set_sequencer_configuration(
			origin: OriginFor<T>,
			chain: T::ChainId,
			minimal_stake_amount: BalanceOf<T>,
			slash_fine_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let _ = ensure_root(origin)?;

			<MinimalStakeAmount<T>>::put(minimal_stake_amount);
			<SlashFineAmount<T>>::put(slash_fine_amount);

			let active_sequencers = ActiveSequencers::<T>::get();
			let deactivating_sequencers = active_sequencers
				.get(&chain)
				.ok_or(Error::<T>::UnknownChainId)?
				.clone()
				.into_inner()
				.into_iter()
				.filter(|s| SequencerStake::<T>::get((&s, &chain)) < minimal_stake_amount)
				.collect::<_>();

			Pallet::<T>::remove_sequencers_from_active_set(chain, deactivating_sequencers);

			Ok(().into())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::DbWeight::get().reads_writes(2.saturating_add(T::MaxSequencers::get().into()), 5.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		/// Allows to configure alias_account for active sequencer. This extrinisic can only be called
		/// by active sequencer
		/// - `chain` -
		/// - `alias_account` - optional parameter, alias account is eligible to create manual bataches
		///                     of updates in pallet-rolldown. Alias account can not be set to another
		///                     active sequencer or to some account that is already used as
		///                     alias_account for another sequencer
		pub fn set_updater_account_for_sequencer(
			origin: OriginFor<T>,
			chain: T::ChainId,
			alias_account: Option<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let sequencer = ensure_signed(origin)?;

			ensure!(
				Self::is_active_sequencer(chain, &sequencer),
				Error::<T>::SequencerIsNotInActiveSet
			);

			if let Some(alias) = alias_account {
				ensure!(
					!Self::is_active_sequencer(chain, &alias),
					Error::<T>::AliasAccountIsActiveSequencer
				);
				ensure!(
					!AliasAccountInUse::<T>::contains_key(alias.clone()),
					Error::<T>::AddressInUse
				);
				AliasAccount::<T>::insert((sequencer.clone(), chain), alias.clone());
				AliasAccountInUse::<T>::insert(alias, ());
			} else {
				if let Some(alias) = AliasAccount::<T>::take((sequencer, chain)) {
					AliasAccountInUse::<T>::remove(alias);
				}
			}

			Ok(().into())
		}

		/// This extrinsic should be used to distribute rewards for sequencer.
		///
		/// params:
		/// - sequencer - account id
		/// - number_of_sessions - number of rewards periods that should be processed within extrinsic.
		#[pallet::call_index(6)]
		#[pallet::weight(number_of_sessions.unwrap_or(T::DefaultPayoutLimit::get()) * (T::DbWeight::get().reads_writes(3, 3).saturating_add(Weight::from_parts(40_000_000, 0))))]
		#[transactional]
		pub fn payout_sequencer_rewards(
			origin: OriginFor<T>,
			sequencer: T::AccountId,
			number_of_sessions: Option<u32>,
		) -> DispatchResultWithPostInfo {
			let _caller = ensure_signed(origin)?;
			Self::do_payout_sequencer_rewards(sequencer, number_of_sessions)
		}
	}
}

impl<T: Config> Pallet<T> {
	fn announce_sequencer_joined_active_set(chain: T::ChainId, sequencer: T::AccountId) {
		T::RolldownProvider::new_sequencer_active(chain, &sequencer);
		Pallet::<T>::deposit_event(Event::<T>::SequencerJoinedActiveSet(chain, sequencer));
	}

	fn announce_sequencers_removed_from_active_set(
		chain: T::ChainId,
		sequencers: Vec<T::AccountId>,
	) {
		T::RolldownProvider::handle_sequencer_deactivations(chain, sequencers.clone());
		Pallet::<T>::deposit_event(Event::<T>::SequencersRemovedFromActiveSet(chain, sequencers));
	}

	fn maybe_remove_sequencer_from_active_set(chain: T::ChainId, sequencer: T::AccountId) {
		if <SequencerStake<T>>::get((sequencer.clone(), chain)) < MinimalStakeAmount::<T>::get() &&
			Self::is_active_sequencer(chain, &sequencer)
		{
			Self::remove_sequencers_from_active_set(chain, [sequencer].iter().cloned().collect());
		}
	}

	// Caller should check if the sequencer is infact in the active set
	fn remove_sequencers_from_active_set(
		chain: T::ChainId,
		deactivating_sequencers: BTreeSet<T::AccountId>,
	) {
		if deactivating_sequencers.is_empty() {
			return
		}

		let active_seqs = ActiveSequencers::<T>::get();
		let seqs_per_chain = active_seqs.get(&chain).cloned().unwrap_or_default();
		let active_set: &[AccountIdOf<T>] = seqs_per_chain.as_ref();

		NextSequencerIndex::<T>::mutate(|idxs| {
			if let Some(next_pos) = idxs.get_mut(&chain) {
				let shift = deactivating_sequencers
					.iter()
					.filter_map(|seq| active_set.iter().position(|elem| elem == seq))
					.into_iter()
					.filter(|pos| *pos < *next_pos as usize)
					.count();
				if shift > 0 {
					*next_pos = next_pos.saturating_sub(shift as u32);
				}
			}
		});

		ActiveSequencers::<T>::mutate(|active_set| {
			if let Some(set) = active_set.get_mut(&chain) {
				set.retain(|elem| !deactivating_sequencers.contains(elem))
			}
		});

		SelectedSequencer::<T>::mutate(|selected| {
			if matches!(
				selected.get(&chain),
				Some(elem) if deactivating_sequencers.contains(elem))
			{
				selected.remove(&chain);
			}
		});

		Self::announce_sequencers_removed_from_active_set(
			chain,
			deactivating_sequencers.iter().cloned().collect(),
		);
	}

	#[cfg(test)]
	fn set_active_sequencers(
		iter: impl IntoIterator<Item = (T::ChainId, T::AccountId)>,
	) -> Result<(), Error<T>> {
		ActiveSequencers::<T>::kill();
		ActiveSequencers::<T>::try_mutate(|active_sequencers| {
			for (chain, seq) in iter.into_iter() {
				active_sequencers
					.entry(chain)
					.or_default()
					.try_push(seq)
					.or(Err(Error::<T>::MaxSequencersLimitReached))?;
			}
			Ok::<_, Error<T>>(())
		})?;
		Ok(())
	}

	fn do_payout_sequencer_rewards(
		sequencer: T::AccountId,
		number_of_sessions: Option<u32>,
	) -> DispatchResultWithPostInfo {
		let mut rounds = Vec::<RoundIndex>::new();

		let limit = number_of_sessions.unwrap_or(T::DefaultPayoutLimit::get());
		for (id, (round, reward)) in
			RoundSequencerRewardInfo::<T>::iter_prefix(sequencer.clone()).enumerate()
		{
			if (id as u32) >= limit {
				break
			}

			Self::payout_reward(round, sequencer.clone(), reward)?;

			RoundSequencerRewardInfo::<T>::remove(sequencer.clone(), round);
			rounds.push(round);
		}

		ensure!(!rounds.is_empty(), Error::<T>::SequencerRoundRewardsDNE);

		if let Some(_) = RoundSequencerRewardInfo::<T>::iter_prefix(sequencer.clone()).next() {
			Self::deposit_event(Event::SequencerRewardsDistributed(
				sequencer,
				PayoutRounds::Partial(rounds),
			));
		} else {
			Self::deposit_event(Event::SequencerRewardsDistributed(sequencer, PayoutRounds::All));
		}

		Ok(().into())
	}

	pub fn payout_reward(round: RoundIndex, to: T::AccountId, amt: BalanceOf<T>) -> DispatchResult {
		let _ = <T as pallet::Config>::Currency::transfer(
			&<T as pallet::Config>::SequencerIssuanceVault::get(),
			&to,
			amt,
			ExistenceRequirement::AllowDeath,
		)?;
		Self::deposit_event(Event::Rewarded(round, to.clone(), amt));
		Ok(())
	}
}

impl<T: Config> SequencerStakingProviderTrait<AccountIdOf<T>, BalanceOf<T>, ChainIdOf<T>>
	for Pallet<T>
{
	fn is_active_sequencer(
		chain: <T as pallet::Config>::ChainId,
		sequencer: &AccountIdOf<T>,
	) -> bool {
		matches!(
			ActiveSequencers::<T>::get().get(&chain), Some(set) if set.contains(&sequencer)
		)
	}

	fn is_active_sequencer_alias(
		chain: <T as pallet::Config>::ChainId,
		sequencer_account: &AccountIdOf<T>,
		alias_account: &AccountIdOf<T>,
	) -> bool {
		matches!(
			AliasAccount::<T>::get((sequencer_account, chain)),
			Some(alias) if alias == *alias_account
		)
	}

	fn is_selected_sequencer(chain: ChainIdOf<T>, sequencer: &AccountIdOf<T>) -> bool {
		matches!(
			SelectedSequencer::<T>::get().get(&chain),
			Some(selected) if selected == sequencer
		)
	}

	fn slash_sequencer(
		chain: ChainIdOf<T>,
		to_be_slashed: &T::AccountId,
		maybe_to_reward: Option<&T::AccountId>,
	) -> DispatchResult {
		// Use slashed amount partially to reward canceler, partially to vault to pay for l1 fees
		let maybe_leaving_sequencer = to_be_slashed.clone();
		<SequencerStake<T>>::mutate((to_be_slashed, chain), |stake| -> DispatchResult {
			let slash_fine_amount = SlashFineAmount::<T>::get();
			let slash_fine_amount_actual = (*stake).min(slash_fine_amount);
			*stake = stake.saturating_sub(slash_fine_amount_actual);
			let mut burned_amount = slash_fine_amount_actual;
			if let Some(to_reward) = maybe_to_reward {
				let mut repatriate_amount = T::CancellerRewardPercentage::get() * slash_fine_amount; // this raw * is safe since result is a fraction of input
				repatriate_amount = repatriate_amount.min(slash_fine_amount_actual);
				burned_amount = slash_fine_amount_actual.saturating_sub(repatriate_amount);
				let _ = T::Currency::repatriate_reserved(
					to_be_slashed,
					to_reward,
					repatriate_amount,
					frame_support::traits::BalanceStatus::Free,
				);
			}
			let _ = T::Currency::slash_reserved(to_be_slashed, burned_amount);
			Ok(())
		});

		Self::maybe_remove_sequencer_from_active_set(chain, maybe_leaving_sequencer);

		Ok(().into())
	}

	fn selected_sequencer(chain: ChainIdOf<T>) -> Option<AccountIdOf<T>> {
		SelectedSequencer::<T>::get().get(&chain).cloned()
	}
}

impl<T: Config> SequencerStakingRewardsTrait<AccountIdOf<T>, RoundIndex> for Pallet<T> {
	fn note_update_author(author: &AccountIdOf<T>) {
		let now = <CurrentRound<T>>::get();
		let score_plus_20 = <AwardedPts<T>>::get(now, &author).saturating_add(20);
		<AwardedPts<T>>::insert(now, author, score_plus_20);
		<Points<T>>::mutate(now, |x| *x = x.saturating_add(20));
	}
	fn pay_sequencers(round: RoundIndex) {
		<CurrentRound<T>>::put(round);
		// payout is now - duration rounds ago => now - duration > 0 else return early
		let duration = T::RewardPaymentDelay::get();
		if round < duration {
			return
		}
		let round_to_payout = round.saturating_sub(duration);
		let total = <Points<T>>::take(round_to_payout);
		if total.is_zero() {
			return
		}
		let total_issuance =
			T::Issuance::get_sequencer_issuance(round_to_payout).unwrap_or(Zero::zero());

		for (author, pts) in <AwardedPts<T>>::drain_prefix(round_to_payout) {
			let author_issuance_perbill = Perbill::from_rational(pts, total);
			let author_rewards = author_issuance_perbill.mul_floor(total_issuance);

			RoundSequencerRewardInfo::<T>::insert(author, round_to_payout, author_rewards);
		}
	}
}

/// Simple ensure origin struct to filter for the active sequencer accounts.
pub struct EnsureActiveSequencer<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> EnsureOrigin<<T as frame_system::Config>::RuntimeOrigin>
	for EnsureActiveSequencer<T>
{
	type Success = T::AccountId;
	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		o.into().and_then(|o| match o {
			frame_system::RawOrigin::Signed(ref who)
				if <Pallet<T> as SequencerStakingProviderTrait<
					AccountIdOf<T>,
					BalanceOf<T>,
					ChainIdOf<T>,
				>>::is_active_sequencer(todo!(), &who) =>
				Ok(who.clone()),
			r => Err(T::RuntimeOrigin::from(r)),
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		unimplemented!();
		// let founder = Founder::<T>::get().ok_or(())?;
		// Ok(T::RuntimeOrigin::from(frame_system::RawOrigin::Signed(founder)))
	}
}
