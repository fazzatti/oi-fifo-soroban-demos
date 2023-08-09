
use crate::rules::hasSpenderAchievedOutflowLimit;


pub trait AssetControllerTrait {

    fn initialize(e: Env, asset: Address, admin: Address, outflow_limit: i128 );
    // fn inflow();
    // fn delegated_inflow();

    //Transfer goin out
    fn preprocess_outflow(env: soroban_sdk::Env, 
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,) -> bool;
    // fn allow();

    //Transfer from goin out
    fn preprocess_approved_outflow( env: soroban_sdk::Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,) -> bool;

    //Approve an account (allowance)
    fn approve(env: soroban_sdk::Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,);

    // burn
    // delegated burn
    // 
    // mint

    // METHOD TO SET APPROVAL OF THE USER after probation
}

#[contract]
pub struct AssetController;

#[contractimpl]
impl AssetControllerTrait for AssetController {

    fn initialize(e: Env, admin: Address, asset5: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        write_asset(&e, &asset);
        write_outflow_limit(&e,&outflow_limit);
      
    }

    fn preprocess_outflow(e: soroban_sdk::Env, 
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,) -> bool  {


        //make sure invoker is spender
        hasSpenderAchievedOutflowLimit(&e, &spender,&amount);



        return true;
    }

}
