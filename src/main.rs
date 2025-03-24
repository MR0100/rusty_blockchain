use rsm_en::pallets::{BalanceConfig, Pallet as Balances};
use rsm_en::system::{Pallet as System, SystemConfig};
use rsm_en::types::{AccountId, Balance, BlockNumber, Nonce};

#[derive(Debug)]
pub struct Runtime {
    system: System<Runtime>,
    balances: Balances<Runtime>,
}

impl SystemConfig for Runtime {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Nonce = Nonce;
}

impl BalanceConfig for Runtime {
    type Balance = Balance;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: System::new(),
            balances: Balances::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "Alice".to_string();
    let bob = "Bob".to_string();
    let mark = "Mark".to_string();

    runtime.balances.set_balance(alice.clone(), 10000);
    runtime.balances.set_balance(bob.clone(), 10000);
    runtime.balances.set_balance(mark.clone(), 10000);

    match runtime.balances.transfer(alice, bob, 1000) {
        Ok(_) => {
            println!("Transfer successful!");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("Runtime : {:?}", runtime);
}
