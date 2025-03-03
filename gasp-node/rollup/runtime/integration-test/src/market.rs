use crate::setup::*;

use pallet_market::{AtomicSwap, Event, PoolKind};
use sp_runtime::{traits::Zero, DispatchResult, BoundedVec};
use frame_support::{assert_err, assert_ok};
use test_case::test_case;
use orml_tokens::MultiTokenCurrencyExtended;
use rollup_runtime::config::{
	BnbAccountIdOf, TreasuryAccountIdOf,
};
use rollup_runtime::MultiTokenCurrency;

const ASSET_ID_1: u32 = NATIVE_ASSET_ID + 1;
const ASSET_ID_2: u32 = ASSET_ID_1 + 1;
const ASSET_ID_3: u32 = ASSET_ID_2 + 1;
const POOL_ID_1: u32 = ASSET_ID_3 + 1;
const POOL_ID_2: u32 = POOL_ID_1 + 1;
const POOL_ID_3: u32 = POOL_ID_2 + 1;

fn test_env() -> TestExternalities {
	ExtBuilder {
		balances: vec![
			(AccountId::from(ALICE), NATIVE_ASSET_ID, 100 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_1, 100 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_2, Balance::MAX),
			(AccountId::from(ALICE), ASSET_ID_3, Balance::MAX),
		],
		..ExtBuilder::default()
	}
	.build()
}

type Market = pallet_market::Pallet<Runtime>;

fn origin() -> RuntimeOrigin {
	RuntimeOrigin::signed(AccountId::from(ALICE))
}

fn create_pool(kind: PoolKind, assets: (u32, u32)) -> DispatchResult {
	Market::create_pool(origin(), kind, assets.0, 10 * UNIT, assets.1, 10 * UNIT)
}

fn create_pool_unb(kind: PoolKind, assets: (u32, u32)) -> DispatchResult {
	Market::create_pool(origin(), kind, assets.0, 10 * UNIT, assets.1, 5 * UNIT)
}

pub(crate) fn events() -> Vec<RuntimeEvent> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| match e {
			RuntimeEvent::Xyk(_) | RuntimeEvent::Market(_) | RuntimeEvent::StableSwap(_) => Some(e),
			_ => None,
		})
		.collect::<Vec<_>>()
}

fn assert_has_event(generic_event: RuntimeEvent) {
	let events = events();
	let system_event: RuntimeEvent = generic_event.clone().into();
	assert!(
		events.iter().any(|record| *record == system_event),
		"expected event {generic_event:?} not found in events {events:?}",
	);
}

fn assert_last_event(generic_event: RuntimeEvent) {
	let events = events();
	let system_event: RuntimeEvent = generic_event.clone().into();
	let last_event = events.last().expect("events expected").clone();
	assert_eq!(
		last_event, system_event,
		"expected event {generic_event:?} is not equal to the last event {last_event:?}",
	);
}

