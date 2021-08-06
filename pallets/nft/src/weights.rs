// This file is part of Basilisk.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-17, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/basilisk
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_nft
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=nft_weights.rs
// --template=.maintain/pallet-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nft.
pub trait WeightInfo {
	fn create_class() -> Weight;
	fn mint(i: u32) -> Weight;
	fn transfer() -> Weight;
	fn destroy_class() -> Weight;
	fn burn() -> Weight;
	fn buy_from_pool() -> Weight;
	fn sell_to_pool() -> Weight;
	fn on_finalize() -> Weight;
}

/// Weights for pallet_nft using the Basilisk node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	fn create_class() -> Weight {
		(66_848_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	fn mint(i: u32) -> Weight {
		(0 as Weight)
			.saturating_add((63_220_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
	}
	fn transfer() -> Weight {
		(59_111_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn destroy_class() -> Weight {
		(40_885_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn burn() -> Weight {
		(56_851_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn buy_from_pool() -> Weight {
		(68_066_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn sell_to_pool() -> Weight {
		(68_287_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn on_finalize() -> Weight {
		(9_634_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_class() -> Weight {
        (66_848_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(5 as Weight))
    }
    fn mint(i: u32) -> Weight {
		(0 as Weight)
			.saturating_add((63_220_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(i as Weight)))
	}
    fn transfer() -> Weight {
        (59_111_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn destroy_class() -> Weight {
        (40_885_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn burn() -> Weight {
        (56_851_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn buy_from_pool() -> Weight {
        (68_066_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn sell_to_pool() -> Weight {
        (68_287_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn on_finalize() -> Weight {
        (9_634_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
    }
}
