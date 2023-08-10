use soroban_sdk::{ Address, Env};
use crate::storage_types::{DataKey, UserActivityData, BALANCE_BUMP_AMOUNT};


///should add the BUMPS?

pub fn read_outflow_limit(e: &Env) -> i128 {
    let key = DataKey::OutflowLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_outflow_limit(e: &Env, amount: i128) {
    let key = DataKey::OutflowLimit;
    e.storage().instance().set(&key, &amount);
}

pub fn read_user_outflow(e: &Env, user: Address) -> i128 {
    let key = &DataKey::UserActivity(user);
    if let Some(user_data) = e.storage().instance().get::<_, UserActivityData>(key) {
        e.storage().persistent().bump(key, BALANCE_BUMP_AMOUNT);
        user_data.recent_outflow
    } else {
        0
    }
}

