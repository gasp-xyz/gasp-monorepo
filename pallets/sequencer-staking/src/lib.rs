#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{
		Currency, DefensiveSaturating, Get, OneSessionHandler, ReservableCurrency, StorageVersion,
	},
};
use frame_system::{ensure_signed, pallet_prelude::*};
use sp_std::collections::btree_map::BTreeMap;

use sp_std::{convert::TryInto, prelude::*};
use sp_std::collections::btree_set::BTreeSet;

use codec::alloc::string::{String, ToString};
use mangata_types::ChainId;
pub use mangata_support::traits::{
	InformSessionDataTrait, RolldownProviderTrait, SequencerStakingProviderTrait,
};
use scale_info::prelude::format;
use sp_core::U256;
use sp_runtime::{
	serde::{Deserialize, Serialize},
	traits::{CheckedAdd, One, Zero},
	RuntimeAppPublic, Saturating,
};
use sp_std::collections::btree_map::Entry::{Occupied, Vacant};
pub use sp_runtime::{BoundedVec, BoundedBTreeMap};

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
type SequencerIndex = u32;
type RoundIndex = u32;
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

#[frame_support::pallet]
pub mod pallet {

	use frame_support::BoundedBTreeSet;

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
				//TODO: fix other way
				// assert!(ActiveSequencers::<T>::get().len() < T::MaxSequencers::get() as usize);
				assert!(stake_amount >= &MinimalStakeAmount::<T>::get());

				<SequencerStake<T>>::insert((sender, &chain), stake_amount);

				ActiveSequencers::<T>::try_mutate(|active_sequencers| {
					active_sequencers
						.get_mut(chain)
						.and_then(|sequencers| sequencers.try_insert(sender.clone()).ok())
						.ok_or(Error::<T>::MaxSequencersLimitReached)
				}).expect("setting active sequencers is successfull");

				T::RolldownProvider::new_sequencer_active(&sender);

