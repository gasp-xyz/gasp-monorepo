#![cfg(not(feature = "runtime-benchmarks"))]
use super::*;
use mock::*;

use frame_support::{assert_err, assert_ok};
use serial_test::serial;
use sp_runtime::traits::BadOrigin;
use test_case::test_case;

const USER_ID: u64 = 0;
const PROVISION_USER1_ID: u64 = 200;
const PROVISION_USER2_ID: u64 = 201;
const ANOTHER_USER_ID: u64 = 100;
const YET_ANOTHER_USER_ID: u64 = 900;
const INITIAL_AMOUNT: u128 = 1_000_000;
const DUMMY_ID: u32 = 2;
const LIQ_TOKEN_ID: TokenId = 10_u32;
const LIQ_TOKEN_AMOUNT: Balance = 1_000_000_u128;
const DEFAULT_RATIO: (u128, u128) = (1_u128, 10_000_u128);
const POOL_CREATE_DUMMY_RETURN_VALUE: Option<(TokenId, Balance)> =
	Some((LIQ_TOKEN_ID, LIQ_TOKEN_AMOUNT));

fn set_up() {
	// for backwards compatibility
	ArchivedBootstrap::<mock::Test>::mutate(|v| {
		v.push(Default::default());
	});
	let mga_id = Bootstrap::create_new_token(&USER_ID, INITIAL_AMOUNT);
	let ksm_id = Bootstrap::create_new_token(&USER_ID, INITIAL_AMOUNT);
	let dummy_id = Bootstrap::create_new_token(&USER_ID, INITIAL_AMOUNT);
	assert_eq!(mga_id, MGAId::get());
	assert_eq!(ksm_id, KSMId::get());
	assert_eq!(dummy_id, DUMMY_ID);
	assert_eq!(INITIAL_AMOUNT, Bootstrap::balance(KSMId::get(), USER_ID));
	assert_eq!(INITIAL_AMOUNT, Bootstrap::balance(MGAId::get(), USER_ID));
}

fn jump_to_whitelist_phase() {
	let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
	pool_exists_mock.expect().return_const(false);
	Bootstrap::schedule_bootstrap(
		RuntimeOrigin::root(),
		KSMId::get(),
		MGAId::get(),
		10_u32.into(),
		Some(10),
		10,
		Some(DEFAULT_RATIO),
		false,
	)
	.unwrap();
	Bootstrap::on_initialize(15_u32.into());
	assert_eq!(BootstrapPhase::Whitelist, Phase::<Test>::get());
}

fn jump_to_public_phase() {
	let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
	pool_exists_mock.expect().return_const(false);

	Bootstrap::schedule_bootstrap(
		RuntimeOrigin::root(),
		KSMId::get(),
		MGAId::get(),
		10_u32.into(),
		Some(10),
		10,
		Some(DEFAULT_RATIO),
		false,
	)
	.unwrap();
	Bootstrap::on_initialize(25_u32.into());
	assert_eq!(BootstrapPhase::Public, Phase::<Test>::get());
}

#[test]
#[serial]
fn do_not_allow_for_provision_in_unsupported_currency() {
	new_test_ext().execute_with(|| {
		set_up();
		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), DUMMY_ID, 1000),
			Error::<Test>::UnsupportedTokenId
		)
	});
}

#[test]
#[serial]
fn test_first_provision_with_ksm_fails() {
	new_test_ext().execute_with(|| {
		set_up();
		jump_to_public_phase();

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 1),
			Error::<Test>::FirstProvisionInSecondTokenId
		);
	});
}

#[test]
#[serial]
fn test_event_is_published_after_successful_provision() {
	new_test_ext().execute_with(|| {
		set_up();
		jump_to_public_phase();
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1).unwrap();

		let event = crate::mock::RuntimeEvent::Bootstrap(crate::Event::<Test>::Provisioned(
			MGAId::get(),
			1,
		));

		assert!(System::events().iter().any(|record| record.event == event));
	});
}

#[test]
#[serial]
fn test_dont_allow_for_ksm_donation_before_minimal_valuation_fro_mga_is_provided() {
	new_test_ext().execute_with(|| {
		set_up();

		jump_to_public_phase();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1).unwrap();
		assert_eq!(1, Bootstrap::provisions(USER_ID, MGAId::get()));
		assert_eq!((1, 0), Bootstrap::valuations());

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 1),
			Error::<Test>::ValuationRatio
		);

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 9999).unwrap();
		assert_eq!(10000, Bootstrap::provisions(USER_ID, MGAId::get()));
		assert_eq!((10000, 0), Bootstrap::valuations());

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 1).unwrap();
		assert_eq!(1, Bootstrap::provisions(USER_ID, KSMId::get()));
		assert_eq!((10000, 1), Bootstrap::valuations());

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 1),
			Error::<Test>::ValuationRatio
		);
	});
}

#[test]
#[serial]
fn test_donation_in_supported_tokens() {
	new_test_ext().execute_with(|| {
		set_up();
		jump_to_public_phase();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 10000).unwrap();
		assert_eq!(10000, Bootstrap::provisions(USER_ID, MGAId::get()));

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 1).unwrap();
		assert_eq!(1, Bootstrap::provisions(USER_ID, KSMId::get()));

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 20000).unwrap();
		assert_eq!(30000, Bootstrap::provisions(USER_ID, MGAId::get()));

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 2).unwrap();
		assert_eq!(3, Bootstrap::provisions(USER_ID, KSMId::get()));
	});
}

#[test]
#[serial]
fn test_donation_with_more_tokens_than_available() {
	new_test_ext().execute_with(|| {
		set_up();

		jump_to_public_phase();

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), INITIAL_AMOUNT * 2),
			Error::<Test>::NotEnoughAssets
		);

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), INITIAL_AMOUNT * 2),
			Error::<Test>::NotEnoughAssets
		);
	});
}

#[test]
#[serial]
fn test_prevent_provisions_in_before_start_phase() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			20,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), INITIAL_AMOUNT * 2),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
#[serial]
fn test_fail_scheudle_bootstrap_with_same_token() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				100,
				100,
				100_u32.into(),
				Some(10),
				20,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::SameToken
		);
	});
}

#[test]
#[serial]
fn test_prevent_schedule_bootstrap_with_pair_that_does_not_exists() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				100,
				101,
				100_u32.into(),
				Some(10),
				20,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::TokenIdDoesNotExists
		);
	});
}

#[test]
#[serial]
fn test_prevent_provisions_in_finished_phase() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			20,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Phase::<Test>::put(BootstrapPhase::Finished);

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), INITIAL_AMOUNT * 2),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
#[serial]
fn test_prevent_non_whitelited_account_to_provision_in_whitelisted_phase() {
	new_test_ext().execute_with(|| {
		set_up();

		jump_to_whitelist_phase();

		assert!(!Bootstrap::is_whitelisted(&USER_ID));
		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 1000),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
#[serial]
fn test_allow_non_whitelited_account_to_provision_in_whitelisted_phase_with_mga() {
	new_test_ext().execute_with(|| {
		set_up();

		jump_to_whitelist_phase();

		assert!(!Bootstrap::is_whitelisted(&USER_ID));
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1000).unwrap();
	});
}

#[test]
#[serial]
fn test_whitelist_account_deposit_event() {
	new_test_ext().execute_with(|| {
		set_up();
		Bootstrap::whitelist_accounts(RuntimeOrigin::root(), vec![USER_ID]).unwrap();

		assert!(System::events().iter().any(|record| record.event ==
			crate::mock::RuntimeEvent::Bootstrap(crate::Event::<Test>::AccountsWhitelisted)));
	});
}

#[test]
#[serial]
fn test_incremental_whitliested_donation() {
	new_test_ext().execute_with(|| {
		set_up();

		jump_to_whitelist_phase();

		Bootstrap::whitelist_accounts(RuntimeOrigin::root(), vec![USER_ID]).unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1000).unwrap();

		Bootstrap::transfer(MGAId::get(), USER_ID, ANOTHER_USER_ID, 10_000).unwrap();
		Bootstrap::whitelist_accounts(RuntimeOrigin::root(), vec![ANOTHER_USER_ID]).unwrap();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1000).unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(ANOTHER_USER_ID), MGAId::get(), 1000).unwrap();
		assert_ne!(USER_ID, ANOTHER_USER_ID);
	});
}

