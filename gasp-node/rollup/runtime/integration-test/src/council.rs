use crate::setup::*;

use frame_support::dispatch::{DispatchInfo};
use rollup_runtime::{ Council,
	config::{
		pallet_transaction_payment::{OnChargeHandler, ToAuthor},
	},
	OnChargeTransactionHandler,
};

fn test_env() -> TestExternalities {
	ExtBuilder {
		balances: vec![
			(AccountId::from(ALICE), NATIVE_ASSET_ID, 100 * UNIT),
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

#[test]
fn council_call_by_non_member_is_charged() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		let fee = 1000;
		assert!(!Council::is_member(&who));
		let withdrawn = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Council(pallet_collective_mangata::Call::vote {
				proposal: Default::default(),
				index: Default::default(),
				approve: Default::default(),
			}),
			&DispatchInfo::default(),
			fee,
			0,
		).unwrap();
		assert!(withdrawn.is_some());
	})
}

#[test]
fn council_call_by_member_is_not_charged() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		let fee = 1000;
		assert_ok!(Council::set_members(RuntimeOrigin::root(), vec![who], None, Default::default()));
		assert!(Council::is_member(&who));
		let withdrawn = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Council(pallet_collective_mangata::Call::vote {
				proposal: Default::default(),
				index: Default::default(),
				approve: Default::default(),
			}),
			&DispatchInfo::default(),
			fee,
			0,
		).unwrap();
		assert!(withdrawn.is_none());
	})
}

#[test]
fn non_council_call_by_non_member_is_charged() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		let fee = 1000;
		assert!(!Council::is_member(&who));
		let withdrawn = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Tokens(orml_tokens::Call::transfer {
				dest: Default::default(),
				currency_id: Default::default(),
				amount: Default::default(),
			}),
			&DispatchInfo::default(),
			fee,
			0,
		).unwrap();
		assert!(withdrawn.is_some());
	})
}

#[test]
fn non_council_call_by_member_is_charged() {
	test_env().execute_with(|| {
		let who = AccountId::from(ALICE);
		let fee = 1000;
		assert_ok!(Council::set_members(RuntimeOrigin::root(), vec![who], None, Default::default()));
		assert!(Council::is_member(&who));
		let withdrawn = <Handler as OnChargeTransaction<Runtime>>::withdraw_fee(
			&who,
			&RuntimeCall::Tokens(orml_tokens::Call::transfer {
				dest: Default::default(),
				currency_id: Default::default(),
				amount: Default::default(),
			}),
			&DispatchInfo::default(),
			fee,
			0,
		).unwrap();
		assert!(withdrawn.is_some());
	})
}