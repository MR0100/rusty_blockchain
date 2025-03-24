use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

/// Create the System Configuration Trait to define the types of the System struct.
/// SystemConfig trait defines the types of the AccountId, BlockNumber, and Nonce.
pub trait SystemConfig {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + CheckedAdd + CheckedSub + PartialOrd + Copy + From<u8>;
    type Nonce: Zero + CheckedAdd + CheckedSub + PartialOrd + Copy + From<u8>;
}

/// The System struct holds the information about the Nonce and Blocks.
/// It has a BTreeMap to store the Nonce of the accounts.
/// The Nonce is a number that is incremented each time a transaction is made by an account.
#[derive(Debug)]
pub struct Pallet<T: SystemConfig> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: SystemConfig> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::from(1))
            .unwrap_or(T::BlockNumber::zero());
    }

    pub fn inc_nonce(&mut self, who: T::AccountId) {
        let nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::zero());
        self.nonce.insert(
            who.clone(),
            nonce
                .checked_add(&T::Nonce::from(1))
                .unwrap_or(T::Nonce::zero()),
        );
    }

    pub fn get_nonce(&self, who: T::AccountId) -> T::Nonce {
        *self.nonce.get(&who).unwrap_or(&T::Nonce::zero())
    }
}
