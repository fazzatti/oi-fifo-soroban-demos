
use soroban_sdk::{Address, Env};
use crate::data::{read_outflow_limit, read_inflow_limit, read_account_probation_start, read_probation_period, write_account_probation_start};
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


// If an account has passed through its probation time
// the start timestamp is set as 0 and it doesn't
// have to face the probation limits again
pub fn is_account_in_probation(e:&Env, id: &Address)-> bool{

    let account_probation_start = read_account_probation_start(&e, id);
    if account_probation_start == 0 {
        return false;
    }

    let probation_period = read_probation_period(&e);
    if probation_period > ( e.ledger().timestamp() - account_probation_start)  {
        
        write_account_probation_start(&e, &id, account_probation_start);
        return true;
    }
    else{
        write_account_probation_start(&e, &id, 0);
        return false;
    }
}