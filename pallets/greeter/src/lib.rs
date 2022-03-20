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

	// StorageMap containing entries of AccountId and Member
	// object defined above
	#[pallet::storage]
	pub(super) type Members<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Member<T>>;

	// events this pallet will
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Whenever user greets
		Greeted { user: T::AccountId },
		// when account is initialized for the first time
		AccountInitialized { user: T::AccountId },
		// Whenever greeting count is beyond allowed one
		QuotaExceeded { user: T::AccountId, membership: Membership },
		// when member upgrades the membership
		MembershipUpgraded { user: T::AccountId, old_membership: Membership },
	}

	// error to report if case of undesired situation
	#[pallet::error]
	pub enum Error<T> {
		// when gree quota exceeded
		QuotaExceeded,
		// when member upgrade is invalid
		InvalidUpgrade,
	}

	// Greeter Pallet's callables.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Greet Operation
		///
		/// let signed origins greet
		#[pallet::weight(0)]
		pub fn greet(origin: OriginFor<T>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let user = ensure_signed(origin)?;

			// Generate unique DNA and Gender using a helper function
			Self::do_greet(user)?;
			Ok(())
		}
	}

	// methods internal to pallet
	impl<T: Config> Pallet<T> {
		// logic of storing and updating the greet info
		fn do_greet(user: T::AccountId) -> Result<(), DispatchError> {
			let opt_member = Members::<T>::get(&user);

			if let Some(mut member) = opt_member {
				match member.member_type {
					Membership::Standard => Err(Error::<T>::QuotaExceeded.into()),

					Membership::Platinum | Membership::Gold => {
						if member.greet_count > member.member_type.get_quota() {
							Err(Error::<T>::QuotaExceeded.into())
						} else {
							member.greet_count += 1;
							Ok(())
						}
					},
				}
			} else {
				Members::<T>::insert(
					&user,
					Member { greet_count: 1, member_type: Membership::Standard, id: user.clone() },
				);

				log::info!("account initialize successfully!!");
				Ok(())
			}
		}
	}
}