#[test]
#[serial]
fn test_bootstrap_promotion_can_be_updated() {
	new_test_ext().execute_with(|| {
		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock.expect().times(1).return_const(POOL_CREATE_DUMMY_RETURN_VALUE);

		let mock = MockRewardsApi::enable_context();
		mock.expect().times(1).return_const(());

		set_up();
		assert_ok!(Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(1),
			9,
			Some(DEFAULT_RATIO),
			false,
		));

		System::set_block_number(109);
		Bootstrap::on_initialize(109_u32.into());

		assert_ok!(Bootstrap::update_promote_bootstrap_pool(RuntimeOrigin::root(), true));

		System::set_block_number(111);
		Bootstrap::on_initialize(111_u32.into());

		assert_err!(
			Bootstrap::update_promote_bootstrap_pool(RuntimeOrigin::root(), false),
			Error::<Test>::BootstrapFinished
		);
	});
}

#[test]
#[serial]
fn test_scheduled_bootstrap_can_be_updated() {
	new_test_ext().execute_with(|| {
		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		set_up();
		assert_ok!(Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(1),
			9,
			Some(DEFAULT_RATIO),
			false,
		));

		System::set_block_number(80);
		Bootstrap::on_initialize(80_u32.into());

		assert_ok!(Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(1),
			9,
			Some((100, 10)),
			false,
		));

		System::set_block_number(95);
		Bootstrap::on_initialize(95_u32.into());

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				100_u32.into(),
				Some(1),
				9,
				Some((1000, 1)),
				false,
			),
			Error::<Test>::TooLateToUpdateBootstrap
		);
	});
}

#[test]
#[serial]
fn test_scheduled_bootstrap_can_be_cancelled() {
	new_test_ext().execute_with(|| {
		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		set_up();
		assert_ok!(Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(1),
			9,
			Some(DEFAULT_RATIO),
			false,
		));

		System::set_block_number(95);
		Bootstrap::on_initialize(95_u32.into());

		assert_err!(
			Bootstrap::cancel_bootstrap(RuntimeOrigin::root()),
			Error::<Test>::TooLateToUpdateBootstrap
		);

		System::set_block_number(80);
		Bootstrap::on_initialize(80_u32.into());

		assert_ok!(Bootstrap::cancel_bootstrap(RuntimeOrigin::root()));
	});
}

#[test]
#[serial]
fn test_non_root_user_can_not_schedule_bootstrap() {
	new_test_ext().execute_with(|| {
		set_up();
		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::signed(USER_ID),
				KSMId::get(),
				MGAId::get(),
				0_u32.into(),
				Some(1),
				1,
				Some(DEFAULT_RATIO),
				false,
			),
			BadOrigin
		);
	});
}

#[test]
#[serial]
fn test_non_root_user_can_not_whitelist_accounts() {
	new_test_ext().execute_with(|| {
		set_up();
		assert_err!(
			Bootstrap::whitelist_accounts(RuntimeOrigin::signed(USER_ID), vec![],),
			BadOrigin
		);
	});
}

#[test]
#[serial]
fn test_only_root_can_whitelist_accounts() {
	new_test_ext().execute_with(|| {
		set_up();
		Bootstrap::whitelist_accounts(RuntimeOrigin::root(), vec![]).unwrap();
	});
}

#[test]
#[serial]
fn test_ido_start_cannot_happen_in_the_past() {
	new_test_ext().execute_with(|| {
		set_up();
		System::set_block_number(1000);
		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				999_u32.into(),
				Some(1),
				1,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::BootstrapStartInThePast
		);
	});
}

#[test]
#[serial]
fn test_ido_start_can_not_be_initialize_with_0_ratio() {
	new_test_ext().execute_with(|| {
		set_up();
		System::set_block_number(10);
		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				999_u32.into(),
				Some(1),
				1,
				Some((1, 0)),
				false,
			),
			Error::<Test>::WrongRatio
		);
		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				999_u32.into(),
				Some(1),
				1,
				Some((0, 1)),
				false,
			),
			Error::<Test>::WrongRatio
		);
	});
}

#[test]
#[serial]
fn test_can_schedule_bootstrap_with_whitelist_phase_length_equal_zero() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		assert_ok!(Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(0),
			1,
			Some(DEFAULT_RATIO),
			false,
		));
	});
}

#[test]
#[serial]
fn test_cannot_schedule_bootstrap_with_public_phase_length_equal_zero() {
	new_test_ext().execute_with(|| {
		set_up();
		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				100_u32.into(),
				Some(1),
				0,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::PhaseLengthCannotBeZero
		);
	});
}

#[test]
#[serial]
fn test_bootstrap_can_be_modified_only_before_its_started() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			50_u32.into(),
			Some(10),
			20,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			20,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::on_initialize(100_u32.into());

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				100_u32.into(),
				Some(10),
				20,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::AlreadyStarted
		);

		assert_eq!(Some((100_u32.into(), 10_u32, 20_u32, DEFAULT_RATIO)), Bootstrap::config());
	});
}

#[test]
#[serial]
fn test_bootstrap_state_transitions() {
	new_test_ext().execute_with(|| {
		set_up();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		const BOOTSTRAP_WHITELIST_START: u64 = 100;
		const BOOTSTRAP_PUBLIC_START: u64 = 110;
		const BOOTSTRAP_FINISH: u64 = 130;

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock.expect().times(1).return_const(POOL_CREATE_DUMMY_RETURN_VALUE);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			BOOTSTRAP_WHITELIST_START,
			Some((BOOTSTRAP_PUBLIC_START - BOOTSTRAP_WHITELIST_START).try_into().unwrap()),
			(BOOTSTRAP_FINISH - BOOTSTRAP_PUBLIC_START).try_into().unwrap(),
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		for i in 1..BOOTSTRAP_WHITELIST_START {
			Bootstrap::on_initialize(i);
			assert_eq!(Bootstrap::phase(), BootstrapPhase::BeforeStart);
		}

		Bootstrap::on_initialize(BOOTSTRAP_WHITELIST_START);
		assert_eq!(Bootstrap::phase(), BootstrapPhase::Whitelist);

		for i in BOOTSTRAP_WHITELIST_START..BOOTSTRAP_PUBLIC_START {
			Bootstrap::on_initialize(i);
			assert_eq!(Bootstrap::phase(), BootstrapPhase::Whitelist);
		}

		Bootstrap::on_initialize(BOOTSTRAP_PUBLIC_START);
		assert_eq!(Bootstrap::phase(), BootstrapPhase::Public);

		for i in BOOTSTRAP_PUBLIC_START..BOOTSTRAP_FINISH {
			Bootstrap::on_initialize(i);
			assert_eq!(Bootstrap::phase(), BootstrapPhase::Public);
		}

		Bootstrap::on_initialize(BOOTSTRAP_FINISH);
		assert_eq!(Bootstrap::phase(), BootstrapPhase::Finished);
	});
}

#[test]
#[serial]
fn test_bootstrap_state_transitions_when_on_initialized_is_not_called() {
	new_test_ext().execute_with(|| {
		set_up();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock.expect().times(1).return_const(POOL_CREATE_DUMMY_RETURN_VALUE);

		const BOOTSTRAP_WHITELIST_START: u64 = 100;
		const BOOTSTRAP_PUBLIC_START: u64 = 110;
		const BOOTSTRAP_FINISH: u64 = 130;
		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			BOOTSTRAP_WHITELIST_START,
			Some((BOOTSTRAP_PUBLIC_START - BOOTSTRAP_WHITELIST_START).try_into().unwrap()),
			(BOOTSTRAP_FINISH - BOOTSTRAP_PUBLIC_START).try_into().unwrap(),
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		assert_eq!(Bootstrap::phase(), BootstrapPhase::BeforeStart);
		Bootstrap::on_initialize(200);
		assert_eq!(Bootstrap::phase(), BootstrapPhase::Finished);
	});
}

