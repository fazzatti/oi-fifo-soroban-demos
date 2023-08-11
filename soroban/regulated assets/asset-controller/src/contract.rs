
use soroban_sdk::{contract, contractimpl, Address, Env, Vec,vec};
use crate::rules::{has_spender_achieved_outflow_limit, has_receiver_achieved_inflow_limit};
use crate::asset::{write_asset,read_asset};
use crate::data::{write_outflow_limit,write_inflow_limit, read_user_quota,record_transaction, write_quota_time_limit};
use crate::admin::{has_administrator,write_administrator,read_administrator};
use crate::validations::is_invoker_the_asset_contract;

pub trait AssetControllerTrait {

    fn initialize(e: Env, asset: Address, admin: Address, outflow_limit: i128, inflow_limit: i128, quota_time_limit: u64 );
    // fn inflow();
    // fn delegated_inflow();

    //Transfer goin out
    fn preprocess_outflow(env: Env, 
        from: Address,
        to: Address,
        amount: i128,) -> bool;
    
    fn test(env: Env)  ;
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

    fn get_quota(e:Env, id: Address) -> Vec<i128>;

}




#[contract]
pub struct AssetController;

#[contractimpl]
impl AssetControllerTrait for AssetController {

    fn initialize(e: Env, admin: Address, asset: Address, outflow_limit: i128, inflow_limit: i128 , quota_time_limit: u64) {
        if has_administrator(&e) {
            panic!("Already initialized!")
        }
        write_administrator(&e, &admin);
        write_asset(&e, &asset);

        write_outflow_limit(&e,outflow_limit);
        write_inflow_limit(&e,inflow_limit);
        write_quota_time_limit(&e,quota_time_limit);
      
    }

    fn preprocess_outflow(e: Env, 
        from: Address,
        to: Address,
        amount: i128,) -> bool  {

        is_invoker_the_asset_contract(&e);    
        has_spender_achieved_outflow_limit(&e, &from, amount);
        has_receiver_achieved_inflow_limit(&e, &to, amount);


        record_transaction(&e, from, amount, true);
        record_transaction(&e, to, amount, false);
        
        return true;
    }





    fn get_quota(e:Env, id: Address) -> Vec<i128>{

        let recent_user_inflow = read_user_quota(&e,&id,false);
        let recent_user_outflow = read_user_quota(&e,&id,true);

        vec![&e, recent_user_inflow, recent_user_outflow]

  }




    fn test(e: Env){
        
        is_invoker_the_asset_contract(&e);
        
    }



}
