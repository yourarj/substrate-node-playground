//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

//FIXME: Benchmarks needs to be rewritten and fixed
benchmarks! {
	alter_membership {
		let s:Vec[u8] in ["".into(), standard.into(), "gold".into(), "platinum".into(), "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq".into()];
		let caller: T::AccountId = whitelisted_caller();
	}: alter_membership(RawOrigin::Signed(caller), s)
	verify {
		assert_eq!(Members::<T>::get(&caller)?.member_type, s);
	}

	impl_benchmark_test_suite!(SubsGreeter, crate::mock::new_test_ext(), crate::mock::Test);
}