#[test]
fn create_pool_works() {
	test_env().execute_with(|| {
		assert_ok!(create_pool(PoolKind::Xyk, (NATIVE_ASSET_ID, ASSET_ID_1)));
		System::assert_has_event(RuntimeEvent::Market(Event::PoolCreated {
			creator: AccountId::from(ALICE),
			pool_id: POOL_ID_1,
			lp_token: POOL_ID_1,
			assets: (0, 1),
		}));
		System::assert_has_event(RuntimeEvent::Market(Event::LiquidityMinted {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_1,
			amounts_provided: (10000000000000000000, 10000000000000000000),
			lp_token: POOL_ID_1,
			lp_token_minted: 10000000000000000000,
			total_supply: 10000000000000000000,
		}));

		assert_ok!(create_pool(PoolKind::StableSwap, (NATIVE_ASSET_ID, ASSET_ID_1)));

		let p = pallet_stable_swap::Pools::<Runtime>::get(POOL_ID_2).unwrap();
		assert_eq!(p.rate_multipliers[0], UNIT);
		assert_eq!(p.rate_multipliers[1], UNIT);
		System::assert_has_event(RuntimeEvent::Market(Event::PoolCreated {
			creator: AccountId::from(ALICE),
			pool_id: POOL_ID_2,
			lp_token: POOL_ID_2,
			assets: (0, 1),
		}));
		System::assert_has_event(RuntimeEvent::Market(Event::LiquidityMinted {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_2,
			amounts_provided: (10000000000000000000, 10000000000000000000),
			lp_token: POOL_ID_2,
			lp_token_minted: 20000000000000000000,
			total_supply: 20000000000000000000,
		}));

		assert_ok!(create_pool_unb(PoolKind::StableSwap, (NATIVE_ASSET_ID, ASSET_ID_1)));

		let p = pallet_stable_swap::Pools::<Runtime>::get(POOL_ID_3).unwrap();
		println!("{:?}", p);
		assert_eq!(p.rate_multipliers[0], 5 * UNIT);
		assert_eq!(p.rate_multipliers[1], 10 * UNIT);

		System::assert_has_event(RuntimeEvent::Market(Event::PoolCreated {
			creator: AccountId::from(ALICE),
			pool_id: POOL_ID_3,
			lp_token: POOL_ID_3,
			assets: (0, 1),
		}));
		// lp_token_minted are correct since we applied the rates, and the pool is balanced
		System::assert_has_event(RuntimeEvent::Market(Event::LiquidityMinted {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_3,
			amounts_provided: (10000000000000000000, 5000000000000000000),
			lp_token: POOL_ID_3,
			lp_token_minted: 100000000000000000000,
			total_supply: 100000000000000000000,
		}));
	})
}

#[test]
fn create_pool_works_ss_min_max() {
	test_env().execute_with(|| {
		let min = 1;
		let max_round_to_decimal: u128 = 300000000000000000000000000000000000000 / 10;

		assert_ok!(Market::create_pool(
			origin(),
			PoolKind::StableSwap,
			ASSET_ID_2,
			min,
			ASSET_ID_3,
			min,
		));

		let max_rate = UNIT; // 1e18
		let max_rate_fail = UNIT * 10; // 1e19
		assert_ok!(Market::create_pool(
			origin(),
			PoolKind::StableSwap,
			ASSET_ID_2,
			min,
			ASSET_ID_3,
			max_rate,
		));
		assert_err!(
			Market::create_pool(
				origin(),
				PoolKind::StableSwap,
				ASSET_ID_2,
				min,
				ASSET_ID_3,
				max_rate_fail,
			),
			pallet_stable_swap::Error::<Runtime>::InitialPoolRateOutOfRange
		);

		assert_ok!(Market::create_pool(
			origin(),
			PoolKind::StableSwap,
			ASSET_ID_2,
			max_round_to_decimal,
			ASSET_ID_3,
			max_round_to_decimal,
		));
	});
}

#[test]
fn add_liquidity_works() {
	test_env().execute_with(|| {
		assert_ok!(create_pool_unb(PoolKind::Xyk, (ASSET_ID_2, ASSET_ID_1)));
		// note that the created pool will be sorted as (ASSET_ID_1, ASSET_ID_2)
		assert_ok!(create_pool_unb(PoolKind::StableSwap, (ASSET_ID_2, ASSET_ID_1)));

		let expected =
			Market::calculate_expected_amount_for_minting(POOL_ID_1, ASSET_ID_2, UNIT).unwrap();
		let lp_expected =
			Market::calculate_expected_lp_minted(POOL_ID_1, (UNIT, expected)).unwrap();
		assert_ok!(Market::mint_liquidity(origin(), POOL_ID_1, ASSET_ID_2, UNIT, 10 * UNIT));
		System::assert_last_event(RuntimeEvent::Market(Event::LiquidityMinted {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_1,
			amounts_provided: (1000000000000000000, expected),
			lp_token: POOL_ID_1,
			lp_token_minted: lp_expected,
			total_supply: 8250000000000000000,
		}));

		let expected =
			Market::calculate_expected_amount_for_minting(POOL_ID_2, ASSET_ID_2, UNIT).unwrap();
		let lp_expected =
			Market::calculate_expected_lp_minted(POOL_ID_2, (expected, UNIT)).unwrap();
		assert_ok!(Market::mint_liquidity(origin(), POOL_ID_2, ASSET_ID_2, UNIT, 10 * UNIT));
		System::assert_last_event(RuntimeEvent::Market(Event::LiquidityMinted {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_2,
			amounts_provided: (1000000000000000000, expected),
			lp_token: POOL_ID_2,
			lp_token_minted: lp_expected,
			total_supply: 110000000000000000000,
		}));
	})
}

