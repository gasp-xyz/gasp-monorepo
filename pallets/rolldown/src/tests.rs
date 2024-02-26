use frame_support::assert_err;

use sp_runtime::traits::ConvertBack;

use crate::{
	mock::{consts::*, *},
	*,
};
use hex_literal::hex;
use messages::{L1Update, L1UpdateRequest};
use serial_test::serial;

pub const ETH_TOKEN_ADDRESS: [u8; 20] = hex!("2CD2188119797153892438E57364D95B32975560");
pub const ETH_TOKEN_ADDRESS_MGX: TokenId = 100_u32;
pub const ETH_RECIPIENT_ACCOUNT: [u8; 20] = hex!("0000000000000000000000000000000000000004");
pub const ETH_RECIPIENT_ACCOUNT_MGX: AccountId = CHARLIE;

pub type TokensOf<Test> = <Test as crate::Config>::Tokens;

struct L1UpdateBuilder(messages::L1Update);
impl L1UpdateBuilder {
	fn new() -> Self {
		Self(Default::default())
	}

	fn with_offset(mut self, offset: u128) -> Self {
		self.0.offset = offset.into();
		self
	}

	fn with_last_accepted(mut self, last_accepted: u128) -> Self {
		self.0.lastAcceptedRequestOnL1 = last_accepted.into();
		self
	}

	fn with_last_processed(mut self, last_processed: u128) -> Self {
		self.0.lastProccessedRequestOnL1 = last_processed.into();
		self
	}

	fn with_requests(mut self, requests: Vec<L1UpdateRequest>) -> Self {
		for r in requests {
			match r {
				L1UpdateRequest::Deposit(d) => {
					self.0.order.push(messages::PendingRequestType::DEPOSIT);
					self.0.pendingDeposits.push(d);
				},
				L1UpdateRequest::Withdraw(w) => {
					self.0.order.push(messages::PendingRequestType::WITHDRAWAL);
					self.0.pendingWithdraws.push(w);
				},
				L1UpdateRequest::Cancel(c) => {
					self.0.order.push(messages::PendingRequestType::CANCEL_RESOLUTION);
					self.0.pendingCancelResultions.push(c);
				},
				L1UpdateRequest::Remove(r) => {
					self.0.order.push(messages::PendingRequestType::L2_UPDATES_TO_REMOVE);
					self.0.pendingL2UpdatesToRemove.push(r);
				},
			}
		}
		self
	}

	fn build(self) -> messages::L1Update {
		self.0
	}
}

#[test]
#[serial]
fn error_on_empty_update() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		forward_to_block::<Test>(36);
		let update = L1UpdateBuilder::new().build();
		assert_err!(
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update),
			Error::<Test>::EmptyUpdate
		);
	});
}

#[test]
#[serial]
fn process_single_deposit() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		forward_to_block::<Test>(36);
		let current_block_number =
			<frame_system::Pallet<Test>>::block_number().saturated_into::<u128>();
		let dispute_period: u128 = Rolldown::get_dispute_period();
		let update = L1UpdateBuilder::new()
			.with_requests(vec![L1UpdateRequest::Deposit(Default::default())])
			.build();
		Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update).unwrap();

		assert_event_emitted!(Event::PendingRequestStored((
			ALICE,
			current_block_number + dispute_period,
			sp_core::U256::from(0u128),
			sp_core::U256::from(0u128),
			H256::from(hex!("2a48bbdcd86e4e5571feef2579e2c4098c95b5aecc82a603c873429bf72651c3"))
		)));
	});
}

