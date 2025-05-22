#![cfg_attr(not(feature = "std"), no_std)]
use sp_runtime::DispatchResult;
use sp_std::vec::Vec;

pub trait UpdateValutaion<CurrencyId> {
	fn update_eq_assets(
		asset_id: CurrencyId,
		eq_assets_update: Vec<(CurrencyId, bool)>,
	) -> DispatchResult;
}
