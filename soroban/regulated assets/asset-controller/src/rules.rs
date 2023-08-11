
use soroban_sdk::{Address, Env};
use crate::data::{read_outflow_limit,read_user_outflow};


pub fn has_spender_achieved_outflow_limit(e:&Env, spender: Address, amount :i128){

    let outflow_limit = read_outflow_limit(&e);
    let recent_user_outflow = read_user_outflow_quota(&e,spender);

    if (recent_user_outflow + amount) > outflow_limit{
        panic!("Spender exceeded the outflow quota.");
    }

}


pub fn has_receiver_achieved_inflow_limit(e:&Env, receiver: Address, amount :i128){

    let inflow_limit = read_inflow_limit(&e);
    let recent_user_inflow = read_user_inflow_quota(&e,receiver);

    if (recent_user_inflow + amount) > inflow_limit{
        panic!("Receiver exceeded the inflow quota.");
    }

}