#[test]
#[serial]
fn test_bootstrap_schedule_overflow() {
	new_test_ext().execute_with(|| {
		set_up();

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				u64::MAX,
				Some(u32::MAX),
				1_u32,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::MathOverflow
		);

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				u64::MAX,
				Some(1_u32),
				u32::MAX,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::MathOverflow
		);

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				u64::MAX,
				Some(u32::MAX),
				u32::MAX,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::MathOverflow
		);
	});
}
#[serial]
fn test_do_not_allow_for_creating_starting_bootstrap_for_existing_pool() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(true);

		assert_err!(
			Bootstrap::schedule_bootstrap(
				RuntimeOrigin::root(),
				KSMId::get(),
				MGAId::get(),
				100_u32.into(),
				Some(10),
				10,
				Some(DEFAULT_RATIO),
				false,
			),
			Error::<Test>::PoolAlreadyExists
		);
	});
}

#[test]
#[serial]
fn test_crate_pool_is_called_with_proper_arguments_after_bootstrap_finish() {
	new_test_ext().execute_with(|| {
		set_up();

		use mockall::predicate::eq;
		const KSM_PROVISON: Balance = 30;
		const MGA_PROVISON: Balance = 500_000;

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.with(
				eq(Bootstrap::vault_address()),
				eq(KSMId::get()),
				eq(KSM_PROVISON),
				eq(MGAId::get()),
				eq(MGA_PROVISON),
			)
			.times(1)
			.return_const(POOL_CREATE_DUMMY_RETURN_VALUE);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::on_initialize(110_u32.into());
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), MGA_PROVISON).unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), KSM_PROVISON).unwrap();
		Bootstrap::on_initialize(120_u32.into());
	});
}

#[test]
#[serial]
fn test_cannot_claim_liquidity_tokens_when_bootstrap_is_not_finished() {
	new_test_ext().execute_with(|| {
		set_up();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Whitelist, Phase::<Test>::get());

		Bootstrap::on_initialize(110_u32.into());
		assert_eq!(BootstrapPhase::Public, Phase::<Test>::get());

		assert_err!(
			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)),
			Error::<Test>::NotFinishedYet
		);
	});
}

#[test]
#[serial]
fn test_rewards_are_distributed_properly_with_single_user() {
	new_test_ext().execute_with(|| {
		use std::sync::{Arc, Mutex};
		set_up();

		const KSM_PROVISON: Balance = 10;
		const MGA_PROVISON: Balance = 100_000;
		let liq_token_id: Arc<Mutex<TokenId>> = Arc::new(Mutex::new(0_u32));
		let ref_liq_token_id = liq_token_id.clone();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				*(ref_liq_token_id.lock().unwrap()) = id;
				Some((id, issuance))
			});

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Whitelist, Phase::<Test>::get());

		Bootstrap::on_initialize(110_u32.into());
		assert_eq!(BootstrapPhase::Public, Phase::<Test>::get());

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), MGA_PROVISON).unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), KSM_PROVISON).unwrap();

		Bootstrap::on_initialize(120_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
		let liquidity_token_id = *(liq_token_id.lock().unwrap());
		let liquidity_token_amount = (mga_valuation + ksm_valuation) / 2;

		assert_eq!(
			Bootstrap::balance(liquidity_token_id, Bootstrap::vault_address()),
			liquidity_token_amount
		);
		assert_eq!(Bootstrap::minted_liquidity(), (liquidity_token_id, liquidity_token_amount));

		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)).unwrap();

		assert_eq!(Bootstrap::claimed_rewards(USER_ID, MGAId::get()), liquidity_token_amount / 2);

		assert_eq!(Bootstrap::claimed_rewards(USER_ID, KSMId::get()), liquidity_token_amount / 2);

		assert_eq!(
			Bootstrap::balance(liquidity_token_id, USER_ID),
			// KSM rewards                  MGA rewards
			(liquidity_token_amount / 2) + (liquidity_token_amount / 2)
		);
	});
}

#[test]
#[serial]
fn test_rewards_are_distributed_properly_with_multiple_user() {
	new_test_ext().execute_with(|| {
		use std::sync::{Arc, Mutex};
		set_up();

		let provisioned_ev = |id, amount| {
			crate::mock::RuntimeEvent::Bootstrap(crate::Event::<Test>::Provisioned(id, amount))
		};

		let rewards_claimed_ev = |id, amount| {
			crate::mock::RuntimeEvent::Bootstrap(crate::Event::<Test>::RewardsClaimed(id, amount))
		};

		const USER_KSM_PROVISON: Balance = 15;
		const USER_MGA_PROVISON: Balance = 400_000;
		const ANOTHER_USER_KSM_PROVISON: Balance = 20;
		const ANOTHER_USER_MGA_PROVISON: Balance = 100_000;
		let liq_token_id: Arc<Mutex<TokenId>> = Arc::new(Mutex::new(0_u32));
		let ref_liq_token_id = liq_token_id.clone();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				*(ref_liq_token_id.lock().unwrap()) = id;
				Some((id, issuance))
			});

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Whitelist, Phase::<Test>::get());

		Bootstrap::on_initialize(110_u32.into());
		assert_eq!(BootstrapPhase::Public, Phase::<Test>::get());

		Bootstrap::transfer(MGAId::get(), USER_ID, ANOTHER_USER_ID, 500_000).unwrap();
		Bootstrap::transfer(KSMId::get(), USER_ID, ANOTHER_USER_ID, 500_000).unwrap();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), USER_MGA_PROVISON)
			.unwrap();
		Bootstrap::provision(
			RuntimeOrigin::signed(ANOTHER_USER_ID),
			MGAId::get(),
			ANOTHER_USER_MGA_PROVISON,
		)
		.unwrap();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), USER_KSM_PROVISON)
			.unwrap();
		Bootstrap::provision(
			RuntimeOrigin::signed(ANOTHER_USER_ID),
			KSMId::get(),
			ANOTHER_USER_KSM_PROVISON,
		)
		.unwrap();

		assert!(System::events()
			.iter()
			.any(|record| record.event == provisioned_ev(MGAId::get(), USER_MGA_PROVISON)));
		assert!(System::events()
			.iter()
			.any(|record| record.event == provisioned_ev(KSMId::get(), USER_KSM_PROVISON)));
		assert!(System::events()
			.iter()
			.any(|record| record.event == provisioned_ev(MGAId::get(), ANOTHER_USER_MGA_PROVISON)));
		assert!(System::events()
			.iter()
			.any(|record| record.event == provisioned_ev(KSMId::get(), ANOTHER_USER_KSM_PROVISON)));

		Bootstrap::on_initialize(120_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
		assert_eq!(mga_valuation, 500_000);
		assert_eq!(ksm_valuation, 35);
		let liquidity_token_id = *(liq_token_id.lock().unwrap());
		let liquidity_token_amount = (mga_valuation + ksm_valuation) / 2;

		assert_eq!(
			Bootstrap::balance(liquidity_token_id, Bootstrap::vault_address()),
			liquidity_token_amount
		);
		assert_eq!(Bootstrap::minted_liquidity(), (liquidity_token_id, liquidity_token_amount));

		assert_eq!(Bootstrap::claimed_rewards(ANOTHER_USER_ID, MGAId::get()), 0);
		assert_eq!(Bootstrap::claimed_rewards(ANOTHER_USER_ID, KSMId::get()), 0);
		assert_eq!(Bootstrap::claimed_rewards(USER_ID, MGAId::get()), 0);
		assert_eq!(Bootstrap::claimed_rewards(USER_ID, KSMId::get()), 0);

		let user_expected_ksm_rewards =
			liquidity_token_amount / 2 * USER_KSM_PROVISON / ksm_valuation;
		let user_expected_mga_rewards =
			liquidity_token_amount / 2 * USER_MGA_PROVISON / mga_valuation;
		let user_expected_liq_amount = user_expected_ksm_rewards + user_expected_mga_rewards;

		let user2_expected_ksm_rewards =
			liquidity_token_amount / 2 * ANOTHER_USER_KSM_PROVISON / ksm_valuation;
		let user2_expected_mga_rewards =
			liquidity_token_amount / 2 * ANOTHER_USER_MGA_PROVISON / mga_valuation;
		let user2_expected_liq_amount = user2_expected_ksm_rewards + user2_expected_mga_rewards;

		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)).unwrap();
		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(ANOTHER_USER_ID)).unwrap();

		assert_eq!(Bootstrap::claimed_rewards(USER_ID, MGAId::get()), user_expected_mga_rewards);
		assert_eq!(Bootstrap::claimed_rewards(USER_ID, KSMId::get()), user_expected_ksm_rewards);
		assert_eq!(
			Bootstrap::claimed_rewards(ANOTHER_USER_ID, MGAId::get()),
			user2_expected_mga_rewards
		);
		assert_eq!(
			Bootstrap::claimed_rewards(ANOTHER_USER_ID, KSMId::get()),
			user2_expected_ksm_rewards
		);

		assert_err!(
			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)),
			Error::<Test>::NothingToClaim
		);
		assert_err!(
			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(ANOTHER_USER_ID)),
			Error::<Test>::NothingToClaim
		);

		assert_eq!(Bootstrap::balance(liquidity_token_id, USER_ID), user_expected_liq_amount);
		assert_eq!(
			Bootstrap::balance(liquidity_token_id, ANOTHER_USER_ID),
			user2_expected_liq_amount
		);

		assert!(System::events().iter().any(|record| record.event ==
			rewards_claimed_ev(liquidity_token_id, user_expected_liq_amount)));
		assert!(System::events().iter().any(|record| record.event ==
			rewards_claimed_ev(liquidity_token_id, user2_expected_liq_amount)));
	});
}

