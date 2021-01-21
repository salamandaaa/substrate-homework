use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
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

#[test]
fn create_claim_works() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

		assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Module::<Test>::block_number()));
	})
}

#[test]
fn create_claim_failed_claim_already_exist() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
	})
}

#[test]
fn revoke_claim_failed_claim_not_exist() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofNotExist
		);
	})
}

#[test]
fn revoke_cliam_failed_not_claim_owner() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn revoke_cliam_failed_claim_too_long() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimTooLong
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
	})
}

#[test]
fn transfer_claim_failed_claim_not_exist() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ProofNotExist
		);
	})
}

#[test]
fn transfer_claim_failed_dest_owner_same() {
	new_test_ext().execute_with( || {
		let claim: Vec<u8> = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 1),
			Error::<Test>::DestOwnerSame
		);
	})
}