use crate::data::read_asset_controller;
use soroban_sdk::{Address, Env};

mod asset_controller_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/asset_controller.wasm"
    );
}

pub fn review_transfer(e: &Env, from: &Address, to: &Address, amount: &i128) {
    let asset_controller = read_asset_controller(&e);
    let asset_controller_client = asset_controller_contract::Client::new(&e, &asset_controller);

    asset_controller_client.review_transfer(from, to, amount);
}
