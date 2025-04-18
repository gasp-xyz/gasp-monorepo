use std::vec;

use frame_support::{assert_err, assert_ok};
use sp_runtime::BoundedVec;

use crate::{assert_event_emitted, mock::*, Config, Error, Event, Pallet};

const UNIT: u128 = 10_u128.pow(18);

fn prep_pool() -> (AccountId, Balance, Balance) {
	let account: AccountId = 2;
	let amount: Balance = 1_000_000 * UNIT;
	let mint: Balance = 1_000 * UNIT;

	StableSwap::create_new_token(&account, amount);
	StableSwap::create_new_token(&account, amount);
	StableSwap::create_new_token(&account, amount);
	StableSwap::create_pool(
		RuntimeOrigin::signed(account),
		vec![0, 1, 2],
		vec![UNIT, UNIT, UNIT],
		200,
	)
	.unwrap();

	assert_ok!(StableSwap::add_liquidity(
		RuntimeOrigin::signed(account),
		3,
		vec![mint, mint, mint],
		0,
	));

	return (account, amount, mint)
}

// create pool tests

#[test]
fn create_pool_should_work() {
	new_test_ext().execute_with(|| {
		let account: AccountId = 2;
		let amount: Balance = 1_000_000_000;
		let rate: u128 = 10_u128.pow(18);

		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);

		StableSwap::create_pool(
			RuntimeOrigin::signed(account),
			vec![0, 1, 2],
			vec![rate, rate, rate],
			100,
		)
		.unwrap();

		assert!(<Test as Config>::Currency::exists(3));
	});
}

#[test]
fn create_pool_should_fail_on_nonexistent_asset() {
	new_test_ext().execute_with(|| {
		let account: AccountId = 2;
		let amount: Balance = 1_000_000_000;
		let rate: u128 = 10_u128.pow(18);

		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);

		assert_err!(
			StableSwap::create_pool(
				RuntimeOrigin::signed(account),
				vec![0, 1, 2],
				vec![rate, rate, rate],
				100
			),
			Error::<Test>::AssetDoesNotExist,
		);
	});
}

#[test]
fn create_pool_should_fail_on_same_asset() {
	new_test_ext().execute_with(|| {
		let account: AccountId = 2;
		let amount: Balance = 1_000_000_000;
		let rate: u128 = 10_u128.pow(18);

		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);

		assert_err!(
			StableSwap::create_pool(
				RuntimeOrigin::signed(account),
				vec![0, 1, 1],
				vec![rate, rate, rate],
				100
			),
			Error::<Test>::SameAsset,
		);
	});
}

#[test]
fn create_pool_should_fail_on_too_many_assets() {
	new_test_ext().execute_with(|| {
		let account: AccountId = 2;
		let amount: Balance = 1_000_000_000;
		let rate: u128 = 10_u128.pow(18);

		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);

		assert_err!(
			StableSwap::create_pool(
				RuntimeOrigin::signed(account),
				vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
				vec![rate, rate, rate],
				100
			),
			Error::<Test>::TooManyAssets,
		);
	});
}

#[test]
fn create_pool_should_fail_on_coeff_out_out_range() {
	new_test_ext().execute_with(|| {
		let account: AccountId = 2;
		let amount: Balance = 1_000_000_000;
		let rate: u128 = 10_u128.pow(18);

		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);

		assert_err!(
			StableSwap::create_pool(
				RuntimeOrigin::signed(account),
				vec![0, 1],
				vec![rate, rate, rate],
				0
			),
			Error::<Test>::AmpCoeffOutOfRange,
		);
		assert_err!(
			StableSwap::create_pool(
				RuntimeOrigin::signed(account),
				vec![0, 1],
				vec![rate, rate, rate],
				u128::MAX
			),
			Error::<Test>::AmpCoeffOutOfRange,
		);
	});
}

