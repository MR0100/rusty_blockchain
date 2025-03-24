use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

use crate::system::SystemConfig;

pub trait BalanceConfig: SystemConfig {
    type Balance: Zero + CheckedAdd + CheckedSub + PartialOrd + Copy;
}

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

    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        self.balances.insert(who, amount);
    }

    pub fn get_balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        from: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &str> {
        let from_balance = self.get_balance(&from);
        let to_balance = self.get_balance(&to);

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
            return Err("Insufficient balance!");
        }

        Ok(())
    }
}
