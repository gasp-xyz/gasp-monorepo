// Copyright (C) 2020 Mangata team

use super::*;

use crate as pallet_multipurpose_liquidity;
use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{Contains, Nothing},
	PalletId,
};
use frame_system as system;
use orml_tokens::MultiTokenCurrencyAdapter;
use orml_traits::parameter_type_with_key;
use sp_runtime::{traits::AccountIdConversion, BuildStorage, Permill};
use sp_std::convert::TryFrom;

pub const NATIVE_CURRENCY_ID: u32 = 0;

pub(crate) type AccountId = u64;
pub(crate) type Amount = i128;
pub(crate) type Balance = u128;
pub(crate) type TokenId = u32;

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test {
		System: frame_system,
		Tokens: orml_tokens,
		Vesting: pallet_vesting_mangata,
		MultiPurposeLiquidity: pallet_multipurpose_liquidity,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: TokenId| -> Balance {
		match currency_id {
			_ => 0,
		}
	};
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		*a == TreasuryAccount::get()
	}
}

parameter_types! {
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
	pub const MaxLocks: u32 = 50;
}

impl orml_tokens::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = TokenId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type CurrencyHooks = ();
	type NontransferableTokens = Nothing;
	type NontransferableTokensAllowList = Nothing;
}

parameter_types! {
	pub const NativeCurrencyId: u32 = NATIVE_CURRENCY_ID;
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const BnbTreasurySubAccDerive: [u8; 4] = *b"bnbt";
}

parameter_types! {
	pub const MinVestedTransfer: Balance = 0;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting_mangata::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Tokens = MultiTokenCurrencyAdapter<Test>;
	type BlockNumberToBalance = sp_runtime::traits::ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = pallet_vesting_mangata::weights::SubstrateWeight<Test>;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type BlockNumberProvider = System;
	// `VestingInfo` encode length is 36bytes. 28 schedules gets encoded as 1009 bytes, which is the
	// highest number of schedules that encodes less than 2^10.
	const MAX_VESTING_SCHEDULES: u32 = 50;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxRelocks = MaxLocks;
	type Tokens = MultiTokenCurrencyAdapter<Test>;
	type NativeCurrencyId = NativeCurrencyId;
	type VestingProvider = Vesting;
	type Xyk = MockXyk<Test>;
	type WeightInfo = ();
}

pub struct MockXyk<T>(PhantomData<T>);
impl<T: Config> XykFunctionsTrait<AccountId, Balance, TokenId> for MockXyk<T> {
	fn create_pool(
		_sender: AccountId,
		_first_asset_id: TokenId,
		_first_asset_amount: Balance,
		_second_asset_id: TokenId,
		_second_asset_amount: Balance,
	) -> Result<TokenId, DispatchError> {
		unimplemented!()
	}

	fn sell_asset(
		_sender: AccountId,
		_sold_asset_id: TokenId,
		_bought_asset_id: TokenId,
		_sold_asset_amount: Balance,
		_min_amount_out: Balance,
		_err_upon_bad_slippage: bool,
	) -> Result<Balance, DispatchError> {
		unimplemented!()
	}

	fn do_multiswap_sell_asset(
		_sender: AccountId,
		_swap_token_list: Vec<TokenId>,
		_sold_asset_amount: Balance,
		_min_amount_out: Balance,
	) -> Result<Balance, DispatchError> {
		unimplemented!()
	}
	fn do_multiswap_buy_asset(
		_sender: AccountId,
		_swap_token_list: Vec<TokenId>,
		_bought_asset_amount: Balance,
		_max_amount_in: Balance,
	) -> Result<Balance, DispatchError> {
		unimplemented!()
	}

	fn buy_asset(
		_sender: AccountId,
		_sold_asset_id: TokenId,
		_bought_asset_id: TokenId,
		_bought_asset_amount: Balance,
		_max_amount_in: Balance,
		_err_upon_bad_slippage: bool,
	) -> Result<Balance, DispatchError> {
		unimplemented!()
	}

	fn multiswap_sell_asset(
		_sender: AccountId,
		_swap_token_list: Vec<TokenId>,
		_sold_asset_amount: Balance,
		_min_amount_out: Balance,
		_err_upon_bad_slippage: bool,
		_err_upon_non_slippage_fail: bool,
	) -> Result<Balance, DispatchError> {
		unimplemented!()
	}

	fn multiswap_buy_asset(
		_sender: AccountId,
		_swap_token_list: Vec<TokenId>,
		_bought_asset_amount: Balance,
		_max_amount_in: Balance,
		_err_upon_bad_slippage: bool,
		_err_upon_non_slippage_fail: bool,
	) -> Result<Balance, DispatchError> {
		unimplemented!()
	}

	fn mint_liquidity(
		_sender: AccountId,
		_first_asset_id: TokenId,
		_second_asset_id: TokenId,
		_first_asset_amount: Balance,
		_expected_second_asset_amount: Balance,
		_activate_minted_liquidity: bool,
	) -> Result<(TokenId, Balance, Balance), DispatchError> {
		unimplemented!()
	}

	fn provide_liquidity_with_conversion(
		_sender: AccountId,
		_first_asset_id: TokenId,
		_second_asset_id: TokenId,
		_provided_asset_id: TokenId,
		_provided_asset_amount: Balance,
		_activate_minted_liquidity: bool,
	) -> Result<(TokenId, Balance), DispatchError> {
		unimplemented!()
	}

	fn burn_liquidity(
		_sender: AccountId,
		_first_asset_id: TokenId,
		_second_asset_id: TokenId,
		_liquidity_asset_amount: Balance,
	) -> Result<(Balance, Balance), DispatchError> {
		unimplemented!()
	}

	fn get_tokens_required_for_minting(
		_liquidity_asset_id: TokenId,
		_liquidity_token_amount: Balance,
	) -> Result<(TokenId, Balance, TokenId, Balance), DispatchError> {
		unimplemented!()
	}

	fn is_liquidity_token(_liquidity_asset_id: TokenId) -> bool {
		true
	}

	fn do_compound_rewards(
		_sender: AccountId,
		_liquidity_asset_id: TokenId,
		_amount_permille: Permill,
	) -> DispatchResult {
		unimplemented!()
	}

	fn settle_pool_fees(
		_who: &AccountId,
		_pool_id: TokenId,
		_asset_id: TokenId,
		_fee: Balance,
	) -> Result<(), DispatchError> {
		unimplemented!()
	}
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
