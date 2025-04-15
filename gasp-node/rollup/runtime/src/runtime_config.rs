#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::*;

pub mod currency {
	use super::Balance;

	pub const MILLICENTS: Balance = CENTS / 1000;
	pub const CENTS: Balance = DOLLARS / 100; // assume this is worth about a cent.
	pub const DOLLARS: Balance = super::consts::UNIT;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 5000 * DOLLARS + (bytes as Balance) * 60 * CENTS
	}
}

pub mod types {
	use super::*;

	pub type TokenId = u32;
	pub type Balance = u128;
	pub type Amount = i128;

	// /// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
	pub type Signature = EthereumSignature;

	pub type Signer = <Signature as Verify>::Signer;

	// /// Some way of identifying an account on the chain. We intentionally make it equivalent
	// /// to the public key of our transaction signing scheme.
	pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

	// /// Index of a transaction in the chain.
	pub type Nonce = u32;

	// /// A hash of some data used by the chain.
	pub type Hash = sp_core::H256;

	// /// An index to a block.
	pub type BlockNumber = u32;

	// /// The address format for describing accounts.
	pub type Address = AccountId;
}

pub mod tokens {
	use super::*;
	pub const RX_TOKEN_ID: TokenId = 0;
	pub const ETH_TOKEN_ID: TokenId = 1;

	parameter_types! {
		pub const RxTokenId: TokenId = RX_TOKEN_ID;
		pub const EthTokenId: TokenId = ETH_TOKEN_ID;
		pub ArbitrageBotAddr: AccountId = sp_runtime::AccountId20::from(
			hex_literal::hex!["0286Ffa54213778E064179E9B6F083ecb584E862"]
		);
	}
}

pub mod fees {
	use super::*;

	pub type MarketPoolFeePercentage = frame_support::traits::ConstU128<20>;
	pub type MarketTreasuryFeePercentage = frame_support::traits::ConstU128<5>;
	pub type MarketBuyAndBurnFeePercentage = frame_support::traits::ConstU128<5>;
	pub type MarketFeeDenominator = ConstU128<10_000>;

	pub type MinSwapFee = ConstU128<6>;

	// We set these to 0 to not have the stable swap pallet charge any exchange commission fees
	// We charge commission in the market pallet on the input assetof the first atomic swap
	// xyk uses 10_000 as fee multiplier
	pub type PoolFeePercentage = frame_support::traits::ConstU128<0>;
	pub type TreasuryFeePercentage = frame_support::traits::ConstU128<0>;
	pub type BuyAndBurnFeePercentage = frame_support::traits::ConstU128<0>;

	// We set these to 0 to not have the stable swap pallet charge any exchange commission fees
	// We charge commission in the market pallet on the input assetof the first atomic swap
	// market & stableswap uses 1e10 precision; 1*1e10 == 100%
	// 0.3%, sum of above fees
	pub type SSwapTotalFee = ConstU128<0>;
	// 33% of total fee goes to treasury
	pub type SSwapTreasuryFeePart = ConstU128<0>;
	// 50% of treasury fee gets to burn
	pub type SSwapBnBFeePart = ConstU128<0>;

	pub const FEE_PRECISION: u128 = pallet_stable_swap::Pallet::<Runtime>::FEE_DENOMINATOR;
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	use sp_runtime::{
		generic,
		traits::{BlakeTwo256, Hash as HashT},
	};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	pub type Header = generic::HeaderVer<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque block hash type.
	pub type Hash = <BlakeTwo256 as HashT>::Output;
}

pub mod runtime_types {
	use super::*;

	pub type SignedExtra<Runtime> = (
		frame_system::CheckSpecVersion<Runtime>,
		frame_system::CheckTxVersion<Runtime>,
		frame_system::CheckGenesis<Runtime>,
		frame_system::CheckEra<Runtime>,
		frame_system::CheckNonce<Runtime>,
		frame_system::CheckWeight<Runtime>,
		pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
		frame_system::CheckNonZeroSender<Runtime>,
	);

	pub type SignedPayload<Runtime, RuntimeCall> =
		generic::SignedPayload<RuntimeCall, SignedExtra<Runtime>>;
	pub type UncheckedExtrinsic<Runtime, RuntimeCall> =
		generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra<Runtime>>;
	pub type CheckedExtrinsic<Runtime, RuntimeCall> =
		generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra<Runtime>>;
	pub type Header = generic::HeaderVer<BlockNumber, BlakeTwo256>;
	pub type Block<Runtime, RuntimeCall> =
		generic::Block<Header, UncheckedExtrinsic<Runtime, RuntimeCall>>;
	pub type SignedBlock<Runtime, RuntimeCall> = generic::SignedBlock<Block<Runtime, RuntimeCall>>;
	pub type BlockId<Runtime, RuntimeCall> = generic::BlockId<Block<Runtime, RuntimeCall>>;

	pub type OpaqueBlock = generic::Block<Header, sp_runtime::OpaqueExtrinsic>;
	pub type OpaqueBlockId = generic::BlockId<OpaqueBlock>;
}

pub mod consts {
	use super::*;
	/// This determines the average expected block time that we are targeting.
	/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
	/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
	/// up by `pallet_aura` to implement `fn slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLISECS_PER_BLOCK: u64 = 6000;
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

	// Time is measured by number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	// Unit = the base number of indivisible units for balance
	pub const UNIT: Balance = 1_000_000_000_000_000_000;
	pub const MILLIUNIT: Balance = 1_000_000_000_000_000;
	pub const MICROUNIT: Balance = 1_000_000_000_000;

	/// We allow for 2 seconds of compute with a 6 second average block time, with maximum proof size.
	/// NOTE: reduced by half comparing to origin impl as we want to fill block only up to 50%
	/// so there is room for new extrinsics in the next block
	pub const MAXIMUM_BLOCK_WEIGHT: Weight =
		Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND, u64::MAX);
}

#[derive(Eq, PartialEq, Encode, Decode, Clone)]
pub enum CallType {
	UnlockFee,
	UtilityInnerCall,
	Other,
	Swap {
		swap_pool_list: Vec<TokenId>,
		asset_id_in: TokenId,
		asset_amount_in: Balance,
		asset_id_out: TokenId,
		asset_amount_out: Balance,
	},
	CouncilCall,
}

pub mod config {
	use super::*;

	pub type TreasuryPalletIdOf<T> = <T as ::pallet_treasury::Config>::PalletId;