				// add full rights to sequencer (create whole entry in sequencer_rights @ rolldown)
				// add +1 cancel right to all other sequencers (non active are deleted from sequencer_rights @ rolldown)
				assert!(T::Currency::reserve(&sender, *stake_amount).is_ok());
			}


			for chain  in self.sequencers_stake.iter().map(|(_,chain,_)| chain).collect::<BTreeSet<_>>() {
				if let Some(seq) = ActiveSequencers::<T>::get().get(chain).and_then(|sequencers| sequencers.first()) {
					SelectedSequencer::<T>::mutate(|selected| {
						selected.insert(*chain, seq.clone())
					});
				}
			}
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
			//TODO: uncomment
			let active_sequencers = ActiveSequencers::<T>::get();
			if (n % T::BlocksForSequencerUpdate::get().into()).is_zero() {

				SelectedSequencer::<T>::mutate(|selected| {
					for (chain, set) in active_sequencers.iter() {
						if let Some(selected_seq) = set.iter().cycle().skip(NextSequencerIndex::<T>::get() as usize).next(){
							selected.insert(*chain, selected_seq.clone());
						}else{
							selected.remove(chain);
						}
					}
				});

				NextSequencerIndex::<T>::mutate(|idx| {
					let next_index = idx.saturating_add(One::one());
					if idx.saturating_add(One::one()) > T::MaxSequencers::get() {
						*idx = SequencerIndex::zero();
					}else{
						*idx = next_index;
					}
				});

			}
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn get_sequencer_stake)]
	pub type SequencerStake<T: Config> =
		StorageMap<_, Blake2_128Concat, (AccountIdOf<T>, T::ChainId), BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_eligible_to_be_sequencers)]
	pub type EligibleToBeSequencers<T: Config> =
		StorageValue<_, BTreeMap<AccountIdOf<T>, RoundRefCount>, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_active_sequencers)]
	pub type ActiveSequencers<T: Config> = StorageValue<_, BTreeMap<T::ChainId, BoundedBTreeSet<AccountIdOf<T>,T::MaxSequencers>> , ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_selected_sequencer)]
	#[pallet::unbounded]
	pub type SelectedSequencer<T: Config> = StorageValue<_, BTreeMap<T::ChainId, T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_current_round)]
	pub type CurrentRound<T: Config> = StorageValue<_, RoundIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_next_sequencer_index)]
	pub type NextSequencerIndex<T: Config> = StorageValue<_, SequencerIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_round_collators)]
	pub type RoundCollators<T: Config> =
		StorageMap<_, Blake2_128Concat, RoundIndex, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_slash_fine_amount)]
	pub type SlashFineAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn get_minimal_stake_amount)]
	pub type MinimalStakeAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	/// Errors
	pub enum Error<T> {
		OperationFailed,
		MathOverflow,
		SequencerIsNotInActiveSet,
		SequencerAlreadyInActiveSet,
		CantUnstakeWhileInActiveSet,
		NotEligibleToBeSequencer,
		NotEnoughSequencerStake,
		MaxSequencersLimitReached,
		TestUnstakingError,
		UnknownChainId,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: ReservableCurrency<Self::AccountId>;
		#[pallet::constant]
		type MinimumSequencers: Get<u32>;
		type RolldownProvider: RolldownProviderTrait<Self::AccountId>;
		type ChainId: Parameter + Member + Default + TypeInfo + MaybeSerializeDeserialize + MaxEncodedLen + PartialOrd + codec::Decode + codec::Encode + Ord + Copy;
		#[pallet::constant]
		type NoOfPastSessionsForEligibility: Get<u32>;
		#[pallet::constant]
		// TODO: rename 'PerChain'
		type MaxSequencers: Get<u32>;
		#[pallet::constant]
		type BlocksForSequencerUpdate: Get<u32>;
		#[pallet::constant]
		type CancellerRewardPercentage: Get<sp_runtime::Permill>;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::DbWeight::get().reads_writes(5, 5.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn provide_sequencer_stake(
			origin: OriginFor<T>,
			chain: T::ChainId,
			stake_amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			<SequencerStake<T>>::try_mutate((&sender, &chain), |stake| -> DispatchResult {
				*stake = stake.checked_add(&stake_amount).ok_or(Error::<T>::MathOverflow)?;
				if *stake >= MinimalStakeAmount::<T>::get() &&
					!Self::is_active_sequencer(chain, &sender) &&
					Self::is_eligible_to_be_sequencer(&sender)
				{
					ActiveSequencers::<T>::try_mutate(|active_sequencers| {
						active_sequencers
							.entry(chain)
							.or_default()
							.try_insert(sender.clone())
							.or(Err(Error::<T>::MaxSequencersLimitReached))
					})?;
					T::RolldownProvider::new_sequencer_active(&sender);
					Ok(())
				}else{
					Ok(())
				}
			})?;

			// add full rights to sequencer (create whole entry in sequencer_rights @ rolldown)
			// add +1 cancel right to all other sequencers (non active are deleted from sequencer_rights @ rolldown)
			T::Currency::reserve(&sender, stake_amount)?;

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::DbWeight::get().reads_writes(2, 3.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn leave_active_sequencers(origin: OriginFor<T>, chain: T::ChainId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Self::is_active_sequencer(chain, &sender), Error::<T>::SequencerIsNotInActiveSet);

			//TODO: uncomment
			// Self::remove_sequencers_from_active_set(vec![sender]);

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::DbWeight::get().reads_writes(3, 3.saturating_add(T::MaxSequencers::get().into())).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn rejoin_active_sequencers(origin: OriginFor<T>, chain: T::ChainId) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Self::is_active_sequencer(chain, &sender), Error::<T>::SequencerAlreadyInActiveSet);
			ensure!(
				ActiveSequencers::<T>::get().len() < T::MaxSequencers::get() as usize,
				Error::<T>::MaxSequencersLimitReached
			);
			ensure!(
				SequencerStake::<T>::get((&sender, &chain)) >= MinimalStakeAmount::<T>::get(),
				Error::<T>::NotEnoughSequencerStake
			);
			ensure!(
				Self::is_eligible_to_be_sequencer(&sender),
				Error::<T>::NotEligibleToBeSequencer
			);

			ActiveSequencers::<T>::try_mutate(|active_sequencers| {
				active_sequencers
					.entry(chain)
					.or_default()
					.try_insert(sender.clone())
			}).or(Err(Error::<T>::MaxSequencersLimitReached))?;

			T::RolldownProvider::new_sequencer_active(&sender);

			Ok(().into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::DbWeight::get().reads_writes(6, 6).saturating_add(Weight::from_parts(40_000_000, 0)))]
		pub fn unstake(origin: OriginFor<T>,
			chain: T::ChainId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Self::is_active_sequencer(chain, &sender), Error::<T>::CantUnstakeWhileInActiveSet);

			T::RolldownProvider::sequencer_unstaking(&sender)?;

			let sequencer_stake = SequencerStake::<T>::get((&sender, &chain));
			let unreserve_remaining = T::Currency::unreserve(&sender, sequencer_stake);
			if !unreserve_remaining.is_zero() {
				log!(error, "unstake unreserve_remaining is non-zero - sender {:?}, sequencer {:?}, unreserve_remaining {:?}", &sender, sequencer_stake, unreserve_remaining);
			}

			SequencerStake::<T>::remove((&sender,&chain));
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
			// TODO: uncomment
			let deactivating_sequencers = active_sequencers
				.get(&chain)
				.ok_or(Error::<T>::UnknownChainId)?
				.clone()
				.into_inner()
				.into_iter()
				.filter_map(|s| {
					if SequencerStake::<T>::get((&s,&chain)) < minimal_stake_amount{
						Some((chain,s))
					}else{
						None
					}
				})
				.collect::<_>();

			Pallet::<T>::remove_sequencers_from_active_set(deactivating_sequencers);

			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn is_eligible_to_be_sequencer(account: &T::AccountId) -> bool {
		EligibleToBeSequencers::<T>::get().contains_key(account)
	}

	fn initialize_genesis_eligible_sequencers(collators: Vec<T::AccountId>) {
		EligibleToBeSequencers::<T>::put(BTreeMap::from(
			collators
				.clone()
				.into_iter()
				.map(|s| (s, RoundRefCount::one()))
				.collect::<BTreeMap<AccountIdOf<T>, RoundRefCount>>(),
		));
		let round_index = CurrentRound::<T>::get();
		RoundCollators::<T>::insert(round_index, collators);
	}

	// we assume that the elements are UNIQUE!
	// as they should be
	fn process_new_session_collators(collators: Vec<T::AccountId>) {
		let round_index = CurrentRound::<T>::get().saturating_add(One::one());
		let mut eligible_to_be_sequencers = EligibleToBeSequencers::<T>::get();
		let mut exiting_collators = BTreeSet::new();
		if round_index >= T::NoOfPastSessionsForEligibility::get() {
			let first_round_collators_of_window = RoundCollators::<T>::take(
				round_index.saturating_sub(T::NoOfPastSessionsForEligibility::get()),
			);
			for col in first_round_collators_of_window {
				match eligible_to_be_sequencers.entry(col.clone()) {
					Vacant(x) => {
						log!(
							error,
							"exiting_collator {:?} not in eligible_to_be_sequencers {:?}",
							col,
							eligible_to_be_sequencers
						)
					},
					Occupied(x) if *x.get() == 1 => {
						x.remove();
						exiting_collators.insert(col);
					},
					Occupied(mut x) => {
						*x.get_mut() = x
							.get()
							.checked_sub(One::one())
							.expect("This is safe cause x should never be 0 ");
					},
				}
			}
		}

		for col in collators.clone() {
			eligible_to_be_sequencers
				.entry(col)
				.and_modify(|x| *x = x.saturating_add(One::one()))
				.or_insert(One::one());
		}

		RoundCollators::<T>::insert(round_index, collators);
		EligibleToBeSequencers::<T>::put(eligible_to_be_sequencers);
		CurrentRound::<T>::put(round_index);

		// TODO
		// Remove from the storage item ActiveSequencers
		// To remove rights - call handle_sequencer_deactivation?
		// dedup maybe?
		Self::maybe_remove_sequencers_from_active_set(exiting_collators);
	}

	fn maybe_remove_sequencers_from_active_set(
		exiting_collators: BTreeSet<T::AccountId>,
	) {
		let to_remove = ActiveSequencers::<T>::get().iter().filter_map(|(chain,active)| {
			active.iter()
				.find(|seq| exiting_collators.contains(seq))
				.map(|seq| (*chain,seq.clone()))
		}).collect::<Vec<_>>();

		Self::remove_sequencers_from_active_set(to_remove);
	}

	// Caller should check if the sequencer is infact in the active set
	fn remove_sequencers_from_active_set(deactivating_sequencers: Vec<(T::ChainId, T::AccountId)>) {
		if deactivating_sequencers.is_empty() {
			return
		}

		ActiveSequencers::<T>::mutate(|active_set| {
			for (chain, seq) in deactivating_sequencers{
				if let Some(ref mut set) = active_set.get_mut(&chain){
					set.remove(&seq);
				}
			}
		});
	}
}

impl<T: Config> InformSessionDataTrait<T::AccountId> for Pallet<T> {
	fn inform_initialized_authorities(accounts: Vec<T::AccountId>) {
		Self::initialize_genesis_eligible_sequencers(accounts);
	}

	fn inform_on_new_session(accounts: Vec<T::AccountId>) {
		Self::process_new_session_collators(accounts);
	}
}

impl<T: Config> SequencerStakingProviderTrait<AccountIdOf<T>, BalanceOf<T>, ChainIdOf<T>> for Pallet<T> {
	fn is_active_sequencer(chain: <T as pallet::Config>::ChainId, sequencer: &AccountIdOf<T>) -> bool {
		//TODO: uncomment
		// ActiveSequencers::<T>::get().contains(sequencer)
		true
	}

	fn is_selected_sequencer(chain: ChainIdOf<T>, sequencer: &AccountIdOf<T>) -> bool {
		//TODO: uncomment
		// SelectedSequencer::<T>::get().as_ref() == Some(sequencer)
		true
	}

	fn slash_sequencer(
		chain: ChainIdOf<T>,
		to_be_slashed: &T::AccountId,
		maybe_to_reward: Option<&T::AccountId>,
	) -> DispatchResult {
		//TODO: uncomment
		// Use slashed amount partially to reward canceler, partially to vault to pay for l1 fees
		<SequencerStake<T>>::try_mutate((to_be_slashed,chain), |stake| -> DispatchResult {
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
		})?;

		let mut to_remove = BTreeSet::new();
		to_remove.insert(to_be_slashed.clone());
		Self::maybe_remove_sequencers_from_active_set(to_remove);

		Ok(().into())
	}
}

/// Simple ensure origin struct to filter for the active sequencer accounts.
pub struct EnsureActiveSequencer<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> EnsureOrigin<<T as frame_system::Config>::RuntimeOrigin>
	for EnsureActiveSequencer<T>
{
	type Success = T::AccountId;
	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		o.into().and_then(|o| {
			match o {
				frame_system::RawOrigin::Signed(ref who)
					if <Pallet<T> as SequencerStakingProviderTrait<
						AccountIdOf<T>,
						BalanceOf<T>,
						ChainIdOf<T>,
					>>::is_active_sequencer(todo!(), &who) =>
					Ok(who.clone()),
				r => Err(T::RuntimeOrigin::from(r)),
			}
		})
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		unimplemented!();
		// let founder = Founder::<T>::get().ok_or(())?;
		// Ok(T::RuntimeOrigin::from(frame_system::RawOrigin::Signed(founder)))
	}
}
