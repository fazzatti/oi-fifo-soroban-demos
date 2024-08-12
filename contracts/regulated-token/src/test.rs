#![cfg(test)]
extern crate std;

use crate::contract::RegulatedAssetClient;

// use crate::{contract::Token, TokenClient};
use soroban_sdk::{
    testutils::{Address as TestAddress, Ledger},
    Address, Env, String,
};

mod asset_controller_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/regulated_token_controller.wasm"
    );
}

fn advance_ledger(e: &Env, delta: u64) {
    e.ledger().with_mut(|l| {
        l.timestamp += delta;
    });
}

fn initialize_use_cases<'a>() -> (
    Env,
    RegulatedAssetClient<'a>,
    asset_controller_contract::Client<'a>,
    Address,
    Address,
    Address,
    Address,
) {
    // INITIALIZATION
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user_a = Address::generate(&e);
    let user_b = Address::generate(&e);
    let user_c = Address::generate(&e);

    // Deploys Asset Controller and Regulated Asset
    let asset_controler_id = e.register_contract_wasm(None, asset_controller_contract::WASM);
    let ac_client = asset_controller_contract::Client::new(&e, &asset_controler_id);

    let regulated_asset_id = e.register_contract(None, crate::RegulatedAsset {});
    let ra_client = RegulatedAssetClient::new(&e, &regulated_asset_id);

    // Regulated Asset Parameters
    let decimal: u32 = 7;
    let name = String::from_str(&e, "Fifocoin");
    let symbol = String::from_str(&e, "Fifo");

    // Asset Controller Parameters
    let probation_period: u64 = 60_000;
    let outflow_limit: i128 = 1000;
    let inflow_limit: i128 = 700;
    let quota_time_limit: u64 = 100;

    // Initializes AC and RA with crossed reference.
    ac_client.initialize(
        &admin,
        &ra_client.address,
        &probation_period,
        &quota_time_limit,
        &inflow_limit,
        &outflow_limit,
    );
    ra_client.initialize(&admin, &decimal, &name, &symbol, &ac_client.address);

    // ra_client.mint(&user_a, &10_000);
    // ra_client.mint(&user_b, &10_000);
    // ra_client.mint(&user_c, &10_000);

    e.budget().reset_default();

    // Needs to start higher than 0 due to the probation
    // period, which considers 0 as not in probation
    advance_ledger(&e, 1);

    (e, ra_client, ac_client, admin, user_a, user_b, user_c)
}

#[test]
fn test() {
    let (_e, ra_client, _ac_client, _admin, user_a, user_b, _user_c) = initialize_use_cases();

    ra_client.mint(&user_a, &1000);
    assert_eq!(ra_client.balance(&user_a), 1000);

    ra_client.approve(&user_a, &user_b, &500, &200);
    assert_eq!(ra_client.allowance(&user_a, &user_b), 500);

    ra_client.transfer(&user_a, &user_b, &600);
    assert_eq!(ra_client.balance(&user_a), 400);
    assert_eq!(ra_client.balance(&user_b), 600);
}
