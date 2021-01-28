use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop, traits::{OnFinalize, OnInitialize}};
use super::*;

// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }

pub type System = frame_system::Module<Test>;

fn run_to_block(n: u64) {
	while System::block_number() < n {
		KittiesModule::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		KittiesModule::on_initialize(System::block_number());
	}
}

#[test]
fn create_kitty_work() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		run_to_block(10);
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));

		let expected_event = TestEvent::hello_kitty(RawEvent::Created(1, 0));
		assert_eq!(
			System::events()[0].event,
			expected_event
		);
	});
}

#[test]
fn breed_kitty_work() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		run_to_block(10);
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
		assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));

		let expected_event = TestEvent::hello_kitty(RawEvent::Breeded(1, 2));
		assert_eq!(
			System::events()[2].event,
			expected_event
		);
	});
}

#[test]
fn breed_kitty_failed_parents_the_same() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		run_to_block(10);
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 0, 0),
			Error::<Test>::ParentsTheSame
		);
	});
}

#[test]
fn breed_kitty_failed_invalid_kitty_id() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		run_to_block(10);
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 0, 2),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn transfer_kitty_work() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		run_to_block(10);
		assert_eq!(KittiesModule::create(Origin::signed(1),), Ok(()));
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));

		let expected_event = TestEvent::hello_kitty(RawEvent::Transfered(1, 2, 0));
		assert_eq!(
			System::events()[1].event,
			expected_event
		);
	});
}