#![cfg(test)]
extern crate std;

use crate::contract::{self, AssetControllerClient};
use standard_traits::classic_wrapper::enforced::{
    ClassicWrapperClient, EnforcedClassicWrapperInterfaceTrait,
};

use soroban_sdk::{
    contractimport,
    testutils::{Address as _, Ledger},
    token::{self, TokenClient},
    Address, Env,
};

//
// =================================================
// IMPORTANT
// =================================================
// This contract cannot be properly tested along with the
// Classic Asset Wrapper contract, as the use case requires
// that the Classic Asset is set with specific authorization
// flags such as Revocable and Authorization Required, which
// currently are not supported by the token utils in the SDK.
//
//

mod wrapper_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/enforced_classic_asset_wrapper.wasm"
    );
}

fn advance_ledger(e: &Env, delta: u64) {
    e.ledger().with_mut(|l| {
        l.timestamp += delta;
    });
}

fn initialize_use_cases<'a>() -> (
    Env,
    AssetControllerClient<'a>,
    wrapper_contract::Client<'a>,
    token::StellarAssetClient<'a>,
    TokenClient<'a>,
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
    let asset_controler_id = e.register_contract(None, crate::AssetController {});
    let ac_client = AssetControllerClient::new(&e, &asset_controler_id);

    let wrapper_id = e.register_contract_wasm(None, wrapper_contract::WASM);
    let wr_client = wrapper_contract::Client::new(&e, &wrapper_id);

    let asset_id = e.register_stellar_asset_contract(admin.clone());
    let sac_client = token::StellarAssetClient::new(&e, &asset_id);
    let a_client = token::TokenClient::new(&e, &asset_id);

    // Regulated Asset Parameters
    // let decimal: u32 = 7;
    // let name = String::from_str(&e, "Fifocoin");
    // let symbol = String::from_str(&e, "Fifo");

    // Asset Controller Parameters
    let probation_period: u64 = 60_000;
    let outflow_limit: i128 = 1000;
    let inflow_limit: i128 = 700;
    let quota_time_limit: u64 = 100;

    // Initializes AC and RA with crossed reference.
    ac_client.initialize(
        &admin,
        &wr_client.address,
        &a_client.address,
        &probation_period,
        &quota_time_limit,
        &inflow_limit,
        &outflow_limit,
    );

    wr_client.initialize(&admin, &sac_client.address, &ac_client.address);

    sac_client.mint(&user_a, &10_000);
    sac_client.mint(&user_b, &10_000);
    sac_client.mint(&user_c, &10_000);

    // sac_client.set_authorized(&user_a, &false);
    // sac_client.set_authorized(&user_b, &false);
    // sac_client.set_authorized(&user_c, &false);

    e.budget().reset_default();

    // Needs to start higher than 0 due to the probation
    // period, which considers 0 as not in probation
    advance_ledger(&e, 1);

    (
        e, ac_client, wr_client, sac_client, a_client, admin, user_a, user_b, user_c,
    )
}

