// File: src/main.rs

// ...

// Define the `call` function
#[no_mangle]
pub extern "C" fn call() {
    // ...

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

    // ...

    // Call the `counter_inc` entry point
    counter::counter_inc();

    // Call the `counter_get` entry point
    let count: i32 = runtime::call_versioned_contract(
        stored_contract_hash,
        None,
        ENTRY_POINT_COUNTER_GET,
        Vec::new(),
    );
    runtime::println(&format!("Current count: {}", count));
}
