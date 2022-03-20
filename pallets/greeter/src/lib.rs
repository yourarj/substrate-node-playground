#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

/// ## Greeter Pallet
/// this pallet lets user greet the
/// blockchain
#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// ### Greeter Pallet Struct
	/// pallet struct of Greeter
	/// business logic & callable gather around it
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// Membership enum
	// What type of membership a member can have
	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Membership {
		Platinum,
		Gold,
		Standard,
	}

	// implementation for Membership
	impl Membership {
		fn get_quota(&self) -> u8 {
			match *self {
				Membership::Platinum => 10,
				Membership::Gold => 5,
				Membership::Standard => 1,
			}
		}
	}

	/// ### A Member
	/// Member of the Greeter community\
	/// stores info\
	/// 1. greet_count: the number of times user has greeted
	/// 2. what is type of his membership
	/// 3. id, to recognize the use with his accountID
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Member<T: Config> {
		pub greet_count: u8,
		pub member_type: Membership,
		pub id: T::AccountId,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// error to report if case of undesired situation
	#[pallet::error]
	pub enum Error<T> {}

	// Greeter Pallet's callables.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
