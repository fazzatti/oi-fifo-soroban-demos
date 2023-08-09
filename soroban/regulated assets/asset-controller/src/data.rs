

pub fn read_outflow_limit(e: &Env) -> Address {
    let key = DataKey::OutflowLimit;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_outflow_limit(e: &Env, id: &Address) {
    let key = DataKey::OutflowLimit;
    e.storage().instance().set(&key, id);
}

pub fn read_user_outflow(e: &Env, user: &Address) {
    let key = DataKey::UserActivity(user);
    if let Some(outflow) = e.storage().instance().get::<DataKey, Address>(&key) {
        e.storage().persistent().bump(&key, BALANCE_BUMP_AMOUNT);
        recent_outflow
    } else {
        0
    }
}


///BUMPS?????