// add liquidity tests
#[test]
fn add_liquidity_should_work() {
	new_test_ext().execute_with(|| {
		let (account, _, mint) = prep_pool();

		// check first balanced add
		assert_event_emitted!(Event::LiquidityMinted {
			who: 2,
			pool_id: 3,
			amounts_provided: BoundedVec::truncate_from(vec![mint, mint, mint]),
			lp_token: 3,
			lp_token_minted: 3_000 * UNIT,
			total_supply: 3_000 * UNIT,
			fees: BoundedVec::truncate_from(vec![]),
		});

		assert_eq!(StableSwap::get_virtual_price(&3).unwrap(), 1 * UNIT);

		let amounts = vec![5 * mint, mint, 40 * mint];
		let expected = StableSwap::calc_lp_token_amount(&3, amounts.clone(), true).unwrap();

		// imbalanced add, should have fees
		assert_ok!(StableSwap::add_liquidity(
			RuntimeOrigin::signed(account),
			3,
			amounts.clone(),
			1,
		));

		assert_event_emitted!(Event::LiquidityMinted {
			who: 2,
			pool_id: 3,
			amounts_provided: BoundedVec::truncate_from(amounts),
			lp_token: 3,
			lp_token_minted: expected,
			total_supply: 3_000 * UNIT + expected,
			fees: BoundedVec::truncate_from(vec![0, 0, 0]),
		});

		// half of fees goes to treasury
		assert_eq!(StableSwap::balance(0, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(1, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(2, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::get_virtual_price(&3).unwrap(), UNIT);
	});
}

#[test]
fn add_liquidity_balanced_for_single_asset_minimal_fees() {
	new_test_ext().execute_with(|| {
		let account: AccountId = 2;
		let amount: Balance = 1_000_000 * UNIT;

		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);
		StableSwap::create_pool(
			RuntimeOrigin::signed(account),
			vec![0, 1, 2],
			vec![UNIT, UNIT, UNIT],
			200,
		)
		.unwrap();

		assert_ok!(StableSwap::add_liquidity(
			RuntimeOrigin::signed(account),
			3,
			vec![2 * UNIT, 3 * UNIT, 10 * UNIT],
			1,
		));

		let input_0 = 10_000 * UNIT;
		let reserves = StableSwap::get_pool_reserves(&3).unwrap();
		let rate = input_0 / reserves[0];
		let amounts = vec![input_0, reserves[1] * rate, reserves[2] * rate];
		let exp = Pallet::<Test>::calc_lp_token_amount(&3, amounts.clone(), true).unwrap();

		assert_ok!(StableSwap::add_liquidity(
			RuntimeOrigin::signed(account),
			3,
			amounts.clone(),
			1,
		));

		assert_event_emitted!(Event::LiquidityMinted {
			who: 2,
			pool_id: 3,
			amounts_provided: BoundedVec::truncate_from(amounts),
			lp_token: 3,
			lp_token_minted: exp,
			total_supply: 74881186777958934408404,
			fees: BoundedVec::truncate_from(vec![0, 0, 0]),
		});
	});
}

// remove liquidity
#[test]
fn remove_liquidity_one_asset_should_work() {
	new_test_ext().execute_with(|| {
		let (account, _, _) = prep_pool();
		let total_supply = StableSwap::total_supply(3);

		assert_ok!(StableSwap::remove_liquidity_one_asset(
			RuntimeOrigin::signed(account),
			3,
			0,
			10 * UNIT,
			0
		));

		assert_event_emitted!(Event::LiquidityBurnedOne {
			who: 2,
			pool_id: 3,
			asset_id: 0,
			amount: 9999833236889663201,
			burned_amount: 10 * UNIT,
			total_supply: total_supply - 10 * UNIT,
		});

		assert_eq!(StableSwap::balance(0, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(1, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(2, TreasuryAccount::get()), 0);
	});
}

#[test]
fn remove_liquidity_imbalanced_should_work() {
	new_test_ext().execute_with(|| {
		let (account, balance, mint) = prep_pool();
		let amounts = vec![mint / 100, mint / 50, mint / 200];

		assert_eq!(StableSwap::balance(0, 2), balance - mint);
		assert_eq!(StableSwap::balance(1, 2), balance - mint);
		assert_eq!(StableSwap::balance(2, 2), balance - mint);

		assert_ok!(StableSwap::remove_liquidity_imbalanced(
			RuntimeOrigin::signed(account),
			3,
			amounts.clone(),
			u128::MAX,
		));

		let total_supply = StableSwap::total_supply(3);
		assert_event_emitted!(Event::LiquidityBurned {
			who: 2,
			pool_id: 3,
			amounts: BoundedVec::truncate_from(amounts.clone()),
			burned_amount: 35000294130481446030,
			total_supply,
			fees: BoundedVec::truncate_from(vec![0, 0, 0,]),
		});

		// got what requested
		assert_eq!(StableSwap::balance(0, 2), balance - mint + amounts[0]);
		assert_eq!(StableSwap::balance(1, 2), balance - mint + amounts[1]);
		assert_eq!(StableSwap::balance(2, 2), balance - mint + amounts[2]);
		assert_eq!(StableSwap::balance(3, 2), 3 * mint - 35000294130481446030);

		assert_eq!(StableSwap::balance(0, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(1, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(2, TreasuryAccount::get()), 0);
	});
}

#[test]
fn remove_liquidity_to_zero_should_work() {
	new_test_ext().execute_with(|| {
		let (account, balance, mint) = prep_pool();
		let total_supply = StableSwap::total_supply(3);

		assert_ok!(StableSwap::remove_liquidity(
			RuntimeOrigin::signed(account),
			3,
			total_supply,
			vec![mint, mint, mint],
		));

		assert_event_emitted!(Event::LiquidityBurned {
			who: 2,
			pool_id: 3,
			amounts: BoundedVec::truncate_from(vec![mint, mint, mint]),
			burned_amount: 3 * mint,
			total_supply: 0,
			fees: BoundedVec::truncate_from(vec![]),
		});

		// all back
		assert_eq!(StableSwap::balance(0, 2), balance);
		assert_eq!(StableSwap::balance(1, 2), balance);
		assert_eq!(StableSwap::balance(2, 2), balance);
		assert_eq!(StableSwap::balance(3, 2), 0);

		assert_eq!(StableSwap::balance(0, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(1, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(2, TreasuryAccount::get()), 0);
	});
}

// swaps
#[test]
fn swap_should_work_dy() {
	new_test_ext().execute_with(|| {
		let (account, _, _) = prep_pool();

		let dy = StableSwap::get_dy(&3, 0, 2, 100 * UNIT).unwrap();

		assert_ok!(StableSwap::swap(RuntimeOrigin::signed(account), 3, 0, 2, 100 * UNIT, 0));

		assert_event_emitted!(Event::AssetsSwapped {
			who: 2,
			pool_id: 3,
			asset_in: 0,
			amount_in: 100 * UNIT,
			asset_out: 2,
			amount_out: dy
		});

		assert_eq!(StableSwap::balance(0, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(1, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(2, TreasuryAccount::get()), 0);
	});
}

#[test]
fn swap_should_work_dx() {
	new_test_ext().execute_with(|| {
		let (account, _, _) = prep_pool();

		let dx = StableSwap::get_dx(&3, 0, 2, 100 * UNIT).unwrap();

		assert_ok!(StableSwap::swap(RuntimeOrigin::signed(account), 3, 0, 2, dx, 0));

		assert_event_emitted!(Event::AssetsSwapped {
			who: 2,
			pool_id: 3,
			asset_in: 0,
			amount_in: dx,
			asset_out: 2,
			amount_out: 100 * UNIT,
		});

		assert_eq!(StableSwap::balance(0, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(1, TreasuryAccount::get()), 0);
		assert_eq!(StableSwap::balance(2, TreasuryAccount::get()), 0);
	});
}

#[test]
fn dy_dx_should_work() {
	new_test_ext().execute_with(|| {
		let account: AccountId = 2;
		let amount: Balance = 1_000_000 * UNIT;

		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);

		StableSwap::create_pool(
			RuntimeOrigin::signed(account),
			vec![0, 1],
			vec![5 * UNIT, 10 * UNIT],
			100_000,
		)
		.unwrap();

		assert_ok!(StableSwap::add_liquidity(
			RuntimeOrigin::signed(account),
			2,
			vec![10_000_000, 5_000_000],
			0,
		));

		let swap = 250_000;
		let dx = StableSwap::get_dx(&2, 0, 1, swap).unwrap();
		let dy = StableSwap::get_dy(&2, 0, 1, dx).unwrap();

		assert_eq!(dy, swap);
	});
}

#[test_case::test_matrix(
    // Pool configurations
    [200, 1000],
    [
        vec![1_000 * UNIT, 1_000 * UNIT],
        vec![500 * UNIT, 10_000 * UNIT]
    ],
    [
        vec![UNIT, UNIT],
        vec![UNIT, UNIT * 2]
    ],
    // Dy values to test
    [10 * UNIT, 50 * UNIT, 100 * UNIT, 200 * UNIT, UNIT / 10]
)]
fn test_dx_calculations_matrix(
	amp_coeff: u128,
	initial_liquidity: Vec<Balance>,
	rate_multipliers: Vec<Balance>,
	dy: Balance,
) {
	use insta::assert_yaml_snapshot;
	use serde::Serialize;

	#[derive(Serialize)]
	struct TestCase {
		amp_coeff: u128,
		initial_liquidity: Vec<Balance>,
		rate_multipliers: Vec<Balance>,
		dy: Balance,
		calculated_dx: Balance,
	}

	// Use a static vector to collect results across all test cases
	static RESULTS: std::sync::Mutex<Vec<TestCase>> = std::sync::Mutex::new(Vec::new());

	new_test_ext().execute_with(|| {
		// Create a pool with parameterized values
		let account: AccountId = 2;
		let amount: Balance = 1_000_000 * UNIT;

		// Create tokens and pool
		StableSwap::create_new_token(&account, amount);
		StableSwap::create_new_token(&account, amount);
		StableSwap::create_pool(
			RuntimeOrigin::signed(account),
			vec![0, 1],
			rate_multipliers.clone(),
			amp_coeff,
		)
		.unwrap();

		// Add initial liquidity
		assert_ok!(StableSwap::add_liquidity(
			RuntimeOrigin::signed(account),
			2,
			initial_liquidity.clone(),
			0,
		));

		// Calculate dx for given dy
		let calculated_dx = StableSwap::get_dx(&2, 0, 1, dy).unwrap();

		// Add result to collection
		RESULTS.lock().unwrap().push(TestCase {
			amp_coeff,
			initial_liquidity,
			rate_multipliers,
			dy,
			calculated_dx,
		});

		// Create snapshot on the last test case (2 amp coeffs × 2 liquidity levels × 2 rate configs × 5 dy values = 40 total)
		if RESULTS.lock().unwrap().len() == 40 {
			let mut results = std::mem::take(&mut *RESULTS.lock().unwrap());
			// Sort results by calculated_dx only
			results.sort_by(|a, b| a.calculated_dx.cmp(&b.calculated_dx));
			assert_yaml_snapshot!("dx_calculations_matrix", results);
		}
	});
}