#[test]
#[serial]
fn create_pending_update_after_dispute_period() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		let update1 = L1UpdateBuilder::new()
			.with_requests(vec![L1UpdateRequest::Deposit(messages::Deposit {
				depositRecipient: ETH_RECIPIENT_ACCOUNT,
				tokenAddress: ETH_TOKEN_ADDRESS,
				amount: sp_core::U256::from(MILLION),
			})])
			.build();

		let update2 = L1UpdateBuilder::new()
			.with_requests(vec![L1UpdateRequest::Withdraw(messages::Withdraw {
				depositRecipient: ETH_RECIPIENT_ACCOUNT,
				tokenAddress: ETH_TOKEN_ADDRESS,
				amount: sp_core::U256::from(MILLION),
			})])
			.with_offset(1u128)
			.build();

		forward_to_block::<Test>(10);
		Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update1).unwrap();

		forward_to_block::<Test>(11);
		Rolldown::update_l2_from_l1(RuntimeOrigin::signed(BOB), update2).unwrap();

		assert_eq!(pending_updates::<Test>::iter().next(), None);
		assert!(pending_updates::<Test>::get(sp_core::U256::from(0u128)).is_none());
		assert!(pending_updates::<Test>::get(sp_core::U256::from(1u128)).is_none());

		forward_to_block::<Test>(15);
		assert!(pending_updates::<Test>::get(sp_core::U256::from(0u128)).is_some());

		forward_to_block::<Test>(16);
		assert!(pending_updates::<Test>::get(sp_core::U256::from(1u128)).is_some());
	});
}

#[test]
#[serial]
fn l2_counter_updates_when_requests_are_processed() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		let update1 = L1UpdateBuilder::new()
			.with_requests(vec![L1UpdateRequest::Deposit(messages::Deposit {
				depositRecipient: ETH_RECIPIENT_ACCOUNT,
				tokenAddress: ETH_TOKEN_ADDRESS,
				amount: sp_core::U256::from(MILLION),
			})])
			.build();

		let update2 = L1UpdateBuilder::new()
			.with_requests(vec![L1UpdateRequest::Withdraw(messages::Withdraw {
				depositRecipient: ETH_RECIPIENT_ACCOUNT,
				tokenAddress: ETH_TOKEN_ADDRESS,
				amount: sp_core::U256::from(MILLION),
			})])
			.with_offset(1u128)
			.build();

		forward_to_block::<Test>(10);
		assert_eq!(Rolldown::get_last_processed_request_on_l2(), 0_u128);
		Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update1).unwrap();

		forward_to_block::<Test>(11);
		Rolldown::update_l2_from_l1(RuntimeOrigin::signed(BOB), update2).unwrap();

		forward_to_block::<Test>(15);
		assert_eq!(Rolldown::get_last_processed_request_on_l2(), 0_u128);

		forward_to_block::<Test>(16);
		assert_eq!(Rolldown::get_last_processed_request_on_l2(), 1_u128);
	});
}

#[test]
#[serial]
fn deposit_executed_after_dispute_period() {
	ExtBuilder::new()
		.issue(ALICE, ETH_TOKEN_ADDRESS_MGX, 0u128)
		.execute_with_default_mocks(|| {
			forward_to_block::<Test>(10);
			let update = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Deposit(messages::Deposit {
					depositRecipient: DummyAddressConverter::convert_back(CHARLIE),
					tokenAddress: ETH_TOKEN_ADDRESS,
					amount: sp_core::U256::from(MILLION),
				})])
				.build();

			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update).unwrap();
			forward_to_block::<Test>(14);
			assert!(!pending_updates::<Test>::contains_key(sp_core::U256::from(0u128)));
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), 0_u128);

			forward_to_block::<Test>(15);
			assert_eq!(
				pending_updates::<Test>::get(sp_core::U256::from(0u128)),
				Some(PendingUpdate::RequestResult((true, UpdateType::DEPOSIT)))
			);
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), MILLION);
		});
}

