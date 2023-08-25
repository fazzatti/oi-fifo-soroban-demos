use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

// pub fn has_asset_controller(e: &Env) -> bool {
//     let key = DataKey::AssetController;
//     e.storage().instance().has(&key)
// }

pub fn read_asset_controller(e: &Env) -> Address {
    let key = DataKey::AssetController;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_asset_controller(e: &Env, id: &Address) {
    let key = DataKey::AssetController;
    e.storage().instance().set(&key, id);
}