// File: src/counter.rs

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{CLValue, URef};

const COUNT_KEY: &str = "count_key";

pub fn counter_inc() {
    // Get the current count from storage
    let count_uref = storage::get_uref(COUNT_KEY).unwrap_or_revert();
    let mut count: i32 = storage::read(count_uref).unwrap_or_revert();

    // Increment the count
    count += 1;

    // Store the new count in storage
    storage::write(count_uref, count);
}

pub fn counter_get() -> CLValue {
    // Get the current count from storage
    let count_uref = storage::get_uref(COUNT_KEY).unwrap_or_revert();
    let count: i32 = storage::read(count_uref).unwrap_or_revert();

    // Return the count as a CLValue
    CLValue::from_t(count).unwrap_or_revert()
}
