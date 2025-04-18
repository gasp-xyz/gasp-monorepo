// This file is part of Mangata.

// Copyright (C) 2020-2022 Mangata Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_issuance
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("rollup-local"), DB CACHE: 1024

// Executed Command:
// target/release/rollup-node
// benchmark
// pallet
// -l=info,runtime::collective=warn,xyk=warn
// --chain
// rollup-local
// --wasm-execution
// compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template
// ./templates/module-weight-template.hbs
// --output
// ./benchmarks/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_issuance.
pub trait WeightInfo {
	fn init_issuance_config() -> Weight;
	fn finalize_tge() -> Weight;
	fn execute_tge(x: u32, ) -> Weight;
	fn set_issuance_config() -> Weight;
}

/// Weights for pallet_issuance using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_issuance::WeightInfo for ModuleWeight<T> {
	// Storage: `Issuance::IssuanceConfigStore` (r:1 w:1)
	// Proof: `Issuance::IssuanceConfigStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Issuance::IsTGEFinalized` (r:1 w:0)
	// Proof: `Issuance::IsTGEFinalized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn init_issuance_config() -> Weight {
		(Weight::from_parts(17_750_000, 0))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: `Issuance::IsTGEFinalized` (r:1 w:1)
	// Proof: `Issuance::IsTGEFinalized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn finalize_tge() -> Weight {
		(Weight::from_parts(9_809_000, 0))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: `Issuance::IsTGEFinalized` (r:1 w:0)
	// Proof: `Issuance::IsTGEFinalized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:100 w:100)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:100 w:100)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Issuance::TGETotal` (r:1 w:1)
	// Proof: `Issuance::TGETotal` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn execute_tge(x: u32, ) -> Weight {
		(Weight::from_parts(26_290_460, 0))
			// Standard Error: 10_945
			.saturating_add((Weight::from_parts(22_350_615, 0)).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(x as u64)))
	}
	// Storage: `Issuance::IssuanceConfigStore` (r:1 w:1)
	// Proof: `Issuance::IssuanceConfigStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_issuance_config() -> Weight {
		(Weight::from_parts(11_770_000, 0))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: `Issuance::IssuanceConfigStore` (r:1 w:1)
	// Proof: `Issuance::IssuanceConfigStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Issuance::IsTGEFinalized` (r:1 w:0)
	// Proof: `Issuance::IsTGEFinalized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn init_issuance_config() -> Weight {
		(Weight::from_parts(17_750_000, 0))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: `Issuance::IsTGEFinalized` (r:1 w:1)
	// Proof: `Issuance::IsTGEFinalized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn finalize_tge() -> Weight {
		(Weight::from_parts(9_809_000, 0))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: `Issuance::IsTGEFinalized` (r:1 w:0)
	// Proof: `Issuance::IsTGEFinalized` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `Tokens::Accounts` (r:100 w:100)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:100 w:100)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	// Storage: `Issuance::TGETotal` (r:1 w:1)
	// Proof: `Issuance::TGETotal` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn execute_tge(x: u32, ) -> Weight {
		(Weight::from_parts(26_290_460, 0))
			// Standard Error: 10_945
			.saturating_add((Weight::from_parts(22_350_615, 0)).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(x as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(x as u64)))
	}
	// Storage: `Issuance::IssuanceConfigStore` (r:1 w:1)
	// Proof: `Issuance::IssuanceConfigStore` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_issuance_config() -> Weight {
		(Weight::from_parts(11_770_000, 0))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
}