#[test]
#[serial]
fn dont_allow_for_provision_in_vested_tokens_without_dedicated_extrinsic() {
	new_test_ext().execute_with(|| {
		set_up();
		jump_to_public_phase();

		let mga_amount = Bootstrap::balance(MGAId::get(), USER_ID);
		<Test as Config>::VestingProvider::lock_tokens(
			&USER_ID,
			MGAId::get(),
			mga_amount,
			None,
			100_u32.into(),
		)
		.unwrap();

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1),
			Error::<Test>::NotEnoughAssets
		);
	});
}

// #[test]
// #[serial]
// fn fail_vested_token_provision_if_user_doesnt_have_vested_tokens() {
// 	new_test_ext().execute_with(|| {
// 		set_up();
// 		jump_to_public_phase();

// 		assert_err!(
// 			Bootstrap::provision_vested(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1),
// 			Error::<Test>::NotEnoughVestedAssets
// 		);
// 	});
// }

// #[test]
// #[serial]
// fn successful_vested_provision_using_vested_tokens_only_when_user_has_both_vested_and_non_vested_tokens(
// ) {
// 	new_test_ext().execute_with(|| {
// 		set_up();
// 		jump_to_public_phase();

// 		let provision_amount = 10_000;
// 		let lock_start: u128 = 1;
// 		let lock_end: u128 = 150;

// 		<Test as Config>::VestingProvider::lock_tokens(
// 			&USER_ID,
// 			MGAId::get(),
// 			provision_amount,
// 			Some(lock_start.saturated_into()),
// 			lock_end.into(),
// 		)
// 		.unwrap();

// 		let non_vested_initial_amount = Bootstrap::balance(MGAId::get(), USER_ID);

// 		Bootstrap::provision_vested(RuntimeOrigin::signed(USER_ID), MGAId::get(), provision_amount)
// 			.unwrap();

// 		assert_eq!(non_vested_initial_amount, Bootstrap::balance(MGAId::get(), USER_ID));

// 		assert_eq!(0, Bootstrap::locked_balance(MGAId::get(), USER_ID));

// 		assert_eq!(
// 			(provision_amount, lock_start, lock_end + 1),
// 			Bootstrap::vested_provisions(USER_ID, MGAId::get())
// 		);
// 	});
// }

// #[test]
// #[serial]
// fn successful_vested_provision_is_stored_properly_in_storage() {
// 	new_test_ext().execute_with(|| {
// 		set_up();
// 		jump_to_public_phase();

// 		let mga_amount = Bootstrap::balance(MGAId::get(), USER_ID);
// 		let lock_start: u128 = 1;
// 		let lock_end: u128 = 150;

// 		<Test as Config>::VestingProvider::lock_tokens(
// 			&USER_ID,
// 			MGAId::get(),
// 			mga_amount,
// 			Some(lock_start.saturated_into()),
// 			lock_end.into(),
// 		)
// 		.unwrap();
// 		Bootstrap::provision_vested(RuntimeOrigin::signed(USER_ID), MGAId::get(), mga_amount).unwrap();

// 		assert_eq!(
// 			(mga_amount, lock_start, lock_end + 1),
// 			Bootstrap::vested_provisions(USER_ID, MGAId::get())
// 		);
// 	});
// }

// #[test]
// #[serial]
// fn successful_merged_vested_provision_is_stored_properly_in_storage() {
// 	new_test_ext().execute_with(|| {
// 		set_up();
// 		jump_to_public_phase();

// 		let mga_amount = Bootstrap::balance(MGAId::get(), USER_ID);
// 		let first_lock_start: u128 = 1;
// 		let first_lock_end: u128 = 150;
// 		let first_lock_amount = mga_amount / 2;
// 		let second_lock_start: u128 = 2;
// 		let second_lock_end: u128 = 300;
// 		let second_lock_amount = mga_amount - first_lock_amount;

// 		<Test as Config>::VestingProvider::lock_tokens(
// 			&USER_ID,
// 			MGAId::get(),
// 			first_lock_amount,
// 			Some(first_lock_start.saturated_into()),
// 			first_lock_end.into(),
// 		)
// 		.unwrap();
// 		<Test as Config>::VestingProvider::lock_tokens(
// 			&USER_ID,
// 			MGAId::get(),
// 			second_lock_amount,
// 			Some(second_lock_start.saturated_into()),
// 			second_lock_end.into(),
// 		)
// 		.unwrap();

// 		Bootstrap::provision_vested(RuntimeOrigin::signed(USER_ID), MGAId::get(), first_lock_amount)
// 			.unwrap();
// 		assert_eq!(
// 			(first_lock_amount, first_lock_start, first_lock_end + 1),
// 			Bootstrap::vested_provisions(USER_ID, MGAId::get())
// 		);

// 		Bootstrap::provision_vested(RuntimeOrigin::signed(USER_ID), MGAId::get(), second_lock_amount)
// 			.unwrap();
// 		assert_eq!(
// 			(mga_amount, second_lock_start, second_lock_end + 1),
// 			Bootstrap::vested_provisions(USER_ID, MGAId::get())
// 		);
// 	});
// }

#[macro_export]
macro_rules! init_mocks {
	() => {
		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				Some((id, issuance))
			});
	};
}

fn provisions(provisions: Vec<(<Test as frame_system::Config>::AccountId, TokenId, Balance)>) {
	for (user_id, token_id, amount) in provisions {
		<Test as Config>::Currency::transfer(
			token_id,
			&USER_ID,
			&user_id,
			amount,
			frame_support::traits::ExistenceRequirement::KeepAlive,
		)
		.unwrap();

		// if let ProvisionKind::Vested(begin, end) = lock {
		// 	<Test as Config>::VestingProvider::lock_tokens(
		// 		&user_id,
		// 		token_id,
		// 		amount,
		// 		Some(begin.saturated_into()),
		// 		end.into(),
		// 	)
		// 	.unwrap();
		// 	Bootstrap::provision_vested(RuntimeOrigin::signed(user_id), token_id, amount).unwrap();
		// } else {
		Bootstrap::provision(RuntimeOrigin::signed(user_id), token_id, amount).unwrap();
		// }
	}
}

// #[test]
// #[serial]
// fn vested_provision_included_in_valuation() {
// 	new_test_ext().execute_with(|| {
// 		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
// 		set_up();
// 		jump_to_public_phase();
// 		init_mocks!();
// 		let liq_token_id = Tokens::next_asset_id();

// 		// ACT
// 		provisions(vec![
// 			(PROVISION_USER1_ID, MGAId::get(), 1_000_000, ProvisionKind::Vested(1, 150)),
// 			(PROVISION_USER2_ID, KSMId::get(), 100, ProvisionKind::Regular),
// 		]);
// 		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
// 		let liq_token_minted = (mga_valuation + ksm_valuation) / 2;
// 		assert_eq!(mga_valuation, 1_000_000);
// 		assert_eq!(ksm_valuation, 100);

