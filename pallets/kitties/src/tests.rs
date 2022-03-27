use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};

// create_kitty testcases
#[test]
fn a_new_kitty_should_be_created() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		// Dispatch create_kitty() extrinsic
		assert_ok!(SubstrateKitties::create_kitty(Origin::signed(ALICE)));
		// Read pallet storage and assert an expected result.
		assert_eq!(SubstrateKitties::kitty_cnt(), 1);
	});
}

// set_price testcases
#[test]
fn origin_can_set_price_for_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		assert_ok!(SubstrateKitties::set_price(Origin::signed(ALICE), kitty_id, Some(1000)));

		let kitty = SubstrateKitties::kitties(&kitty_id).unwrap();

		assert_eq!(kitty.price, Some(1000));
	});
}

#[test]
fn others_cannot_set_price_for_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		assert_err!(
			SubstrateKitties::set_price(Origin::signed(BOB), kitty_id, Some(1000)),
			Error::<Test>::NotKittyOwner
		);
	});
}

/// Transfer kitty testcases

#[test]
fn owner_can_transfer_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		// transfer to bob
		assert_ok!(SubstrateKitties::transfer(Origin::signed(ALICE), BOB, kitty_id));

		let kitty = SubstrateKitties::kitties(&kitty_id).unwrap();

		// check owner of kitty
		assert_eq!(kitty.owner, 2);
	});
}

#[test]
fn others_cannot_transfer_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		assert_err!(
			SubstrateKitties::transfer(Origin::signed(BOB), ALICE_2, kitty_id),
			Error::<Test>::NotKittyOwner
		);
	});
}

#[test]
fn owner_cannot_transfer_to_self() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		// send kitty to self
		assert_err!(
			SubstrateKitties::transfer(Origin::signed(ALICE), ALICE, kitty_id),
			Error::<Test>::TransferToSelf
		);
	});
}

// #[test]
// fn cannot_transfer_to_account_hasnot_the_capacity() {
// 	new_test_ext().execute_with(|| {
// 		System::set_block_number(1);

// 		// create a new kitty
// 		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

// 		// get kitty id
// 		let kitty_id = SubstrateKitties::kitties_owned(1)[0];

// 		assert_err!(
// 			SubstrateKitties::transfer(Origin::signed(ALICE), 1, kitty_id),
// 			Error::<Test>::TransferToSelf
// 		);
// 	});
// }

/// buy_kitty testcases

// #[test]
// fn buy_kitty_should_return_error_when_kitty_does_not_exists() {
// 	new_test_ext().execute_with(|| {
// 		System::set_block_number(1);

// 		// create a new kitty
// 		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

// 		// get kitty id
// 		let kitty_id = SubstrateKitties::kitties_owned(1)[0];

// 		let other_kitty = Kitty::<Test> {
// 			dna: None.unwrap_or_else(Pallet::<Test>::gen_dna),
// 			price: None,
// 			gender: None.unwrap_or_else(Pallet::<Test>::gen_gender),
// 			owner: 3,
// 			date_created: pallet_timestamp::Pallet::<Test>::now(),
// 		};

// 		let other_kitty_id = Hashing::<Test>::hash_of(&other_kitty);

// 		assert_err!(
// 			SubstrateKitties::buy_kitty(Origin::signed(2), kitty_id, 1000),
// 			Error::<Test>::TransferToSelf
// 		);
// 	});
// }

#[test]
fn buy_kitty_should_return_error_when_kitty_is_not_for_sale() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		assert_err!(
			SubstrateKitties::buy_kitty(Origin::signed(BOB), kitty_id, 1000),
			Error::<Test>::KittyNotForSale
		);
	});
}

#[test]
fn buy_kitty_should_return_error_when_bid_price_is_lower_than_price() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		SubstrateKitties::set_price(Origin::signed(ALICE), kitty_id, Some(1000));

		assert_err!(
			SubstrateKitties::buy_kitty(Origin::signed(BOB), kitty_id, 500),
			Error::<Test>::KittyBidPriceTooLow
		);
	});
}

#[test]
fn buy_kitty_should_return_error_when_bid_price_is_higher_than_current_balance() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		SubstrateKitties::set_price(Origin::signed(ALICE), kitty_id, Some(2000));

		assert_err!(
			SubstrateKitties::buy_kitty(Origin::signed(BOB_2), kitty_id, 3000),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn users_can_buy_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_id = SubstrateKitties::kitties_owned(ALICE)[0];

		let _ = SubstrateKitties::set_price(Origin::signed(ALICE), kitty_id, Some(2000));

		let _ = SubstrateKitties::buy_kitty(Origin::signed(ALICE_2), kitty_id, 3000);

		let kitty = SubstrateKitties::kitties(&kitty_id).unwrap();

		assert_eq!(kitty.owner, ALICE_2);

		// TODO check ballance of ALICE_2
	});
}

// breed_kitty

#[test]
fn users_can_breed_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// create a new kitty
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// create another
		System::set_block_number(2);
		let _ = SubstrateKitties::create_kitty(Origin::signed(ALICE));

		// get kitty id
		let kitty_ids = SubstrateKitties::kitties_owned(ALICE);

		assert_ok!(SubstrateKitties::breed_kitty(
			Origin::signed(ALICE),
			kitty_ids[0],
			kitty_ids[1]
		));

		// check dna
	});
}