#[test]
#[serial]
fn l1_upate_executed_immaidately_if_force_submitted() {
	ExtBuilder::new()
		.issue(ALICE, ETH_TOKEN_ADDRESS_MGX, 0u128)
		.execute_with_default_mocks(|| {
			forward_to_block::<Test>(10);
			let update = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Deposit(messages::Deposit {
					depositRecipient: DummyAddressConverter::convert_back(CHARLIE),
					tokenAddress: ETH_TOKEN_ADDRESS,
					amount: sp_core::U256::from(MILLION),
				})])
				.build();

			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), 0_u128);
			assert!(!pending_updates::<Test>::contains_key(sp_core::U256::from(0u128)));
			Rolldown::force_update_l2_from_l1(RuntimeOrigin::root(), update).unwrap();
			assert!(pending_updates::<Test>::contains_key(sp_core::U256::from(0u128)));
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), MILLION);
		});
}

#[test]
#[serial]
fn each_request_executed_only_once() {
	ExtBuilder::new()
		.issue(ALICE, ETH_TOKEN_ADDRESS_MGX, 0u128)
		.execute_with_default_mocks(|| {
			forward_to_block::<Test>(10);
			let update = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Deposit(messages::Deposit {
					depositRecipient: DummyAddressConverter::convert_back(CHARLIE),
					tokenAddress: ETH_TOKEN_ADDRESS,
					amount: sp_core::U256::from(MILLION),
				})])
				.build();
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update.clone()).unwrap();

			forward_to_block::<Test>(11);
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(BOB), update).unwrap();

			forward_to_block::<Test>(14);
			assert!(!pending_updates::<Test>::contains_key(sp_core::U256::from(0u128)));
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), 0_u128);

			forward_to_block::<Test>(15);
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), MILLION);

			forward_to_block::<Test>(20);
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), MILLION);
		});
}

#[test]
#[serial]
fn withdraw_executed_after_dispute_period() {
	ExtBuilder::new()
		.issue(ETH_RECIPIENT_ACCOUNT_MGX, ETH_TOKEN_ADDRESS_MGX, MILLION)
		.execute_with_default_mocks(|| {
			forward_to_block::<Test>(10);
			let update = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Withdraw(messages::Withdraw {
					depositRecipient: DummyAddressConverter::convert_back(CHARLIE),
					tokenAddress: ETH_TOKEN_ADDRESS,
					amount: sp_core::U256::from(MILLION),
				})])
				.build();

			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update).unwrap();
			forward_to_block::<Test>(14);
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), MILLION);

			forward_to_block::<Test>(15);
			assert_eq!(
				pending_updates::<Test>::get(sp_core::U256::from(0u128)),
				Some(PendingUpdate::RequestResult((true, UpdateType::WITHDRAWAL)))
			);
			assert_eq!(TokensOf::<Test>::free_balance(ETH_TOKEN_ADDRESS_MGX, &CHARLIE), 0_u128);
		});
}

#[test]
#[serial]
fn withdraw_executed_after_dispute_period_when_not_enough_tokens() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		forward_to_block::<Test>(10);
		let update = L1UpdateBuilder::new()
			.with_requests(vec![L1UpdateRequest::Withdraw(messages::Withdraw {
				depositRecipient: ETH_RECIPIENT_ACCOUNT,
				tokenAddress: ETH_TOKEN_ADDRESS,
				amount: sp_core::U256::from(MILLION),
			})])
			.build();

		Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update).unwrap();

		forward_to_block::<Test>(15);
		assert_eq!(
			pending_updates::<Test>::get(sp_core::U256::from(0u128)),
			Some(PendingUpdate::RequestResult((false, UpdateType::WITHDRAWAL)))
		);
	});
}

