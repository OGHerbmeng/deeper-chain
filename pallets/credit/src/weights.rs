// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_credit
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-15, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/deeper-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_credit
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/credit/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_credit.
pub trait WeightInfo {	fn update_credit_setting() -> Weight;	fn add_or_update_credit_data() -> Weight;	fn burn_for_add_credit() -> Weight;}

/// Weights for pallet_credit using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {	fn update_credit_setting() -> Weight {
		(12_472_000 as Weight)			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}	fn add_or_update_credit_data() -> Weight {
		(17_707_000 as Weight)			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn burn_for_add_credit() -> Weight {
		(45_811_000 as Weight)			.saturating_add(T::DbWeight::get().reads(3 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}}

// For backwards compatibility and tests
impl WeightInfo for () {	fn update_credit_setting() -> Weight {
		(12_472_000 as Weight)			.saturating_add(RocksDbWeight::get().writes(2 as Weight))	}	fn add_or_update_credit_data() -> Weight {
		(17_707_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(2 as Weight))			.saturating_add(RocksDbWeight::get().writes(1 as Weight))	}	fn burn_for_add_credit() -> Weight {
		(45_811_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(3 as Weight))			.saturating_add(RocksDbWeight::get().writes(3 as Weight))	}}
