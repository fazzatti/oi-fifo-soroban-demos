use soroban_sdk::{ Address, Env, Vec, vec};
use crate::storage_types::{DataKey, UserActivityData, ActivityEntry, BALANCE_BUMP_AMOUNT,TEMPORARY_BUMP_AMOUNT};


///should add the BUMPS?
/// persistent vs instance vs temporary
/// 

pub fn read_outflow_limit(e: &Env) -> i128 {
    let key = DataKey::OutflowLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_outflow_limit(e: &Env, amount: i128) {
    let key = DataKey::OutflowLimit;
    e.storage().instance().set(&key, &amount);
}
pub fn read_inflow_limit(e: &Env) -> i128 {
    let key = DataKey::InflowLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_inflow_limit(e: &Env, amount: i128) {
    let key = DataKey::InflowLimit;
    e.storage().instance().set(&key, &amount);
}
pub fn read_quota_time_limit(e: &Env) -> u64 {
    let key = DataKey::QuotaTimeLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_quota_time_limit(e: &Env, amount: u64) {
    let key = DataKey::QuotaTimeLimit;
    e.storage().instance().set(&key, &amount);
}



pub fn read_user_activity(e: &Env, user: &Address) -> UserActivityData {
    let key = &DataKey::UserActivity(user.clone());
    if let Some(user_data) = e.storage().temporary().get::<_, UserActivityData>(key) {
        user_data
    } else {
        UserActivityData {
            inflow:  vec![&e],
            outflow: vec![&e],
        }
    }
}

pub fn write_user_activity(e: &Env, user: Address, user_activity: UserActivityData) {
    let key = DataKey::UserActivity(user);
    e.storage().temporary().set(&key, &user_activity);  
    e.storage().temporary().bump(&key, TEMPORARY_BUMP_AMOUNT);
}


pub fn read_user_quota(e: &Env, user: &Address, is_outflow: bool) -> i128 {

    let user_activity= read_user_activity(e,&user);

    let recorded_flow = if is_outflow {
        user_activity.outflow
    } else {
        user_activity.inflow
    };

    let recent_activity = get_quota_consumed_for_period(e,recorded_flow);

    let mut total_consumed_quota = 0;

    for tx in recent_activity.iter() {
        total_consumed_quota += tx.amount;
        
    }

    total_consumed_quota
}




// pub fn read_user_inflow_quota(e: &Env, user: Address) -> i128 {
//     let key = &DataKey::UserActivity(user);
//     if let Some(user_data) = e.storage().instance().get::<_, UserActivityData>(key) {
//         e.storage().persistent().bump(key, BALANCE_BUMP_AMOUNT);
//         user_data.recent_inflow
//     } else {
//         0
//     }
// }


pub fn record_transaction(e: &Env, user: Address, amount: i128, is_outflow: bool) {

    let recorded_user_activity= read_user_activity(&e,&user);
    let mut recent_outflow = get_quota_consumed_for_period(&e,recorded_user_activity.outflow);
    let mut recent_inflow = get_quota_consumed_for_period(&e,recorded_user_activity.inflow);

    if is_outflow {
        recent_outflow.push_back(ActivityEntry {
            amount: amount,
            timestamp: e.ledger().timestamp(),
        });
    }
    else{
        recent_inflow.push_back(ActivityEntry {
            amount: amount,
            timestamp: e.ledger().timestamp(),
        });

    }

    let user_activity = UserActivityData {
        inflow:  recent_inflow,
        outflow: recent_outflow,
    };

    write_user_activity(&e, user, user_activity);
}



fn get_quota_consumed_for_period(e:&Env, activity: Vec<ActivityEntry>) -> Vec<ActivityEntry>{
    let mut filtered_activity = vec![&e];

    let quota_time_limit = read_quota_time_limit(&e);
    for entry in activity.iter() {


        if (e.ledger().timestamp() - entry.timestamp) > quota_time_limit {
            filtered_activity.push_back(ActivityEntry {
                amount: entry.amount,
                timestamp: entry.timestamp,
            });
        }
    }

    filtered_activity
}