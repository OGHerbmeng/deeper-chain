// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Macro for creating the tests for the module.

use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

use frame_support::traits::{ConstU32, OnFinalize, OnInitialize};
use frame_support::{assert_ok, parameter_types, weights::Weight};

use super::*;
use crate::{self as pallet_operation};
use node_primitives::{BlockNumber, Moment};
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Operation: pallet_operation::{Pallet, Call, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u128; // u64 is not enough to hold bytes used to generate bounty account
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}
parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
}

parameter_types! {
    pub const MinimumPeriod: Moment = 5u64;
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

pub const MILLISECS_PER_BLOCK: Moment = 5000;
pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
pub const BLOCKS_PER_DAY: Moment = 24 * 3600 / 5;

parameter_types! {
    pub const MaxMember: u32 = 100;
    pub const BlocksPerEra: BlockNumber =  6 * EPOCH_DURATION_IN_BLOCKS;
}

impl pallet_operation::Config for Test {
    type Event = Event;
    type OPWeightInfo = ();
    type BlocksPerEra = BlocksPerEra;
    type MaxMember = MaxMember;
    type Currency = Balances;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_balances::GenesisConfig::<Test> {
        // Total issuance will be 200 with treasury account initialized at ED.
        balances: vec![(0, 100), (1, 98), (2, 1)],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        Timestamp::set_timestamp(System::block_number() * 5000);
        System::on_finalize(System::block_number());
        Operation::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
    }
}

#[test]
fn set_lock_members_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(Balances::set_balance(Origin::root(), 1, 1_000, 0));
        assert_ok!(Operation::set_reserve_members(Origin::root(), vec!(2)));
        assert_ok!(Operation::force_reserve_by_member(Some(2).into(), 1, 500));
        assert_eq!(Balances::free_balance(&1), 500);
        assert_ok!(Balances::force_unreserve(Origin::root(), 1, 500));
        assert_eq!(Balances::free_balance(&1), 1000);
    });
}

#[test]
fn set_release_owner_address() {
    new_test_ext().execute_with(|| {
        assert_ok!(Operation::set_release_owner_address(Origin::root(), 1));
    });
}

#[test]
fn set_release_limit_parameter() {
    new_test_ext().execute_with(|| {
        assert_ok!(Operation::set_release_limit_parameter(
            Origin::root(),
            10,
            1000
        ));
        assert_eq!(Operation::single_max_limit(), 10);
        assert_eq!(Operation::daily_max_limit(), 1000);
    });
}

#[test]
fn set_staking_release_info() {
    new_test_ext().execute_with(|| {
        assert_ok!(Operation::set_release_owner_address(Origin::root(), 1));
        assert_ok!(Operation::set_release_limit_parameter(
            Origin::root(),
            2000,
            2000
        ));
        let info1 = ReleaseInfo::<Test>::new(3, 2, 0, 2000);
        let info2 = ReleaseInfo::<Test>::new(4, 2, 0, 1000);
        assert_ok!(Balances::set_balance(Origin::root(), 2, 10, 0));
        assert_eq!(Balances::free_balance(&3), 0);
        assert_ok!(Operation::set_staking_release_info(
            Origin::signed(1),
            vec!(info1, info2)
        ));
        run_to_block(BLOCKS_PER_DAY + 3);
        assert_eq!(Balances::free_balance(&3), 1000);
        assert_eq!(Balances::free_balance(&4), 500);

        run_to_block(BLOCKS_PER_DAY * 2 + 3);
        assert_eq!(Balances::free_balance(&3), 2000);
        assert_eq!(Balances::free_balance(&4), 1000);

        run_to_block(BLOCKS_PER_DAY * 3 + 3);
        assert_eq!(Operation::accounts_release_info(&3).is_none(), true);
        assert_eq!(Operation::accounts_release_info(&4).is_none(), true);
    });
}