	pub struct TreasuryAccountIdOf<T: ::pallet_treasury::Config>(PhantomData<T>);
	impl<T: ::pallet_treasury::Config> Get<T::AccountId> for TreasuryAccountIdOf<T> {
		fn get() -> T::AccountId {
			TreasuryPalletIdOf::<T>::get().into_account_truncating()
		}
	}
	pub struct BnbAccountIdOf<T: ::pallet_treasury::Config>(PhantomData<T>);
	impl<T: ::pallet_treasury::Config> Get<T::AccountId> for BnbAccountIdOf<T> {
		fn get() -> T::AccountId {
			TreasuryPalletIdOf::<T>::get()
				.into_sub_account_truncating(pallet_xyk::BnbTreasurySubAccDerive::get())
		}
	}

	pub type ExistentialDepositsOf<T> = <T as ::orml_tokens::Config>::ExistentialDeposits;
	pub type MaxLocksOf<T> = <T as ::orml_tokens::Config>::MaxLocks;
	pub type SessionLenghtOf<T> = <T as ::parachain_staking::Config>::BlocksPerRound;

	pub mod frame_system {
		use super::*;

		/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
		/// used to limit the maximal weight of a single extrinsic.
		pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

		/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
		/// `Operational` extrinsics.
		pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

		pub type MaxConsumers = frame_support::traits::ConstU32<16>;

		parameter_types! {
			pub const BaseWeightOffset: Weight = Weight::from_parts(<Runtime as ::frame_system::Config>::DbWeight::get().read, 0);
			pub const VerExtrinsicBaseWeight: Weight = weights::VerExtrinsicBaseWeight::get().saturating_add(BaseWeightOffset::get());
			pub const BlockHashCount: BlockNumber = 2400;
			pub const Version: sp_version::RuntimeVersion = crate::VERSION;
			// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
			//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
			// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
			// the lazy contract deletion.
			pub RuntimeBlockLength: BlockLength =
				BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
			pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
				.base_block(weights::VerBlockExecutionWeight::get())
				.for_class(DispatchClass::all(), |weights| {
					weights.base_extrinsic = VerExtrinsicBaseWeight::get();
				})
				.for_class(DispatchClass::Normal, |weights| {
					weights.max_total = Some(NORMAL_DISPATCH_RATIO * consts::MAXIMUM_BLOCK_WEIGHT);
				})
				.for_class(DispatchClass::Operational, |weights| {
					weights.max_total = Some(consts::MAXIMUM_BLOCK_WEIGHT);
					// Operational transactions have some extra reserved space, so that they
					// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
					weights.reserved = Some(
						consts::MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * consts::MAXIMUM_BLOCK_WEIGHT
					);
				})
				.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
				.build_or_panic();
			pub const SS58Prefix: u16 = 42;
		}

		pub struct MaintenanceGatedSetCode<T, E>(PhantomData<T>, PhantomData<E>);

		impl<T, E> SetCode<T> for MaintenanceGatedSetCode<T, E>
		where
			T: ::pallet_maintenance::Config,
			E: SetCode<T>,
		{
			fn set_code(code: Vec<u8>) -> DispatchResult {
				if !::pallet_maintenance::Pallet::<T>::is_upgradable() {
					return Err(::pallet_maintenance::Error::<T>::UpgradeBlockedByMaintenance.into())
				}
				E::set_code(code)
			}
		}
	}

	pub mod pallet_timestamp {
		use super::*;

		// NOTE: Currently it is not possible to change the slot duration after the chain has started.
		//       Attempting to do so will brick block production.
		parameter_types! {
			pub const MinimumPeriod: u64 = consts::MILLISECS_PER_BLOCK / 2;
		}
	}

	pub mod pallet_treasury {
		use super::*;

		parameter_types! {
			pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
		}

		parameter_types! {
			pub const ProposalBond: Permill = Permill::from_percent(5);
			pub const ProposalBondMinimum: Balance = 1 * currency::DOLLARS;
			pub const ProposalBondMaximum: Option<Balance> = None;
			pub const SpendPeriod: BlockNumber = 1 * consts::DAYS;
			pub const Burn: Permill = Permill::from_percent(0);
			pub const MaxApprovals: u32 = 100;
			pub const SpendPayoutPeriod: BlockNumber = 30 * consts::DAYS;
		}
	}

	pub mod orml_tokens {
		use super::*;

		parameter_types! {
			pub const MaxLocks: u32 = 50;
		}

		parameter_type_with_key! {
			pub ExistentialDeposits: |_currency_id: TokenId| -> Balance {
				0
			};
		}

		pub struct DustRemovalWhitelist<T: Get<AccountId>>(PhantomData<T>);
		impl<T: Get<AccountId>> Contains<AccountId> for DustRemovalWhitelist<T> {
			fn contains(a: &AccountId) -> bool {
				*a == T::get()
			}
		}

		pub type ReserveIdentifier = [u8; 8];
	}

	pub mod pallet_xyk {
		use super::*;

		parameter_types! {
			pub const BnbTreasurySubAccDerive: [u8; 4] = *b"bnbt";
		}

		pub struct TestTokensFilter;
		impl Contains<TokenId> for TestTokensFilter {
			fn contains(_: &TokenId) -> bool {
				// we dont want to allow doing anything with dummy assets previously
				// used for testing
				false
			}
		}

		pub struct AssetRegisterFilter<Runtime>(PhantomData<Runtime>);
		impl<T> Contains<TokenId> for AssetRegisterFilter<T>
		where
			T: ::orml_asset_registry::Config<
				CustomMetadata = CustomMetadata,
				AssetId = TokenId,
				Balance = Balance,
			>,
		{
			fn contains(t: &TokenId) -> bool {
				let meta: Option<_> = ::orml_asset_registry::Metadata::<T>::get(*t);
				if let Some(xyk) = meta.and_then(|m| m.additional.xyk) {
					return xyk.operations_disabled
				}
				return false
			}
		}

		pub struct AssetMetadataMutation<Runtime>(PhantomData<Runtime>);

		impl<T> AssetMetadataMutationTrait<TokenId> for AssetMetadataMutation<T>
		where
			T: ::orml_asset_registry::Config<
				CustomMetadata = CustomMetadata,
				AssetId = TokenId,
				Balance = Balance,
				StringLimit = orml_asset_registry::StringLimit,
			>,
		{
			fn set_asset_info(
				asset: TokenId,
				name: Vec<u8>,
				symbol: Vec<u8>,
				decimals: u32,
			) -> DispatchResult {
				let metadata = AssetMetadata {
					name: BoundedVec::truncate_from(name),
					symbol: BoundedVec::truncate_from(symbol),
					decimals,
					existential_deposit: Default::default(),
					additional: Default::default(),
				};
				::orml_asset_registry::Pallet::<T>::do_register_asset_without_asset_processor(
					metadata, asset,
				)?;
				Ok(())
			}
		}
	}

