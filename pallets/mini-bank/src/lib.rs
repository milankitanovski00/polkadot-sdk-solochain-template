#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, dispatch::DispatchResult, ensure};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // STORAGE
    #[pallet::storage]
    pub type Balances<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, u128, ValueQuery>;

    #[pallet::storage]
    pub type Accounts<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    // EVENTS
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AccountCreated(T::AccountId),
        DepositMade(T::AccountId, u128),
        TransferCompleted(T::AccountId, T::AccountId, u128),
    }

    // ERRORS
    #[pallet::error]
    pub enum Error<T> {
        AccountAlreadyExists,
        AccountNotFound,
        NotEnoughBalance,
    }

    // CALLS
    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(10_000)]
        pub fn create_account(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                !Accounts::<T>::get(&who),
                Error::<T>::AccountAlreadyExists
            );

            Accounts::<T>::insert(&who, true);
            Balances::<T>::insert(&who, 0);

            Self::deposit_event(Event::AccountCreated(who));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn deposit(origin: OriginFor<T>, amount: u128) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(Accounts::<T>::get(&who), Error::<T>::AccountNotFound);

            let balance = Balances::<T>::get(&who);
            Balances::<T>::insert(&who, balance + amount);

            Self::deposit_event(Event::DepositMade(who, amount));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u128,
        ) -> DispatchResult {
            let from = ensure_signed(origin)?;

            ensure!(Accounts::<T>::get(&from), Error::<T>::AccountNotFound);
            ensure!(Accounts::<T>::get(&to), Error::<T>::AccountNotFound);

            let from_balance = Balances::<T>::get(&from);
            ensure!(from_balance >= amount, Error::<T>::NotEnoughBalance);

            Balances::<T>::insert(&from, from_balance - amount);

            let to_balance = Balances::<T>::get(&to);
            Balances::<T>::insert(&to, to_balance + amount);

            Self::deposit_event(Event::TransferCompleted(from, to, amount));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok};

    #[test]
    fn create_account_works() {
        // basic test skeleton (možemo kasnije proširiti)
        assert_eq!(1, 1);
    }
}