#[test]
fn test() {
    let (_e, ac_client, _wr_client, _sac_client, a_client, admin, _user_a, _user_b, _user_c) =
        initialize_use_cases();

    //
    // Validate initialization
    //
    assert_eq!(ac_client.get_admin(), admin);
    assert_eq!(ac_client.get_asset(), a_client.address);
    assert_eq!(ac_client.get_outflow_limit(), 1000);
    assert_eq!(ac_client.get_inflow_limit(), 700);
    assert_eq!(ac_client.get_quota_time_limit(), 100);
    assert_eq!(ac_client.get_probation_period(), 60_000); //1 month

    //
    // Validate quota history with a series of transactions within limits.
    //
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 0]);
    // assert_eq!(ac_client.get_quota(&user_b), vec![&e, 0, 0]);

    // // A->B 100
    // wr_client.transfer(&user_a, &user_b, &100);
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 100]);
    // assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 0]);
    // e.budget().reset_default();

    // // B->C 50
    // ra_client.transfer(&user_b, &user_c, &50);
    // assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 50]);
    // assert_eq!(ac_client.get_quota(&user_c), vec![&e, 50, 0]);
    // e.budget().reset_default();

    // // C->A 50
    // ra_client.transfer(&user_c, &user_a, &50);
    // assert_eq!(ac_client.get_quota(&user_c), vec![&e, 50, 50]);
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 50, 100]);
    // e.budget().reset_default();

    // // A->C 225
    // ra_client.transfer(&user_a, &user_c, &225);
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 50, 325]);
    // assert_eq!(ac_client.get_quota(&user_c), vec![&e, 275, 50]);
    // e.budget().reset_default();

    // // A->B 675
    // ra_client.transfer(&user_a, &user_b, &575);
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 50, 900]);
    // assert_eq!(ac_client.get_quota(&user_b), vec![&e, 675, 50]);
    // e.budget().reset_default();

    // // A->C 100
    // ra_client.transfer(&user_a, &user_c, &100);
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 50, 1000]);
    // assert_eq!(ac_client.get_quota(&user_c), vec![&e, 375, 50]);
    // e.budget().reset_default();

    // // C->B 25
    // ra_client.transfer(&user_c, &user_b, &25);
    // assert_eq!(ac_client.get_quota(&user_c), vec![&e, 375, 75]);
    // assert_eq!(ac_client.get_quota(&user_b), vec![&e, 700, 50]);
    // e.budget().reset_default();

    // //
    // // Test the quota reset through time limit
    // //
    // advance_ledger(&e, 100);
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 50, 1000]);
    // assert_eq!(ac_client.get_quota(&user_c), vec![&e, 375, 75]);
    // assert_eq!(ac_client.get_quota(&user_b), vec![&e, 700, 50]);
    // e.budget().reset_default();

    // advance_ledger(&e, 1);
    // assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 0]);
    // assert_eq!(ac_client.get_quota(&user_c), vec![&e, 0, 0]);
    // assert_eq!(ac_client.get_quota(&user_b), vec![&e, 0, 0]);
    // e.budget().reset_default();
}

// #[test]
// fn quota_updated_through_time() {
//     let (e, ac_client, ra_client, _admin, user_a, user_b, _user_c) = initialize_use_cases();

//     //
//     // Test the quota being updated as it drops
//     // older payments that have expired
//     //

//     // A->B 100
//     // ledger 0
//     ra_client.transfer(&user_a, &user_b, &100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 100]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 0]);
//     e.budget().reset_default();

//     // A->B 100
//     // ledger 10
//     advance_ledger(&e, 10);
//     ra_client.transfer(&user_a, &user_b, &100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 200]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 200, 0]);
//     e.budget().reset_default();

//     // A->B 100
//     // ledger 20
//     advance_ledger(&e, 10);
//     ra_client.transfer(&user_a, &user_b, &100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 300]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 300, 0]);
//     e.budget().reset_default();

//     // A->B 100
//     // ledger 30
//     advance_ledger(&e, 10);
//     ra_client.transfer(&user_a, &user_b, &100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 400]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 400, 0]);
//     e.budget().reset_default();

//     // A->B 100
//     // ledger 40
//     advance_ledger(&e, 10);
//     ra_client.transfer(&user_a, &user_b, &100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 500]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 500, 0]);
//     e.budget().reset_default();

//     // A->B 100
//     // ledger 50
//     advance_ledger(&e, 10);
//     ra_client.transfer(&user_a, &user_b, &100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 600]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 600, 0]);
//     e.budget().reset_default();

//     // ledger 100
//     advance_ledger(&e, 50);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 600]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 600, 0]);

//     // ledger 101
//     advance_ledger(&e, 1);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 500]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 500, 0]);

//     // ledger 110
//     advance_ledger(&e, 9);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 500]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 500, 0]);