// 		Bootstrap::on_initialize(100_u32.into());
// 		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());
// 		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER1_ID)).unwrap();
// 		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER2_ID)).unwrap();

// 		// ASSERT
// 		assert_eq!(
// 			liq_token_minted / 2,
// 			Bootstrap::locked_balance(liq_token_id, PROVISION_USER1_ID)
// 		);
// 		assert_eq!(0, Bootstrap::balance(liq_token_id, PROVISION_USER1_ID));

// 		assert_eq!(liq_token_minted / 2, Bootstrap::balance(liq_token_id, PROVISION_USER2_ID));
// 		assert_eq!(0, Bootstrap::locked_balance(liq_token_id, PROVISION_USER2_ID));
// 	});
// }

// #[test]
// #[serial]
// fn multi_provisions() {
// 	new_test_ext().execute_with(|| {
// 		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
// 		set_up();
// 		jump_to_public_phase();
// 		init_mocks!();
// 		let liq_token_id = Tokens::next_asset_id();

// 		// ACT
// 		provisions(vec![
// 			(PROVISION_USER1_ID, MGAId::get(), 100_000, ProvisionKind::Vested(1, 150)),
// 			(PROVISION_USER1_ID, KSMId::get(), 10, ProvisionKind::Regular),
// 			(PROVISION_USER2_ID, MGAId::get(), 300_000, ProvisionKind::Vested(1, 150)),
// 			(PROVISION_USER2_ID, KSMId::get(), 30, ProvisionKind::Regular),
// 		]);
// 		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
// 		let liq_token_minted = (mga_valuation + ksm_valuation) / 2;
// 		assert_eq!(mga_valuation, 400_000);
// 		assert_eq!(ksm_valuation, 40);

// 		Bootstrap::on_initialize(100_u32.into());
// 		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());
// 		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER1_ID)).unwrap();
// 		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER2_ID)).unwrap();

// 		// ASSERT
// 		assert_eq!(
// 			liq_token_minted / 2 * 10 / ksm_valuation,
// 			Bootstrap::locked_balance(liq_token_id, PROVISION_USER1_ID)
// 		);
// 		assert_eq!(
// 			liq_token_minted / 2 * 100_000 / mga_valuation,
// 			Bootstrap::balance(liq_token_id, PROVISION_USER1_ID)
// 		);

// 		assert_eq!(
// 			liq_token_minted / 2 * 30 / ksm_valuation,
// 			Bootstrap::locked_balance(liq_token_id, PROVISION_USER2_ID)
// 		);
// 		assert_eq!(
// 			liq_token_minted / 2 * 300_000 / mga_valuation,
// 			Bootstrap::balance(liq_token_id, PROVISION_USER2_ID)
// 		);
// 	});
// }

#[test]
#[serial]
fn multi_provisions_only_with_non_vested() {
	new_test_ext().execute_with(|| {
		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
		set_up();
		jump_to_public_phase();
		init_mocks!();
		let liq_token_id = Tokens::next_asset_id();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		// ACT
		provisions(vec![
			(PROVISION_USER1_ID, MGAId::get(), 100_000),
			(PROVISION_USER1_ID, KSMId::get(), 10),
			(PROVISION_USER2_ID, MGAId::get(), 300_000),
			(PROVISION_USER2_ID, KSMId::get(), 30),
		]);
		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
		let liq_token_minted = (mga_valuation + ksm_valuation) / 2;
		assert_eq!(mga_valuation, 400_000);
		assert_eq!(ksm_valuation, 40);

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());
		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER1_ID)).unwrap();
		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER2_ID)).unwrap();

		// ASSERT
		assert_eq!(
			liq_token_minted / 2 * 100_000 / mga_valuation * 2,
			Bootstrap::balance(liq_token_id, PROVISION_USER1_ID)
		);

		assert_eq!(
			liq_token_minted / 2 * 300_000 / mga_valuation * 2,
			Bootstrap::balance(liq_token_id, PROVISION_USER2_ID)
		);
	});
}

// // formula KSM/MGA for provision calculation
// // (KSM valuation + MGA valuation)   User KSM/MGA token provision
// // ------------------------------- * ---------------------------- = liquidity tokens rewards
// //                2                      KSM/MGA valuation
// //  EX:

// #[test_case(
// 			vec![
// 				(PROVISION_USER1_ID, MGAId::get(), 100_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER1_ID, KSMId::get(), 10, ProvisionKind::Regular),
// 				(PROVISION_USER2_ID, MGAId::get(), 300_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER2_ID, KSMId::get(), 30, ProvisionKind::Regular),
// 			],
// 			(25002, 25002),
// 			(75007, 75007);
// 			"two users provision both vested and non vested tokens")
// ]
// #[test_case(
// 			vec![
// 				(PROVISION_USER1_ID, MGAId::get(), 100_000, ProvisionKind::Regular),
// 				(PROVISION_USER1_ID, KSMId::get(), 10, ProvisionKind::Regular),
// 			],
// 			(50_004, 0),
// 			(0, 0);
// 			"non vested provisions from single user")
// ]
// #[test_case(
// 			vec![
// 				(PROVISION_USER1_ID, MGAId::get(), 100_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER1_ID, KSMId::get(), 10, ProvisionKind::Vested(1, 150)),
// 			],
// 			(0, 50_004),
// 			(0, 0);
// 			"vested provisions from single user")
// ]
// #[test_case(
// 			vec![
// 				(PROVISION_USER1_ID, MGAId::get(), 100_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER1_ID, KSMId::get(), 10, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER2_ID, MGAId::get(), 300_000, ProvisionKind::Regular),
// 				(PROVISION_USER2_ID, KSMId::get(), 30, ProvisionKind::Regular),
// 			],
// 			(0, 50_004), // 400040 / 2 / 2 * 1 / 4
// 			(150014, 0); // 400040 / 2 / 2 * 3 / 4
// 			"vested provisions from single user & non vested form second one")
// ]
// #[test_case(
// 			vec![
// 				(PROVISION_USER1_ID, MGAId::get(), 10_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER2_ID, KSMId::get(), 1, ProvisionKind::Regular),
// 				(PROVISION_USER1_ID, MGAId::get(), 20_000, ProvisionKind::Regular),
// 				(PROVISION_USER2_ID, KSMId::get(), 1, ProvisionKind::Regular),
// 				(PROVISION_USER2_ID, KSMId::get(), 1, ProvisionKind::Regular),
// 				(PROVISION_USER1_ID, MGAId::get(), 30_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER1_ID, KSMId::get(), 1, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER2_ID, KSMId::get(), 1, ProvisionKind::Regular),
// 				(PROVISION_USER2_ID, KSMId::get(), 1, ProvisionKind::Regular),
// 				(PROVISION_USER1_ID, MGAId::get(), 40_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER2_ID, MGAId::get(), 200_000, ProvisionKind::Regular),
// 				(PROVISION_USER2_ID, MGAId::get(), 100_000, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER2_ID, KSMId::get(), 10, ProvisionKind::Regular),
// 				(PROVISION_USER2_ID, KSMId::get(), 15, ProvisionKind::Regular),
// 				(PROVISION_USER1_ID, KSMId::get(), 4, ProvisionKind::Vested(1, 150)),
// 				(PROVISION_USER1_ID, KSMId::get(), 5, ProvisionKind::Vested(1, 150)),
// 			],
// 			(5_000, 45_004),
// 			(125012, 25_002);
// 			"multiple provisions from multiple accounts mixed")
// ]
// #[serial]
// fn test_multi_provisions(
// 	provisions_list: Vec<(
// 		<Test as frame_system::Config>::AccountId,
// 		TokenId,
// 		Balance,
// 		ProvisionKind,
// 	)>,
// 	user1_rewards: (Balance, Balance),
// 	user2_rewards: (Balance, Balance),
// ) {
// 	new_test_ext().execute_with(|| {
// 		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
// 		set_up();
// 		jump_to_public_phase();
// 		init_mocks!();
// 		let liq_token_id = Tokens::next_asset_id();
// 		let user1_has_provisions =
// 			provisions_list.iter().any(|(who, _, _, _)| *who == PROVISION_USER1_ID);
// 		let user2_has_provisions =
// 			provisions_list.iter().any(|(who, _, _, _)| *who == PROVISION_USER2_ID);
// 		let total_ksm_provision: u128 = provisions_list
// 			.iter()
// 			.filter_map(
// 				|(_, token_id, amount, _)| {
// 					if *token_id == KSMId::get() {
// 						Some(amount)
// 					} else {
// 						None
// 					}
// 				},
// 			)
// 			.sum();
// 		let total_mga_provision: u128 = provisions_list
// 			.iter()
// 			.filter_map(
// 				|(_, token_id, amount, _)| {
// 					if *token_id == MGAId::get() {
// 						Some(amount)
// 					} else {
// 						None
// 					}
// 				},
// 			)
// 			.sum();

