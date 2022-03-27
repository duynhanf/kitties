//! Benchmarking setup for pallet-kitties

use super::*;

use crate::Pallet as KittiesPallet;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

const SEED: u32 = 0;

benchmarks! {
	// create kitty
	create_kitty {
		let caller: T::AccountId = account("Alice", 1, SEED);
	}: create_kitty(RawOrigin::Signed(caller))

	// set price for kitty
	// set_price {
	// 	let s in 0 .. 1000;
	// 	let caller: T::AccountId = account("Alice", 1, SEED);
	// 	// let kitty_id = crate::lib::pallet::KittiesPa
	// 	let _ = pallet_balances::TotalIssuance().unwrap()
	// }: _(RawOrigin::Signed(caller), kitty_id, Some(1000u32.into()))
	impl_benchmark_test_suite!(KittiesPallet, crate::mock::new_test_ext(), crate::mock::Test);
}
