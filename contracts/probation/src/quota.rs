use crate::data::{read_account_activity, read_quota_time_limit, write_account_activity};
use crate::storage_types::{AccountActivityData, AccountQuotaReleaseData, TxEntry, TxReleaseEntry};
use soroban_sdk::{vec, Address, Env, Vec};

pub fn read_account_quota(e: &Env, id: &Address, is_outflow: bool) -> i128 {
    let account_activity = read_account_activity(e, &id);

    let recorded_flow = if is_outflow {
        account_activity.outflow
    } else {
        account_activity.inflow
    };

    let recent_activity = get_quota_consumed_for_period(e, recorded_flow);

    let mut total_consumed_quota = 0;

    for tx in recent_activity.iter() {
        total_consumed_quota += tx.amount;
    }

    total_consumed_quota
}

pub fn get_account_quota_release(e: &Env, id: &Address) -> AccountQuotaReleaseData {
    let account_activity = read_account_activity(e, &id);

    let recent_inflow_activity = get_quota_consumed_for_period(e, account_activity.inflow);
    let recent_outflow_activity = get_quota_consumed_for_period(e, account_activity.outflow);

    let inflow_quota_release = convert_acount_activity_into_time_left(e, recent_inflow_activity);
    let outflow_quota_release = convert_acount_activity_into_time_left(e, recent_outflow_activity);

    AccountQuotaReleaseData {
        inflow: inflow_quota_release,
        outflow: outflow_quota_release,
    }
}

pub fn record_transaction(e: &Env, id: Address, amount: i128, is_outflow: bool) {
    let recorded_account_activity = read_account_activity(&e, &id);
    let mut recent_outflow = get_quota_consumed_for_period(&e, recorded_account_activity.outflow);
    let mut recent_inflow = get_quota_consumed_for_period(&e, recorded_account_activity.inflow);

    if is_outflow {
        recent_outflow.push_back(TxEntry {
            amount: amount,
            timestamp: e.ledger().timestamp(),
        });
    } else {
        recent_inflow.push_back(TxEntry {
            amount: amount,
            timestamp: e.ledger().timestamp(),
        });
    }

    let account_activity = AccountActivityData {
        inflow: recent_inflow,
        outflow: recent_outflow,
    };

    write_account_activity(&e, id, account_activity);
}

pub fn clear_recorded_transactions(e: &Env, id: Address) {
    let account_activity = AccountActivityData {
        inflow: vec![&e],
        outflow: vec![&e],
    };

    write_account_activity(&e, id, account_activity);
}

fn get_quota_consumed_for_period(e: &Env, activity: Vec<TxEntry>) -> Vec<TxEntry> {
    let mut filtered_activity = vec![&e];

    let quota_time_limit = read_quota_time_limit(&e);
    for tx in activity.iter() {
        if (e.ledger().timestamp() - tx.timestamp) <= quota_time_limit {
            filtered_activity.push_back(TxEntry {
                amount: tx.amount,
                timestamp: tx.timestamp,
            });
        }
    }

    filtered_activity
}

fn convert_acount_activity_into_time_left(e: &Env, activity: Vec<TxEntry>) -> Vec<TxReleaseEntry> {
    let mut converted_activity = vec![&e];

    let quota_time_limit = read_quota_time_limit(&e);
    for tx in activity.iter() {
        let time_left = quota_time_limit - (e.ledger().timestamp() - tx.timestamp);
        converted_activity.push_back(TxReleaseEntry {
            amount: tx.amount,
            time_left: time_left,
        });
    }

    converted_activity
}