//     // ledger 111
//     advance_ledger(&e, 1);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 400]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 400, 0]);

//     // ledger 121
//     advance_ledger(&e, 10);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 300]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 300, 0]);

//     // ledger 131
//     advance_ledger(&e, 10);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 200]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 200, 0]);

//     // ledger 141
//     advance_ledger(&e, 10);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 100]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 0]);

//     // ledger 150
//     advance_ledger(&e, 9);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 100]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 0]);

//     // A->B 100
//     ra_client.transfer(&user_a, &user_b, &50);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 150]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 150, 0]);
//     e.budget().reset_default();

//     // ledger 151
//     advance_ledger(&e, 1);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 50]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 50, 0]);

//     // ledger 251
//     advance_ledger(&e, 100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 0]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 0, 0]);
// }

// #[test]
// fn quota_time_left() {
//     let (e, ac_client, ra_client, _admin, user_a, user_b, _user_c) = initialize_use_cases();

//     //
//     // Test the time left in a quota
//     // for a user with no transaction.
//     // There should be no quota
//     //

//     let mut release_data = ac_client.get_quota_release_time(&user_a);
//     assert_eq!(release_data.inflow.len(), 0);
//     assert_eq!(release_data.outflow.len(), 0);

//     //
//     // After performing a tranaction
//     // both parties should have a quota
//     // timed for the duration of the limit
//     //
//     // A->B 100
//     // ledger 0
//     ra_client.transfer(&user_a, &user_b, &100);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 100]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 0]);
//     e.budget().reset_default();

//     release_data = ac_client.get_quota_release_time(&user_a);

//     // verify user A inflow quotas
//     assert_eq!(release_data.inflow.len(), 0);

//     // verify user A outflow quotas
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.first_unchecked();

//         assert_eq!(outflow_entry.amount, 100);
//         assert_eq!(outflow_entry.time_left, 100);
//     }

//     release_data = ac_client.get_quota_release_time(&user_b);

//     // verify user B inflow quotas
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.first_unchecked();

//         assert_eq!(inflow_entry.amount, 100);
//         assert_eq!(inflow_entry.time_left, 100);
//     }

//     // verify user B outflow quotas
//     assert_eq!(release_data.outflow.len(), 0);

//     e.budget().reset_default();

//     //
//     // After the time passes both parties
//     // should have their quotas updated
//     // with the time that passed
//     //
//     // ledger 40
//     advance_ledger(&e, 40);

//     release_data = ac_client.get_quota_release_time(&user_a);

//     // verify user A inflow quotas
//     assert_eq!(release_data.inflow.len(), 0);

//     // verify user A outflow quotas
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.first_unchecked();

//         assert_eq!(outflow_entry.amount, 100);
//         assert_eq!(outflow_entry.time_left, 60);
//     }

//     release_data = ac_client.get_quota_release_time(&user_b);

//     // verify user B inflow quotas
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.first_unchecked();

//         assert_eq!(inflow_entry.amount, 100);
//         assert_eq!(inflow_entry.time_left, 60);
//     }

//     // verify user B outflow quotas
//     assert_eq!(release_data.outflow.len(), 0);

//     //
//     // Introducing new transactions in oposing flow
//     // should afect the existing quota and its tracking
//     //
//     //
//     // B->A 50
//     // ledger 40
//     ra_client.transfer(&user_b, &user_a, &50);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 50, 100]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 50]);
//     e.budget().reset_default();

//     release_data = ac_client.get_quota_release_time(&user_a);

//     // verify user A inflow quotas
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.first_unchecked();

//         assert_eq!(inflow_entry.amount, 50);
//         assert_eq!(inflow_entry.time_left, 100);
//     }

//     // verify user A outflow quotas
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.first_unchecked();

//         assert_eq!(outflow_entry.amount, 100);
//         assert_eq!(outflow_entry.time_left, 60);
//     }

