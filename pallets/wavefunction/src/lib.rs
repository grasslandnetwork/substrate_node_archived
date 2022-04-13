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
    pub struct WaveFunction<T: Config> {
        pub observation: Vec<u8>,
        pub prediction: Vec<u8>,
        pub author: <T as frame_system::Config>::AccountId,
    }



    /// Storage Map for WaveFunctions 
	#[pallet::storage]
	#[pallet::getter(fn wave_functions)]
    pub(super) type WaveFunctions<T: Config> = StorageMap<_, Twox64Concat, T::Hash, WaveFunction<T>>;
    

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		WaveFunctionAdded(Vec<u8>, T::AccountId, T::Hash),
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
		pub fn add_wavefunction(
            origin: OriginFor<T>,
            observation: Vec<u8>,
            prediction: Vec<u8>,
        ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let author = ensure_signed(origin)?;


            let wave_function = WaveFunction {
                observation: observation.clone(),
                prediction: prediction.clone(),
                author: author.clone()
            };

            let wave_function_id = T::Hashing::hash_of(&wave_function);

			// Update storage.
			<WaveFunctions<T>>::insert(wave_function_id, wave_function);

			// Emit an event.
			Self::deposit_event(Event::WaveFunctionAdded(observation, author, wave_function_id));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}


        

	}

	impl<T: Config> Pallet<T> {}
}