	pub mod pallet_stable_swap {
		use super::*;

		parameter_types! {
			pub const MaxEqAssets: u32 = 10;
		}
	}

	pub mod pallet_bootstrap {
		use super::*;

		parameter_types! {
			pub const BootstrapUpdateBuffer: BlockNumber = 300;
			pub const DefaultBootstrapPromotedPoolWeight: u8 = 0u8;
			pub const ClearStorageLimit: u32 = 100u32;
		}

		pub struct EnableAssetPoolApi<Runtime>(PhantomData<Runtime>);
		impl<T> AssetRegistryApi<TokenId> for EnableAssetPoolApi<T>
		where
			T: ::orml_asset_registry::Config<
				CustomMetadata = CustomMetadata,
				AssetId = TokenId,
				Balance = Balance,
			>,
		{
			fn enable_pool_creation(assets: (TokenId, TokenId)) -> bool {
				for &asset in [assets.0, assets.1].iter() {
					let meta_maybe: Option<_> = ::orml_asset_registry::Metadata::<T>::get(asset);
					if let Some(xyk) = meta_maybe.clone().and_then(|m| m.additional.xyk) {
						let mut additional = meta_maybe.unwrap().additional;
						if xyk.operations_disabled {
							additional.xyk = Some(XykMetadata { operations_disabled: false });
							match ::orml_asset_registry::Pallet::<T>::do_update_asset(
								asset,
								None,
								None,
								None,
								None,
								Some(additional),
							) {
								Ok(_) => {},
								Err(e) => {
									log::error!(target: "bootstrap", "cannot modify {} asset: {:?}!", asset, e);
									return false
								},
							}
						}
					}
				}
				true
			}
		}
	}

	pub mod pallet_transaction_payment {
		use crate::*;

		parameter_types! {
			pub const OperationalFeeMultiplier: u8 = 5;
			pub const TransactionByteFee: Balance = 5 * consts::MILLIUNIT;
			pub ConstFeeMultiplierValue: Multiplier = Multiplier::saturating_from_rational(1, 1);
		}

		pub type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
		pub type FeeMultiplierUpdate = ConstFeeMultiplier<ConstFeeMultiplierValue>;

		pub type ORMLCurrencyAdapterNegativeImbalance<Runtime> =
			<::orml_tokens::MultiTokenCurrencyAdapter<Runtime> as MultiTokenCurrency<
				<Runtime as ::frame_system::Config>::AccountId,
			>>::NegativeImbalance;

		pub trait OnMultiTokenUnbalanced<
			TokenIdType,
			Imbalance: ::frame_support::traits::TryDrop + MultiTokenImbalanceWithZeroTrait<TokenIdType>,
		>
		{
			/// Handler for some imbalances. The different imbalances might have different origins or
			/// meanings, dependent on the context. Will default to simply calling on_unbalanced for all
			/// of them. Infallible.
			fn on_unbalanceds<B>(token_id: TokenIdType, amounts: impl Iterator<Item = Imbalance>)
			where
				Imbalance: ::frame_support::traits::Imbalance<B>,
			{
				Self::on_unbalanced(amounts.fold(Imbalance::from_zero(token_id), |i, x| x.merge(i)))
			}

			/// Handler for some imbalance. Infallible.
			fn on_unbalanced(amount: Imbalance) {
				amount.try_drop().unwrap_or_else(Self::on_nonzero_unbalanced)
			}

			/// Actually handle a non-zero imbalance. You probably want to implement this rather than
			/// `on_unbalanced`.
			fn on_nonzero_unbalanced(amount: Imbalance) {
				drop(amount);
			}
		}

		pub struct ToAuthor<Runtime>(PhantomData<Runtime>);
		impl<T: ::orml_tokens::Config + ::pallet_authorship::Config>
			OnMultiTokenUnbalanced<T::CurrencyId, ORMLCurrencyAdapterNegativeImbalance<T>> for ToAuthor<T>
		{
			fn on_nonzero_unbalanced(amount: ORMLCurrencyAdapterNegativeImbalance<T>) {
				if let Some(author) = ::pallet_authorship::Pallet::<T>::author() {
					<::orml_tokens::MultiTokenCurrencyAdapter<T> as MultiTokenCurrency<
						<T as ::frame_system::Config>::AccountId,
					>>::resolve_creating(amount.0, &author, amount);
				}
			}
		}

		#[derive(Encode, Decode, TypeInfo)]
		pub enum LiquidityInfoEnum<C: MultiTokenCurrency<T::AccountId>, T: frame_system::Config> {
			Imbalance((C::CurrencyId, NegativeImbalanceOf<C, T>)),
		}

		pub struct FeeHelpers<T, Currency, FeeLock>(PhantomData<(T, Currency, FeeLock)>);
		impl<T, Currency, FeeLock> FeeHelpers<T, Currency, FeeLock>
		where
			T: pallet_fee_lock::Config<Tokens = Currency>,
			Currency:
				MultiTokenCurrencyExtended<T::AccountId, Balance = Balance, CurrencyId = TokenId>,
			FeeLock: FeeLockTriggerTrait<T::AccountId, Balance, TokenId>,
			sp_runtime::AccountId20: From<T::AccountId> + Into<T::AccountId>,
		{
			pub fn can_withdraw_amount_in(
				who: &<T>::AccountId,
				asset_id_in: TokenId,
				asset_amount_in: Balance,
			) -> Result<(), TransactionValidityError> {
				Currency::ensure_can_withdraw(
					asset_id_in,
					who,
					asset_amount_in,
					// Last two args are ignored by orml_tokens
					WithdrawReasons::all(),
					Default::default(),
				)
				.map_err(|_| {
					TransactionValidityError::Invalid(InvalidTransaction::SwapPrevalidation.into())
				})?;
				Ok(())
			}
		}

		#[derive(Encode, Decode, Clone, TypeInfo)]
		pub struct OnChargeHandler<C, OU, OCA, OFLA>(PhantomData<(C, OU, OCA, OFLA)>);

