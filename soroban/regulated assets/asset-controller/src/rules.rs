
use crate::data::read_outflow_limit;


pub fn hasSpenderAchievedOutflowLimit(e:Env, spender: Address, amount :i128)->bool{

    let outflow_limit = read_outflow_limit(&e);
    let recent_user_outflow = read_user_outflow(&e,&spender);

    if (recent_user_outflow + amount) > outflow_limit{
        panic!("User exceeded the outflow limit.");
    }

};