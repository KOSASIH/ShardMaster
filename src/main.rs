// File: src/main.rs

#[no_mangle]
pub extern "C" fn call() {
    contract::init_contract();
}