#[test]
fn add_liquidity_fixed_works() {
	test_env().execute_with(|| {
		assert_ok!(create_pool_unb(PoolKind::Xyk, (ASSET_ID_2, ASSET_ID_1)));
		assert_ok!(create_pool_unb(PoolKind::StableSwap, (ASSET_ID_2, ASSET_ID_1)));

		assert_ok!(Market::mint_liquidity_fixed_amounts(origin(), POOL_ID_1, (UNIT, 0), 0));
		System::assert_last_event(RuntimeEvent::Market(Event::LiquidityMinted {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_1,
			amounts_provided: (1000000000000000000, 0),
			lp_token: POOL_ID_1,
			lp_token_minted: 366066361276136601,
			total_supply: 7866066361276136601,
		}));

		let expected = Market::calculate_expected_lp_minted(POOL_ID_2, (5 * UNIT, UNIT)).unwrap();
		assert_ok!(Market::mint_liquidity_fixed_amounts(origin(), POOL_ID_2, (5 * UNIT, UNIT), 0));
		System::assert_last_event(RuntimeEvent::Market(Event::LiquidityMinted {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_2,
			amounts_provided: (5000000000000000000, 1000000000000000000),
			lp_token: POOL_ID_2,
			lp_token_minted: expected,
			total_supply: 154992874585943193114,
		}));
	})
}

#[test]
fn remove_liquidity_works() {
	test_env().execute_with(|| {
		assert_ok!(create_pool_unb(PoolKind::Xyk, (NATIVE_ASSET_ID, ASSET_ID_1)));
		assert_ok!(create_pool_unb(PoolKind::StableSwap, (NATIVE_ASSET_ID, ASSET_ID_1)));

		assert_ok!(Market::burn_liquidity(origin(), POOL_ID_1, UNIT, 0, 0));
		System::assert_last_event(RuntimeEvent::Market(Event::LiquidityBurned {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_1,
			amounts: (1333333333333333333, 666666666666666666),
			burned_amount: 1000000000000000000,
			total_supply: 6500000000000000000,
		}));

		assert_ok!(Market::burn_liquidity(origin(), POOL_ID_2, UNIT, 0, 0));
		System::assert_last_event(RuntimeEvent::Market(Event::LiquidityBurned {
			who: AccountId::from(ALICE),
			pool_id: POOL_ID_2,
			amounts: (100000000000000000, 50000000000000000),
			burned_amount: 1000000000000000000,
			total_supply: 99000000000000000000,
		}));
	})
}