//     release_data = ac_client.get_quota_release_time(&user_b);

//     // verify user B inflow quotas
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.first_unchecked();

//         assert_eq!(inflow_entry.amount, 100);
//         assert_eq!(inflow_entry.time_left, 60);
//     }

//     // verify user B outflow quotas
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.first_unchecked();

//         assert_eq!(outflow_entry.amount, 50);
//         assert_eq!(outflow_entry.time_left, 100);
//     }

//     //
//     // Introducing new transactions in existing
//     // flows should afect the existing quota and
//     // populate the array
//     //
//     //
//     // B->A 50
//     // ledger 60
//     advance_ledger(&e, 20);

//     ra_client.transfer(&user_b, &user_a, &50);
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 100, 100]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 100, 100]);
//     e.budget().reset_default();

//     release_data = ac_client.get_quota_release_time(&user_a);

//     // verify user A inflow quotas (first)
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.get_unchecked(0);

//         assert_eq!(inflow_entry.amount, 50);
//         assert_eq!(inflow_entry.time_left, 80);
//     }

//     // verify user A inflow quotas (second)
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.get_unchecked(1);

//         assert_eq!(inflow_entry.amount, 50);
//         assert_eq!(inflow_entry.time_left, 100);
//     }

//     // verify user A outflow quotas
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.first_unchecked();

//         assert_eq!(outflow_entry.amount, 100);
//         assert_eq!(outflow_entry.time_left, 40);
//     }

//     release_data = ac_client.get_quota_release_time(&user_b);

//     // verify user B inflow quotas
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.first_unchecked();

//         assert_eq!(inflow_entry.amount, 100);
//         assert_eq!(inflow_entry.time_left, 40);
//     }

//     // verify user B outflow quotas(first)
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.get_unchecked(0);

//         assert_eq!(outflow_entry.amount, 50);
//         assert_eq!(outflow_entry.time_left, 80);
//     }

//     // verify user B outflow quotas (second)
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.get_unchecked(1);

//         assert_eq!(outflow_entry.amount, 50);
//         assert_eq!(outflow_entry.time_left, 100);
//     }

//     //
//     // As time goes by we drop the oldest transaction
//     // from the quota history and release quota
//     //
//     //
//     // ledger 110
//     advance_ledger(&e, 50);

//     e.budget().reset_default();

//     release_data = ac_client.get_quota_release_time(&user_a);

//     // verify user A inflow quotas (first)
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.get_unchecked(0);

//         assert_eq!(inflow_entry.amount, 50);
//         assert_eq!(inflow_entry.time_left, 30);
//     }

//     // verify user A inflow quotas (second)
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.get_unchecked(1);

//         assert_eq!(inflow_entry.amount, 50);
//         assert_eq!(inflow_entry.time_left, 50);
//     }

//     // verify user A outflow quotas
//     assert_eq!(release_data.outflow.len(), 0);

//     release_data = ac_client.get_quota_release_time(&user_b);

//     // verify user B inflow quotas
//     assert_eq!(release_data.inflow.len(), 0);

//     // verify user B outflow quotas (first)
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.get_unchecked(0);

//         assert_eq!(outflow_entry.amount, 50);
//         assert_eq!(outflow_entry.time_left, 30);
//     }

//     // verify user B outflow quotas (second)
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.get_unchecked(1);

//         assert_eq!(outflow_entry.amount, 50);
//         assert_eq!(outflow_entry.time_left, 50);
//     }

//     //
//     // As time goes by we drop the oldest transaction
//     // from the quota history again and release quota
//     //
//     //
//     // ledger 150
//     advance_ledger(&e, 40);

//     e.budget().reset_default();

//     release_data = ac_client.get_quota_release_time(&user_a);

//     // verify user A inflow quotas
//     if !release_data.inflow.is_empty() {
//         let inflow_entry = release_data.inflow.first_unchecked();

