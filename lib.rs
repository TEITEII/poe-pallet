#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec; 

    #[pallet::config]
	pub trait Config: frame_system::Config {
        
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
       
        ClaimCreated(T::AccountId, Vec<u8>),
    	ClaimTransfer(T::AccountId,Vec<u8>,T::AccountId),
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error] 
	pub enum Error<T> {
		
		ClaimAlreadyClaimed,
		
		NoSuchClaim,
		
		NotClaimOwner,
	}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage] 
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;   

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]  
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub(super) fn create_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
		) -> DispatchResultWithPostInfo {

			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ClaimAlreadyClaimed);

			let block_number = <frame_system::Pallet<T>>::block_number();

			Proofs::<T>::insert(&claim, (&sender, block_number));

			Self::deposit_event(Event::ClaimCreated(sender, claim));

			Ok(().into())
		}

		#[pallet::weight(10_000)]
        	pub fn transfer_claim(
            		origin: OriginFor<T>,
            		claim: Vec<u8>,
			new_owner: T::AccountId,
   
       		 ) -> DispatchResultWithPostInfo {

            		let sender = ensure_signed(origin)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			Proofs::<T>::mutate(&claim,|v|*v = Some((new_owner.clone(),frame_system::Pallet::<T>::block_number()));

			Self::deposit_event(Event::ClaimTransfer(sender,claim,new_owner));

			Ok(().into())
		}
			 
		#[pallet::weight(10_000)]
		fn revoke_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			
			let sender = ensure_signed(origin)?;

			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::NoSuchClaim);

			let (owner, _) = Proofs::<T>::get(&claim);

			ensure!(sender == owner, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevoked(sender, claim));

			Ok(().into())
		}
	}
}