// 		// ACT
// 		provisions(provisions_list);

// 		Bootstrap::on_initialize(100_u32.into());
// 		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

// 		if user1_has_provisions {
// 			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER1_ID)).unwrap();
// 		}

// 		if user2_has_provisions {
// 			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER2_ID)).unwrap();
// 		}

// 		// ASSERT
// 		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
// 		assert_eq!(total_ksm_provision, ksm_valuation);
// 		assert_eq!(total_mga_provision, mga_valuation);

// 		assert_eq!(user1_rewards.0, Bootstrap::balance(liq_token_id, PROVISION_USER1_ID));
// 		assert_eq!(user1_rewards.1, Bootstrap::locked_balance(liq_token_id, PROVISION_USER1_ID));

// 		assert_eq!(user2_rewards.0, Bootstrap::balance(liq_token_id, PROVISION_USER2_ID));
// 		assert_eq!(user2_rewards.1, Bootstrap::locked_balance(liq_token_id, PROVISION_USER2_ID));
// 	})
// }

// formula KSM/MGA for provision calculation
// (KSM valuation + MGA valuation)   User KSM/MGA token provision
// ------------------------------- * ---------------------------- = liquidity tokens rewards
//                2                      KSM/MGA valuation
//  EX:

#[test_case(
			vec![
				(PROVISION_USER1_ID, MGAId::get(), 100_000),
				(PROVISION_USER1_ID, KSMId::get(), 10),
				(PROVISION_USER2_ID, MGAId::get(), 300_000),
				(PROVISION_USER2_ID, KSMId::get(), 30),
			],
			(50_004, 0),
			(150014, 0);
			"two users provision both vested and non vested tokens")
]
#[test_case(
			vec![
				(PROVISION_USER1_ID, MGAId::get(), 100_000),
				(PROVISION_USER1_ID, KSMId::get(), 10),
			],
			(50_004, 0),
			(0, 0);
			"non vested provisions from single user")
]
#[test_case(
			vec![
				(PROVISION_USER1_ID, MGAId::get(), 100_000),
				(PROVISION_USER1_ID, KSMId::get(), 10),
			],
			(50_004, 0),
			(0, 0);
			"vested provisions from single user")
]
#[test_case(
			vec![
				(PROVISION_USER1_ID, MGAId::get(), 100_000),
				(PROVISION_USER1_ID, KSMId::get(), 10),
				(PROVISION_USER2_ID, MGAId::get(), 300_000),
				(PROVISION_USER2_ID, KSMId::get(), 30),
			],
			(50_004, 0), // 400040 / 2 / 2 * 1 / 4
			(150014, 0); // 400040 / 2 / 2 * 3 / 4
			"vested provisions from single user & non vested form second one")
]
#[test_case(
			vec![
				(PROVISION_USER1_ID, MGAId::get(), 10_000),
				(PROVISION_USER2_ID, KSMId::get(), 1),
				(PROVISION_USER1_ID, MGAId::get(), 20_000),
				(PROVISION_USER2_ID, KSMId::get(), 1),
				(PROVISION_USER2_ID, KSMId::get(), 1),
				(PROVISION_USER1_ID, MGAId::get(), 30_000),
				(PROVISION_USER1_ID, KSMId::get(), 1),
				(PROVISION_USER2_ID, KSMId::get(), 1),
				(PROVISION_USER2_ID, KSMId::get(), 1),
				(PROVISION_USER1_ID, MGAId::get(), 40_000),
				(PROVISION_USER2_ID, MGAId::get(), 200_000),
				(PROVISION_USER2_ID, MGAId::get(), 100_000),
				(PROVISION_USER2_ID, KSMId::get(), 10),
				(PROVISION_USER2_ID, KSMId::get(), 15),
				(PROVISION_USER1_ID, KSMId::get(), 4),
				(PROVISION_USER1_ID, KSMId::get(), 5),
			],
			(50004, 0),
			(150014, 0);
			"multiple provisions from multiple accounts mixed")
]
#[serial]
fn test_multi_provisions_only_with_non_vested(
	provisions_list: Vec<(<Test as frame_system::Config>::AccountId, TokenId, Balance)>,
	user1_rewards: (Balance, Balance),
	user2_rewards: (Balance, Balance),
) {
	new_test_ext().execute_with(|| {
		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
		set_up();
		jump_to_public_phase();
		init_mocks!();
		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let liq_token_id = Tokens::next_asset_id();
		let user1_has_provisions =
			provisions_list.iter().any(|(who, _, _)| *who == PROVISION_USER1_ID);
		let user2_has_provisions =
			provisions_list.iter().any(|(who, _, _)| *who == PROVISION_USER2_ID);
		let total_ksm_provision: u128 = provisions_list
			.iter()
			.filter_map(
				|(_, token_id, amount)| {
					if *token_id == KSMId::get() {
						Some(amount)
					} else {
						None
					}
				},
			)
			.sum();
		let total_mga_provision: u128 = provisions_list
			.iter()
			.filter_map(
				|(_, token_id, amount)| {
					if *token_id == MGAId::get() {
						Some(amount)
					} else {
						None
					}
				},
			)
			.sum();

		// ACT
		provisions(provisions_list);

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

		if user1_has_provisions {
			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER1_ID)).unwrap();
		}

		if user2_has_provisions {
			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER2_ID)).unwrap();
		}

		// ASSERT
		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
		assert_eq!(total_ksm_provision, ksm_valuation);
		assert_eq!(total_mga_provision, mga_valuation);

		assert_eq!(user1_rewards.0, Bootstrap::balance(liq_token_id, PROVISION_USER1_ID));

		assert_eq!(user2_rewards.0, Bootstrap::balance(liq_token_id, PROVISION_USER2_ID));
	})
}

#[test]
#[serial]
fn test_restart_bootstrap() {
	new_test_ext().execute_with(|| {
		set_up();

		const USER_KSM_PROVISON: Balance = 15;
		const USER_MGA_PROVISON: Balance = 400_000;
		const ANOTHER_USER_KSM_PROVISON: Balance = 20;
		const ANOTHER_USER_MGA_PROVISON: Balance = 100_000;
		let liq_token_id = Tokens::next_asset_id();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				assert_eq!(id, liq_token_id);
				Some((id, issuance))
			});

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();
		Bootstrap::on_initialize(110_u32.into());
		Bootstrap::transfer(MGAId::get(), USER_ID, ANOTHER_USER_ID, 500_000).unwrap();
		Bootstrap::transfer(KSMId::get(), USER_ID, ANOTHER_USER_ID, 500_000).unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), USER_MGA_PROVISON)
			.unwrap();
		Bootstrap::provision(
			RuntimeOrigin::signed(ANOTHER_USER_ID),
			MGAId::get(),
			ANOTHER_USER_MGA_PROVISON,
		)
		.unwrap();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), USER_KSM_PROVISON)
			.unwrap();
		Bootstrap::provision(
			RuntimeOrigin::signed(ANOTHER_USER_ID),
			KSMId::get(),
			ANOTHER_USER_KSM_PROVISON,
		)
		.unwrap();

		assert_err!(
			Bootstrap::pre_finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)),
			Error::<Test>::NotFinishedYet
		);

		Bootstrap::on_initialize(120_u32.into());

		assert_eq!(0, Bootstrap::balance(liq_token_id, USER_ID));
		assert_eq!(0, Bootstrap::balance(liq_token_id, ANOTHER_USER_ID));

		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)).unwrap();

		// not all rewards claimed
		assert_err!(
			Bootstrap::pre_finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)),
			Error::<Test>::BootstrapNotReadyToBeFinished
		);

		Bootstrap::claim_liquidity_tokens_for_account(
			RuntimeOrigin::signed(USER_ID),
			ANOTHER_USER_ID,
			false,
		)
		.unwrap();

		assert_ne!(0, Bootstrap::balance(liq_token_id, USER_ID));
		assert_ne!(0, Bootstrap::balance(liq_token_id, ANOTHER_USER_ID));

		Bootstrap::pre_finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)).unwrap();
		Bootstrap::finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)).unwrap();
		assert!(Provisions::<Test>::iter_keys().next().is_none());
		assert!(VestedProvisions::<Test>::iter_keys().next().is_none());
		assert!(WhitelistedAccount::<Test>::iter_keys().next().is_none());
		assert!(ClaimedRewards::<Test>::iter_keys().next().is_none());
		assert!(ProvisionAccounts::<Test>::iter_keys().next().is_none());
		assert_eq!(Valuations::<Test>::get(), (0, 0));
		assert_eq!(Phase::<Test>::get(), BootstrapPhase::BeforeStart);
		assert_eq!(BootstrapSchedule::<Test>::get(), None);
		assert_eq!(MintedLiquidity::<Test>::get(), (0, 0));
		assert_eq!(ActivePair::<Test>::get(), None);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			200_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();
	});
}

