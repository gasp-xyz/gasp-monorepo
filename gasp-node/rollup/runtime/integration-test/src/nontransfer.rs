use crate::setup::*;

pub use mangata_support::utils::ConvertError;
pub use sp_runtime::{traits::MaybeConvert, DispatchError};

const ASSET_ID_1: TokenId = NATIVE_ASSET_ID + 1;
const ASSET_ID_2: TokenId = NATIVE_ASSET_ID + 2;
const POOL_ID: TokenId = ASSET_ID_2 + 1;
const ARB: [u8; 20] = hex_literal::hex!["fc741134c82b81b7ab7efbf334b0c90ff8dbf22c"];

fn test_env() -> TestExternalities {
	ExtBuilder {
		balances: vec![
			(AccountId::from(ALICE), NATIVE_ASSET_ID, 1000 * UNIT),
			(AccountId::from(BOB), NATIVE_ASSET_ID, 100 * UNIT),
			(AccountId::from(ARB), NATIVE_ASSET_ID, 100 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_1, 500 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_2, 500 * UNIT),
			(AccountId::from(BOB), ASSET_ID_1, 100 * UNIT),
		],
		..ExtBuilder::default()
	}
	.build()
}

fn root() -> RuntimeOrigin {
	RuntimeOrigin::root().into()
}

