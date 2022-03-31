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