#[test]
#[serial]
fn updates_to_remove_executed_after_dispute_period() {
	ExtBuilder::new()
		.issue(CHARLIE, ETH_TOKEN_ADDRESS_MGX, MILLION)
		.execute_with_default_mocks(|| {
			forward_to_block::<Test>(10);

			let withdraw_update = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Withdraw(messages::Withdraw {
					depositRecipient: DummyAddressConverter::convert_back(CHARLIE),
					tokenAddress: ETH_TOKEN_ADDRESS,
					amount: sp_core::U256::from(MILLION),
				})])
				.build();

			let l2_updates_to_remove = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Remove(messages::L2UpdatesToRemove {
					l2UpdatesToRemove: vec![sp_core::U256::from(0u128)],
				})])
				.with_offset(1u128)
				.build();

			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), withdraw_update).unwrap();

			forward_to_block::<Test>(15);
			assert!(pending_updates::<Test>::contains_key(sp_core::U256::from(0u128)));

			forward_to_block::<Test>(100);
			assert_eq!(
				pending_updates::<Test>::get(sp_core::U256::from(0u128)),
				Some(PendingUpdate::RequestResult((true, UpdateType::WITHDRAWAL)))
			);
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), l2_updates_to_remove)
				.unwrap();

			forward_to_block::<Test>(104);
			assert!(pending_updates::<Test>::contains_key(sp_core::U256::from(0u128)));
			assert!(!pending_updates::<Test>::contains_key(sp_core::U256::from(1u128)));

			forward_to_block::<Test>(105);
			assert!(pending_updates::<Test>::contains_key(sp_core::U256::from(1u128)));
			assert!(!pending_updates::<Test>::contains_key(sp_core::U256::from(0u128)));
		});
}

#[test]
#[serial]
fn cancel_request() {
	ExtBuilder::new()
		.issue(ETH_RECIPIENT_ACCOUNT_MGX, ETH_TOKEN_ADDRESS_MGX, MILLION)
		.execute_with_default_mocks(|| {
			forward_to_block::<Test>(10);

			let slash_sequencer_mock = MockSequencerStakingProviderApi::slash_sequencer_context();
			slash_sequencer_mock.expect().return_const(Ok(().into()));

			let withdraw_update = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Withdraw(messages::Withdraw {
					depositRecipient: ETH_RECIPIENT_ACCOUNT,
					tokenAddress: ETH_TOKEN_ADDRESS,
					amount: sp_core::U256::from(MILLION),
				})])
				.build();

			let cancel_resolution = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Cancel(messages::CancelResolution {
					l2RequestId: U256::from(1u128),
					cancelJustified: true,
				})])
				.with_offset(0u128)
				.build();

			assert_eq!(
				sequencer_rights::<Test>::get(ALICE).unwrap(),
				SequencerRights { readRights: 1u128, cancelRights: 1u128 }
			);
			assert_eq!(
				sequencer_rights::<Test>::get(BOB).unwrap(),
				SequencerRights { readRights: 1u128, cancelRights: 1u128 }
			);

			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), withdraw_update).unwrap();
			assert!(pending_requests::<Test>::contains_key(U256::from(15u128)));

			Rolldown::cancel_requests_from_l1(RuntimeOrigin::signed(BOB), 15u128.into()).unwrap();

			assert!(!pending_requests::<Test>::contains_key(U256::from(15u128)));
			assert_eq!(
				sequencer_rights::<Test>::get(ALICE).unwrap(),
				SequencerRights { readRights: 0u128, cancelRights: 1u128 }
			);
			assert_eq!(
				sequencer_rights::<Test>::get(BOB).unwrap(),
				SequencerRights { readRights: 1u128, cancelRights: 0u128 }
			);

			forward_to_block::<Test>(11);
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(BOB), cancel_resolution).unwrap();
			assert_eq!(
				sequencer_rights::<Test>::get(BOB).unwrap(),
				SequencerRights { readRights: 0u128, cancelRights: 0u128 }
			);

			forward_to_block::<Test>(16);
			assert_eq!(
				sequencer_rights::<Test>::get(ALICE).unwrap(),
				SequencerRights { readRights: 1u128, cancelRights: 1u128 }
			);
			assert_eq!(
				sequencer_rights::<Test>::get(BOB).unwrap(),
				SequencerRights { readRights: 1u128, cancelRights: 1u128 }
			);
		});
}

