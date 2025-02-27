use super::*;

use crate::Pallet as Market;
use frame_benchmarking::{v2::*, whitelisted_caller};
use frame_support::assert_ok;
use frame_system::RawOrigin as SystemOrigin;
use mangata_support::traits::ComputeIssuance;
use sp_runtime::{traits::Bounded, SaturatedConversion};

const UNIT: u128 = 1_000_000_000_000_000_000;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	let events = frame_system::Pallet::<T>::events();
	let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
	// compare to the last event record
	let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
	assert_eq!(event, &system_event);
}

fn create_asset<T: Config>(who: &T::AccountId) -> T::CurrencyId
where
	T::Balance: From<u128>,
{
	let native = T::NativeCurrencyId::get();
	let id = T::Currency::create(who, (1_000_000_000 * UNIT).into()).expect("is ok");
	// create some default entries for stable swap to work
	let _ = T::AssetRegistry::create_pool_asset(id, native, native);
	id
}

fn make_pool<T: Config>(
	who: &T::AccountId,
	kind: PoolKind,
) -> (T::CurrencyId, T::CurrencyId, T::CurrencyId)
where
	T::Balance: From<u128>,
{
	let asset1 = create_asset::<T>(&who);
	let asset2 = create_asset::<T>(&who);
	let lp_token = T::Currency::get_next_currency_id();
	assert_ok!(Market::<T>::create_pool(
		SystemOrigin::Signed(who.clone()).into(),
		kind,
		asset1,
		(1_000_000 * UNIT).into(),
		asset2,
		(UNIT / 1_000_000).into()
	));

	(asset1, asset2, lp_token)
}

fn forward_to_next_session<T: Config>() {
	let current_block: u32 = frame_system::Pallet::<T>::block_number().saturated_into::<u32>();

	let blocks_per_session: u32 = T::Rewards::rewards_period();
	let target_block_nr: u32;
	let target_session_nr: u32;

	if current_block == 0_u32 || current_block == 1_u32 {
		target_session_nr = 1_u32;
		target_block_nr = blocks_per_session;
	} else {
		// to fail on user trying to manage block nr on its own
		assert!(current_block % blocks_per_session == 0);
		target_session_nr = (current_block / blocks_per_session) + 1_u32;
		target_block_nr = target_session_nr * blocks_per_session;
	}

	frame_system::Pallet::<T>::set_block_number(target_block_nr.into());
	T::ComputeIssuance::compute_issuance(target_session_nr);
}

