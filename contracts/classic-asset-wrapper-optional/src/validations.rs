use soroban_sdk::Env;
use standard_traits::classic_wrapper::common::Metadatakey;

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

fn has_administrator(e: &Env) -> bool {
    let key = Metadatakey::WrapperMetadada;
    e.storage().instance().has(&key)
}
