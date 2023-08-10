
use soroban_sdk::{contract, contractimpl, Address, Env};
use crate::rules::has_spender_achieved_outflow_limit;
use crate::asset::write_asset;
use crate::data::write_outflow_limit;
use crate::admin::{has_administrator,write_administrator};
use crate::validations::is_invoker_the_asset_contract;

pub trait AssetControllerTrait {

    fn initialize(e: Env, asset: Address, admin: Address, outflow_limit: i128 );
    // fn inflow();
    // fn delegated_inflow();

    //Transfer goin out
    fn preprocess_outflow(env: Env, 
        from: Address,
        to: Address,
        amount: i128,) -> bool;
    
    fn test(env: Env);
    // fn allow();

    //Transfer from goin out
    // fn preprocess_approved_outflow( env: soroban_sdk::Env,
    //     spender: Address,
    //     from: Address,
    //     to: Address,
    //     amount: i128,) -> bool;

    // //Approve an account (allowance)
    // fn approve(env: soroban_sdk::Env,
    //     from: Address,
    //     spender: Address,
    //     amount: i128,
    //     expiration_ledger: u32,);

    // burn
    // delegated burn
    // 
    // mint

    // METHOD TO SET APPROVAL OF THE USER after probation


    //read only

}

#[contract]
pub struct AssetController;

#[contractimpl]
impl AssetControllerTrait for AssetController {

    fn initialize(e: Env, admin: Address, asset: Address, outflow_limit: i128) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        write_asset(&e, &asset);
        write_outflow_limit(&e,outflow_limit);
      
    }

    fn preprocess_outflow(e: Env, 
        from: Address,
        to: Address,
        amount: i128,) -> bool  {

    
    
        //make sure invoker is asset contract
        has_spender_achieved_outflow_limit(&e, from,amount);



        return true;
    }

    fn test(e: Env){
        is_invoker_the_asset_contract(e);
    }


}
