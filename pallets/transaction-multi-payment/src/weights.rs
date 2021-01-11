// This file is part of hack.HydraDX-node.

// Copyright (C) 2021 Intergalactic Ltd.
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

//! Autogenerated weights for transaction_multi_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2021-01-08, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/hack-hydra-dx
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet
// transaction_multi_payment
// --extrinsic
// *
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/transaction-multi-payment/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for transaction_multi_payment.
pub trait WeightInfo {
	fn swap_currency() -> Weight;
	fn set_currency() -> Weight;
}

/// Weights for transaction_multi_payment using the hack.hydraDX node and recommended hardware.
pub struct HackHydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for HackHydraWeight<T> {
	fn swap_currency() -> Weight {
		(270_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn set_currency() -> Weight {
		(61_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn swap_currency() -> Weight {
		(270_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn set_currency() -> Weight {
		(61_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
