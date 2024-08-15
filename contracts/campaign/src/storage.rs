use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,             //Address
    Asset,             //Address
    Wrapper,           //Address
    InflowPoints,      //i128
    OutflowPoints,     //i128
    TargetPoints,      //i128
    PrizeAmount,       //i128
    WaitInterval,      //u64
    EndDate,           //u64
    UserData(Address), //UserData
}

#[derive(Clone)]
#[contracttype]
pub struct UserData {
    pub points: i128,
    pub wait_until: u64,
}

pub fn write_user_data(e: &Env, user: Address, data: &UserData) {
    e.storage().instance().set(&DataKey::UserData(user), data);
}

pub fn read_user_data(e: &Env, user: Address) -> UserData {
    e.storage()
        .instance()
        .get(&DataKey::UserData(user))
        .unwrap_or(UserData {
            points: 0,
            wait_until: 0,
        })
}

pub fn write_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, &admin);
}

pub fn read_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn write_wrapper(e: &Env, wrapper: &Address) {
    e.storage().instance().set(&DataKey::Wrapper, &wrapper);
}

pub fn read_wrapper(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Wrapper).unwrap()
}

pub fn write_asset(e: &Env, asset: &Address) {
    e.storage().instance().set(&DataKey::Asset, asset);
}

pub fn read_asset(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Asset).unwrap()
}

pub fn write_target_points(e: &Env, target_points: &i128) {
    e.storage()
        .instance()
        .set(&DataKey::TargetPoints, target_points);
}

pub fn read_target_points(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::TargetPoints).unwrap()
}

pub fn write_prize_amount(e: &Env, prize_amount: &i128) {
    e.storage()
        .instance()
        .set(&DataKey::PrizeAmount, prize_amount);
}

pub fn read_prize_amount(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::PrizeAmount).unwrap()
}

pub fn write_inflow_points(e: &Env, inflow_points: &i128) {
    e.storage()
        .instance()
        .set(&DataKey::InflowPoints, inflow_points);
}

pub fn read_inflow_points(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::InflowPoints).unwrap()
}

pub fn write_outflow_points(e: &Env, outflow_points: &i128) {
    e.storage()
        .instance()
        .set(&DataKey::OutflowPoints, outflow_points);
}

pub fn read_outflow_points(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::OutflowPoints).unwrap()
}

pub fn write_wait_interval(e: &Env, wait_interval: &u64) {
    e.storage()
        .instance()
        .set(&DataKey::WaitInterval, wait_interval);
}

pub fn read_wait_interval(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::WaitInterval).unwrap()
}

pub fn write_end_date(e: &Env, end_date: &u64) {
    e.storage().instance().set(&DataKey::EndDate, end_date);
}

pub fn read_end_date(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::EndDate).unwrap()
}
