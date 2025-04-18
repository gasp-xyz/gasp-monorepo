// Copyright (C) 2020 Mangata team

use super::*;

use crate as xyk;
use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{
		tokens::currency::MultiTokenCurrency, ConstU128, ConstU32, Contains, Everything, Nothing,
	},
	PalletId,
};
use frame_system as system;
pub use mangata_support::pools::{PoolInfo, ValuateFor};
use mangata_types::assets::CustomMetadata;
use orml_tokens::{MultiTokenCurrencyAdapter, MultiTokenCurrencyExtended};
use orml_traits::{asset_registry::AssetMetadata, parameter_type_with_key};
use sp_runtime::{traits::AccountIdConversion, BuildStorage, Perbill, Percent};
use std::{collections::HashMap, sync::Mutex};

pub const NATIVE_CURRENCY_ID: u32 = 0;

pub(crate) type AccountId = u128;
pub(crate) type Amount = i128;
pub(crate) type Balance = u128;
pub(crate) type TokenId = u32;

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
	pub enum Test {
		System: frame_system,
		Tokens: orml_tokens,
		XykStorage: xyk,
		ProofOfStake: pallet_proof_of_stake,
		Vesting: pallet_vesting_mangata,
		Issuance: pallet_issuance,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type AccountId = AccountId;
	type Block = Block;
	type Lookup = sp_runtime::traits::IdentityLookup<AccountId>;
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
	const MAX_VESTING_SCHEDULES: u32 = 28;
}

parameter_types! {
	pub LiquidityMiningIssuanceVault: AccountId = LiquidityMiningIssuanceVaultId::get().into_account_truncating();
	pub const StakingIssuanceVaultId: PalletId = PalletId(*b"py/stkiv");
	pub StakingIssuanceVault: AccountId = StakingIssuanceVaultId::get().into_account_truncating();
	pub const SequencerIssuanceVaultId: PalletId = PalletId(*b"py/seqiv");
	pub SequencerIssuanceVault: AccountId = SequencerIssuanceVaultId::get().into_account_truncating();
	pub const MgaTokenId: TokenId = 0u32;


	pub const TotalCrowdloanAllocation: Balance = 200_000_000;
	pub const LinearIssuanceAmount: Balance = 100_000_000_000_000_000;
	pub const LinearIssuanceBlocks: u32 = 22_222u32;
	pub const LiquidityMiningSplit: Perbill = Perbill::from_parts(555555556);
	pub const StakingSplit: Perbill = Perbill::from_parts(344444444);
	pub const SequencerSplit: Perbill = Perbill::from_parts(100000000);
	pub const ImmediateTGEReleasePercent: Percent = Percent::from_percent(20);
	pub const TGEReleasePeriod: u32 = 100u32; // 2 years
	pub const TGEReleaseBegin: u32 = 10u32; // Two weeks into chain start
	pub const BlocksPerRound: u32 = 5u32;
	pub const HistoryLimit: u32 = 10u32;
}

