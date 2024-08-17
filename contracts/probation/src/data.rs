use crate::storage_types::{
    AccountActivityData, DataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_BUMP_THREASHOLD,
};
use soroban_sdk::{vec, Address, Env};

const LEDGER_TIME_SECONDS: u32 = 5;

pub fn read_outflow_limit(e: &Env) -> i128 {
    let key = DataKey::OutflowLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_outflow_limit(e: &Env, amount: i128) {
    let key = DataKey::OutflowLimit;
    e.storage().instance().set(&key, &amount);
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_THREASHOLD, INSTANCE_BUMP_AMOUNT);
}
pub fn read_inflow_limit(e: &Env) -> i128 {
    let key = DataKey::InflowLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_inflow_limit(e: &Env, amount: i128) {
    let key = DataKey::InflowLimit;
    e.storage().instance().set(&key, &amount);
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_THREASHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn read_probation_period(e: &Env) -> u64 {
    let key = DataKey::ProbationPeriod;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_probation_period(e: &Env, amount: u64) {
    let key = DataKey::ProbationPeriod;
    e.storage().instance().set(&key, &amount);
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_THREASHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn read_quota_time_limit(e: &Env) -> u64 {
    let key = DataKey::QuotaTimeLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_quota_time_limit(e: &Env, amount: u64) {
    let key = DataKey::QuotaTimeLimit;
    e.storage().instance().set(&key, &amount);
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_THREASHOLD, INSTANCE_BUMP_AMOUNT);
}

// The account probation defines how long an account still
// have remaining as their probation period. Once they begin
// transacting, their probation time will update. Prior to that
// it is always the full probation period.
pub fn read_account_probation_start(e: &Env, id: &Address) -> u64 {
    let key = DataKey::AccountProbationStart(id.clone());
    if let Some(account_probation) = e.storage().instance().get::<DataKey, u64>(&key) {
        account_probation
    } else {
        e.ledger().timestamp()
    }
}

pub fn write_account_probation_start(e: &Env, id: &Address, start: u64) {
    let key = DataKey::AccountProbationStart(id.clone());
    e.storage().instance().set(&key, &start);
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_BUMP_THREASHOLD, INSTANCE_BUMP_AMOUNT);
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
            inflow: vec![&e],
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
    let bump_amount = get_temporary_bump_amount(&e);
    let bump_threashold = bump_amount / 2;
    e.storage()
        .temporary()
        .extend_ttl(&key, bump_threashold, bump_amount);
}

fn get_temporary_bump_amount(e: &Env) -> u32 {
    read_quota_time_limit(&e) as u32 / LEDGER_TIME_SECONDS
}
