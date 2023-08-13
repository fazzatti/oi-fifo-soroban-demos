
use soroban_sdk::Env;
use crate::asset::read_asset;
use crate::admin::has_administrator;


pub fn is_contract_initialized(e:&Env){

    if !has_administrator(&e) {
        panic!("Contract hasn't been initialized yet!")
    }
}

pub fn is_invoker_the_asset_contract(e:&Env){
    let call_stack = e.call_stack();
    
    if call_stack.len() >= 2 {
        // Get the address of the contract invoked before the current one
        if let Some((address, _)) = call_stack.get(call_stack.len() - 2) {
            assert!(address == read_asset(&e),
                    "Asset controller invoker is not the registered asset.");
        }
    }else{
        panic!("Asset controller was invoked directly.");
    }
}