// #[cfg(not(feature = "runtime-benchmarks"))]
// impl pallet_issuance::Config for Test {
// 	type RuntimeEvent = RuntimeEvent;
// 	type NativeCurrencyId = MgaTokenId;
// 	type Tokens = orml_tokens::MultiTokenCurrencyAdapter<Test>;
// 	type BlocksPerRound = BlocksPerRound;
// 	type HistoryLimit = HistoryLimit;
// 	type LiquidityMiningIssuanceVault = LiquidityMiningIssuanceVault;
// 	type StakingIssuanceVault = StakingIssuanceVault;
// 	type TotalCrowdloanAllocation = TotalCrowdloanAllocation;
// 	type LinearIssuanceAmount = LinearIssuanceAmount;
// 	type LinearIssuanceBlocks = LinearIssuanceBlocks;
// 	type LiquidityMiningSplit = LiquidityMiningSplit;
// 	type StakingSplit = StakingSplit;
// 	type ImmediateTGEReleasePercent = ImmediateTGEReleasePercent;
// 	type TGEReleasePeriod = TGEReleasePeriod;
// 	type TGEReleaseBegin = TGEReleaseBegin;
// 	type VestingProvider = Vesting;
// 	type WeightInfo = ();
// }
//
impl pallet_issuance::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type NativeCurrencyId = MgaTokenId;
	type Tokens = orml_tokens::MultiTokenCurrencyAdapter<Test>;
	type BlocksPerRound = BlocksPerRound;
	type HistoryLimit = HistoryLimit;
	type LiquidityMiningIssuanceVault = LiquidityMiningIssuanceVault;
	type StakingIssuanceVault = StakingIssuanceVault;
	type SequencersIssuanceVault = SequencerIssuanceVault;
	type TotalCrowdloanAllocation = TotalCrowdloanAllocation;
	type LinearIssuanceAmount = LinearIssuanceAmount;
	type LinearIssuanceBlocks = LinearIssuanceBlocks;
	type LiquidityMiningSplit = LiquidityMiningSplit;
	type StakingSplit = StakingSplit;
	type SequencersSplit = SequencerSplit;
	type ImmediateTGEReleasePercent = ImmediateTGEReleasePercent;
	type TGEReleasePeriod = TGEReleasePeriod;
	type TGEReleaseBegin = TGEReleaseBegin;
	type VestingProvider = Vesting;
	type WeightInfo = ();
	type LiquidityMiningApi = ProofOfStake;
}

impl XykBenchmarkingConfig for Test {}

parameter_types! {
	pub const LiquidityMiningIssuanceVaultId: PalletId = PalletId(*b"py/lqmiv");
	pub FakeLiquidityMiningIssuanceVault: AccountId = LiquidityMiningIssuanceVaultId::get().into_account_truncating();
}

pub struct DummyBlacklistedPool;

impl Contains<(TokenId, TokenId)> for DummyBlacklistedPool {
	fn contains(pair: &(TokenId, TokenId)) -> bool {
		pair == &(1_u32, 9_u32) || pair == &(9_u32, 1_u32)
	}
}

pub struct MockAssetRegister;

lazy_static::lazy_static! {
	static ref ASSET_REGISTER: Mutex<HashMap<TokenId, AssetMetadata<Balance, CustomMetadata, ConstU32<20>>>> = {
		let m = HashMap::new();
		Mutex::new(m)
	};
}

#[cfg(test)]
impl MockAssetRegister {
	pub fn instance(
	) -> &'static Mutex<HashMap<TokenId, AssetMetadata<Balance, CustomMetadata, ConstU32<20>>>> {
		&ASSET_REGISTER
	}
}

impl AssetMetadataMutationTrait<TokenId> for MockAssetRegister {
	fn set_asset_info(
		asset: TokenId,
		name: Vec<u8>,
		symbol: Vec<u8>,
		decimals: u32,
	) -> DispatchResult {
		let meta = AssetMetadata {
			name: BoundedVec::truncate_from(name),
			symbol: BoundedVec::truncate_from(symbol),
			decimals,
			additional: Default::default(),
			existential_deposit: 0,
		};
		let mut register = ASSET_REGISTER.lock().unwrap();
		register.insert(asset, meta);
		Ok(())
	}
}

pub struct MockMaintenanceStatusProvider;

lazy_static::lazy_static! {
	static ref MAINTENANCE_STATUS: Mutex<bool> = {
		let m: bool = false;
		Mutex::new(m)
	};
}

#[cfg(test)]
impl MockMaintenanceStatusProvider {
	pub fn instance() -> &'static Mutex<bool> {
		&MAINTENANCE_STATUS
	}
}

impl MockMaintenanceStatusProvider {
	pub fn set_maintenance(value: bool) {
		let mut mutex = Self::instance().lock().unwrap();
		*mutex = value;
	}
}

impl GetMaintenanceStatusTrait for MockMaintenanceStatusProvider {
	fn is_maintenance() -> bool {
		let mutex = Self::instance().lock().unwrap();
		*mutex
	}

	fn is_upgradable() -> bool {
		unimplemented!()
	}
}