#[benchmarks(where <T as Config>::Balance: From<u128>, <T as Config>::CurrencyId: From<u32> + Into<u32>)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_pool_xyk() {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		let asset1 = create_asset::<T>(&caller);
		let asset2 = create_asset::<T>(&caller);
		let lp_token = T::Currency::get_next_currency_id();

		#[extrinsic_call]
		create_pool(
			SystemOrigin::Signed(caller.clone()),
			kind,
			asset1,
			(10 * UNIT).into(),
			asset2,
			10.into(),
		);

		let lp_supply = T::Currency::total_issuance(lp_token);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id: lp_token,
				amounts_provided: ((10 * UNIT).into(), 10.into()),
				lp_token,
				lp_token_minted: lp_supply,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_xyk() {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		let (asset1, _, pool_id) = make_pool::<T>(&caller, kind);
		T::Rewards::enable_native_rewards(pool_id, 1u8);

		let amount1 = (10 * UNIT).into();

		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset1,
			amount1,
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let amount2 =
			Market::<T>::calculate_expected_amount_for_minting(pool_id, asset1, amount1).unwrap();
		let lp_minted =
			Market::<T>::calculate_expected_lp_minted(pool_id, (amount1, amount2)).unwrap();

		#[extrinsic_call]
		mint_liquidity(SystemOrigin::Signed(caller.clone()), pool_id, asset1, amount1, amount2);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: (amount1, amount2),
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_fixed_amounts_xyk() {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		let (asset1, _, pool_id) = make_pool::<T>(&caller, kind);
		T::Rewards::enable_native_rewards(pool_id, 1u8);

		let amounts = ((10 * UNIT).into(), Zero::zero());

		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset1,
			amounts.0,
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		// xyk returns None in this case, does a swap internally
		let lp_minted = Market::<T>::calculate_expected_lp_minted(pool_id, amounts)
			.unwrap_or(2499993499747501522.into());

		#[extrinsic_call]
		mint_liquidity_fixed_amounts(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			amounts,
			Zero::zero(),
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: amounts,
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_using_vesting_native_tokens_by_vesting_index_xyk() {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		let native = T::NativeCurrencyId::get();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		assert_ok!(T::Currency::mint(native, &caller, (1_000 * UNIT).into()));

		let asset2 = create_asset::<T>(&caller);
		let pool_id = T::Currency::get_next_currency_id();
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			kind,
			native,
			UNIT.into(),
			asset2,
			UNIT.into()
		));

		T::Rewards::enable_native_rewards(pool_id, 1u8);
		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset2,
			(10 * UNIT).into(),
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let lock = (5 * UNIT).into();
		assert_ok!(T::Vesting::lock_tokens(&caller, native, lock, None, 1_000_000_u32.into()));

		forward_to_next_session::<T>();

		let amount1 = UNIT.into();
		let amount2 =
			Market::<T>::calculate_expected_amount_for_minting(pool_id, native, amount1).unwrap();
		let lp_minted =
			Market::<T>::calculate_expected_lp_minted(pool_id, (amount1, amount2)).unwrap();

		#[extrinsic_call]
		mint_liquidity_using_vesting_native_tokens_by_vesting_index(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			0,
			Some(amount1),
			T::Balance::max_value(),
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: (amount1, amount2),
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_using_vesting_native_tokens_xyk() {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		let native = T::NativeCurrencyId::get();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		assert_ok!(T::Currency::mint(native, &caller, (1_000 * UNIT).into()));

		let asset2 = create_asset::<T>(&caller);
		let pool_id = T::Currency::get_next_currency_id();
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			kind,
			native,
			UNIT.into(),
			asset2,
			UNIT.into()
		));

		T::Rewards::enable_native_rewards(pool_id, 1u8);
		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset2,
			(10 * UNIT).into(),
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let lock = (5 * UNIT).into();
		assert_ok!(T::Vesting::lock_tokens(&caller, native, lock, None, 1_000_000_u32.into()));

		forward_to_next_session::<T>();

		let amount1 = UNIT.into();
		let amount2 =
			Market::<T>::calculate_expected_amount_for_minting(pool_id, native, amount1).unwrap();
		let lp_minted =
			Market::<T>::calculate_expected_lp_minted(pool_id, (amount1, amount2)).unwrap();

		#[extrinsic_call]
		mint_liquidity_using_vesting_native_tokens(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			amount1,
			amount2,
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: (amount1, amount2),
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn burn_liquidity_xyk() {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		let (asset1, _, pool_id) = make_pool::<T>(&caller, kind);
		T::Rewards::enable_native_rewards(pool_id, 1u8);

		let amount1 = (10 * UNIT).into();

		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset1,
			amount1,
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let burn_amount = UNIT.into();
		let min_amounts = Market::<T>::get_burn_amount(pool_id, burn_amount).unwrap();

		#[extrinsic_call]
		burn_liquidity(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			burn_amount,
			min_amounts.0,
			min_amounts.1,
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityBurned {
				who: caller,
				pool_id,
				amounts: min_amounts,
				burned_amount: burn_amount,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn multiswap_asset_xyk(y: Linear<2, 100>) {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		let native = T::NativeCurrencyId::get();
		assert_ok!(T::Currency::mint(native, &caller, (100_000 * UNIT).into()));

		let from = T::Currency::get_next_currency_id();
		for _ in 0..y {
			create_asset::<T>(&caller);
		}
		let pool_id_start = T::Currency::get_next_currency_id();
		let to = pool_id_start - 1.into();
		for i in 0..(y - 1) {
			assert_ok!(Market::<T>::create_pool(
				SystemOrigin::Signed(caller.clone()).into(),
				kind.clone(),
				from + i.into(),
				UNIT.into(),
				from + (i + 1).into(),
				UNIT.into()
			));
		}
		let pool_id_end = T::Currency::get_next_currency_id();
		// Only the input token requires a pool with native
		// Since the commission fee is charged only on the input token
		// and we will be benchmarking the is_swap_tokens_lockless separately
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			PoolKind::Xyk,
			native,
			UNIT.into(),
			from,
			UNIT.into()
		));
		// We make sure that the fee_lock_metadata_is_init
		// and that there are no whitelisted tokens, so that
		// process_fee_lock is called and not unlock_fee
		// We also require the user to have native tokens to pay for
		// the fee_lock
		// If in case the fee_lock_metadata is not init, then we will have
		// to separately calculate and add the do_fee_lock weight
		assert!(T::FeeLock::is_fee_lock_init());
		// This assumes from till to are contigous swapped assets
		for i in Into::<u32>::into(from)..=to.into() {
			assert!(!pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
				.unwrap()
				.is_whitelisted(i.into()));
		}
		let native_balance_before: u128 = T::Currency::available_balance(native, &caller).into();
		assert!(
			native_balance_before >
				pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
					.unwrap()
					.fee_lock_amount
					.into()
		);
		assert!(pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		let swaps: Vec<T::CurrencyId> =
			(pool_id_start.into()..pool_id_end.into()).map(|id: u32| id.into()).collect();

		let before1 = T::Currency::available_balance(from, &caller);
		let before2 = T::Currency::available_balance(to, &caller);

		let amount = UNIT.into();
		#[extrinsic_call]
		multiswap_asset(
			SystemOrigin::Signed(caller.clone()),
			swaps,
			from,
			amount,
			to,
			Zero::zero(),
		);

		let after1 = T::Currency::available_balance(from, &caller);
		let after2 = T::Currency::available_balance(to, &caller);

		let native_balance_after: u128 = T::Currency::available_balance(native, &caller).into();

		assert!(
			native_balance_before ==
				native_balance_after.saturating_add(
					pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
						.unwrap()
						.fee_lock_amount
						.into()
				)
		);
		assert!(!pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		assert_eq!(before1, after1 + amount);
		assert!(before2 < after2);
	}

	#[benchmark]
	fn multiswap_asset_buy_xyk(y: Linear<2, 100>) {
		let kind = PoolKind::Xyk;
		let caller: T::AccountId = whitelisted_caller();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		let native = T::NativeCurrencyId::get();
		assert_ok!(T::Currency::mint(native, &caller, (100_000 * UNIT).into()));

		let from = T::Currency::get_next_currency_id();
		for _ in 0..y {
			create_asset::<T>(&caller);
		}
		let pool_id_start = T::Currency::get_next_currency_id();
		let to = pool_id_start - 1.into();
		for i in 0..(y - 1) {
			assert_ok!(Market::<T>::create_pool(
				SystemOrigin::Signed(caller.clone()).into(),
				kind.clone(),
				from + i.into(),
				UNIT.into(),
				from + (i + 1).into(),
				UNIT.into()
			));
		}
		let pool_id_end = T::Currency::get_next_currency_id();
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			PoolKind::Xyk,
			native,
			UNIT.into(),
			from,
			UNIT.into()
		));
		// We make sure that the fee_lock_metadata_is_init
		// and that there are no whitelisted tokens, so that
		// process_fee_lock is called and not unlock_fee
		// We also require the user to have native tokens to pay for
		// the fee_lock
		// If in case the fee_lock_metadata is not init, then we will have
		// to separately calculate and add the do_fee_lock weight
		assert!(T::FeeLock::is_fee_lock_init());
		// This assumes from till to are contigous swapped assets
		for i in Into::<u32>::into(from)..=to.into() {
			assert!(!pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
				.unwrap()
				.is_whitelisted(i.into()));
		}
		let native_balance_before: u128 = T::Currency::available_balance(native, &caller).into();
		assert!(
			native_balance_before >
				pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
					.unwrap()
					.fee_lock_amount
					.into()
		);
		assert!(pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		let swaps: Vec<T::CurrencyId> =
			(pool_id_start.into()..pool_id_end.into()).map(|id: u32| id.into()).collect();

		let before1 = T::Currency::available_balance(from, &caller);
		let before2 = T::Currency::available_balance(to, &caller);

		let amount = (UNIT / 1_000).into();
		#[extrinsic_call]
		multiswap_asset_buy(
			SystemOrigin::Signed(caller.clone()),
			swaps,
			to,
			amount,
			from,
			amount * 2.into(),
		);

		let after1 = T::Currency::available_balance(from, &caller);
		let after2 = T::Currency::available_balance(to, &caller);

		let native_balance_after: u128 = T::Currency::available_balance(native, &caller).into();

		assert!(
			native_balance_before ==
				native_balance_after.saturating_add(
					pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
						.unwrap()
						.fee_lock_amount
						.into()
				)
		);
		assert!(!pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		assert!(before1 > after1);
		assert_eq!(before2 + amount, after2);
	}

	// copypaste for stableswap
	#[benchmark]
	fn create_pool_sswap() {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		let asset1 = create_asset::<T>(&caller);
		let asset2 = create_asset::<T>(&caller);
		let lp_token = T::Currency::get_next_currency_id();

		#[extrinsic_call]
		create_pool(
			SystemOrigin::Signed(caller.clone()),
			kind,
			asset1,
			(10 * UNIT).into(),
			asset2,
			10.into(),
		);

		let lp_supply = T::Currency::total_issuance(lp_token);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id: lp_token,
				amounts_provided: ((10 * UNIT).into(), 10.into()),
				lp_token,
				lp_token_minted: lp_supply,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_sswap() {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		let (asset1, _, pool_id) = make_pool::<T>(&caller, kind);
		T::Rewards::enable_native_rewards(pool_id, 1u8);

		let amount1 = (10 * UNIT).into();

		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset1,
			amount1,
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let amount2 =
			Market::<T>::calculate_expected_amount_for_minting(pool_id, asset1, amount1).unwrap();
		let lp_minted =
			Market::<T>::calculate_expected_lp_minted(pool_id, (amount1, amount2)).unwrap();

		#[extrinsic_call]
		mint_liquidity(SystemOrigin::Signed(caller.clone()), pool_id, asset1, amount1, amount2);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: (amount1, amount2),
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_fixed_amounts_sswap() {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		let (asset1, _, pool_id) = make_pool::<T>(&caller, kind);
		T::Rewards::enable_native_rewards(pool_id, 1u8);

		let amounts = ((10 * UNIT).into(), Zero::zero());

		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset1,
			amounts.0,
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		// xyk returns None in this case, does a swap internally
		let lp_minted = Market::<T>::calculate_expected_lp_minted(pool_id, amounts)
			.unwrap_or(4192957199889574550.into());

		#[extrinsic_call]
		mint_liquidity_fixed_amounts(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			amounts,
			Zero::zero(),
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: amounts,
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_using_vesting_native_tokens_by_vesting_index_sswap() {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		let native = T::NativeCurrencyId::get();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		assert_ok!(T::Currency::mint(native, &caller, (1_000 * UNIT).into()));

		let asset2 = create_asset::<T>(&caller);
		let pool_id = T::Currency::get_next_currency_id();
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			kind,
			native,
			UNIT.into(),
			asset2,
			UNIT.into()
		));

		T::Rewards::enable_native_rewards(pool_id, 1u8);
		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset2,
			(10 * UNIT).into(),
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let lock = (5 * UNIT).into();
		assert_ok!(T::Vesting::lock_tokens(&caller, native, lock, None, 1_000_000_u32.into()));

		forward_to_next_session::<T>();

		let amount1 = UNIT.into();
		let amount2 =
			Market::<T>::calculate_expected_amount_for_minting(pool_id, native, amount1).unwrap();
		let lp_minted =
			Market::<T>::calculate_expected_lp_minted(pool_id, (amount1, amount2)).unwrap();

		#[extrinsic_call]
		mint_liquidity_using_vesting_native_tokens_by_vesting_index(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			0,
			Some(amount1),
			T::Balance::max_value(),
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: (amount1, amount2),
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn mint_liquidity_using_vesting_native_tokens_sswap() {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		let native = T::NativeCurrencyId::get();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		assert_ok!(T::Currency::mint(native, &caller, (1_000 * UNIT).into()));

		let asset2 = create_asset::<T>(&caller);
		let pool_id = T::Currency::get_next_currency_id();
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			kind,
			native,
			UNIT.into(),
			asset2,
			UNIT.into()
		));

		T::Rewards::enable_native_rewards(pool_id, 1u8);
		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset2,
			(10 * UNIT).into(),
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let lock = (5 * UNIT).into();
		assert_ok!(T::Vesting::lock_tokens(&caller, native, lock, None, 1_000_000_u32.into()));

		forward_to_next_session::<T>();

		let amount1 = UNIT.into();
		let amount2 =
			Market::<T>::calculate_expected_amount_for_minting(pool_id, native, amount1).unwrap();
		let lp_minted =
			Market::<T>::calculate_expected_lp_minted(pool_id, (amount1, amount2)).unwrap();

		#[extrinsic_call]
		mint_liquidity_using_vesting_native_tokens(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			amount1,
			amount2,
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityMinted {
				who: caller,
				pool_id,
				amounts_provided: (amount1, amount2),
				lp_token: pool_id,
				lp_token_minted: lp_minted,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn burn_liquidity_sswap() {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		let (asset1, _, pool_id) = make_pool::<T>(&caller, kind);
		T::Rewards::enable_native_rewards(pool_id, 1u8);

		let amount1 = (10 * UNIT).into();

		assert_ok!(Market::<T>::mint_liquidity(
			SystemOrigin::Signed(caller.clone()).into(),
			pool_id,
			asset1,
			amount1,
			T::Balance::max_value(),
		));

		forward_to_next_session::<T>();

		let burn_amount = UNIT.into();
		let min_amounts = Market::<T>::get_burn_amount(pool_id, burn_amount).unwrap();

		#[extrinsic_call]
		burn_liquidity(
			SystemOrigin::Signed(caller.clone()),
			pool_id,
			burn_amount,
			min_amounts.0,
			min_amounts.1,
		);

		let lp_supply = T::Currency::total_issuance(pool_id);
		assert_last_event::<T>(
			Event::LiquidityBurned {
				who: caller,
				pool_id,
				amounts: min_amounts,
				burned_amount: burn_amount,
				total_supply: lp_supply,
			}
			.into(),
		);
	}

	#[benchmark]
	fn multiswap_asset_sswap(y: Linear<2, 100>) {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		let native = T::NativeCurrencyId::get();
		assert_ok!(T::Currency::mint(native, &caller, (100_000 * UNIT).into()));

		let from = T::Currency::get_next_currency_id();
		for _ in 0..y {
			create_asset::<T>(&caller);
		}
		let pool_id_start = T::Currency::get_next_currency_id();
		let to = pool_id_start - 1.into();
		for i in 0..(y - 1) {
			assert_ok!(Market::<T>::create_pool(
				SystemOrigin::Signed(caller.clone()).into(),
				kind.clone(),
				from + i.into(),
				UNIT.into(),
				from + (i + 1).into(),
				UNIT.into()
			));
		}
		let pool_id_end = T::Currency::get_next_currency_id();
		// Only the input token requires a pool with native
		// Since the commission fee is charged only on the input token
		// and we will be benchmarking the is_swap_tokens_lockless separately
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			PoolKind::Xyk,
			native,
			UNIT.into(),
			from,
			UNIT.into()
		));
		// We make sure that the fee_lock_metadata_is_init
		// and that there are no whitelisted tokens, so that
		// process_fee_lock is called and not unlock_fee
		// We also require the user to have native tokens to pay for
		// the fee_lock
		// If in case the fee_lock_metadata is not init, then we will have
		// to separately calculate and add the do_fee_lock weight
		assert!(T::FeeLock::is_fee_lock_init());
		// This assumes from till to are contigous swapped assets
		for i in Into::<u32>::into(from)..=to.into() {
			assert!(!pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
				.unwrap()
				.is_whitelisted(i.into()));
		}
		let native_balance_before: u128 = T::Currency::available_balance(native, &caller).into();
		assert!(
			native_balance_before >
				pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
					.unwrap()
					.fee_lock_amount
					.into()
		);
		assert!(pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		let swaps: Vec<T::CurrencyId> =
			(pool_id_start.into()..pool_id_end.into()).map(|id: u32| id.into()).collect();

		let before1 = T::Currency::available_balance(from, &caller);
		let before2 = T::Currency::available_balance(to, &caller);

		let amount = UNIT.into();
		#[extrinsic_call]
		multiswap_asset(
			SystemOrigin::Signed(caller.clone()),
			swaps,
			from,
			amount,
			to,
			Zero::zero(),
		);

		let after1 = T::Currency::available_balance(from, &caller);
		let after2 = T::Currency::available_balance(to, &caller);

		let native_balance_after: u128 = T::Currency::available_balance(native, &caller).into();

		assert!(
			native_balance_before ==
				native_balance_after.saturating_add(
					pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
						.unwrap()
						.fee_lock_amount
						.into()
				)
		);
		assert!(!pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		assert_eq!(before1, after1 + amount);
		assert!(before2 < after2);
	}

	#[benchmark]
	fn multiswap_asset_buy_sswap(y: Linear<2, 100>) {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		let native = T::NativeCurrencyId::get();
		assert_ok!(T::Currency::mint(native, &caller, (100_000 * UNIT).into()));

		let from = T::Currency::get_next_currency_id();
		for _ in 0..y {
			create_asset::<T>(&caller);
		}
		let pool_id_start = T::Currency::get_next_currency_id();
		let to = pool_id_start - 1.into();
		for i in 0..(y - 1) {
			assert_ok!(Market::<T>::create_pool(
				SystemOrigin::Signed(caller.clone()).into(),
				kind.clone(),
				from + i.into(),
				UNIT.into(),
				from + (i + 1).into(),
				UNIT.into()
			));
		}

		let pool_id_end = T::Currency::get_next_currency_id();
		// Only the input token requires a pool with native
		// Since the commission fee is charged only on the input token
		// and we will be benchmarking the is_swap_tokens_lockless separately
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			PoolKind::Xyk,
			native,
			UNIT.into(),
			from,
			UNIT.into()
		));
		// We make sure that the fee_lock_metadata_is_init
		// and that there are no whitelisted tokens, so that
		// process_fee_lock is called and not unlock_fee
		// We also require the user to have native tokens to pay for
		// the fee_lock
		// If in case the fee_lock_metadata is not init, then we will have
		// to separately calculate and add the do_fee_lock weight
		assert!(T::FeeLock::is_fee_lock_init());
		// This assumes from till to are contigous swapped assets
		for i in Into::<u32>::into(from)..=to.into() {
			assert!(!pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
				.unwrap()
				.is_whitelisted(i.into()));
		}
		let native_balance_before: u128 = T::Currency::available_balance(native, &caller).into();
		assert!(
			native_balance_before >
				pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
					.unwrap()
					.fee_lock_amount
					.into()
		);
		assert!(pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		let swaps: Vec<T::CurrencyId> =
			(pool_id_start.into()..pool_id_end.into()).map(|id: u32| id.into()).collect();

		let before1 = T::Currency::available_balance(from, &caller);
		let before2 = T::Currency::available_balance(to, &caller);

		let amount = (UNIT / 1_000).into();
		#[extrinsic_call]
		multiswap_asset_buy(
			SystemOrigin::Signed(caller.clone()),
			swaps,
			to,
			amount,
			from,
			amount * 2.into(),
		);

		let after1 = T::Currency::available_balance(from, &caller);
		let after2 = T::Currency::available_balance(to, &caller);

		let native_balance_after: u128 = T::Currency::available_balance(native, &caller).into();

		assert!(
			native_balance_before ==
				native_balance_after.saturating_add(
					pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
						.unwrap()
						.fee_lock_amount
						.into()
				)
		);
		assert!(!pallet_fee_lock::Pallet::<T>::get_account_fee_lock_data(&caller)
			.total_fee_lock_amount
			.is_zero());

		assert!(before1 > after1);
		assert_eq!(before2 + amount, after2);
	}

	#[benchmark]
	fn is_swap_tokens_lockless() {
		let kind = PoolKind::StableSwap;
		let caller: T::AccountId = whitelisted_caller();
		// in test mock this would be native, in runtime we have genesis
		let _ = create_asset::<T>(&caller);
		let native = T::NativeCurrencyId::get();
		assert_ok!(T::Currency::mint(native, &caller, (100_000 * UNIT).into()));

		let fee_lock_amount = pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
			.unwrap()
			.fee_lock_amount
			.into();

		let token_id = T::Currency::get_next_currency_id();
		create_asset::<T>(&caller);
		let pool_id = T::Currency::get_next_currency_id();
		// The input token requires a pool with native
		// This will be the pool used for the valuation
		assert_ok!(Market::<T>::create_pool(
			SystemOrigin::Signed(caller.clone()).into(),
			PoolKind::Xyk,
			// The order of tokens matters here a lot!
			// We want the Pools to be read twice
			// in the worst case due to unknown order
			token_id,
			(fee_lock_amount * 10).into(),
			native,
			(fee_lock_amount * 10).into()
		));
		// We make sure that the fee_lock_metadata_is_init
		// and that token_id is whitelisted
		assert!(T::FeeLock::is_fee_lock_init());
		let mut fee_lock_metadata = pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata().unwrap();
		let _ = fee_lock_metadata
			.whitelisted_tokens
			.try_insert(Into::<u32>::into(token_id).into())
			.unwrap();
		pallet_fee_lock::FeeLockMetadata::<T>::put(fee_lock_metadata);
		assert!(pallet_fee_lock::Pallet::<T>::get_fee_lock_metadata()
			.unwrap()
			.is_whitelisted(Into::<u32>::into(token_id).into()));

		let fee_lock_metadata_key = pallet_fee_lock::FeeLockMetadata::<T>::hashed_key().to_vec();
		frame_benchmarking::benchmarking::add_to_whitelist(fee_lock_metadata_key.into());

		#[block]
		{
			assert!(T::FeeLock::is_swap_tokens_lockless(token_id, fee_lock_amount.into(),));
		}
	}
}
