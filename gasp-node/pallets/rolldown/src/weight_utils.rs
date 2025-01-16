use sp_runtime::traits::Saturating;

/// accounts for cost of reading huge L1Update from runtime storage (rocksdb)
pub const fn get_read_scalling_factor(size: usize) -> u128 {
	const BASE_READ_COST: u128 = 25;
	let approximated_cost = match size {
		0..=50 => 25u128,
		51..=100 => 45u128,
		101..=500 => 210u128,
		501..=1000 => 400u128,
		1001..=5000 => 1800u128,
		_ => 2800u128,
	};
	approximated_cost.saturating_div(BASE_READ_COST).saturating_add(1u128)
}

/// accounts for cost of writing huge L1Update from runtime storage (rocksdb)
pub const fn get_write_scalling_factor(size: usize) -> u128 {
	const BASE_WRITE_COST: u128 = 100;

	let approximated_cost = match size {
		0..=50 => 25u128,
		51..=100 => 150u128,
		101..=500 => 700u128,
		501..=1000 => 1050u128,
		1001..=5000 => 5000u128,
		_ => 9000u128,
	};
	approximated_cost.saturating_div(BASE_WRITE_COST).saturating_add(1u128)
}
