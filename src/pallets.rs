use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

use crate::system::SystemConfig;

/// Create the Balance Configuration Trait to define the types of the Balance struct.
pub trait BalanceConfig: SystemConfig {
    type Balance: Zero + CheckedAdd + CheckedSub + PartialOrd + Copy;
}

/// Pallet struct to hold the balances of the accounts.
/// It has a BTreeMap to store the balances of the accounts.
/// The balances are stored as a key-value pair where the key is the AccountId and the value is the Balance.
/// The Pallet struct has methods to set the balance, get the balance, and transfer the balance from one account to another.
/// The transfer method checks if the sender has enough balance to transfer and then transfers the balance.
/// If the sender has insufficient balance, it returns an error message.
/// The Pallet struct is generic over the BalanceConfig trait.
/// The BalanceConfig trait defines the types of the Balance.
/// It also inherits the SystemConfig trait to define the types of the AccountId, BlockNumber, and Nonce.
#[derive(Debug)]
pub struct Pallet<T: BalanceConfig> {
    pub balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: BalanceConfig> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // Set the balance of an account
    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        // Insert the balance into the BTreeMap
        self.balances.insert(who, amount);
    }

    // Get the balance of an account
    pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
        // Get the balance from the BTreeMap
        // If the balance does not exist, return zero
        // Dereference the balance to get the value using the "*" operator.
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    // Transfer the balance from one account to another
    pub fn transfer(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &str> {
        // Get the balance of the sender and receiver
        let from_balance = self.get_balance(&from);
        let to_balance = self.get_balance(&to);

        // Check if the sender has enough balance to transfer
        if from_balance >= amount {
            self.set_balance(
                from,
                from_balance
                    .checked_sub(&amount)
                    .unwrap_or(T::Balance::zero()),
            );
            self.set_balance(
                to,
                to_balance
                    .checked_add(&amount)
                    .unwrap_or(T::Balance::zero()),
            );
        } else {
            // Return an error message if the sender has insufficient balance
            return Err("Insufficient balance!");
        }

        Ok(())
    }
}
