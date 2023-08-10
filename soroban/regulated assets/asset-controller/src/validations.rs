
use soroban_sdk::{contract, Address, Env, Account};
use crate::admin::read_administrator;


pub fn is_invoker_the_asset_contract(e:Env){

    // let invoker = ;


    assert!(read_administrator(&e) == invoker, 
    "Invoker is not the registered regulated asset!");

}