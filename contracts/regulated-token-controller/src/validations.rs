use crate::admin::{has_administrator, read_administrator};
use crate::asset::read_asset;
use soroban_sdk::Env;

pub fn is_contract_initialized(e: &Env) {
    if !has_administrator(&e) {
        panic!("Contract hasn't been initialized yet!")
    }
}

pub fn is_invoker_the_asset_contract(e: &Env) {
    read_asset(e).require_auth();
}

pub fn is_authorized_by_admin(e: &Env) {
    read_administrator(e).require_auth();
}