		/// Default implementation for a Currency and an OnUnbalanced handler.
		///
		/// The unbalance handler is given 2 unbalanceds in [`OnUnbalanced::on_unbalanceds`]: fee and
		/// then tip.
		impl<T, C, OU, OCA, OFLA> OnChargeTransaction<T> for OnChargeHandler<C, OU, OCA, OFLA>
		where
			T: pallet_transaction_payment::Config
				+ pallet_treasury::Config
				+ pallet_fee_lock::Config<Tokens = C>
				+ frame_system::Config<RuntimeCall = RuntimeCall, AccountId = sp_runtime::AccountId20>,
			<T as frame_system::Config>::RuntimeCall:
				Into<crate::CallType> + Dispatchable<PostInfo = PostDispatchInfo>,
			C: MultiTokenCurrencyExtended<
				sp_runtime::AccountId20,
				Balance = Balance,
				CurrencyId = TokenId,
			>,
			OU: OnMultiTokenUnbalanced<TokenId, NegativeImbalanceOf<C, T>>,
			OCA: OnChargeTransaction<
				T,
				LiquidityInfo = Option<LiquidityInfoEnum<C, T>>,
				Balance = Balance,
			>,
			OFLA: FeeLockTriggerTrait<T::AccountId, Balance, TokenId>,
			sp_runtime::AccountId20: From<T::AccountId> + Into<T::AccountId>,
		{
			type LiquidityInfo = Option<LiquidityInfoEnum<C, T>>;
			type Balance = Balance;

			/// Withdraw the predicted fee from the transaction origin.
			///
			/// Note: The `fee` already includes the `tip`.
			fn withdraw_fee(
				who: &T::AccountId,
				call: &T::RuntimeCall,
				info: &DispatchInfoOf<T::RuntimeCall>,
				fee: Self::Balance,
				tip: Self::Balance,
			) -> Result<Self::LiquidityInfo, TransactionValidityError> {
				let call_type: crate::CallType = (*call).clone().into();

				match call_type.clone() {
					CallType::Swap {
						swap_pool_list,
						asset_id_in,
						asset_amount_in,
						asset_id_out,
						asset_amount_out,
					} => {
						ensure!(
							tip.is_zero(),
							TransactionValidityError::Invalid(
								InvalidTransaction::TippingNotAllowedForSwaps.into(),
							)
						);
						ensure!(
							!swap_pool_list.len().is_zero(),
							TransactionValidityError::Invalid(
								InvalidTransaction::SwapPrevalidation.into(),
							)
						);
						ensure!(
							swap_pool_list.len() <=
								<Runtime as pallet_market::Config>::MaxSwapListLength::get()
									as usize,
							TransactionValidityError::Invalid(
								InvalidTransaction::SwapPrevalidation.into(),
							)
						);
						let first_pool_info =
							Market::get_pool_info(swap_pool_list[0]).map_err(|_| {
								TransactionValidityError::Invalid(
									InvalidTransaction::SwapPrevalidation.into(),
								)
							})?;
						ensure!(
							first_pool_info.pool.0 == asset_id_in ||
								first_pool_info.pool.1 == asset_id_in,
							TransactionValidityError::Invalid(
								InvalidTransaction::SwapPrevalidation.into(),
							)
						);
						let check_amount = asset_amount_in
							.max(<Runtime as pallet_market::Config>::MinSwapFee::get());
						FeeHelpers::<T, C, OFLA>::can_withdraw_amount_in(
							who,
							asset_id_in,
							check_amount,
						)?;

						let is_sell_swap = match *call {
							RuntimeCall::Market(pallet_market::Call::multiswap_asset {
								..
							}) => true,
							_ => false,
						};

						let native_token_id = tokens::RxTokenId::get();
						// This should only fail if the fee_lock_metadata is uninit
						// And if it is uninit then we use fee_lock_amount as zero to trivially pass these checks here
						// because later on fee lock metadata being uninit we are going to charge normal fees anyway
						let fee_lock_amount = FeeLock::get_fee_lock_amount(who).unwrap_or_default();
						let user_has_fee_lock_amount = match (asset_id_in, native_token_id) {
							(a, b) if a == b => FeeHelpers::<T, C, OFLA>::can_withdraw_amount_in(
								who,
								native_token_id,
								check_amount.checked_add(fee_lock_amount).ok_or(
									TransactionValidityError::Invalid(
										InvalidTransaction::SwapPrevalidation.into(),
									),
								)?,
							)
							.is_ok(),
							_ => FeeHelpers::<T, C, OFLA>::can_withdraw_amount_in(
								who,
								native_token_id,
								fee_lock_amount,
							)
							.is_ok(),
						};
						match (user_has_fee_lock_amount, is_sell_swap) {
							(true, _) => {},
							(false, true) => {
								ensure!(
									FeeLock::is_whitelisted(asset_id_in),
									TransactionValidityError::Invalid(
										InvalidTransaction::SwapPrevalidation.into(),
									)
								);
								ensure!(
									FeeLock::get_token_value_threshold(asset_id_in) <=
										asset_amount_in,
									TransactionValidityError::Invalid(
										InvalidTransaction::SwapPrevalidation.into(),
									)
								);
								ensure!(
									native_token_id == asset_id_out,
									TransactionValidityError::Invalid(
										InvalidTransaction::SwapPrevalidation.into(),
									)
								);
							},
							(false, false) =>
								return Err(TransactionValidityError::Invalid(
									InvalidTransaction::SwapPrevalidation.into(),
								)),
						}
					},
					_ => {},
				};

				match (call_type, pallet_fee_lock::FeeLockMetadata::<T>::get()) {
					(crate::CallType::UnlockFee, _) => {
						let imb = C::withdraw(
							tokens::RxTokenId::get().into(),
							who,
							tip,
							WithdrawReasons::TIP,
							ExistenceRequirement::KeepAlive,
						)
						.map_err(|_| {
							TransactionValidityError::Invalid(InvalidTransaction::Payment.into())
						})?;

						OU::on_unbalanceds(tokens::RxTokenId::get().into(), Some(imb).into_iter());
						OFLA::can_unlock_fee(who).map_err(|_| {
							TransactionValidityError::Invalid(InvalidTransaction::UnlockFee.into())
						})?;
						Ok(None)
					},
					(CallType::Swap { .. }, Some(fee_lock_metadata)) => Ok(None),
					// TODO - very low priority
					// Ideally there should be another arm here for swap but no fee_lock_metadata
					// So that Gasp and Eth swap inputs both can be checked as total to avoid
					// Too little assets for both fees together...
					// But too niche a problem for how annoying the solution is
					(CallType::CouncilCall, _) if Council::is_member(&who.clone().into()) =>
						Ok(None),
					_ => OCA::withdraw_fee(who, call, info, fee, tip),
				}
			}

			/// Hand the fee and the tip over to the `[OnUnbalanced]` implementation.
			/// Since the predicted fee might have been too high, parts of the fee may
			/// be refunded.
			///
			/// Note: The `corrected_fee` already includes the `tip`.
			fn correct_and_deposit_fee(
				who: &T::AccountId,
				dispatch_info: &DispatchInfoOf<T::RuntimeCall>,
				post_info: &PostDispatchInfo,
				corrected_fee: Self::Balance,
				tip: Self::Balance,
				already_withdrawn: Self::LiquidityInfo,
			) -> Result<(), TransactionValidityError> {
				match already_withdrawn {
					Some(LiquidityInfoEnum::Imbalance(_)) => OCA::correct_and_deposit_fee(
						who,
						dispatch_info,
						post_info,
						corrected_fee,
						tip,
						already_withdrawn,
					),
					None => Ok(()),
				}
			}
		}