fn origin() -> RuntimeOrigin {
	RuntimeOrigin::signed(AccountId::from(ALICE))
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

fn assert_last_event_successful_swap() {
	let events = events();
	let last_event = events.last().expect("events expected").clone();
	let found = match last_event {
		RuntimeEvent::Market(pallet_market::Event::AssetsSwapped { .. }) => true,
		_ => false,
	};
	assert!(found, "expected event successful_swap is not equal to the last event {last_event:?}",);
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
fn test_tokens_calls_should_block() {
	let mut ext = test_env();
	ext.execute_with(|| {
		let who = AccountId::from(ALICE);
		let bob = AccountId::from(BOB);
		let amount = UNIT;

		assert_ok!(orml_tokens::Pallet::<Runtime>::mint(root(), NATIVE_ASSET_ID, who, 3 * amount),);
		assert_ok!(orml_tokens::Pallet::<Runtime>::set_balance(
			root(),
			who,
			NATIVE_ASSET_ID,
			3 * amount,
			3 * amount
		),);
		assert_ok!(orml_tokens::Pallet::<Runtime>::transfer(
			origin(),
			bob,
			NATIVE_ASSET_ID,
			amount
		),);
		assert_ok!(orml_tokens::Pallet::<Runtime>::force_transfer(
			root(),
			who,
			bob,
			NATIVE_ASSET_ID,
			amount
		),);
		assert_ok!(orml_tokens::Pallet::<Runtime>::transfer_keep_alive(
			origin(),
			bob,
			NATIVE_ASSET_ID,
			amount
		),);
		assert_ok!(orml_tokens::Pallet::<Runtime>::transfer_all(
			origin(),
			bob,
			NATIVE_ASSET_ID,
			false
		),);
	});
}

#[test]
fn test_tokens_calls_should_work_for_allow_listed() {
	let mut ext = test_env();
	ext.execute_with(|| {
		let who = AccountId::from(ALICE);
		let bob = AccountId::from(BOB);
		let amount = UNIT;

		assert_ok!(TransferMembers::add_member(root(), who));

		assert_ok!(orml_tokens::Pallet::<Runtime>::mint(root(), NATIVE_ASSET_ID, who, amount),);
		assert_ok!(orml_tokens::Pallet::<Runtime>::set_balance(
			root(),
			who,
			NATIVE_ASSET_ID,
			3 * amount,
			amount
		),);
		assert_ok!(orml_tokens::Pallet::<Runtime>::transfer(
			origin(),
			bob,
			NATIVE_ASSET_ID,
			amount
		));
		assert_ok!(orml_tokens::Pallet::<Runtime>::force_transfer(
			root(),
			who,
			bob,
			NATIVE_ASSET_ID,
			amount
		));
		assert_ok!(orml_tokens::Pallet::<Runtime>::transfer_keep_alive(
			origin(),
			bob,
			NATIVE_ASSET_ID,
			amount
		));
		assert_ok!(orml_tokens::Pallet::<Runtime>::transfer_all(
			origin(),
			bob,
			NATIVE_ASSET_ID,
			false
		));
	});
}

#[test]
fn test_market_should_not_block() {
	let mut ext = test_env();
	ext.execute_with(|| {
		let foundation = AccountId::from(ALICE);
		let bob = AccountId::from(BOB);
		let amount = 100 * UNIT;

		// user cannot create pool, foundation can
		assert_ok!(pallet_market::Pallet::<Runtime>::create_pool(
			RuntimeOrigin::signed(bob),
			pallet_market::PoolKind::Xyk,
			NATIVE_ASSET_ID,
			amount,
			ASSET_ID_1,
			amount,
		),);
		assert_ok!(pallet_market::Pallet::<Runtime>::create_pool(
			RuntimeOrigin::signed(foundation),
			pallet_market::PoolKind::Xyk,
			NATIVE_ASSET_ID,
			amount,
			ASSET_ID_2,
			amount,
		));

		// none can mint
		assert_ok!(pallet_market::Pallet::<Runtime>::mint_liquidity(
			RuntimeOrigin::signed(foundation),
			POOL_ID,
			NATIVE_ASSET_ID,
			amount,
			2 * amount,
		),);
		assert_ok!(pallet_market::Pallet::<Runtime>::mint_liquidity_fixed_amounts(
			RuntimeOrigin::signed(foundation),
			POOL_ID,
			(amount, 0u128),
			1u128,
		),);
		assert_err!(
			pallet_market::Pallet::<Runtime>::mint_liquidity_using_vesting_native_tokens(
				RuntimeOrigin::signed(foundation),
				POOL_ID,
				amount,
				amount,
			),
			pallet_market::Error::<Runtime>::NotAPromotedPool
		);
		assert_err!(
			pallet_market::Pallet::<Runtime>::mint_liquidity_using_vesting_native_tokens_by_vesting_index(
				RuntimeOrigin::signed(foundation),
				POOL_ID,
				0,
                None,
                amount,
			),
			pallet_market::Error::<Runtime>::NotAPromotedPool
		);

		// user cannot burn, foundation can
		assert_ok!(pallet_market::Pallet::<Runtime>::burn_liquidity(
			RuntimeOrigin::signed(bob),
			POOL_ID,
			amount,
			1u128,
			1u128,
		),);
		assert_ok!(pallet_market::Pallet::<Runtime>::burn_liquidity(
			RuntimeOrigin::signed(foundation),
			POOL_ID,
			UNIT,
			0,
			0,
		));

		// user, foundation and ARB bot can sell
		assert_ok!(pallet_market::Pallet::<Runtime>::multiswap_asset(
			RuntimeOrigin::signed(bob),
			vec![POOL_ID],
			NATIVE_ASSET_ID,
			UNIT,
			ASSET_ID_1,
			0,
		));
		assert_last_event_successful_swap();
		assert_ok!(pallet_market::Pallet::<Runtime>::multiswap_asset(
			RuntimeOrigin::signed(foundation),
			vec![POOL_ID],
			NATIVE_ASSET_ID,
			UNIT,
			ASSET_ID_1,
			0,
		));
		assert_last_event_successful_swap();
		assert_ok!(pallet_market::Pallet::<Runtime>::multiswap_asset(
			RuntimeOrigin::signed(AccountId::from(ARB)),
			vec![POOL_ID],
			NATIVE_ASSET_ID,
			UNIT,
			ASSET_ID_1,
			0,
		));
		assert_last_event_successful_swap();
		assert_ok!(pallet_market::Pallet::<Runtime>::multiswap_asset_buy(
			RuntimeOrigin::signed(bob),
			vec![POOL_ID],
			ASSET_ID_1,
			UNIT,
			NATIVE_ASSET_ID,
			UNIT * 2,
		));
		assert_last_event_successful_swap();
		assert_ok!(pallet_market::Pallet::<Runtime>::multiswap_asset_buy(
			RuntimeOrigin::signed(foundation),
			vec![POOL_ID],
			ASSET_ID_1,
			UNIT,
			NATIVE_ASSET_ID,
			UNIT * 2,
		));
		assert_last_event_successful_swap();
		assert_ok!(pallet_market::Pallet::<Runtime>::multiswap_asset_buy(
			RuntimeOrigin::signed(AccountId::from(ARB)),
			vec![POOL_ID],
			ASSET_ID_1,
			UNIT,
			NATIVE_ASSET_ID,
			UNIT * 2,
		));
		assert_last_event_successful_swap();
	});
}
