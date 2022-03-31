use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(SubsGreeter::greet(Origin::signed(1)));
		// this should fail as greeting with standard is alllowed only once.
		assert_noop!(SubsGreeter::greet(Origin::signed(1)), Error::<Test>::QuotaExceeded);
	});
}

#[test]
fn test_alter_membership() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(SubsGreeter::alter_membership(Origin::signed(1), "gold".into()));
	});
}

#[test]
fn test_should_fail_alter_membership() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_noop!(
			SubsGreeter::alter_membership(Origin::signed(1), "invalid_membership_string".into()),
			Error::<Test>::InvalidUpgrade
		);
	});
}
