// File: counter.rs

use casper_contract::contract_api::{runtime, storage};
use casper_types::{ApiError, ContractHash, Key, URef, U256};

// Define the contract struct
pub struct Counter {
    pub count: U256,
    pub owner: Key,
}

impl Counter {
    // Initialize the contract
    pub fn new(owner: Key) -> Self {
        Counter {
            count: U256::zero(),
            owner,
        }
    }

    // Increment the counter
    pub fn counter_inc(&mut self, caller: Key) -> Result<(), ApiError> {
        if caller != self.owner {
            return Err(ApiError::Unauthorized);
        }
        self.count += U256::one();
        Ok(())
    }

    // Get the current count
    pub fn counter_get(&self) -> U256 {
        self.count
    }
}

// Define the contract entry points
pub fn counter_inc() {
    let counter: Counter = runtime::get_contract();
    let caller: Key = runtime::get_caller();
    counter.counter_inc(caller).unwrap_or_revert();
}

pub fn counter_get() -> U256 {
    let counter: Counter = runtime::get_contract();
    counter.counter_get()
        }
