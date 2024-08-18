use soroban_sdk::{contracttype, symbol_short, Address, Env, Symbol};

const PROBATION_START: Symbol = symbol_short!("prob_strt");
const CONSUMED_QUOTA_OUT: Symbol = symbol_short!("use_out_q");
const CONSUMED_QUOTA_IN: Symbol = symbol_short!("use_in_q");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct ConsumedQuotaEventData {
    pub account: Address,
    pub timestamp: u64,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct ProbationStartEventData {
    pub account: Address,
    pub timestamp: u64,
}

pub(crate) fn event_consumed_quota_out(e: &Env, id: Address, amount: i128) {
    e.events().publish(
        (CONSUMED_QUOTA_OUT,),
        ConsumedQuotaEventData {
            account: id,
            timestamp: e.ledger().timestamp(),
            amount,
        },
    );
}

pub(crate) fn event_consumed_quota_in(e: &Env, id: Address, amount: i128) {
    e.events().publish(
        (CONSUMED_QUOTA_IN,),
        ConsumedQuotaEventData {
            account: id,
            timestamp: e.ledger().timestamp(),
            amount,
        },
    );
}

pub(crate) fn event_probation_start(e: &Env, id: Address) {
    e.events().publish(
        (PROBATION_START,),
        ProbationStartEventData {
            account: id,
            timestamp: e.ledger().timestamp(),
        },
    );
}