#[test]
fn multiswap_should_work_xyk() {
	test_env().execute_with(|| {
		assert_ok!(create_pool_unb(PoolKind::Xyk, (ASSET_ID_3, ASSET_ID_1)));
		assert_ok!(create_pool_unb(PoolKind::Xyk, (ASSET_ID_1, ASSET_ID_2)));
		assert_ok!(create_pool_unb(PoolKind::Xyk, (ASSET_ID_2, ASSET_ID_3)));

		assert_ok!(Market::multiswap_asset(
			origin(),
			vec![POOL_ID_1, POOL_ID_2, POOL_ID_3],
			ASSET_ID_3,
			UNIT,
			ASSET_ID_3,
			Zero::zero(),
		));

		// println!("{:#?}", events());

		assert_last_event(RuntimeEvent::Market(Event::AssetsSwapped {
			who: AccountId::from(ALICE),
			swaps: vec![
				AtomicSwap {
					pool_id: POOL_ID_1,
					kind: PoolKind::Xyk,
					asset_in: 3,
					asset_out: 1,
					amount_in: 997000000000000000,
					amount_out: 453305446940074565,
				},
				AtomicSwap {
					pool_id: POOL_ID_2,
					kind: PoolKind::Xyk,
					asset_in: 1,
					asset_out: 2,
					amount_in: 453305446940074565,
					amount_out: 216823974598756034,
				},
				AtomicSwap {
					pool_id: POOL_ID_3,
					kind: PoolKind::Xyk,
					asset_in: 2,
					asset_out: 3,
					amount_in: 216823974598756034,
					amount_out: 106111241192873411,
				},
			],
		}));
	})
}

#[test]
fn multiswap_should_work_stable_swap_with_bnb() {
	test_env().execute_with(|| {
		// 2:1 rate
		assert_ok!(create_pool_unb(PoolKind::StableSwap, (ASSET_ID_3, ASSET_ID_1)));
		assert_ok!(create_pool_unb(PoolKind::StableSwap, (ASSET_ID_1, ASSET_ID_2)));
		// 1:1 rate
		assert_ok!(create_pool(PoolKind::StableSwap, (ASSET_ID_2, ASSET_ID_3)));
		// for bnb
		assert_ok!(create_pool_unb(PoolKind::Xyk, (NATIVE_ASSET_ID, ASSET_ID_1)));
		assert_ok!(create_pool_unb(PoolKind::Xyk, (NATIVE_ASSET_ID, ASSET_ID_2)));
		assert_ok!(create_pool_unb(PoolKind::Xyk, (NATIVE_ASSET_ID, ASSET_ID_3)));

		let p = pallet_stable_swap::Pools::<Runtime>::get(POOL_ID_1).unwrap();
		// println!("{:#?}", p);
		// assets are ordered by id, same sa as the corresponding rates
		assert_eq!(p.assets, vec![ASSET_ID_1, ASSET_ID_3]);
		assert_eq!(p.rate_multipliers[0], 10 * UNIT);
		assert_eq!(p.rate_multipliers[1], 5 * UNIT);

		let before = Tokens::total_issuance(NATIVE_ASSET_ID);

		assert_ok!(Market::multiswap_asset(
			origin(),
			vec![POOL_ID_1, POOL_ID_2, POOL_ID_3],
			ASSET_ID_3,
			UNIT,
			ASSET_ID_3,
			Zero::zero(),
		));

		let after = Tokens::total_issuance(NATIVE_ASSET_ID);
		// issuance decreased because of bnb
		assert!(before > after);
		assert_eq!(before, 100000000000000000000);
		assert_eq!(after, 99999000199960007998);

		// println!("{:#?}", events());
		assert_last_event(RuntimeEvent::Market(Event::AssetsSwapped {
			who: AccountId::from(ALICE),
			swaps: vec![
				AtomicSwap {
					pool_id: POOL_ID_1,
					kind: PoolKind::StableSwap,
					asset_in: ASSET_ID_3,
					asset_out: 1,
					amount_in: 997000000000000000,
					amount_out: 498449856817716432,
				},
				AtomicSwap {
					pool_id: POOL_ID_2,
					kind: PoolKind::StableSwap,
					asset_in: 1,
					asset_out: 2,
					amount_in: 498449856817716432,
					amount_out: 249212487980361585,
				},
				AtomicSwap {
					pool_id: POOL_ID_3,
					kind: PoolKind::StableSwap,
					asset_in: 2,
					asset_out: 3,
					amount_in: 249212487980361585,
					amount_out: 249206279805083014,
				},
			],
		}));
	})
}

