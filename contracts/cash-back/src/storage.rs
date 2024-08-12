pub enum DataKey {
    Admin,          //Address
    Asset,          //Address
    TargetAmount,   //i128
    CashBackAmount, //i128
    EndDate,        //u64
}

pub fn write_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(DataKey::Admin, admin);
}

pub fn read_admin(e: &Env) {
    e.storage().instance().get(DataKey::Admin).unwrap()
}

pub fn write_asset(e: &Env, asset: &Address) {
    e.storage().instance().set(DataKey::Asset, asset);
}

pub fn read_asset(e: &Env) {
    e.storage().instance().get(DataKey::Asset).unwrap()
}

pub fn write_target_amount(e: &Env, target_amount: &i128) {
    e.storage()
        .instance()
        .set(DataKey::TargetAmount, target_amount);
}

pub fn read_target_amount(e: &Env) {
    e.storage().instance().get(DataKey::TargetAmount).unwrap()
}

pub fn write_cash_back_amount(e: &Env, cash_back_amount: &i128) {
    e.storage()
        .instance()
        .set(DataKey::CashBackAmount, cash_back_amount);
}

pub fn read_cash_back_amount(e: &Env) {
    e.storage().instance().get(DataKey::CashBackAmount).unwrap()
}

pub fn write_end_date(e: &Env, end_date: &u64) {
    e.storage().instance().set(DataKey::EndDate, end_date);
}

pub fn read_end_date(e: &Env) {
    e.storage().instance().get(DataKey::EndDate).unwrap()
}