#[cfg(not(feature = "runtime-benchmarks"))]
impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaintenanceStatusProvider = MockMaintenanceStatusProvider;
	type ActivationReservesProvider = TokensActivationPassthrough<Test>;
	type Currency = MultiTokenCurrencyAdapter<Test>;
	type NativeCurrencyId = NativeCurrencyId;
	type TreasuryPalletId = TreasuryPalletId;
	type BnbTreasurySubAccDerive = BnbTreasurySubAccDerive;
	type PoolFeePercentage = ConstU128<20>;
	type TreasuryFeePercentage = ConstU128<5>;
	type BuyAndBurnFeePercentage = ConstU128<5>;
	type LiquidityMiningRewards = ProofOfStake;
	type WeightInfo = ();
	type VestingProvider = Vesting;
	type DisallowedPools = DummyBlacklistedPool;
	type DisabledTokens = Nothing;
	type AssetMetadataMutation = MockAssetRegister;
	type FeeLockWeight = ();
}

#[cfg(feature = "runtime-benchmarks")]
impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaintenanceStatusProvider = MockMaintenanceStatusProvider;
	type ActivationReservesProvider = TokensActivationPassthrough<Test>;
	type Currency = MultiTokenCurrencyAdapter<Test>;
	type NativeCurrencyId = NativeCurrencyId;
	type TreasuryPalletId = TreasuryPalletId;
	type BnbTreasurySubAccDerive = BnbTreasurySubAccDerive;
	type PoolFeePercentage = ConstU128<0>;
	type TreasuryFeePercentage = ConstU128<0>;
	type BuyAndBurnFeePercentage = ConstU128<0>;
	type LiquidityMiningRewards = ProofOfStake;
	type WeightInfo = ();
	type VestingProvider = Vesting;
	type DisallowedPools = Nothing;
	type DisabledTokens = Nothing;
	type AssetMetadataMutation = MockAssetRegister;
	type FeeLockWeight = ();
}

mockall::mock! {
	pub ValuationApi {}

	impl mangata_support::pools::Valuate for ValuationApi {
		type CurrencyId = TokenId;
		type Balance = Balance;

		fn find_paired_pool(base_id: TokenId, asset_id: TokenId) -> Result<Vec<PoolInfo<TokenId, Balance>>, DispatchError>;

		fn check_can_valuate(base_id: TokenId, pool_id: TokenId) -> Result<(), DispatchError>;

		fn check_pool_exist(pool_id: TokenId) -> Result<(), DispatchError>;

		fn get_reserve_and_lp_supply(base_id: TokenId, pool_id: TokenId) -> Option<(Balance, Balance)>;

		fn get_valuation_for_paired(base_id: TokenId, pool_id: TokenId, amount: Balance) -> Balance;

		fn find_valuation(base_id: TokenId, asset_id: TokenId, amount: Balance) -> Result<Balance, DispatchError>;
	}
}
impl ValuateFor<NativeCurrencyId> for MockValuationApi {}

#[cfg(not(feature = "runtime-benchmarks"))]
impl pallet_proof_of_stake::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ActivationReservesProvider = TokensActivationPassthrough<Test>;
	type NativeCurrencyId = NativeCurrencyId;
	type Currency = MultiTokenCurrencyAdapter<Test>;
	type LiquidityMiningIssuanceVault = FakeLiquidityMiningIssuanceVault;
	type RewardsDistributionPeriod = ConstU32<10>;
	type WeightInfo = ();
	type RewardsSchedulesLimit = ConstU32<10>;
	type Min3rdPartyRewardValutationPerSession = ConstU128<10>;
	type Min3rdPartyRewardVolume = ConstU128<10>;
	type ValuationApi = MockValuationApi;
	type SchedulesPerBlock = ConstU32<5>;
	type NontransferableTokens = Nothing;
}

#[cfg(feature = "runtime-benchmarks")]
impl pallet_proof_of_stake::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ActivationReservesProvider = TokensActivationPassthrough<Test>;
	type NativeCurrencyId = NativeCurrencyId;
	type Currency = MultiTokenCurrencyAdapter<Test>;
	type LiquidityMiningIssuanceVault = FakeLiquidityMiningIssuanceVault;
	type RewardsDistributionPeriod = ConstU32<1200>;
	type WeightInfo = ();
	type RewardsSchedulesLimit = ConstU32<10>;
	type Min3rdPartyRewardValutationPerSession = ConstU128<10>;
	type Min3rdPartyRewardVolume = ConstU128<10>;
	type ValuationApi = MockValuationApi;
	type SchedulesPerBlock = ConstU32<5>;
	type NontransferableTokens = Nothing;
}

