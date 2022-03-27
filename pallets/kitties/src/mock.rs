use crate as pallet_kitties;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use frame_support::traits::{ConstU128, ConstU16, ConstU32, ConstU64};

pub type AccountId = u64;
pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const ALICE_2: AccountId = 3;
pub const BOB_2: AccountId = 4;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		// Aura: pallet_aura,
		// Grandpa: pallet_grandpa,
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>, Config<T>},
		// TransactionPayment: pallet_transaction_payment,
		// Sudo: pallet_sudo,
		SubstrateKitties: pallet_kitties::{Pallet, Call, Storage, Event<T>},
	}
);

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = u128;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
	type WeightInfo = ();
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {              // <- add this macro
	// One can own at most 9,999 Kitties
	pub const MaxKittyOwned: u32 = 9999;
}

/// Configure the pallet-kitties in pallets/kitties.
impl pallet_kitties::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type KittyRandomness = RandomnessCollectiveFlip;
	type MaxKittyOwned = MaxKittyOwned;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	// ref https://github.com/AcalaNetwork/Acala/blob/master/modules/dex/src/mock.rs#L214
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(ALICE, 1_000_000_000_000_000_000u128),
			(BOB, 1_000_000_000_000_000_000u128),
			(ALICE_2, 1_000_000_000_000_000_000u128),
			(BOB_2, 1_000u128),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}
