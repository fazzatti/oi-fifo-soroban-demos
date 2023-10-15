use soroban_sdk::{Address, Env};
use crate::storage_types::DataKey;




pub fn write_asset(e: &Env, id: &Address) {
    let key = DataKey::Asset;
    e.storage().instance().set(&key, id);
}

pub fn read_asset(e: &Env) -> Address {
    let key = DataKey::Asset;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn read_admin(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_asset_controller(e: &Env, id: &Address) {
    let key = DataKey::AssetController;
    e.storage().instance().set(&key, id);
}

pub fn read_asset_controller(e: &Env) -> Address {
    let key = DataKey::AssetController;
    e.storage().instance().get(&key).unwrap()
}
