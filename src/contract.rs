// File: src/contract.rs

// ...

pub fn call_counter_inc() {
    counter::counter_inc();
}

pub fn call_counter_get() -> CLValue {
    counter::counter_get()
}
