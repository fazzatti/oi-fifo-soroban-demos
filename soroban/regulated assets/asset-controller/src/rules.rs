
use soroban_sdk::{Address, Env};
use crate::data::{read_outflow_limit, read_inflow_limit};
use crate::quota::read_account_quota;


pub fn has_spender_achieved_outflow_limit(e:&Env, spender: &Address, amount :i128){

    let outflow_limit = read_outflow_limit(&e);
    let recent_user_outflow = read_account_quota(&e,&spender,true);

    if (recent_user_outflow + amount) > outflow_limit{
        panic!("Spender exceeded the outflow quota.");
    }

}


pub fn has_receiver_achieved_inflow_limit(e:&Env, receiver: &Address, amount :i128){

    let inflow_limit = read_inflow_limit(&e);
    let recent_user_inflow = read_account_quota(&e,&receiver,false);

    if (recent_user_inflow + amount) > inflow_limit{
        panic!("Receiver exceeded the inflow quota.");
    }

}