		#[derive(Encode, Decode, Clone, TypeInfo)]
		pub struct TwoCurrencyOnChargeAdapter<C, OU, T1, T2, SF, TE>(
			PhantomData<(C, OU, T1, T2, SF, TE)>,
		);

		type NegativeImbalanceOf<C, T> =
			<C as MultiTokenCurrency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

		pub trait TriggerEvent<AccountIdT> {
			fn trigger(who: AccountIdT, token_id: TokenId, fee: Balance, tip: Balance);
		}

		/// Default implementation for a Currency and an OnUnbalanced handler.
		///
		/// The unbalance handler is given 2 unbalanceds in [`OnUnbalanced::on_unbalanceds`]: fee and
		/// then tip.
		impl<T, C, OU, T1, T2, SF, TE> OnChargeTransaction<T>
			for TwoCurrencyOnChargeAdapter<C, OU, T1, T2, SF, TE>
		where
			T: pallet_transaction_payment::Config,
			TE: TriggerEvent<T::AccountId>,
			C: MultiTokenCurrency<T::AccountId, Balance = Balance, CurrencyId = TokenId>,
			OU: OnMultiTokenUnbalanced<TokenId, NegativeImbalanceOf<C, T>>,
			T1: Get<TokenId>,
			T2: Get<TokenId>,
			SF: Get<Balance>,
		{
			type LiquidityInfo = Option<LiquidityInfoEnum<C, T>>;
			type Balance = Balance;

			/// Withdraw the predicted fee from the transaction origin.
			///
			/// Note: The `fee` already includes the `tip`.
			fn withdraw_fee(
				who: &T::AccountId,
				_call: &T::RuntimeCall,
				_info: &DispatchInfoOf<T::RuntimeCall>,
				fee: Self::Balance,
				tip: Self::Balance,
			) -> Result<Self::LiquidityInfo, TransactionValidityError> {
				if fee.is_zero() {
					return Ok(None)
				}

				let withdraw_reason = if tip.is_zero() {
					WithdrawReasons::TRANSACTION_PAYMENT
				} else {
					WithdrawReasons::TRANSACTION_PAYMENT | WithdrawReasons::TIP
				};

				let fee_2 = fee / SF::get();

				match C::withdraw(
					T1::get(),
					who,
					fee,
					withdraw_reason,
					ExistenceRequirement::KeepAlive,
				) {
					Ok(imbalance) => Ok(Some(LiquidityInfoEnum::Imbalance((T1::get(), imbalance)))),
					Err(_) if fee_2.is_zero() => Err(InvalidTransaction::Payment.into()),
					Err(_) => match C::withdraw(
						T2::get(),
						who,
						fee_2,
						withdraw_reason,
						ExistenceRequirement::KeepAlive,
					) {
						Ok(imbalance) =>
							Ok(Some(LiquidityInfoEnum::Imbalance((T2::get(), imbalance)))),
						Err(_) => Err(InvalidTransaction::Payment.into()),
					},
				}
			}

			/// Hand the fee and the tip over to the `[OnUnbalanced]` implementation.
			/// Since the predicted fee might have been too high, parts of the fee may
			/// be refunded.
			///
			/// Note: The `corrected_fee` already includes the `tip`.
			fn correct_and_deposit_fee(
				who: &T::AccountId,
				_dispatch_info: &DispatchInfoOf<T::RuntimeCall>,
				_post_info: &PostDispatchInfoOf<T::RuntimeCall>,
				corrected_fee: Self::Balance,
				tip: Self::Balance,
				already_withdrawn: Self::LiquidityInfo,
			) -> Result<(), TransactionValidityError> {
				if let Some(LiquidityInfoEnum::Imbalance((token_id, paid))) = already_withdrawn {
					let (corrected_fee, tip) = if token_id == T2::get() {
						(corrected_fee / SF::get(), tip / SF::get())
					} else {
						(corrected_fee, tip)
					};
					// Calculate how much refund we should return
					let refund_amount = paid.peek().saturating_sub(corrected_fee);
					// refund to the the account that paid the fees. If this fails, the
					// account might have dropped below the existential balance. In
					// that case we don't refund anything.
					let refund_imbalance = C::deposit_into_existing(token_id, &who, refund_amount)
						.unwrap_or_else(|_| C::PositiveImbalance::from_zero(token_id));
					// merge the imbalance caused by paying the fees and refunding parts of it again.
					let adjusted_paid = match paid.offset(refund_imbalance) {
						SameOrOther::Same(a) => Ok(a),
						SameOrOther::None => Ok(C::NegativeImbalance::from_zero(token_id)),
						SameOrOther::Other(b) => Err(b),
					}
					.map_err(|_| TransactionValidityError::Invalid(InvalidTransaction::Payment))?;
					// Call someone else to handle the imbalance (fee and tip separately)
					let (tip_imb, fee) = adjusted_paid.split(tip);
					OU::on_unbalanceds(token_id, Some(fee).into_iter().chain(Some(tip_imb)));
					TE::trigger(who.clone(), token_id.into(), corrected_fee.into(), tip.into());
				}
				Ok(())
			}
		}
	}

	pub mod pallet_fee_lock {
		use crate::*;
		parameter_types! {
			pub const MaxCuratedTokens: u32 = 100;
		}
	}

