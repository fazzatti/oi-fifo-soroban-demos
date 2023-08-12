
use soroban_sdk::{contract, contractimpl, Address, Env, Vec,vec};
use crate::rules::{has_spender_achieved_outflow_limit, has_receiver_achieved_inflow_limit};
use crate::asset::{write_asset,read_asset};
use crate::data::{write_outflow_limit,write_inflow_limit, write_quota_time_limit, read_outflow_limit,read_inflow_limit,read_quota_time_limit};
use crate::storage_types::AccountQuotaReleaseData;
use crate::quota::{read_account_quota,record_transaction,get_account_quota_release};
use crate::admin::{has_administrator,write_administrator,read_administrator};
use crate::validations::is_invoker_the_asset_contract;


pub trait AssetControllerTrait {

    fn initialize(e: Env, asset: Address, admin: Address, outflow_limit: i128, inflow_limit: i128, quota_time_limit: u64 );
    
    
    // --------------------------------------------------------------------------------
    // Transaction Auditing Functions
    // --------------------------------------------------------------------------------
    // These transactions apply the validation rules to the
    // transactions triggered by the regulated asset contract.
    // They can only be invoked by the registered asset contract
    // which can be verified with function `is_invoker_the_asset_contract()`
    
    // Process a simple transfer transaction
    // enforcing inflow and outflow rules
    // 
    fn review_transfer(env: Env, 
        from: Address,
        to: Address,
        amount: i128,) -> bool;
    



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


    // --------------------------------------------------------------------------------
    // Read-only
    // --------------------------------------------------------------------------------
    // The functions here don't need any authorization and don't emit any events.

    fn get_quota_time_limit(e:Env) -> u64;

    fn get_inflow_limit(e:Env) -> i128;

    fn get_outflow_limit(e:Env) -> i128;

    fn get_asset(e:Env) -> Address;

    fn get_admin(e:Env) -> Address;

    // returns the current state of the allocatted
    // quota for a given accound address in a vec
    // as this: [<consumed inflow quota>, <consumed outlow quota>]
    fn get_quota(e:Env, id: Address) -> Vec<i128>;

    fn get_quota_release_time(e:Env, id: Address) -> AccountQuotaReleaseData;

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

    fn review_transfer(e: Env, 
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

        let recent_account_inflow = read_account_quota(&e,&id,false);
        let recent_account_outflow = read_account_quota(&e,&id,true);

        vec![&e, recent_account_inflow, recent_account_outflow]

    }

    fn get_quota_release_time(e:Env, id: Address) -> AccountQuotaReleaseData {

        get_account_quota_release(&e,&id)
    }
    
    fn get_quota_time_limit(e:Env) -> u64{
        read_quota_time_limit(&e)
    }

    fn get_inflow_limit(e:Env) -> i128{
        read_inflow_limit(&e)
    }

    fn get_outflow_limit(e:Env) -> i128{
        read_outflow_limit(&e)
    }

    fn get_asset(e:Env) -> Address{
        read_asset(&e)
    }   

    fn get_admin(e:Env) -> Address{
        read_administrator(&e)
    }


}
