use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

// pub fn has_asset(e: &Env) -> bool {
//     let key = DataKey::Asset;
//     e.storage().instance().has(&key)
// }

pub fn read_asset(e: &Env) -> Address {
    let key = DataKey::Asset;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_asset(e: &Env, id: &Address) {
    let key = DataKey::Asset;
    e.storage().instance().set(&key, id);
}