#[test]
fn multiswap_should_work_mixed() {
	test_env().execute_with(|| {
		assert_ok!(create_pool_unb(PoolKind::Xyk, (ASSET_ID_3, ASSET_ID_1)));
		assert_ok!(create_pool_unb(PoolKind::StableSwap, (ASSET_ID_1, ASSET_ID_2)));
		assert_ok!(create_pool_unb(PoolKind::Xyk, (ASSET_ID_2, ASSET_ID_3)));

		assert_ok!(Market::multiswap_asset(
			origin(),
			vec![POOL_ID_1, POOL_ID_2, POOL_ID_3],
			ASSET_ID_3,
			UNIT,
			ASSET_ID_3,
			Zero::zero(),
		));

		println!("{:#?}", events());

		assert_last_event(RuntimeEvent::Market(Event::AssetsSwapped {
			who: AccountId::from(ALICE),
			swaps: vec![
				AtomicSwap {
					pool_id: POOL_ID_1,
					kind: PoolKind::Xyk,
					asset_in: ASSET_ID_3,
					asset_out: 1,
					amount_in: 997000000000000000,
					amount_out: 453305446940074565,
				},
				AtomicSwap {
					pool_id: POOL_ID_2,
					kind: PoolKind::StableSwap,
					asset_in: 1,
					asset_out: 2,
					amount_in: 453305446940074565,
					amount_out: 226642438818098374,
				},
				AtomicSwap {
					pool_id: POOL_ID_3,
					kind: PoolKind::Xyk,
					asset_in: 2,
					asset_out: 3,
					amount_in: 226642438818098374,
					amount_out: 110809799097802245,
				},
			],
		}));
	})
}

const DEFAULT_FEE_LOCK_PERIOD: u32 = 10;
const DEFAULT_FEE_LOCK_AMOUNT: Balance = MILLI_UNIT;
const DEFAULT_FEE_LOCK_SWAP_THRESHOLD: Balance = MICRO_UNIT;
const DEFAULT_MINT_AMOUNT: Balance = UNIT * 1_000_000_000;
const DEFAULT_POOL_AMOUNT: Balance = UNIT;

enum SwapKind {
    Sell,
    Buy
}

type TokensAdapter = orml_tokens::MultiTokenCurrencyAdapter<Runtime>;

pub fn create_new_token(who: &AccountId, amount: Balance) -> TokenId {
	TokensAdapter::create(who, amount).expect("Token creation failed")
}

pub fn mint_token(token_id: TokenId, who: &AccountId, amount: Balance) {
	TokensAdapter::mint(token_id, who, amount).expect("Token minting failed")
}

pub fn get_next_currency_id() -> TokenId {
	TokensAdapter::get_next_currency_id()
}
pub fn balance(token_id: TokenId, who: &AccountId) -> Balance {
	TokensAdapter::free_balance(token_id.into(), &who).into()
}

const MILLI_UNIT: Balance = UNIT/1000; 
const MICRO_UNIT: Balance = UNIT/1000000; 

