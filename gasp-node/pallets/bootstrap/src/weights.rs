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

//! Autogenerated weights for pallet_bootstrap
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /home/ubuntu/mangata-node/scripts/..//target/release/mangata-node
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet-bootstrap
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./benchmarks/pallet-bootstrap_weights.rs
// --template
// ./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bootstrap.
pub trait WeightInfo {
	fn schedule_bootstrap() -> Weight;
	fn provision() -> Weight;
	// fn provision_vested() -> Weight;
	fn claim_and_activate_liquidity_tokens() -> Weight;
	fn finalize() -> Weight;
}


// For backwards compatibility and tests
impl WeightInfo for () {
	fn schedule_bootstrap() -> Weight {
		Weight::from_parts(26_587_000, 0)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn provision() -> Weight {
		Weight::from_parts(89_870_000, 0)
			.saturating_add(RocksDbWeight::get().reads(9 as u64))
			.saturating_add(RocksDbWeight::get().writes(6 as u64))
	}
	// fn provision_vested() -> Weight {
	// 	Weight::from_parts(183_414_000, 0)
	// 		.saturating_add(RocksDbWeight::get().reads(11 as u64))
	// 		.saturating_add(RocksDbWeight::get().writes(8 as u64))
	// }
	fn claim_and_activate_liquidity_tokens() -> Weight {
		Weight::from_parts(432_898_000, 0)
			.saturating_add(RocksDbWeight::get().reads(21 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
	}
	fn finalize() -> Weight {
		Weight::from_parts(112_243_000, 0)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(12 as u64))
	}
}
