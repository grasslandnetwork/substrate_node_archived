#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		inherent::Vec,
		pallet_prelude::*,
		sp_runtime::traits::Hash,
		traits::{tokens::ExistenceRequirement},
	};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;


	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
    pub struct ActivationsDigest<T: Config> {
        pub digest: Vec<u8>,
        pub author: <T as frame_system::Config>::AccountId,
    }



    /// Storage Map for ActivationsDigests 
	#[pallet::storage]
	#[pallet::getter(fn activations_digests)]
    pub(super) type ActivationsDigests<T: Config> = StorageMap<_, Twox64Concat, T::Hash, ActivationsDigest<T>>;
    

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ActivationsDigestAdded(Vec<u8>, T::AccountId, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

        #[pallet::weight(10000)]
		pub fn add_activations_digest(
            origin: OriginFor<T>,
            digest: Vec<u8>
        ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let author = ensure_signed(origin)?;


            let activations_digest = ActivationsDigest { digest: digest.clone(), author: author.clone() };

            let activations_digest_id = T::Hashing::hash_of(&activations_digest);

			// Update storage.
			<ActivationsDigests<T>>::insert(activations_digest_id, activations_digest);

			// Emit an event.
			Self::deposit_event(Event::ActivationsDigestAdded(digest, author, activations_digest_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


        

	}

	impl<T: Config> Pallet<T> {}
}
