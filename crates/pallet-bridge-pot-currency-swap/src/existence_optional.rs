//! An implementation that does not require pot account existence and can potentially kill the
//! pot account by withdrawing all the funds from it.

use frame_support::{
    sp_runtime::{traits::Convert, DispatchError},
    traits::{Currency, ExistenceRequirement, Get, Imbalance, WithdrawReasons},
};

use super::{Config, CurrencySwap, Pallet};

/// A marker type for the implementation that does not require pot accounts existence.
pub enum Marker {}

impl<T: Config<I>, I: 'static>
    primitives_currency_swap::CurrencySwap<T::AccountIdFrom, T::AccountIdTo>
    for CurrencySwap<Pallet<T, I>, Marker>
{
    type From = T::CurrencyFrom;
    type To = T::CurrencyTo;
    type Error = DispatchError;

    fn swap(
        incoming_imbalance: <Self::From as Currency<T::AccountIdFrom>>::NegativeImbalance,
    ) -> Result<
        <Self::To as Currency<T::AccountIdTo>>::NegativeImbalance,
        primitives_currency_swap::ErrorFor<Self, T::AccountIdFrom, T::AccountIdTo>,
    > {
        let amount = incoming_imbalance.peek();

        let outgoing_imbalance = match T::CurrencyTo::withdraw(
            &T::PotTo::get(),
            T::BalanceConverter::convert(amount),
            WithdrawReasons::TRANSFER,
            ExistenceRequirement::AllowDeath,
        ) {
            Ok(imbalance) => imbalance,
            Err(error) => {
                return Err(primitives_currency_swap::Error {
                    cause: error,
                    incoming_imbalance,
                })
            }
        };

        T::CurrencyFrom::resolve_creating(&T::PotFrom::get(), incoming_imbalance);

        Ok(outgoing_imbalance)
    }

    fn estimate_swapped_balance(
        balance: <Self::From as Currency<T::AccountIdFrom>>::Balance,
    ) -> <Self::To as Currency<T::AccountIdTo>>::Balance {
        T::BalanceConverter::convert(balance)
    }
}