#[test_case(
    SwapKind::Sell,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![0, 1, 2], // Which assets in the chain are whitelisted
	vec![(0, None), (1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    0, // Amount for output (asset_amount_out/min_amount_out)
	true, // swap_succeeds
	true, // commission_is_charged
	true, // bnb happens
	true // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_1"
)]
#[test_case(
    SwapKind::Sell,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![1, 2], // Which assets in the chain are whitelisted
	vec![(0, None), (1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    0, // Amount for output (asset_amount_out/min_amount_out)
	true, // swap_succeeds
	true, // commission_is_charged
	true, // bnb happens
	false // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_2"
)]
#[test_case(
    SwapKind::Sell,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![0, 1, 2], // Which assets in the chain are whitelisted
	vec![(1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    0, // Amount for output (asset_amount_out/min_amount_out)
	true, // swap_succeeds
	true, // commission_is_charged
	false, // bnb happens
	true // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_3"
)]
#[test_case(
    SwapKind::Sell,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![0, 1, 2], // Which assets in the chain are whitelisted
	vec![(0, None), (1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    UNIT, // Amount for output (asset_amount_out/min_amount_out)
	false, // swap_succeeds
	true, // commission_is_charged
	true, // bnb happens
	true // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_4"
)]
#[test_case(
    SwapKind::Buy,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![0, 1, 2], // Which assets in the chain are whitelisted
	vec![(0, None), (1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT * 2, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    MILLI_UNIT, // Amount for output (asset_amount_out/min_amount_out)
	true, // swap_succeeds
	true, // commission_is_charged
	true, // bnb happens
	true // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_5"
)]
#[test_case(
    SwapKind::Buy,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![1, 2], // Which assets in the chain are whitelisted
	vec![(0, None), (1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT * 2, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    MILLI_UNIT, // Amount for output (asset_amount_out/min_amount_out)
	true, // swap_succeeds
	true, // commission_is_charged
	true, // bnb happens
	false // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_6"
)]
#[test_case(
    SwapKind::Buy,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![0, 1, 2], // Which assets in the chain are whitelisted
	vec![(1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT * 2, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    MILLI_UNIT, // Amount for output (asset_amount_out/min_amount_out)
	true, // swap_succeeds
	true, // commission_is_charged
	false, // bnb happens
	true // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_7"
)]
#[test_case(
    SwapKind::Buy,
    3, // three base assets
	vec![(PoolKind::StableSwap, 0, None, 1, None), (PoolKind::Xyk, 1, None, 2, None)], // The test will make three assets and make pools are provided among them
	vec![0, 1, 2], // Which assets in the chain are whitelisted
	vec![(0, None), (1, None), (2, None)], // Which assets in the chain have a pool with native, and how much is the split first is native balance
    vec![0, 1], // swap_pool_list: Note that base pools and native with base pools are contigous so you can use native_with_base pools with the proper offset from base_asset_pool_id_start
    Some(0), // input asset id, None is native
    MILLI_UNIT/2, // Amount for input (asset_amount_in/max_amount_in)
    Some(2), // output asset id, None is native
    MILLI_UNIT, // Amount for output (asset_amount_out/min_amount_out)
	false, // swap_succeeds
	true, // commission_is_charged
	true, // bnb happens
	true // is_lockless: is the process_fee_lock called or unlock_fee
    ; "test_case_8"
)]
fn test_swaps(
    swap_kind: SwapKind,
    number_of_base_assets: TokenId,
    base_asset_pools: Vec<(PoolKind, TokenId, Option<Balance>, TokenId, Option<Balance>)>,
    whitelisted_assets: Vec<TokenId>,
    native_with_base_pools: Vec<(TokenId, Option<(Balance, Balance)>)>,
    swap_pool_list: Vec<TokenId>,
    swap_input_asset_id: Option<TokenId>,
    swap_input_amount: Balance,
    swap_output_asset_id: Option<TokenId>,
    swap_output_amount: Balance,
	swap_succeeds: bool,
	commission_is_charged: bool,
	bnb_happens: bool,
    is_lockless: bool,
){
    test_env().execute_with(||{
        let user = AccountId::from(ALICE);
        let user2 = AccountId::from(BOB);

        let native_asset_id = NATIVE_ASSET_ID;
        while get_next_currency_id() <= native_asset_id {
            let _ = create_new_token(&user2, DEFAULT_MINT_AMOUNT);
        }

		mint_token(native_asset_id, &user, DEFAULT_MINT_AMOUNT);

        let offset = get_next_currency_id();

        let base_token_id_start = get_next_currency_id();
        for _ in 0..number_of_base_assets{
            let _ = create_new_token(&user, DEFAULT_MINT_AMOUNT);
        }
        assert_eq!(base_token_id_start.saturating_add(number_of_base_assets), get_next_currency_id());

        let base_asset_pool_id_start = get_next_currency_id();
        for (pool_kind, asset_id_1, maybe_amount_1, asset_id_2, maybe_amount_2) in base_asset_pools.clone() {
            Market::create_pool(
                RuntimeOrigin::signed(user),
                pool_kind,
                base_token_id_start + asset_id_1,
                maybe_amount_1.unwrap_or(DEFAULT_POOL_AMOUNT),
                base_token_id_start + asset_id_2,
                maybe_amount_2.unwrap_or(DEFAULT_POOL_AMOUNT),
            ).unwrap();
        }
        assert_eq!(base_asset_pool_id_start.saturating_add(base_asset_pools.len() as u32), get_next_currency_id());

        FeeLock::update_fee_lock_metadata(
            RuntimeOrigin::root(),
            Some(DEFAULT_FEE_LOCK_PERIOD),
            Some(DEFAULT_FEE_LOCK_AMOUNT),
            Some(DEFAULT_FEE_LOCK_SWAP_THRESHOLD),
            Some(whitelisted_assets.iter().map(|x| (base_token_id_start + x, true)).collect())
        ).unwrap();

        let native_with_base_pool_id_start = get_next_currency_id();
        for (asset_id, maybe_amounts) in native_with_base_pools.clone() {
            Market::create_pool(
                RuntimeOrigin::signed(user),
                PoolKind::Xyk,
                native_asset_id,
                maybe_amounts.unwrap_or((DEFAULT_POOL_AMOUNT, DEFAULT_POOL_AMOUNT)).0,
                base_token_id_start + asset_id,
                maybe_amounts.unwrap_or((DEFAULT_POOL_AMOUNT, DEFAULT_POOL_AMOUNT)).1,
            ).unwrap();
        }
        assert_eq!(native_with_base_pool_id_start.saturating_add(native_with_base_pools.len() as u32), get_next_currency_id());

        if is_lockless {
            pallet_fee_lock::AccountFeeLockData::<Runtime>::mutate(user.clone(), |v| {
                v.total_fee_lock_amount = 200u128.into();
            });
        }

		let swap_input_asset_id = swap_input_asset_id.map(|x| base_token_id_start + x).unwrap_or(native_asset_id);
		let swap_output_asset_id = swap_output_asset_id.map(|x| base_token_id_start + x).unwrap_or(native_asset_id);
		let treasury_input_asset_amount_before = balance(swap_input_asset_id, &TreasuryAccountIdOf::<Runtime>::get());
		let treasury_native_asset_amount_before = balance(native_asset_id, &TreasuryAccountIdOf::<Runtime>::get());

		System::set_block_number(DEFAULT_FEE_LOCK_PERIOD + 1000);
		System::reset_events();
        // Perform the swap
        match swap_kind {
            SwapKind::Sell => {
                assert_ok!(Market::multiswap_asset(
                    RuntimeOrigin::signed(user),
                    swap_pool_list.iter().map(|x| base_asset_pool_id_start + x).collect(),
                    swap_input_asset_id,
                    swap_input_amount,
                    swap_output_asset_id,
                    swap_output_amount,
                ));
            },
            SwapKind::Buy => {
                assert_ok!(Market::multiswap_asset_buy(
                    RuntimeOrigin::signed(user),
                    swap_pool_list.iter().map(|x| base_asset_pool_id_start + x).collect(),
                    swap_output_asset_id,
                    swap_output_amount,
                    swap_input_asset_id,
                    swap_input_amount,
                ));
            }
        }
		// println!("{:?}", System::events());
		let treasury_input_asset_amount_after = balance(swap_input_asset_id, &TreasuryAccountIdOf::<Runtime>::get());
		let treasury_native_asset_amount_after = balance(native_asset_id, &TreasuryAccountIdOf::<Runtime>::get());

		// Verify

        match is_lockless {
            true => assert!(pallet_fee_lock::AccountFeeLockData::<Runtime>::get(user).total_fee_lock_amount.is_zero()),
            false => assert!(!pallet_fee_lock::AccountFeeLockData::<Runtime>::get(user).total_fee_lock_amount.is_zero()),
        };

		assert!(!(swap_succeeds ^ events().iter().any(|record| {
			match record {
				RuntimeEvent::Market(Event::AssetsSwapped{..}) => true,
				_ => false
			}
		})));

		assert!(!((!bnb_happens&&commission_is_charged)^(treasury_input_asset_amount_after>treasury_input_asset_amount_before)));
		assert!(!((bnb_happens&&commission_is_charged)^(treasury_native_asset_amount_after>treasury_native_asset_amount_before)));



    })
}