//         assert_eq!(inflow_entry.amount, 50);
//         assert_eq!(inflow_entry.time_left, 10);
//     }

//     // verify user A outflow quotas
//     assert_eq!(release_data.outflow.len(), 0);

//     release_data = ac_client.get_quota_release_time(&user_b);

//     // verify user B inflow quotas
//     assert_eq!(release_data.inflow.len(), 0);

//     // verify user B outflow quotas
//     if !release_data.outflow.is_empty() {
//         let outflow_entry = release_data.outflow.first_unchecked();

//         assert_eq!(outflow_entry.amount, 50);
//         assert_eq!(outflow_entry.time_left, 10);
//     }

//     //
//     // As time goes by we drop the last transactions
//     // from the quota history and fully reset the quota
//     //
//     //
//     // ledger 161
//     advance_ledger(&e, 11);

//     e.budget().reset_default();

//     release_data = ac_client.get_quota_release_time(&user_a);
//     // verify user A inflow quotas
//     assert_eq!(release_data.inflow.len(), 0);
//     // verify user A outflow quotas
//     assert_eq!(release_data.outflow.len(), 0);

//     release_data = ac_client.get_quota_release_time(&user_b);
//     // verify user B inflow quotas
//     assert_eq!(release_data.inflow.len(), 0);

//     // verify user B outflow quotas
//     assert_eq!(release_data.outflow.len(), 0);

//     //verify quota total
//     assert_eq!(ac_client.get_quota(&user_a), vec![&e, 0, 0]);
//     assert_eq!(ac_client.get_quota(&user_b), vec![&e, 0, 0]);
// }

// #[test]
// fn account_probation_period() {
//     let (e, ac_client, ra_client, _admin, user_a, user_b, user_c) = initialize_use_cases();

//     //
//     // Test the initial probation period
//     // being default before any account activities
//     //
//     assert_eq!(ac_client.get_account_probation_period(&user_a), 60_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_b), 60_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_c), 60_000);

//     advance_ledger(&e, 1_000);

//     assert_eq!(ac_client.get_account_probation_period(&user_a), 60_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_b), 60_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_c), 60_000);

//     advance_ledger(&e, 500);

//     assert_eq!(ac_client.get_account_probation_period(&user_a), 60_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_b), 60_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_c), 60_000);

//     //
//     // Test the probation perido being updated
//     // for accounts that had activity
//     //
//     // A->B 100
//     ra_client.transfer(&user_a, &user_b, &100);
//     e.budget().reset_default();

//     advance_ledger(&e, 10_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_a), 50_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_b), 50_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_c), 60_000);

//     //
//     // Test the probation perido being updated
//     // for accounts that had further activity
//     //
//     // B->C 200
//     ra_client.transfer(&user_b, &user_c, &200);

//     advance_ledger(&e, 5_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_a), 45_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_b), 45_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_c), 55_000);

//     //
//     // Test the probation perido ending and
//     // accounts being free to transact
//     //
//     advance_ledger(&e, 45_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_a), 0);
//     assert_eq!(ac_client.get_account_probation_period(&user_b), 0);
//     assert_eq!(ac_client.get_account_probation_period(&user_c), 10_000);

//     ra_client.transfer(&user_a, &user_b, &2000);

//     advance_ledger(&e, 10_000);
//     assert_eq!(ac_client.get_account_probation_period(&user_a), 0);
//     assert_eq!(ac_client.get_account_probation_period(&user_b), 0);
//     assert_eq!(ac_client.get_account_probation_period(&user_c), 0);

//     ra_client.transfer(&user_b, &user_c, &2000);
//     e.budget().reset_default();
// }

// #[test]
// #[should_panic]
// fn avoid_direct_invocation() {
//     let (e, ac_client, ra_client, _admin, user_a, user_b, user_c) = initialize_use_cases();

//     e.set_auths(&[]);

//     ac_client.review_transfer(&user_a, &user_b, &100);
// }
