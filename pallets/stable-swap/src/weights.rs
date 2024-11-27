use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for pallet_stable_pools.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity_one_asset() -> Weight;
	fn remove_liquidity_imbalanced() -> Weight;
	fn remove_liquidity() -> Weight;
}

impl WeightInfo for () {
	fn create_pool() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn add_liquidity() -> Weight {
		Weight::from_parts(0, 0)
	}

	fn remove_liquidity_one_asset() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn remove_liquidity_imbalanced() -> Weight {
		Weight::from_parts(0, 0)
	}
	fn remove_liquidity() -> Weight {
		Weight::from_parts(0, 0)
	}
}
