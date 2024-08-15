use soroban_sdk::{token, Env};
use standard_traits::classic_wrapper::common::{read_admin, read_asset, Metadatakey};

pub fn is_contract_initialized_validation(e: &Env) {
    if !has_administrator(&e) {
        panic!("Contract hasn't been initialized yet!")
    }
}
pub fn is_contract_not_initialized_validation(e: &Env) {
    if has_administrator(&e) {
        panic!("Contract hasa already been initialized!")
    }
}

pub fn is_admin_validation(e: &Env) {
    has_administrator(&e);
    let admin = read_admin(&e);
    admin.require_auth();
}

fn has_administrator(e: &Env) -> bool {
    let key = Metadatakey::WrapperMetadada;
    e.storage().instance().has(&key)
}

pub fn is_wrapper_admin(e: &Env) -> bool {
    let wrapper_address = e.current_contract_address();
    let asset_address = read_asset(&e);
    let asset_admin_client = token::StellarAssetClient::new(&e, &asset_address);
    let asset_admin = asset_admin_client.admin();

    if asset_admin == wrapper_address {
        return true;
    }

    false
}

pub fn is_wrapper_active_validation(e: &Env) {
    if !is_wrapper_admin(&e) {
        panic!("Wrapper it not active!");
    }
}
