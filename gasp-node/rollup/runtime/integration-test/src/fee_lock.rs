use crate::setup::*;

use frame_support::dispatch::{DispatchInfo, PostDispatchInfo};
pub use frame_support::unsigned::TransactionValidityError;
use pallet_market::PoolKind;
use rollup_runtime::{
	config::{
		pallet_transaction_payment::{OnChargeHandler, ToAuthor},
		BnbAccountIdOf, TreasuryAccountIdOf,
	},
	fees::FEE_PRECISION,
	FeeLockTriggerTrait, OnChargeTransactionHandler,
};
use sp_runtime::transaction_validity::InvalidTransaction;

const ASSET_ID_1: u32 = 1;
const ASSET_ID_2: u32 = 2;
const POOL_ID: u32 = 3;
// runtime_config::fees::MarketTotalFee
const FEE: u128 = UNIT * 30_000_000 / FEE_PRECISION;
const TRSY_FEE_P: u128 = FEE * 3_333_333_334 / FEE_PRECISION;
const BNB_FEE: u128 = TRSY_FEE_P * 5_000_000_000 / FEE_PRECISION;
const TRSY_FEE: u128 = TRSY_FEE_P - BNB_FEE;
const POOL_FEE: u128 = FEE - TRSY_FEE - BNB_FEE;
const FEE_LOCK_PERIOD_LENGTH: u32 = 10;
const FEE_LOCK_AMOUNT: u128 = 10 * UNIT;

type Market = pallet_market::Pallet<Runtime>;

fn test_env() -> TestExternalities {
	ExtBuilder {
		balances: vec![
			(AccountId::from(ALICE), NATIVE_ASSET_ID, 1000 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_1, 100 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_2, 100 * UNIT),
			(AccountId::from(BOB), ASSET_ID_2, 100 * UNIT),
		],
		assets: vec![],
		..ExtBuilder::default()
	}
	.build()
}

// NATIVE_ASSET_ID & ASSET_ID_1 are used in TwoCurrencyOnChargeHandler for native fees
type Handler = OnChargeHandler<
	orml_tokens::MultiTokenCurrencyAdapter<Runtime>,
	ToAuthor<Runtime>,
	OnChargeTransactionHandler<Runtime>,
	FeeLock,
>;

fn origin() -> RuntimeOrigin {
	RuntimeOrigin::signed(AccountId::from(ALICE))
}

fn root() -> RuntimeOrigin {
	frame_system::RawOrigin::Root.into()
}

fn init_fee_lock(amount: Balance) {
	FeeLock::update_fee_lock_metadata(
		root(),
		Some(FEE_LOCK_PERIOD_LENGTH),
		Some(amount),
		Some(1),
		Some(vec![]),
	)
	.unwrap();
}

pub(crate) fn events() -> Vec<RuntimeEvent> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| match e {
			RuntimeEvent::FeeLock(_) |
			RuntimeEvent::Tokens(_) |
			RuntimeEvent::TransactionPayment(_) => Some(e),
			_ => None,
		})
		.collect::<Vec<_>>()
}

#[test]
fn unlock_fee_w_is_free() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		init_fee_lock(FEE_LOCK_AMOUNT);

		assert_ok!(pallet_fee_lock::Pallet::<Runtime>::process_fee_lock(&who));

		let current_block_number = System::block_number();
		System::set_block_number(current_block_number.saturating_add(FEE_LOCK_PERIOD_LENGTH));

		let liquidity_info = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::FeeLock(pallet_fee_lock::Call::unlock_fee {}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		)
		.unwrap();

		assert!(liquidity_info.is_none());
	})
}

#[test]
fn unlock_fee_not_w_is_blocked() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		assert!(pallet_fee_lock::FeeLockMetadata::<Runtime>::get().is_none());

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::FeeLock(pallet_fee_lock::Call::unlock_fee {}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::UnlockFee.into())
		);
	})
}

#[test]
fn swap_on_uninit_fee_lock_metadata_is_charged() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		assert!(pallet_fee_lock::FeeLockMetadata::<Runtime>::get().is_none());
		Market::create_pool(origin(), PoolKind::Xyk, ASSET_ID_1, UNIT, ASSET_ID_2, UNIT).unwrap();

		let liquidity_info = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![POOL_ID],
				asset_id_in: ASSET_ID_1,
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		)
		.unwrap();
		assert!(liquidity_info.is_some());
	})
}

#[test]
fn swap_w_is_liquidity_info_is_none() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		init_fee_lock(FEE_LOCK_AMOUNT);
		Market::create_pool(origin(), PoolKind::Xyk, ASSET_ID_1, UNIT, ASSET_ID_2, UNIT).unwrap();

		let liquidity_info = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![POOL_ID],
				asset_id_in: ASSET_ID_1,
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		)
		.unwrap();
		assert!(liquidity_info.is_none());
	})
}

#[test]
fn swap_gasless_is_pre_validated() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		init_fee_lock(FEE_LOCK_AMOUNT);
		Market::create_pool(origin(), PoolKind::Xyk, ASSET_ID_1, UNIT, ASSET_ID_2, UNIT).unwrap();

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![POOL_ID],
				asset_id_in: ASSET_ID_1,
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			1000u32.into(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::TippingNotAllowedForSwaps.into())
		);

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![],
				asset_id_in: ASSET_ID_1,
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
		);
		
		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![POOL_ID; <Runtime as pallet_market::Config>::MaxSwapListLength::get().saturating_add(1).try_into().unwrap()],
				asset_id_in: ASSET_ID_1,
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
		);

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![77u32.into()],
				asset_id_in: ASSET_ID_1,
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
		);

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![POOL_ID],
				asset_id_in: 77u32.into(),
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
		);

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![POOL_ID],
				asset_id_in: 77u32.into(),
				asset_amount_in: UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
		);

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset {
				swap_pool_list: vec![POOL_ID],
				asset_id_in: ASSET_ID_1,
				asset_amount_in: 1_000_000_000 * UNIT,
				asset_id_out: ASSET_ID_2,
				min_amount_out: UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
		);

		let res = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Market(pallet_market::Call::multiswap_asset_buy {
				swap_pool_list: vec![POOL_ID],
				asset_id_out: ASSET_ID_2,
				asset_amount_out: UNIT,
				asset_id_in: ASSET_ID_1,
				max_amount_in: 1_000_000_000 * UNIT,
			}),
			&DispatchInfo::default(),
			// fee here is imp or LiquidityInfo will be None
			// even if fee_lock_metadata is None
			1000u32.into(),
			Default::default(),
		);
		assert_eq!(
			res.err().unwrap(),
			TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
		);
	})
}