	pub mod pallet_aura {
		use crate::*;
		parameter_types! {
			pub const MaxAuthorities: u32 = 100_000;
		}
	}

	pub mod pallet_sudo_origin {
		use crate::*;
		pub type SudoOrigin<CouncilCollective> =
			pallet_collective_mangata::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>;
	}

	pub mod pallet_collective_mangata {
		use crate::*;
		#[cfg(not(feature = "fast-runtime"))]
		parameter_types! {
			pub const CouncilProposalCloseDelay: BlockNumber = 3 * consts::DAYS;
		}

		#[cfg(feature = "fast-runtime")]
		parameter_types! {
			pub const CouncilProposalCloseDelay: BlockNumber = 6 * consts::MINUTES;
		}

		#[cfg(feature = "runtime-benchamarks")]
		parameter_types! {
			pub const CouncilProposalCloseDelay: BlockNumber = 0.into();
		}

		parameter_types! {
			pub const CouncilMotionDuration: BlockNumber = 5 * consts::DAYS;
			pub const CouncilMaxProposals: u32 = 100;
			pub const CouncilMaxMembers: u32 = 100;
			pub MaxProposalWeight: Weight = Perbill::from_percent(50) * config::frame_system::RuntimeBlockWeights::get().max_block;
		}

		pub type SetMembersOrigin<AccountId> = EnsureRoot<AccountId>;
	}

	pub mod pallet_membership {
		use crate::*;

		parameter_types! {
			pub const MaxMembersFoundation: u32 = 3;
			pub const MaxMembersTransfer: u32 = 50;
		}

		// todo change the `Get` to `Contains` trait and use membership pallet directly
		pub struct FoundationAccountsProvider;
		impl Get<Vec<AccountId>> for FoundationAccountsProvider {
			fn get() -> Vec<AccountId> {
				FoundationMembers::members().to_vec()
			}
		}
	}

	pub mod parachain_staking {
		use crate::*;

		pub type StakingIssuanceVaultOf<Runtime> =
			<Runtime as pallet_issuance::Config>::StakingIssuanceVault;
		#[cfg(feature = "fast-runtime")]
		parameter_types! {
			/// Default SessionLenght is every 2 minutes (10 * 12 second block times)
			pub const BlocksPerRound: u32 = 2 * consts::MINUTES;
		}

		#[cfg(not(feature = "fast-runtime"))]
		parameter_types! {
			/// Default SessionLenght is every 4 hours (1200 * 12 second block times)
			pub const BlocksPerRound: u32 = 4 * consts::HOURS;
		}

		parameter_types! {
			pub const DefaultPayoutLimit: u32 = 3;
			/// Collator candidate exit delay (number of rounds)
			pub const LeaveCandidatesDelay: u32 = 2;
			/// Collator candidate bond increases/decreases delay (number of rounds)
			pub const CandidateBondDelay: u32 = 2;
			/// Delegator exit delay (number of rounds)
			pub const LeaveDelegatorsDelay: u32 = 2;
			/// Delegation revocations delay (number of rounds)
			pub const RevokeDelegationDelay: u32 = 2;
			/// Delegation bond increases/decreases delay (number of rounds)
			pub const DelegationBondDelay: u32 = 2;
			/// Reward payments delay (number of rounds)
			pub const RewardPaymentDelay: u32 = 2;
			/// Minimum collators selected per round, default at genesis and minimum forever after
			pub const MinSelectedCandidates: u32 = 50;
			/// Maximum collator candidates allowed
			pub const MaxCollatorCandidates: u32 = 100;
			/// Maximum delegators allowed per candidate
			pub const MaxTotalDelegatorsPerCandidate: u32 = 750;
			/// Maximum delegators counted per candidate
			pub const MaxDelegatorsPerCandidate: u32 = 750;
			/// Maximum delegations per delegator
			pub const MaxDelegationsPerDelegator: u32 = 750;
			/// Default fixed percent a collator takes off the top of due rewards
			pub const DefaultCollatorCommission: Perbill = Perbill::from_percent(20);
			/// Minimum stake required to become a collator
			pub const MinCollatorStk: u128 = 10 * currency::DOLLARS;
			/// Minimum stake required to be reserved to be a candidate
			pub const MinCandidateStk: u128 = if cfg!(feature = "runtime-benchmarks") {
				// For benchmarking
				1 * currency::DOLLARS
			} else {
				// ACTUAL
				500_000 * currency::DOLLARS
			};
			/// Minimum stake required to be reserved to be a delegator
			pub const MinDelegatorStk: u128 = 500 * currency::DOLLARS;
		}
	}

	pub mod pallet_issuance {
		use crate::*;
		parameter_types! {
			pub const HistoryLimit: u32 = 10u32;

			pub const LiquidityMiningIssuanceVaultId: PalletId = PalletId(*b"py/lqmiv");
			pub LiquidityMiningIssuanceVault: AccountId = LiquidityMiningIssuanceVaultId::get().into_account_truncating();
			pub const StakingIssuanceVaultId: PalletId = PalletId(*b"py/stkiv");
			pub StakingIssuanceVault: AccountId = StakingIssuanceVaultId::get().into_account_truncating();
			pub const SequencerIssuanceVaultId: PalletId = PalletId(*b"py/seqiv");
			pub SequencerIssuanceVault: AccountId = SequencerIssuanceVaultId::get().into_account_truncating();

			pub const TotalCrowdloanAllocation: Balance = 0;
			pub const LinearIssuanceAmount: Balance = 10_200_000 * DOLLARS; // LinearIssuanceAmount/(LinearIssuanceBlocks/BlocksPerRound) is the value that is issued every session indefintely FOREVER!
			pub const LinearIssuanceBlocks: u32 = 10_512_000u32; // 2 years
			pub const LiquidityMiningSplit: Perbill = Perbill::from_parts(686000000); // 6_997_200
			pub const StakingSplit: Perbill = Perbill::from_parts(314000000); // 3_202_800
			pub const SequencerSplit: Perbill = Perbill::from_parts(0); // 0
			pub const ImmediateTGEReleasePercent: Percent = Percent::from_percent(100);
			// Just some safe values to avoid zero diision etc
			// TGE happens on L1 either way
			pub const TGEReleasePeriod: u32 = 100u32; // 2 years
			pub const TGEReleaseBegin: u32 = 10u32; // Two weeks into chain start
		}
	}

	pub mod orml_asset_registry {
		use crate::*;

