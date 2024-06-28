// File: src/main.rs

#[no_mangle]
pub extern "C" fn call() {
    contract::init_contract();

    // Call the `counter_inc` function
    contract::call_counter_inc();

    // Call the `counter_get` function
    let count: i32 = contract::call_counter_get();
    runtime::println(&format!("Current count: {}", count));
}