#[test]
#[serial]
fn cancel_request_as_council_executed_immadiately() {
	ExtBuilder::new()
		.issue(ETH_RECIPIENT_ACCOUNT_MGX, ETH_TOKEN_ADDRESS_MGX, MILLION)
		.execute_with_default_mocks(|| {
			forward_to_block::<Test>(10);

			let slash_sequencer_mock = MockSequencerStakingProviderApi::slash_sequencer_context();
			slash_sequencer_mock.expect().return_const(Ok(().into()));

			let withdraw_update = L1UpdateBuilder::new()
				.with_requests(vec![L1UpdateRequest::Withdraw(messages::Withdraw {
					depositRecipient: ETH_RECIPIENT_ACCOUNT,
					tokenAddress: ETH_TOKEN_ADDRESS,
					amount: sp_core::U256::from(MILLION),
				})])
				.build();

			assert_eq!(
				sequencer_rights::<Test>::get(ALICE).unwrap(),
				SequencerRights { readRights: 1u128, cancelRights: 1u128 }
			);
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), withdraw_update).unwrap();
			assert_eq!(
				sequencer_rights::<Test>::get(ALICE).unwrap(),
				SequencerRights { readRights: 0u128, cancelRights: 1u128 }
			);

			Rolldown::force_cancel_requests_from_l1(RuntimeOrigin::root(), 15u128.into()).unwrap();
			assert_eq!(
				sequencer_rights::<Test>::get(ALICE).unwrap(),
				SequencerRights { readRights: 1u128, cancelRights: 1u128 }
			);
		});
}

#[test]
#[serial]
fn reject_update_with_too_many_requests() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		forward_to_block::<Test>(10);

		let requests = vec![
			L1UpdateRequest::Withdraw(messages::Withdraw {
				depositRecipient: ETH_RECIPIENT_ACCOUNT,
				tokenAddress: ETH_TOKEN_ADDRESS,
				amount: sp_core::U256::from(MILLION),
			});
			11
		];

		let withdraw_update = L1UpdateBuilder::new().with_requests(requests).build();

		assert_err!(
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), withdraw_update),
			Error::<Test>::TooManyRequests
		);
	});
}

#[test]
#[ignore]
#[serial]
fn reject_update_with_invalid_too_high_request_id() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		forward_to_block::<Test>(10);

		let requests = vec![
			L1UpdateRequest::Withdraw(messages::Withdraw {
				depositRecipient: ETH_RECIPIENT_ACCOUNT,
				tokenAddress: ETH_TOKEN_ADDRESS,
				amount: sp_core::U256::from(MILLION),
			});
			11
		];

		let offset = 100u128;
		let withdraw_update =
			L1UpdateBuilder::new().with_requests(requests).with_offset(offset).build();

		assert_err!(
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), withdraw_update),
			Error::<Test>::InvalidUpdate
		);
	});
}

#[test]
#[serial]
fn reject_update_with_missing_requests() {
	ExtBuilder::new().execute_with_default_mocks(|| {
		forward_to_block::<Test>(10);

		let update =
			L1Update { order: vec![messages::PendingRequestType::DEPOSIT], ..Default::default() };

		assert_err!(
			Rolldown::update_l2_from_l1(RuntimeOrigin::signed(ALICE), update),
			Error::<Test>::InvalidUpdate
		);
	});
}

#[test]
fn test_conversion_u256() {
	let val = sp_core::U256::from(1u8);
	let eth_val = alloy_primitives::U256::from(1u8);

	assert_eq!(Rolldown::to_eth_u256(val), eth_val);
}

#[test]
fn test_conversion_address() {
	let byte_address: [u8; 20] = DummyAddressConverter::convert_back(consts::CHARLIE);

	assert_eq!(DummyAddressConverter::convert(byte_address), consts::CHARLIE);
}