		const LIQUIDITY_TOKEN_IDENTIFIER: &[u8] = b"LiquidityPoolToken";
		const HEX_INDICATOR: &[u8] = b"0x";
		const TOKEN_SYMBOL: &[u8] = b"TKN";
		const TOKEN_SYMBOL_SEPARATOR: &[u8] = b"-";
		const DEFAULT_DECIMALS: u32 = 18u32;

		parameter_types! {
			pub const StringLimit: u32 = 50;
		}

		pub type AssetMetadataOf = AssetMetadata<Balance, CustomMetadata, StringLimit>;
		type CurrencyAdapter<Runtime> = orml_tokens::MultiTokenCurrencyAdapter<Runtime>;

		pub struct SequentialIdWithCreation<T>(PhantomData<T>);
		impl<T> AssetProcessor<TokenId, AssetMetadataOf> for SequentialIdWithCreation<T>
		where
			T: orml_asset_registry::Config,
			T: orml_tokens::Config,
			T: pallet_treasury::Config,
			TokenId: From<<T as orml_tokens::Config>::CurrencyId>,
		{
			fn pre_register(
				id: Option<TokenId>,
				asset_metadata: AssetMetadataOf,
			) -> Result<(TokenId, AssetMetadataOf), DispatchError> {
				let next_id = CurrencyAdapter::<T>::get_next_currency_id();
				let asset_id = id.unwrap_or(next_id.into());
				let treasury_account =
					config::TreasuryPalletIdOf::<T>::get().into_account_truncating();

				match asset_id.cmp(&next_id.into()) {
					Ordering::Equal =>
						CurrencyAdapter::<T>::create(&treasury_account, Default::default())
							.and_then(|created_asset_id| {
								match created_asset_id.cmp(&asset_id.into()) {
									Ordering::Equal => Ok((asset_id, asset_metadata)),
									_ =>
										Err(orml_asset_registry::Error::<T>::InvalidAssetId.into()),
								}
							}),
					Ordering::Less => Ok((asset_id, asset_metadata)),
					_ => Err(orml_asset_registry::Error::<T>::InvalidAssetId.into()),
				}
			}
		}

		pub struct AssetAuthority<T>(PhantomData<T>);
		impl<T> EnsureOriginWithArg<T::RuntimeOrigin, Option<u32>> for AssetAuthority<T>
		where
			T: frame_system::Config,
		{
			type Success = ();

			fn try_origin(
				origin: T::RuntimeOrigin,
				_asset_id: &Option<u32>,
			) -> Result<Self::Success, T::RuntimeOrigin> {
				<EnsureRoot<_> as EnsureOrigin<T::RuntimeOrigin>>::try_origin(origin)
			}

			#[cfg(feature = "runtime-benchmarks")]
			fn try_successful_origin(_: &Option<u32>) -> Result<T::RuntimeOrigin, ()> {
				Ok(T::RuntimeOrigin::root())
			}
		}

		pub struct AssetRegistryProvider<T>(PhantomData<T>);
		impl<
				T: orml_asset_registry::Config<AssetId = TokenId, CustomMetadata = CustomMetadata>,
			> AssetRegistryProviderTrait<T::AssetId> for AssetRegistryProvider<T>
		{
			fn get_l1_asset_id(l1_asset: L1Asset) -> Option<T::AssetId> {
				orml_asset_registry::L1AssetToId::<T>::get(l1_asset)
			}

			fn create_l1_asset(l1_asset: L1Asset) -> Result<T::AssetId, DispatchError> {
				let metadata = AssetMetadata {
					decimals: 18_u32,
					name: b"L1Asset".to_vec().try_into().unwrap(),
					symbol: b"L1Asset".to_vec().try_into().unwrap(),
					existential_deposit: Zero::zero(),
					additional: CustomMetadata { xcm: None, xyk: None },
				};

				orml_asset_registry::Pallet::<T>::do_register_l1_asset(metadata, None, l1_asset)
			}

			fn get_asset_l1_id(asset_id: T::AssetId) -> Option<L1Asset> {
				orml_asset_registry::IdToL1Asset::<T>::get(asset_id)
			}

			fn create_pool_asset(
				lp_asset: T::AssetId,
				asset_1: T::AssetId,
				asset_2: T::AssetId,
			) -> DispatchResult {
				let name_lp = format_asset_id(lp_asset);
				let name_asset_1 = format_asset_id(asset_1);
				let name_asset_2 = format_asset_id(asset_2);

				let mut name: Vec<u8> = Vec::<u8>::new();
				name.extend_from_slice(LIQUIDITY_TOKEN_IDENTIFIER);
				name.extend_from_slice(HEX_INDICATOR);
				name.extend_from_slice(&name_lp);

				let mut symbol: Vec<u8> = Vec::<u8>::new();
				symbol.extend_from_slice(TOKEN_SYMBOL);
				symbol.extend_from_slice(HEX_INDICATOR);
				symbol.extend_from_slice(&name_asset_1);
				symbol.extend_from_slice(TOKEN_SYMBOL_SEPARATOR);
				symbol.extend_from_slice(TOKEN_SYMBOL);
				symbol.extend_from_slice(HEX_INDICATOR);
				symbol.extend_from_slice(&name_asset_2);

				let metadata = AssetMetadata {
					decimals: DEFAULT_DECIMALS,
					name: BoundedVec::truncate_from(name),
					symbol: BoundedVec::truncate_from(symbol),
					existential_deposit: Zero::zero(),
					additional: Default::default(),
				};

				orml_asset_registry::Pallet::<T>::do_register_asset_without_asset_processor(
					metadata, lp_asset,
				)
			}
		}

		impl<T: orml_asset_registry::Config<AssetId = TokenId>> orml_traits::asset_registry::Inspect
			for AssetRegistryProvider<T>
		{
			type AssetId = T::AssetId;
			type Balance = T::Balance;
			type CustomMetadata = T::CustomMetadata;
			type StringLimit = T::StringLimit;

			fn metadata(
				asset_id: &Self::AssetId,
			) -> Option<AssetMetadata<Self::Balance, Self::CustomMetadata, Self::StringLimit>> {
				orml_asset_registry::Pallet::<T>::metadata(asset_id)
			}
		}

		// 48 in utf-8 '0'
		// 55 in utf-8 '0' + gap between '9' and 'A'
		pub fn format_asset_id(num: TokenId) -> Vec<u8> {
			let mut result = Vec::new();
			for bytes in num.to_be_bytes().iter() {
				match (bytes >> 4) as u8 {
					x @ 0u8..=9u8 => result.push(x + 48),
					x => result.push(x + 55),
				}
				match (bytes & 0b0000_1111) as u8 {
					x @ 0u8..=9u8 => result.push(x + 48),
					x => result.push(x + 55),
				}
			}
			result
		}
	}

