// File: src/contract.rs

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{CLValue, URef};

const CONTRACT_PACKAGE_NAME: &str = "counter_package_name";
const CONTRACT_ACCESS_UREF: &str = "counter_access_uref";

const ENTRY_POINT_COUNTER_INC: &str = "counter_inc";
const ENTRY_POINT_COUNTER_GET: &str = "counter_get";

const COUNT_KEY: &str = "count_key";
const CONTRACT_VERSION_KEY: &str = "contract_version_key";
const CONTRACT_KEY: &str = "contract_key";

pub fn init_contract() {
    // Initialize the count to 0 locally
    let count_start = storage::new_uref(0_i32);

    // Create the entry points for this contract
    let mut counter_entry_points = EntryPoints::new();
    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_GET,
        Vec::new(),
        CLType::I32,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));
    counter_entry_points.add_entry_point(EntryPoint::new(
        ENTRY_POINT_COUNTER_INC,
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    // In the named keys of the counter contract, add a key for the count.
    let mut counter_named_keys = NamedKeys::new();
    let key_name = String::from(COUNT_KEY);
    counter_named_keys.insert(key_name, count_start.into());

    // Create a new contract package that can be upgraded.
    let (stored_contract_hash, contract_version) = storage::new_contract(
        counter_entry_points,
        Some(counter_named_keys),
        Some(CONTRACT_PACKAGE_NAME.to_string()),
        Some(CONTRACT_ACCESS_UREF.to_string()),
    );

    // Store the contract version in the context's named keys.
    let version_uref = storage::new_uref(contract_version);
    runtime::put_key(CONTRACT_VERSION_KEY, version_uref.into());

    // Create a named key for the contract hash.
    runtime::put_key(CONTRACT_KEY, stored_contract_hash.into());
}
