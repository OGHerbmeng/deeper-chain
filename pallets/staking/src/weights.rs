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

//! Autogenerated weights for pallet_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-16, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/deeper-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/staking/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_staking.
pub trait WeightInfo {
    fn bond() -> Weight;
    fn bond_extra() -> Weight;
    fn unbond() -> Weight;
    fn withdraw_unbonded_update(s: u32) -> Weight;
    fn withdraw_unbonded_kill(s: u32) -> Weight;
    fn validate() -> Weight;
    fn delegate(n: u32) -> Weight;
    fn undelegate() -> Weight;
    fn chill() -> Weight;
    fn set_payee() -> Weight;
    fn set_controller() -> Weight;
    fn set_era_validator_reward() -> Weight;
    fn set_validator_count() -> Weight;
    fn increase_validator_count(n: u32) -> Weight;
    fn scale_validator_count(n: u32) -> Weight;
    fn force_no_eras() -> Weight;
    fn force_new_era() -> Weight;
    fn force_new_era_always() -> Weight;
    fn set_invulnerables(v: u32) -> Weight;
    fn set_validator_whitelist(v: u32) -> Weight;
    fn force_unstake(s: u32) -> Weight;
    fn increase_mining_reward(r: u32) -> Weight;
    fn cancel_deferred_slash(s: u32) -> Weight;
    fn rebond(l: u32) -> Weight;
    fn set_history_depth(e: u32) -> Weight;
    fn reap_stash(s: u32) -> Weight;
    fn new_era(v: u32, d: u32) -> Weight;
}

/// Weights for pallet_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn bond() -> Weight {
        (81_039_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn bond_extra() -> Weight {
        (64_865_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    fn unbond() -> Weight {
        (59_356_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn withdraw_unbonded_update(s: u32) -> Weight {
        (61_321_000 as Weight)
            // Standard Error: 0
            .saturating_add((66_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn withdraw_unbonded_kill(s: u32) -> Weight {
        (91_218_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((2_613_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    fn validate() -> Weight {
        (16_906_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn delegate(_n: u32) -> Weight {
        (71_178_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(9 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn undelegate() -> Weight {
        (60_504_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn chill() -> Weight {
        (16_726_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_payee() -> Weight {
        (13_804_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_controller() -> Weight {
        (29_176_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn set_era_validator_reward() -> Weight {
        (2_906_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_validator_count() -> Weight {
        (2_807_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn increase_validator_count(_n: u32) -> Weight {
        (6_970_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn scale_validator_count(n: u32) -> Weight {
        (6_753_000 as Weight)
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn force_no_eras() -> Weight {
        (3_069_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn force_new_era() -> Weight {
        (3_120_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn force_new_era_always() -> Weight {
        (3_038_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_invulnerables(v: u32) -> Weight {
        (3_474_000 as Weight)
            // Standard Error: 0
            .saturating_add((66_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_validator_whitelist(v: u32) -> Weight {
        (3_428_000 as Weight)
            // Standard Error: 0
            .saturating_add((66_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn force_unstake(s: u32) -> Weight {
        (60_402_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((2_605_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    fn increase_mining_reward(_r: u32) -> Weight {
        (6_513_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn cancel_deferred_slash(s: u32) -> Weight {
        (3_436_835_000 as Weight)
            // Standard Error: 225_000
            .saturating_add((20_101_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn rebond(l: u32) -> Weight {
        (41_141_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((80_000 as Weight).saturating_mul(l as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn set_history_depth(e: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 45_000
            .saturating_add((26_785_000 as Weight).saturating_mul(e as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
            .saturating_add(T::DbWeight::get().writes((6 as Weight).saturating_mul(e as Weight)))
    }
    fn reap_stash(s: u32) -> Weight {
        (63_634_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((2_592_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    fn new_era(v: u32, d: u32) -> Weight {
        (16_569_000 as Weight)
            // Standard Error: 605_000
            .saturating_add((50_525_000 as Weight).saturating_mul(v as Weight))
            // Standard Error: 3_000
            .saturating_add((487_000 as Weight).saturating_mul(d as Weight))
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
            .saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(v as Weight)))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn bond() -> Weight {
        (81_039_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(5 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn bond_extra() -> Weight {
        (64_865_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
    }
    fn unbond() -> Weight {
        (59_356_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(5 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn withdraw_unbonded_update(s: u32) -> Weight {
        (61_321_000 as Weight)
            // Standard Error: 0
            .saturating_add((66_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(5 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn withdraw_unbonded_kill(s: u32) -> Weight {
        (91_218_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((2_613_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(7 as Weight))
            .saturating_add(RocksDbWeight::get().writes(7 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    fn validate() -> Weight {
        (16_906_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn delegate(_n: u32) -> Weight {
        (71_178_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(9 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn undelegate() -> Weight {
        (60_504_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(5 as Weight))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
    }
    fn chill() -> Weight {
        (16_726_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_payee() -> Weight {
        (13_804_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_controller() -> Weight {
        (29_176_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(3 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn set_era_validator_reward() -> Weight {
        (2_906_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_validator_count() -> Weight {
        (2_807_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn increase_validator_count(_n: u32) -> Weight {
        (6_970_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn scale_validator_count(n: u32) -> Weight {
        (6_753_000 as Weight)
            // Standard Error: 0
            .saturating_add((3_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn force_no_eras() -> Weight {
        (3_069_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn force_new_era() -> Weight {
        (3_120_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn force_new_era_always() -> Weight {
        (3_038_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_invulnerables(v: u32) -> Weight {
        (3_474_000 as Weight)
            // Standard Error: 0
            .saturating_add((66_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_validator_whitelist(v: u32) -> Weight {
        (3_428_000 as Weight)
            // Standard Error: 0
            .saturating_add((66_000 as Weight).saturating_mul(v as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn force_unstake(s: u32) -> Weight {
        (60_402_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((2_605_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(7 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    fn increase_mining_reward(_r: u32) -> Weight {
        (6_513_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn cancel_deferred_slash(s: u32) -> Weight {
        (3_436_835_000 as Weight)
            // Standard Error: 225_000
            .saturating_add((20_101_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn rebond(l: u32) -> Weight {
        (41_141_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((80_000 as Weight).saturating_mul(l as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn set_history_depth(e: u32) -> Weight {
        (0 as Weight)
            // Standard Error: 45_000
            .saturating_add((26_785_000 as Weight).saturating_mul(e as Weight))
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes((6 as Weight).saturating_mul(e as Weight)))
    }
    fn reap_stash(s: u32) -> Weight {
        (63_634_000 as Weight)
            // Standard Error: 1_000
            .saturating_add((2_592_000 as Weight).saturating_mul(s as Weight))
            .saturating_add(RocksDbWeight::get().reads(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes(7 as Weight))
            .saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
    }
    fn new_era(v: u32, d: u32) -> Weight {
        (16_569_000 as Weight)
            // Standard Error: 605_000
            .saturating_add((50_525_000 as Weight).saturating_mul(v as Weight))
            // Standard Error: 3_000
            .saturating_add((487_000 as Weight).saturating_mul(d as Weight))
            .saturating_add(RocksDbWeight::get().reads(6 as Weight))
            .saturating_add(RocksDbWeight::get().reads((4 as Weight).saturating_mul(v as Weight)))
            .saturating_add(RocksDbWeight::get().writes(4 as Weight))
            .saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(v as Weight)))
    }
}