#[test]
fn swap_is_pre_validated() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		Market::create_pool(origin(), PoolKind::Xyk, ASSET_ID_1, UNIT, ASSET_ID_2, UNIT).unwrap();

		let res = Market::multiswap_asset(
				RuntimeOrigin::signed(who),
				vec![],
				ASSET_ID_1,
				UNIT,
				ASSET_ID_2,
				UNIT,
			);
		assert_eq!(
			res.err().unwrap(),
			pallet_market::Error::<Runtime>::SwapPrevalidation.into()
		);
		
		let res = Market::multiswap_asset(
			RuntimeOrigin::signed(who),
				vec![POOL_ID_1; <Runtime as pallet_market::Config>::MaxSwapListLength::get().saturating_add(1).try_into().unwrap()],
				ASSET_ID_1,
				UNIT,
				ASSET_ID_2,
				UNIT,
			);
		assert_eq!(
			res.err().unwrap(),
			pallet_market::Error::<Runtime>::SwapPrevalidation.into()
		);

		let res = Market::multiswap_asset(
			RuntimeOrigin::signed(who),
				vec![77u32.into()],
				ASSET_ID_1,
				UNIT,
				ASSET_ID_2,
				UNIT,
			);
		assert_eq!(
			res.err().unwrap(),
			pallet_market::Error::<Runtime>::SwapPrevalidation.into()
		);

		let res = Market::multiswap_asset(
			RuntimeOrigin::signed(who),
				vec![POOL_ID_1],
				77u32.into(),
				UNIT,
				ASSET_ID_2,
				UNIT,
			);
		assert_eq!(
			res.err().unwrap(),
			pallet_market::Error::<Runtime>::SwapPrevalidation.into()
		);

		let res = Market::multiswap_asset(
			RuntimeOrigin::signed(who),
				vec![POOL_ID_1],
				77u32.into(),
				UNIT,
				ASSET_ID_2,
				UNIT,
			);
		assert_eq!(
			res.err().unwrap(),
			pallet_market::Error::<Runtime>::SwapPrevalidation.into()
		);

		let res = Market::multiswap_asset(
			RuntimeOrigin::signed(who),
				vec![POOL_ID_1],
				ASSET_ID_1,
				1_000_000_000 * UNIT,
				ASSET_ID_2,
				UNIT,
			);
		assert_eq!(
			res.err().unwrap(),
			orml_tokens::Error::<Runtime>::BalanceTooLow.into()
		);

		let res = Market::multiswap_asset_buy(
				RuntimeOrigin::signed(who),
				vec![POOL_ID_1],
				ASSET_ID_2,
				UNIT,
				ASSET_ID_1,
				1_000_000_000 * UNIT,
			);
		assert_eq!(
			res.err().unwrap(),
			orml_tokens::Error::<Runtime>::BalanceTooLow.into()
		);
	})
}


