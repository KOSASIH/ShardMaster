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
        if caller!= self.owner {
            return Err(ApiError::Unauthorized);
        }
        self.count += U256::one();
        Ok(())
    }

    // Get the current count
    pub fn counter_get(&self) -> U256 {
        self.count
    }

    // Get the contract owner
    pub fn get_owner(&self) -> Key {
        self.owner
    }

    // Transfer ownership
    pub fn transfer_ownership(&mut self, new_owner: Key, caller: Key) -> Result<(), ApiError> {
        if caller!= self.owner {
            return Err(ApiError::Unauthorized);
        }
        self.owner = new_owner;
        Ok(())
    }

    // Reset the counter
    pub fn reset(&mut self, caller: Key) -> Result<(), ApiError> {
        if caller!= self.owner {
            return Err(ApiError::Unauthorized);
        }
        self.count = U256::zero();
        Ok(())
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

pub fn get_owner() -> Key {
    let counter: Counter = runtime::get_contract();
    counter.get_owner()
}

pub fn transfer_ownership(new_owner: Key) {
    let counter: Counter = runtime::get_contract();
    let caller: Key = runtime::get_caller();
    counter.transfer_ownership(new_owner, caller).unwrap_or_revert();
}

pub fn reset() {
    let counter: Counter = runtime::get_contract();
    let caller: Key = runtime::get_caller();
    counter.reset(caller).unwrap_or_revert();
        }