#[test]
#[serial]
fn claim_liquidity_tokens_even_if_sum_of_rewards_is_zero_because_of_small_provision() {
	new_test_ext().execute_with(|| {
		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		ArchivedBootstrap::<mock::Test>::mutate(|v| {
			v.push(Default::default());
		});

		Bootstrap::create_new_token(&USER_ID, u128::MAX);
		Bootstrap::create_new_token(&USER_ID, u128::MAX);
		let liq_token_id = Tokens::next_asset_id();

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				assert_eq!(id, liq_token_id);
				Some((id, issuance))
			});

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();
		Bootstrap::on_initialize(110_u32.into());
		Bootstrap::transfer(MGAId::get(), USER_ID, ANOTHER_USER_ID, 1).unwrap();
		Bootstrap::transfer(KSMId::get(), USER_ID, ANOTHER_USER_ID, 1).unwrap();

		Bootstrap::provision(
			RuntimeOrigin::signed(USER_ID),
			MGAId::get(),
			1_000_000_000_000_000_000_000_000_000_000_000_u128,
		)
		.unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(ANOTHER_USER_ID), MGAId::get(), 1).unwrap();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 1_000_000_u128).unwrap();

		Bootstrap::on_initialize(120_u32.into());

		assert_eq!(0, Bootstrap::balance(liq_token_id, USER_ID));
		assert_eq!(0, Bootstrap::balance(liq_token_id, ANOTHER_USER_ID));

		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)).unwrap();
		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(ANOTHER_USER_ID)).unwrap();

		assert_err!(
			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)),
			Error::<Test>::NothingToClaim
		);

		assert_err!(
			Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(ANOTHER_USER_ID)),
			Error::<Test>::NothingToClaim
		);
	});
}

#[test]
#[serial]
fn transfer_dust_to_treasury() {
	new_test_ext().execute_with(|| {
		Bootstrap::create_new_token(&USER_ID, u128::MAX);
		Bootstrap::create_new_token(&USER_ID, u128::MAX);
		let liq_token_id = Tokens::next_asset_id();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				assert_eq!(id, liq_token_id);
				Some((id, issuance))
			});

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();
		Bootstrap::on_initialize(110_u32.into());
		Bootstrap::transfer(MGAId::get(), USER_ID, ANOTHER_USER_ID, 1).unwrap();
		Bootstrap::transfer(KSMId::get(), USER_ID, ANOTHER_USER_ID, 1).unwrap();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1_000_000_000_u128)
			.unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(ANOTHER_USER_ID), MGAId::get(), 1).unwrap();

		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 100_u128).unwrap();

		Bootstrap::on_initialize(120_u32.into());

		assert_eq!(0, Bootstrap::balance(liq_token_id, USER_ID));
		assert_eq!(0, Bootstrap::balance(liq_token_id, ANOTHER_USER_ID));

		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)).unwrap();
		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(ANOTHER_USER_ID)).unwrap();

		let before_finalize = Bootstrap::balance(
			liq_token_id,
			<mock::Test as Config>::TreasuryPalletId::get().into_account_truncating(),
		);

		Bootstrap::pre_finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)).unwrap();
		Bootstrap::finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)).unwrap();

		let after_finalize = Bootstrap::balance(
			liq_token_id,
			<mock::Test as Config>::TreasuryPalletId::get().into_account_truncating(),
		);
		assert!(after_finalize > before_finalize);
	});
}

#[test]
#[serial]
fn archive_previous_bootstrap_schedules() {
	new_test_ext().execute_with(|| {
		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		Bootstrap::create_new_token(&USER_ID, u128::MAX);
		Bootstrap::create_new_token(&USER_ID, u128::MAX);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				Some((id, issuance))
			});

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();
		Bootstrap::on_initialize(110_u32.into());
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), MGAId::get(), 1_000_000_000_u128)
			.unwrap();
		Bootstrap::provision(RuntimeOrigin::signed(USER_ID), KSMId::get(), 100_u128).unwrap();
		Bootstrap::on_initialize(120_u32.into());
		Bootstrap::claim_liquidity_tokens(RuntimeOrigin::signed(USER_ID)).unwrap();
		assert_eq!(0, Bootstrap::archived().len());
		Bootstrap::pre_finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)).unwrap();
		Bootstrap::finalize(RuntimeOrigin::signed(YET_ANOTHER_USER_ID)).unwrap();
		assert_eq!(0, Bootstrap::provisions(USER_ID, KSMId::get()));

		assert_eq!(1, Bootstrap::archived().len());
	})
}

#[test]
#[serial]
fn test_activate_liq_tokens_is_called_with_all_liq_tokens_when_pool_is_promoted_and_all_provisions_are_non_vested(
) {
	new_test_ext().execute_with(|| {
		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
		set_up();
		jump_to_public_phase();
		init_mocks!();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let mga_provision = 1_000_000_u128;
		let ksm_provision = 100_u128;
		let expected_liq_tokens_amount = (mga_provision + ksm_provision) / 2;
		let expected_activated_tokens_amount = expected_liq_tokens_amount / 2;

		let is_enabled_mock = MockRewardsApi::is_enabled_context();
		is_enabled_mock.expect().return_const(true);

		let activate_liquidity = MockRewardsApi::activate_liquidity_context();
		activate_liquidity.expect().returning(move |_, _, activated_amount, _| {
			assert_eq!(expected_activated_tokens_amount, activated_amount);
			Ok(())
		});

		// ACT
		provisions(vec![
			(PROVISION_USER1_ID, MGAId::get(), mga_provision),
			(PROVISION_USER2_ID, KSMId::get(), ksm_provision),
		]);
		let (mga_valuation, ksm_valuation) = Bootstrap::valuations();
		assert_eq!(mga_valuation, 1_000_000);
		assert_eq!(ksm_valuation, 100);

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

		Bootstrap::claim_and_activate_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER1_ID))
			.unwrap();
		Bootstrap::claim_and_activate_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER2_ID))
			.unwrap();
	});
}

// #[test]
// #[serial]
// fn test_dont_activate_liq_tokens_when_pool_is_promoted_but_all_provisions_are_vested() {
// 	new_test_ext().execute_with(|| {
// 		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
// 		set_up();
// 		jump_to_public_phase();
// 		init_mocks!();

// 		let mga_provision = 1_000_000_u128;
// 		let ksm_provision = 100_u128;

// 		let is_enabled_mock = MockRewardsApi::is_enabled_context();
// 		is_enabled_mock.expect().return_const(true);

// 		let activate_liquidity = MockRewardsApi::activate_liquidity_context();
// 		activate_liquidity.expect().times(0);

// 		// ACT
// 		provisions(vec![
// 			(PROVISION_USER1_ID, MGAId::get(), mga_provision, ProvisionKind::Regular),
// 			(PROVISION_USER2_ID, KSMId::get(), ksm_provision, ProvisionKind::Vested(1, 150)),
// 		]);

