use crate::admin::read_administrator;
use crate::asset::read_asset;
use crate::data::{
    read_account_probation_start, read_inflow_limit, read_outflow_limit, read_probation_period,
    write_account_probation_start,
};
use crate::events::event_probation_start;
use crate::quota::read_account_quota;
use soroban_sdk::{token, Address, Env};

pub fn has_spender_achieved_outflow_limit(e: &Env, spender: &Address, amount: i128) {
    let outflow_limit = read_outflow_limit(&e);
    let recent_user_outflow = read_account_quota(&e, &spender, true);

    if (recent_user_outflow + amount) > outflow_limit {
        panic!("Spender exceeded the outflow quota.");
    }
}

pub fn has_receiver_achieved_inflow_limit(e: &Env, receiver: &Address, amount: i128) {
    let inflow_limit = read_inflow_limit(&e);
    let recent_user_inflow = read_account_quota(&e, &receiver, false);

    if (recent_user_inflow + amount) > inflow_limit {
        panic!("Receiver exceeded the inflow quota.");
    }
}

//
// On the account's first interaction with the asset,
// we start its probation period. Once complete, we set
// its start time to '0', removing any further limits.
//
pub fn is_account_in_probation(e: &Env, id: &Address) -> bool {
    let probation_start_time = read_account_probation_start(&e, id);

    //Admin doesnt have probation
    if read_administrator(e) == *id {
        return false;
    }

    // if the trustline is approved, return false.
    let asset_contract = read_asset(e);
    let asset_client = token::StellarAssetClient::new(&e, &asset_contract);
    if asset_client.authorized(id) == true {
        return false;
    }

    // If the account probation has ended, return false.
    if probation_start_time == 0 {
        return false;
    }

    let time_since_probation_start = e.ledger().timestamp() - probation_start_time;
    let probation_duration = read_probation_period(&e);

    // Check if the probation has just started.
    if time_since_probation_start == 0 {
        write_account_probation_start(&e, &id, probation_start_time);
        event_probation_start(&e, id.clone());
        return true;
    }

    // Check if the probation period is still ongoing.
    if time_since_probation_start < probation_duration {
        return true;
    }

    // If we reach here, the probation period has ended since last checked.
    write_account_probation_start(&e, &id, 0);
    return false;
}