pub struct TokensActivationPassthrough<T: Config>(PhantomData<T>);

impl<T: Config> ActivationReservesProviderTrait<AccountId, Balance, TokenId>
	for TokensActivationPassthrough<T>
where
	<T as pallet::Config>::Currency:
		MultiTokenReservableCurrency<AccountId, Balance = Balance, CurrencyId = TokenId>,
{
	fn get_max_instant_unreserve_amount(token_id: TokenId, account_id: &AccountId) -> Balance {
		ProofOfStake::get_rewards_info(account_id, token_id).activated_amount
	}

	fn can_activate(
		token_id: TokenId,
		account_id: &AccountId,
		amount: Balance,
		_use_balance_from: Option<ActivateKind>,
	) -> bool {
		<T as pallet::Config>::Currency::can_reserve(token_id, account_id, amount)
	}

	fn activate(
		token_id: TokenId,
		account_id: &AccountId,
		amount: Balance,
		_use_balance_from: Option<ActivateKind>,
	) -> DispatchResult {
		<T as pallet::Config>::Currency::reserve(token_id, account_id, amount)
	}

	fn deactivate(token_id: TokenId, account_id: &AccountId, amount: Balance) -> Balance {
		<T as pallet::Config>::Currency::unreserve(token_id, account_id, amount)
	}
}

impl<T: Config> Pallet<T>
where
	<T as pallet::Config>::Currency: MultiTokenReservableCurrency<AccountId, Balance = Balance, CurrencyId = TokenId>
		+ MultiTokenCurrencyExtended<AccountId>,
{
	pub fn balance(id: TokenId, who: AccountId) -> Balance {
		<T as Config>::Currency::free_balance(id.into(), &who).into()
	}
	pub fn reserved(id: TokenId, who: AccountId) -> Balance {
		<T as Config>::Currency::reserved_balance(id.into(), &who).into()
	}
	pub fn total_supply(id: TokenId) -> Balance {
		<T as Config>::Currency::total_issuance(id.into()).into()
	}
	pub fn transfer(
		currency_id: TokenId,
		source: AccountId,
		dest: AccountId,
		value: Balance,
	) -> DispatchResult {
		<T as Config>::Currency::transfer(
			currency_id,
			&source,
			&dest,
			value,
			ExistenceRequirement::KeepAlive,
		)
	}
	pub fn create_new_token(who: &AccountId, amount: Balance) -> TokenId {
		<T as Config>::Currency::create(who, amount).expect("Token creation failed")
	}

	pub fn mint_token(token_id: TokenId, who: &AccountId, amount: Balance) {
		<T as Config>::Currency::mint(token_id, who, amount).expect("Token minting failed")
	}
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities =
		system::GenesisConfig::<Test>::default().build_storage().unwrap().into();
	ext.execute_with(|| {
		System::set_block_number(1);
		MockMaintenanceStatusProvider::set_maintenance(false);
	});
	ext
}

pub(crate) fn events() -> Vec<pallet::Event<Test>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let RuntimeEvent::XykStorage(inner) = e { Some(inner) } else { None })
		.collect::<Vec<_>>()
}

/// Compares the system events with passed in events
/// Prints highlighted diff iff assert_eq fails
#[macro_export]
macro_rules! assert_eq_events {
	($events:expr) => {
		match &$events {
			e => similar_asserts::assert_eq!(*e, $crate::mock::events()),
		}
	};
}

/// Panics if an event is not found in the system log of events
#[macro_export]
macro_rules! assert_event_emitted {
	($event:expr) => {
		match &$event {
			e => {
				assert!(
					$crate::mock::events().iter().find(|x| *x == e).is_some(),
					"Event {:?} was not found in events: \n {:?}",
					e,
					crate::mock::events()
				);
			},
		}
	};
}
