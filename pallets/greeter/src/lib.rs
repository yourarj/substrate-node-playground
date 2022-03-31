#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// mocks for test
#[cfg(test)]
mod mock;

// tests
#[cfg(test)]
mod tests;

// benchmarking the pallet
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// ## Greeter Pallet
/// this pallet lets user greet the
/// blockchain
///
/// ### membership values supported are
/// standard OR gold OR platinum
///
/// ### Goals ACHIEVED:
/// implmement custom TYPES
/// implement custom STORAGE
/// implement custom EVENTS
/// implement custom ERRORS
/// implement EXTRINSICS without input
/// implement EXTRINSICS with input
/// implement types without 'std' LIB
/// implement custom HOOKS
///
/// ### Scenarios TESTED:
///	should_succeed - greet when no account
///	should_fail - greet twice with STANDARD
///	should_succeed - upgrade membership with no account
///	should_succeed - upgrade membership with account
///	should_succeed - upgrade from smaller to larger type and greet
///	should_fail - upgrade from larger to smaller and tweet
///	should_fail - invalid input (both content and length) when upgrading
///	should_succeed - hooks defined should get invoked
#[frame_support::pallet]
pub mod pallet {
	use core::str::FromStr;
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

	// implement FromStr for enum
	impl FromStr for Membership {
		type Err = scale_info::prelude::string::String;

		fn from_str(s: &str) -> Result<Self, Self::Err> {
			match &s.to_uppercase()[..] {
				"PLATINUM" => Ok(Membership::Platinum),
				"GOLD" => Ok(Membership::Gold),
				"STANDARD" => Ok(Membership::Standard),
				_ => Err(scale_info::prelude::string::String::from("Invalid membership specified")),
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
		MembershipUpgraded { user: T::AccountId, new_membership: Membership },
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

		/// Alter membership Operation
		///
		/// let alter membership
		#[pallet::weight(0)]
		pub fn alter_membership(
			origin: OriginFor<T>,
			membership: scale_info::prelude::vec::Vec<u8>,
		) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let user = ensure_signed(origin)?;

			// throw error if string lenght is unreasonably long
			log::info!("vec length: {}", membership.len());
			if membership.len() > 50 {
				return Err(Error::<T>::InvalidUpgrade.into())
			}

			let res_encode = scale_info::prelude::string::String::from_utf8(membership)
				.map(|str_membership| str_membership)
				.map_err(|_| Error::<T>::InvalidUpgrade)?;

			// Generate unique DNA and Gender using a helper function
			Self::do_alter_membership(user, &res_encode)?;
			Ok(())
		}
	}

	// Hooks
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// ### on_finalize
		/// it will get invoked once the block is finalized
		fn on_finalize(block_number: T::BlockNumber) {
			log::info!("GREETER_HOOK[on_finalize]: bock {:?} has been finalized", block_number);
		}

		/// ### on_idle
		/// it will get invoked when the block is in the process of being finalized
		fn on_idle(block_number: T::BlockNumber, remaining_weight: Weight) -> Weight {
			log::info!(
				"GREETER_HOOK[on_idle]: bock {:?} is being finalized remaining weight is {}",
				block_number,
				remaining_weight
			);
			// assuming a millis weight is spent
			frame_support::weights::constants::WEIGHT_PER_MILLIS
		}

		/// ### on_initialize
		/// gets invoked when the blick is initialized
		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			log::info!("GREETER_HOOK[on_finalize]: block {:?} is being initialized", block_number);
			// assuming a millis weight is spent
			frame_support::weights::constants::WEIGHT_PER_MILLIS
		}

		/// ### on_runtime_upgrade
		/// gets invoked if the runtime (wasm) is upgraded
		fn on_runtime_upgrade() -> Weight {
			log::info!("GREETER_HOOK[on_runtime_upgrade]: runtime has been upgraded");
			// assuming a millis weight is spent
			frame_support::weights::constants::WEIGHT_PER_MILLIS
		}

		/// ### offchain_worker
		/// get invoked after every block is imported (fully synced)
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!(
				"GREETER_HOOK[offchain_worker]: I'm doing work off the chain on block: {:?}",
				block_number
			);
		}
	}

	// methods internal to pallet
	impl<T: Config> Pallet<T> {
		// logic of storing and updating the greet info
		fn do_greet(user: T::AccountId) -> Result<(), DispatchError> {
			let opt_member = Members::<T>::get(&user);

			if let Some(mut member) = opt_member {
				match member.member_type {
					Membership::Standard => {
						// emit quota exceeded event
						Self::deposit_event(Event::QuotaExceeded {
							user: user.clone(),
							membership: Membership::Standard,
						});
						Err(Error::<T>::QuotaExceeded.into())
					},

					Membership::Platinum | Membership::Gold => {
						if member.greet_count > member.member_type.get_quota() {
							// emit quota exceeded event
							Self::deposit_event(Event::QuotaExceeded {
								user: user.clone(),
								membership: member.member_type,
							});
							Err(Error::<T>::QuotaExceeded.into())
						} else {
							member.greet_count += 1;
							Members::<T>::insert(&user, member);
							Self::deposit_event(Event::Greeted { user: user.clone() });
							Ok(())
						}
					},
				}
			} else {
				Members::<T>::insert(
					&user,
					Member { greet_count: 1, member_type: Membership::Standard, id: user.clone() },
				);

				// emit account initialized event
				Self::deposit_event(Event::AccountInitialized { user: user.clone() });
				Self::deposit_event(Event::Greeted { user: user.clone() });

				log::info!("account initialize successfully!!");
				Ok(())
			}
		}

		// logic for upgrading membership
		fn do_alter_membership(user: T::AccountId, membership: &str) -> Result<(), DispatchError> {
			let mem_enum = Membership::from_str(membership)
				.map(|mem| mem)
				.map_err(|_| Error::<T>::InvalidUpgrade)?;

			let opt_member = Members::<T>::get(&user);

			if let Some(mut member) = opt_member {
				member.member_type = mem_enum;
				Members::<T>::insert(&user, member);
				// emit account initialized event
				Self::deposit_event(Event::MembershipUpgraded {
					user: user.clone(),
					new_membership: mem_enum,
				});
			} else {
				Members::<T>::insert(
					&user,
					Member { greet_count: 0, member_type: mem_enum, id: user.clone() },
				);

				// emit account initialized event
				Self::deposit_event(Event::MembershipUpgraded {
					user: user.clone(),
					new_membership: mem_enum,
				});
			}
			Ok(())
		}
	}
}