	pub mod pallet_identity {
		use crate::*;
		parameter_types! {
			// difference of 26 bytes on-chain for the registration and 9 bytes on-chain for the identity
			// information, already accounted for by the byte deposit
			pub const BasicDeposit: Balance = deposit(1, 17);
			pub const ByteDeposit: Balance = deposit(0, 1);
			// Add item in storage, and takes 97 bytes, AccountId + (AccountId, [u8,32])
			pub const SubAccountDeposit: Balance = deposit(1, 97);
			pub const MaxSubAccounts: u32 = 100;
			pub const MaxAdditionalFields: u32 = 100;
			pub const MaxRegistrars: u32 = 20;
			pub const PendingUsernameExpiration: u32 = 7 * consts::DAYS;
			pub const MaxSuffixLength: u32 = 7;
			pub const MaxUsernameLength: u32 = 32;
		}

		pub type IdentityForceOrigin = EnsureRoot<AccountId>;
		pub type IdentityRegistrarOrigin = EnsureRoot<AccountId>;
	}

	pub mod pallet_utility_mangata {
		use super::*;

		#[derive(
			Copy,
			Clone,
			Eq,
			PartialEq,
			Ord,
			PartialOrd,
			Encode,
			Decode,
			RuntimeDebug,
			MaxEncodedLen,
			TypeInfo,
		)]
		pub struct DisallowedInBatch<Runtime>(PhantomData<Runtime>);

		impl<T> Contains<T::RuntimeCall> for DisallowedInBatch<T>
		where
			T: ::frame_system::Config,
			<T as ::frame_system::Config>::RuntimeCall: Into<crate::CallType>,
		{
			fn contains(c: &T::RuntimeCall) -> bool {
				let call: crate::CallType = (c.clone()).into();

				match call {
					CallType::Swap { .. } => true,
					_ => false,
				}
			}
		}
	}

	pub mod pallet_vesting_mangata {
		use super::*;
		parameter_types! {
			pub const MinVestedTransfer: Balance = 100 * currency::DOLLARS;
			pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
				WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
		}
	}

	pub mod pallet_crowdloan_rewards {
		use super::*;
		parameter_types! {
			pub const Initialized: bool = false;
			pub const InitializationPayment: Perbill = Perbill::from_parts(214285700);
			pub const MaxInitContributorsBatchSizes: u32 = 100;
			pub const MinimumReward: Balance = 0;
			pub const RelaySignaturesThreshold: Perbill = Perbill::from_percent(100);
			pub const SigantureNetworkIdentifier: &'static [u8] = b"mangata-";
		}
	}

	pub mod pallet_proxy {
		use super::*;
		// Proxy Pallet
		/// The type used to represent the kinds of proxying allowed.
		#[derive(
			Copy,
			Clone,
			Eq,
			PartialEq,
			Ord,
			PartialOrd,
			Encode,
			Decode,
			RuntimeDebug,
			MaxEncodedLen,
			TypeInfo,
		)]
		pub enum ProxyType {
			AutoCompound,
		}

		impl Default for ProxyType {
			fn default() -> Self {
				Self::AutoCompound
			}
		}

		parameter_types! {
			pub const ProxyDepositBase: Balance = deposit(1, 16);
			pub const ProxyDepositFactor: Balance = deposit(0, 33);
			pub const AnnouncementDepositBase: Balance = deposit(1, 16);
			pub const AnnouncementDepositFactor: Balance = deposit(0, 68);
		}
	}

	pub mod pallet_proof_of_stake {
		use super::*;

		parameter_types! {
			pub const RewardsSchedulesLimit: u32 = 10_000u32;
			// NOTE: 1725 is how much USDT you get for one MGX as of 12.2023
			pub const Min3rdPartyRewardValutationPerSession: u128 = 10 * 1725 * currency::DOLLARS;
			pub const Min3rdPartyRewardVolume: u128 = 10_000 * 1725 * currency::DOLLARS;
			pub const SchedulesPerBlock: u32 = 5;
		}
	}

	pub mod pallet_sequencer_staking {
		use super::*;

		parameter_types! {
			pub const CancellerRewardPercentage: Permill = Permill::from_percent(20);
		}
	}

	pub mod pallet_rolldown {
		use super::*;

		parameter_types! {
			pub const CancellerRewardPercentage: Permill = Permill::from_percent(20);
			pub const RequestsPerBlock: u128 = 50;
			pub const RightsMultiplier: u128 = 1;
		}

		#[cfg(feature = "fast-runtime")]
		parameter_types! {
			pub const MerkleRootAutomaticBatchPeriod: u128 = 25;
			pub const MerkleRootAutomaticBatchSize: u128 = 32;
		}

		#[cfg(not(feature = "fast-runtime"))]
		parameter_types! {
			pub const MerkleRootAutomaticBatchPeriod: u128 = 600;
			pub const MerkleRootAutomaticBatchSize: u128 = 500;
		}

		pub struct WithdrawFee;
		impl Convert<::pallet_rolldown::messages::Chain, Balance> for WithdrawFee {
			fn convert(chain: ::pallet_rolldown::messages::Chain) -> Balance {
				match chain {
					::pallet_rolldown::messages::Chain::Ethereum => 5 * currency::DOLLARS,
					::pallet_rolldown::messages::Chain::Arbitrum => 5 * currency::DOLLARS,
					::pallet_rolldown::messages::Chain::Base => 5 * currency::DOLLARS,
					::pallet_rolldown::messages::Chain::Monad => 5 * currency::DOLLARS,
					::pallet_rolldown::messages::Chain::MegaEth => 5 * currency::DOLLARS,
					::pallet_rolldown::messages::Chain::Sonic => 5 * currency::DOLLARS,
				}
			}
		}
	}

	pub mod pallet_market {
		use super::*;

		// in case of other valuation use a struct
		// that impl the ValuateFor trait and delegates to Market for Valuate impl
		// use such struct in pallet configs instead of Market
		impl ValuateFor<tokens::RxTokenId> for Market {}

		parameter_types! {

			pub const MaxSwapListLength: u32 = if cfg!(feature = "runtime-benchmarks") {
				// For benchmarking
				1000
			} else {
				// ACTUAL
				10
			};
		}
	}
}