// 		Bootstrap::on_initialize(100_u32.into());
// 		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

// 		Bootstrap::claim_and_activate_liquidity(RuntimeOrigin::signed(PROVISION_USER2_ID)).unwrap();
// 	});
// }

// #[test]
// #[serial]
// fn test_activate_liquidity_is_called_only_with_non_vested_liq_tokens_when_pool_is_promoted()
// {
// 	new_test_ext().execute_with(|| {
// 		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
// 		set_up();
// 		jump_to_public_phase();
// 		init_mocks!();

// 		let mga_provision = 1_000_000_u128;
// 		let ksm_provision = 100_u128;
// 		let expected_liq_tokens_amount = (mga_provision + ksm_provision) / 2;
// 		let expected_liq_tokens_amount_per_user = expected_liq_tokens_amount / 2;
// 		let expected_non_vested_liq_tokens_amount_per_user =
// 			expected_liq_tokens_amount_per_user / 2;

// 		let is_enabled_mock = MockRewardsApi::is_enabled_context();
// 		is_enabled_mock.expect().return_const(true);

// 		let activate_liquidity = MockRewardsApi::activate_liquidity_context();
// 		activate_liquidity.expect().returning(move |_, _, activated_amount| {
// 			assert_eq!(expected_non_vested_liq_tokens_amount_per_user, activated_amount);
// 			Ok(().into())
// 		});

// 		// ACT
// 		provisions(vec![
// 			(PROVISION_USER1_ID, MGAId::get(), mga_provision, ProvisionKind::Regular),
// 			(PROVISION_USER2_ID, KSMId::get(), ksm_provision / 2, ProvisionKind::Regular),
// 			(PROVISION_USER2_ID, KSMId::get(), ksm_provision / 2, ProvisionKind::Vested(1, 150)),
// 		]);

// 		Bootstrap::on_initialize(100_u32.into());
// 		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

// 		Bootstrap::claim_and_activate_liquidity(RuntimeOrigin::signed(PROVISION_USER2_ID)).unwrap();
// 	});
// }

#[test]
#[serial]
fn test_dont_activate_liquidity_when_pool_is_not_promoted_and_provisions_are_not_vested() {
	new_test_ext().execute_with(|| {
		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
		set_up();
		jump_to_public_phase();
		init_mocks!();

		let mga_provision = 1_000_000_u128;
		let ksm_provision = 100_u128;

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let is_enabled_mock = MockRewardsApi::is_enabled_context();
		is_enabled_mock.expect().return_const(false);

		let activate_liquidity = MockRewardsApi::activate_liquidity_context();
		activate_liquidity.expect().times(0);

		// ACT
		provisions(vec![
			(PROVISION_USER1_ID, MGAId::get(), mga_provision),
			(PROVISION_USER2_ID, KSMId::get(), ksm_provision),
		]);

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

		Bootstrap::claim_and_activate_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER1_ID))
			.unwrap();
		Bootstrap::claim_and_activate_liquidity_tokens(RuntimeOrigin::signed(PROVISION_USER2_ID))
			.unwrap();
	});
}

#[test]
#[serial]
fn test_claim_and_activate_doesnt_fail_when_tokens_activations_fails() {
	new_test_ext().execute_with(|| {
		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
		set_up();
		jump_to_public_phase();
		init_mocks!();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let mga_provision = 1_000_000_u128;
		let ksm_provision = 100_u128;

		let is_enabled_mock = MockRewardsApi::is_enabled_context();
		is_enabled_mock.expect().return_const(true);

		let activate_liquidity = MockRewardsApi::activate_liquidity_context();
		activate_liquidity
			.expect()
			// inject any error
			.returning(move |_, _, _, _| Err(Error::<Test>::NotEnoughAssets.into()));

		// ACT
		provisions(vec![
			(PROVISION_USER1_ID, MGAId::get(), mga_provision),
			(PROVISION_USER2_ID, KSMId::get(), ksm_provision),
		]);
		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());

		assert_ok!(Bootstrap::claim_and_activate_liquidity_tokens(RuntimeOrigin::signed(
			PROVISION_USER1_ID
		)));
	});
}

#[test]
#[serial]
fn test_pool_is_promoted_if_scheduled_to() {
	new_test_ext().execute_with(|| {
		use std::sync::{Arc, Mutex};
		set_up();

		let liq_token_id: Arc<Mutex<TokenId>> = Arc::new(Mutex::new(0_u32));
		let ref_liq_token_id = liq_token_id;

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				*(ref_liq_token_id.lock().unwrap()) = id;
				Some((id, issuance))
			});

		let mock = MockRewardsApi::enable_context();
		mock.expect().times(1).return_const(());

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			true,
		)
		.unwrap();

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Whitelist, Phase::<Test>::get());

		Bootstrap::on_initialize(110_u32.into());
		assert_eq!(BootstrapPhase::Public, Phase::<Test>::get());

		Bootstrap::on_initialize(120_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());
	});
}

#[test]
#[serial]
fn test_pool_is_not_promoted_if_not_scheduled_to() {
	new_test_ext().execute_with(|| {
		use std::sync::{Arc, Mutex};
		set_up();

		let liq_token_id: Arc<Mutex<TokenId>> = Arc::new(Mutex::new(0_u32));
		let ref_liq_token_id = liq_token_id;

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(true);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock
			.expect()
			.times(1)
			.returning(move |addr, _, ksm_amount, _, mga_amount| {
				let issuance = (ksm_amount + mga_amount) / 2;
				let id = Bootstrap::create_new_token(&addr, issuance);
				*(ref_liq_token_id.lock().unwrap()) = id;
				Some((id, issuance))
			});

		let mock = MockRewardsApi::enable_context();
		mock.expect().times(0).return_const(());

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::on_initialize(100_u32.into());
		assert_eq!(BootstrapPhase::Whitelist, Phase::<Test>::get());

		Bootstrap::on_initialize(110_u32.into());
		assert_eq!(BootstrapPhase::Public, Phase::<Test>::get());

		Bootstrap::on_initialize(120_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());
	});
}

#[test]
#[serial]
fn test_pool_bootstrap_finalize_continues_if_asset_metadata_update_fails() {
	new_test_ext().execute_with(|| {
		set_up();

		let enable_pool_creation_mock = MockAssetRegistryApi::enable_pool_creation_context();
		enable_pool_creation_mock.expect().return_const(false);

		let pool_exists_mock = MockPoolCreateApi::pool_exists_context();
		pool_exists_mock.expect().return_const(false);

		let pool_create_mock = MockPoolCreateApi::pool_create_context();
		pool_create_mock.expect().times(1).return_const(POOL_CREATE_DUMMY_RETURN_VALUE);

		Bootstrap::schedule_bootstrap(
			RuntimeOrigin::root(),
			KSMId::get(),
			MGAId::get(),
			100_u32.into(),
			Some(10),
			10,
			Some(DEFAULT_RATIO),
			false,
		)
		.unwrap();

		Bootstrap::on_initialize(120_u32.into());
		assert_eq!(BootstrapPhase::Finished, Phase::<Test>::get());
	});
}

#[test]
#[serial]
fn provisions_not_allowed_in_maintenance_mode() {
	new_test_ext().execute_with(|| {
		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
		set_up();
		jump_to_public_phase();

		MockMaintenanceStatusProvider::set_maintenance(true);

		assert_err!(
			Bootstrap::provision(RuntimeOrigin::signed(0), 0, 10000),
			Error::<Test>::ProvisioningBlockedByMaintenanceMode
		);
	});
}

// #[test]
// #[serial]
// fn vested_provisions_not_allowed_in_maintenance_mode() {
// 	new_test_ext().execute_with(|| {
// 		// ARRANGE - USER provides vested MGA tokens, ANOTHER_USER provides KSM tokens
// 		set_up();
// 		jump_to_public_phase();

// 		MockMaintenanceStatusProvider::set_maintenance(true);

// 		assert_err!(
// 			Bootstrap::provision_vested(RuntimeOrigin::signed(0), 0, 10000),
// 			Error::<Test>::ProvisioningBlockedByMaintenanceMode
// 		);

// 	});
// }
