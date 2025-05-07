use crate::setup::*;

use frame_support::dispatch::{DispatchInfo, PostDispatchInfo};
pub use frame_support::unsigned::TransactionValidityError;
use orml_asset_registry::{IdToL1Asset, L1AssetToId};
use pallet_market::PoolKind;
use rollup_runtime::{
	config::{
		pallet_transaction_payment::{OnChargeHandler, ToAuthor},
		BnbAccountIdOf, TreasuryAccountIdOf,
	},
	fees::FEE_PRECISION,
	AssetMetadata, FeeLockTriggerTrait, L1Asset, OnChargeTransactionHandler,
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
const NATIVE_ASSET_ID_ADDRESS: L1Asset = L1Asset::Ethereum([7u8; 20]);

type Market = pallet_market::Pallet<Runtime>;

fn test_env() -> TestExternalities {
	ExtBuilder {
		balances: vec![
			(AccountId::from(ALICE), NATIVE_ASSET_ID, 1000 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_1, 100 * UNIT),
			(AccountId::from(ALICE), ASSET_ID_2, 100 * UNIT),
			(AccountId::from(BOB), ASSET_ID_2, 100 * UNIT),
		],
		assets: vec![(
			NATIVE_ASSET_ID,
			AssetMetadata {
				decimals: 18,
				name: BoundedVec::truncate_from("Dummy token".as_bytes().to_vec()),
				symbol: BoundedVec::truncate_from("DUMMY".as_bytes().to_vec()),
				existential_deposit: 0,
				additional: CustomMetadata { xcm: None, xyk: None },
			},
			Some(NATIVE_ASSET_ID_ADDRESS),
		)],
		..ExtBuilder::default()
	}
	.build()
}

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
fn register_l1_asset() {
	test_env().execute_with(|| {
		let metadata = AssetMetadataOf {
			decimals: 18,
			name: BoundedVec::truncate_from("Dummy token".as_bytes().to_vec()),
			symbol: BoundedVec::truncate_from("DUMMY".as_bytes().to_vec()),
			existential_deposit: 0,
			additional: CustomMetadata { xcm: None, xyk: None },
		};

		assert_eq!(IdToL1Asset::<Runtime>::get(NATIVE_ASSET_ID), Some(NATIVE_ASSET_ID_ADDRESS));

		assert_eq!(L1AssetToId::<Runtime>::get(NATIVE_ASSET_ID_ADDRESS), Some(NATIVE_ASSET_ID));

		let updated_asset_id = L1Asset::Ethereum([8u8; 20]);
		AssetRegistry::update_l1_asset_data(
			root(),
			NATIVE_ASSET_ID,
			Some(updated_asset_id.clone()),
		)
		.unwrap();

		assert_eq!(IdToL1Asset::<Runtime>::get(NATIVE_ASSET_ID), Some(updated_asset_id.clone()));

		assert_eq!(L1AssetToId::<Runtime>::get(updated_asset_id), Some(NATIVE_ASSET_ID));

		assert_eq!(L1AssetToId::<Runtime>::get(NATIVE_ASSET_ID_ADDRESS), None);
	})
}
