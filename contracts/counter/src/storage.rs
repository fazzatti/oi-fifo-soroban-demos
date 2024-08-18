use soroban_sdk::{contracttype, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Odd,  //u64
    Even, //u64
}

pub fn write_odd(e: &Env, odd_count: u64) {
    e.storage().instance().set(&DataKey::Odd, &odd_count);
}

pub fn read_odd(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::Odd).unwrap_or(0)
}

pub fn write_even(e: &Env, even_count: u64) {
    e.storage().instance().set(&DataKey::Even, &even_count);
}

pub fn read_even(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::Even).unwrap_or(0)
}
