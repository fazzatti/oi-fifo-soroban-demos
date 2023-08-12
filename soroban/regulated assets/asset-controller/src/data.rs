use soroban_sdk::{ Address, Env, vec};
use crate::storage_types::{DataKey, AccountActivityData, TxEntry};


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

//
// This function attempts to read the account recorded activity,
// returning an empty array when not found. Using a temporary
// storage type here, ensures the data automaticaly expires after
// a while if there is no activity recorded.
//
pub fn read_account_activity(e: &Env, id: &Address) -> AccountActivityData {
    let key = &DataKey::AccountActivity(id.clone());
    if let Some(account_data) = e.storage().temporary().get::<_, AccountActivityData>(key) {
        account_data
    } else {
        AccountActivityData {
            inflow:  vec![&e],
            outflow: vec![&e],
        }
    }
}

//
// Since the temporary data only needs to exist in relation
// to the quota time limit defined when initializing the 
// contract, here we use it as the bump value to ensure
// the temporary data doesn't have to live any longer than
// necessary. Optimizing the contract cost to store data.
//
pub fn write_account_activity(e: &Env, id: Address, account_activity: AccountActivityData) {
    let key = DataKey::AccountActivity(id);
    e.storage().temporary().set(&key, &account_activity);  
    e.storage().temporary().bump(&key, get_temporary_bump_amount(&e));
}

fn get_temporary_bump_amount(e:&Env) -> u32{
    read_quota_time_limit(&e) as u32
}