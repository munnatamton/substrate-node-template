#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec;
  
  
/// Configure the pallet by specifying the parameters and types on which it depends.
#[pallet::config]
pub trait Config: frame_system::Config {
/// Because this pallet emits events, it depends on the runtime's definition of an event.
type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
}

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
// https://docs.substrate.io/v3/runtime/events-and-errors
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
  /// Event emitted when a proof has been complianced. [who, compliance]
  ComplianceCreated(T::AccountId, Vec<u8>),
  /// Event emitted when a compliance is revoked by the owner. [who, compliance]
  ComplianceRevoked(T::AccountId, Vec<u8>),
}

#[pallet::error]
pub enum Error<T> {
  /// The proof has already been complianced.
  ProofAlreadyComplianced,
  /// The proof does not exist, so it cannot be revoked.
  NoSuchProof,
  /// The proof is complianced by another account, so caller can't revoke it.
  NotProofOwner,
}
  
#[pallet::pallet]
#[pallet::generate_store(pub(super) trait Store)]
//#[pallet::generate_storage_info]
pub struct Pallet<T>(_);

#[pallet::storage]
pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;#[pallet::hooks]


impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
// Dispatchable functions allow users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::weight(1_000)]
  pub fn create_compliance(
    origin: OriginFor<T>,
    proof: Vec<u8>,
    ) -> DispatchResult {
      // Check that the extrinsic was signed and get the signer.
      // This function will return an error if the extrinsic is not signed.
      // https://docs.substrate.io/v3/runtime/origins
      let sender = ensure_signed(origin)?;

      // Verify that the specified proof has not already been complianced.
      ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyComplianced);

      // Get the block number from the FRAME System pallet.
      let current_block = <frame_system::Pallet<T>>::block_number();

      // Store the proof with the sender and block number.
      Proofs::<T>::insert(&proof, (&sender, current_block));

      // Emit an event that the compliance was created.
      Self::deposit_event(Event::ComplianceCreated(sender, proof));

      Ok(())
      }

      #[pallet::weight(10_000)]
      pub fn revoke_compliance(
        origin: OriginFor<T>,
        proof: Vec<u8>,
        ) -> DispatchResult {
          // Check that the extrinsic was signed and get the signer.
          // This function will return an error if the extrinsic is not signed.
          // https://docs.substrate.io/v3/runtime/origins
          let sender = ensure_signed(origin)?;

          // Verify that the specified proof has been complianced.
          ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

          // Get owner of the compliance.
          let (owner, _) = Proofs::<T>::get(&proof);

          // Verify that sender of the current call is the compliance owner.
          ensure!(sender == owner, Error::<T>::NotProofOwner);

          // Remove compliance from storage.
          Proofs::<T>::remove(&proof);

          // Emit an event that the compliance was erased.
          Self::deposit_event(Event::ComplianceRevoked(sender, proof));
          Ok(())
        }
  }
}