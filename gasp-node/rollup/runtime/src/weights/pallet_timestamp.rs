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

//! Autogenerated weights for pallet_timestamp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-29, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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

/// Weight functions needed for pallet_timestamp.
pub trait WeightInfo {
	fn set() -> Weight;
	fn on_finalize() -> Weight;
}

/// Weights for pallet_timestamp using the Mangata node and recommended hardware.
pub struct ModuleWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for ModuleWeight<T> {
	// Storage: `Timestamp::Now` (r:1 w:1)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn set() -> Weight {
		(Weight::from_parts(7_200_000, 0))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn on_finalize() -> Weight {
		(Weight::from_parts(4_560_000, 0))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: `Timestamp::Now` (r:1 w:1)
	// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn set() -> Weight {
		(Weight::from_parts(7_200_000, 0))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn on_finalize() -> Weight {
		(Weight::from_parts(4_560_000, 0))
	}
}
