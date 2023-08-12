#![cfg(test)]
extern crate std;
use super::*;

use crate::contract::AssetControllerClient;


use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger, budget},
    Address, BytesN, Env, IntoVal, String, vec
    
};


mod asset_contract {
    soroban_sdk::contractimport!(file = "../regulated-asset/target/wasm32-unknown-unknown/release/regulated_asset.wasm");
}

fn advance_ledger(e: &Env, delta: u64) {
    e.ledger().with_mut(|l| {
        l.timestamp += delta;
    });
}

fn initialize_use_cases<'a>() -> (Env, AssetControllerClient<'a>, asset_contract::Client<'a>, Address, Address, Address, Address, Address) {
    // INITIALIZATION
    let e = Env::default();
    e.mock_all_auths();

    let mut admin = Address::random(&e);
    let mut user_a = Address::random(&e);
    let mut user_b = Address::random(&e);
    let mut user_c = Address::random(&e);
    let mut user_d = Address::random(&e);

    // Deploys Asset Controller and Regulated Asset
    let asset_controler_id = e.register_contract(None, crate::AssetController {});
    let ac_client = AssetControllerClient::new(&e, &asset_controler_id);

    let regulated_asset_id = e.register_contract_wasm(None, asset_contract::WASM);
    let ra_client = asset_contract::Client::new(&e, &regulated_asset_id);

    // Regulated Asset Parameters
    let decimal: u32 = 7;
    let name = String::from_slice(&e, "Fifocoin");
    let symbol = String::from_slice(&e, "Fifo");

    // Asset Controller Parameters
    let outflow_limit: i128 = 1000;
    let inflow_limit: i128 = 700;
    let quota_time_limit: u64 = 100;

    // Initializes AC and RA with crossed reference.
    ac_client.initialize(&admin, &ra_client.address, &outflow_limit, &inflow_limit, &quota_time_limit);
    ra_client.initialize(&admin, &decimal, &name, &symbol, &ac_client.address);

    ra_client.mint(&user_a, &10000);
    ra_client.mint(&user_b, &10000);
    ra_client.mint(&user_c, &10000);

    e.budget().reset_default();

    (
        e,
        ac_client,
        ra_client,
        admin,
        user_a,
        user_b,
        user_c,
        user_d,
    )
}


#[test]
fn test() {

    let (e, ac_client, ra_client, admin, user_a, user_b, user_c, user_d) = initialize_use_cases();


    //
    // Validate initialization
    //
    assert_eq!(ac_client.get_admin(), admin);
    assert_eq!(ac_client.get_asset(), ra_client.address);
    assert_eq!(ac_client.get_outflow_limit(), 1000);
    assert_eq!(ac_client.get_inflow_limit(), 700);
    assert_eq!(ac_client.get_quota_time_limit(), 100);
    

    //
    // Validate quota history with a series of transactions within limits. 
    //
    ra_client.mint(&user_a, &10000);
    ra_client.mint(&user_b, &10000);
    ra_client.mint(&user_c, &10000);
    e.budget().reset_default();
    

    // A->B 100
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,100]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,0]);
    e.budget().reset_default();


    // B->C 50
    ra_client.transfer(&user_b, &user_c, &50);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,50]);
    assert_eq!(ac_client.get_quota(&user_c), vec![&e,50,0]);
    e.budget().reset_default();


    // C->A 50
    ra_client.transfer(&user_c, &user_a, &50);
    assert_eq!(ac_client.get_quota(&user_c), vec![&e,50,50]);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,50,100]);
    e.budget().reset_default();
   
       
    // A->C 225
    ra_client.transfer(&user_a, &user_c, &225);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,50,325]);
    assert_eq!(ac_client.get_quota(&user_c), vec![&e,275,50]);
    e.budget().reset_default();

    // A->B 675
    ra_client.transfer(&user_a, &user_b, &575);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,50,900]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,675,50]);
    e.budget().reset_default();

    // A->C 100
    ra_client.transfer(&user_a, &user_c, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,50,1000]);
    assert_eq!(ac_client.get_quota(&user_c), vec![&e,375,50]);
    e.budget().reset_default();

    // C->B 25
    ra_client.transfer(&user_c, &user_b, &25);
    assert_eq!(ac_client.get_quota(&user_c), vec![&e,375,75]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,700,50]);
    e.budget().reset_default();
    


    //
    // Test the quota reset through time limit
    //
    advance_ledger(&e, 100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,50,1000]);
    assert_eq!(ac_client.get_quota(&user_c), vec![&e,375,75]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,700,50]);
    e.budget().reset_default();

    advance_ledger(&e, 1);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,0]);
    assert_eq!(ac_client.get_quota(&user_c), vec![&e,0,0]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,0,0]);
    e.budget().reset_default();


    

}



#[test]
fn quota_updated_through_time() {

   
    let (e, ac_client, ra_client, admin, user_a, user_b, user_c, user_d) = initialize_use_cases();

    
    //
    // Test the quota being updated as it drops
    // older payments that have expired
    //

    // A->B 100
    // ledger 0
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,100]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,0]);
    e.budget().reset_default();
    
    // A->B 100
    // ledger 10
    advance_ledger(&e, 10);
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,200]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,200,0]);
    e.budget().reset_default();
    
    // A->B 100
    // ledger 20
    advance_ledger(&e, 10);
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,300]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,300,0]);
    e.budget().reset_default();
    
    // A->B 100
    // ledger 30
    advance_ledger(&e, 10);
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,400]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,400,0]);
    e.budget().reset_default();
    
    // A->B 100
    // ledger 40
    advance_ledger(&e, 10);
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,500]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,500,0]);
    e.budget().reset_default();

    // A->B 100
    // ledger 50
    advance_ledger(&e, 10);
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,600]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,600,0]);
    e.budget().reset_default();
    
    // ledger 100
    advance_ledger(&e, 50);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,600]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,600,0]);

    // ledger 101
    advance_ledger(&e, 1);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,500]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,500,0]);

    // ledger 110
    advance_ledger(&e, 9);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,500]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,500,0]);

    // ledger 111
    advance_ledger(&e, 1);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,400]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,400,0]);

    // ledger 121
    advance_ledger(&e, 10);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,300]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,300,0]);

    // ledger 131
    advance_ledger(&e, 10);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,200]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,200,0]);

    // ledger 141
    advance_ledger(&e, 10);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,100]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,0]);

    // ledger 150
    advance_ledger(&e, 9);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,100]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,0]);

    // A->B 100
    ra_client.transfer(&user_a, &user_b, &50);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,150]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,150,0]);
    e.budget().reset_default();

    // ledger 151
    advance_ledger(&e, 1);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,50]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,50,0]);

    // ledger 251
    advance_ledger(&e, 100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,0]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,0,0]);
}


#[test]
fn quota_time_left() {

    let (e, ac_client, ra_client, admin, user_a, user_b, user_c, user_d) = initialize_use_cases();

    
    //
    // Test the time left in a quota
    // for a user with no transaction. 
    // There should be no quota
    //

    let mut release_data = ac_client.get_quota_release_time(&user_a);
    assert_eq!(release_data.inflow.len(),0);
    assert_eq!(release_data.outflow.len(),0);
    

    //
    // After performing a tranaction
    // both parties should have a quota
    // timed for the duration of the limit
    //
    // A->B 100
    // ledger 0
    ra_client.transfer(&user_a, &user_b, &100);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,100]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,0]);
    e.budget().reset_default();


    release_data = ac_client.get_quota_release_time(&user_a);

    // verify user A inflow quotas
    assert_eq!(release_data.inflow.len(),0);

    // verify user A outflow quotas
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.first_unchecked();
    
        assert_eq!(outflow_entry.amount,100);
        assert_eq!(outflow_entry.time_left,100);
    }

    release_data = ac_client.get_quota_release_time(&user_b);

    // verify user B inflow quotas
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.first_unchecked();

        assert_eq!(inflow_entry.amount,100);
        assert_eq!(inflow_entry.time_left,100);
    }

    // verify user B outflow quotas
    assert_eq!(release_data.outflow.len(),0);

    e.budget().reset_default();  

    //
    // After the time passes both parties
    // should have their quotas updated
    // with the time that passed
    //
    // ledger 40
    advance_ledger(&e, 40);

    release_data = ac_client.get_quota_release_time(&user_a);

    // verify user A inflow quotas
    assert_eq!(release_data.inflow.len(),0);

    // verify user A outflow quotas
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.first_unchecked();
    
        assert_eq!(outflow_entry.amount,100);
        assert_eq!(outflow_entry.time_left,60);
    }

    release_data = ac_client.get_quota_release_time(&user_b);

    // verify user B inflow quotas
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.first_unchecked();

        assert_eq!(inflow_entry.amount,100);
        assert_eq!(inflow_entry.time_left,60);
    }

    // verify user B outflow quotas
    assert_eq!(release_data.outflow.len(),0);
    
    //
    // Introducing new transactions in oposing flow  
    // should afect the existing quota and its tracking
    //
    //
    // B->A 50
    // ledger 40
    ra_client.transfer(&user_b, &user_a, &50);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,50,100]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,50]);
    e.budget().reset_default();


    release_data = ac_client.get_quota_release_time(&user_a);

    // verify user A inflow quotas
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.first_unchecked();
    
        assert_eq!(inflow_entry.amount,50);
        assert_eq!(inflow_entry.time_left,100);
    }

    // verify user A outflow quotas
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.first_unchecked();
    
        assert_eq!(outflow_entry.amount,100);
        assert_eq!(outflow_entry.time_left,60);
    }

    release_data = ac_client.get_quota_release_time(&user_b);

    // verify user B inflow quotas
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.first_unchecked();
    
        assert_eq!(inflow_entry.amount,100);
        assert_eq!(inflow_entry.time_left,60);
    }

    // verify user B outflow quotas
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.first_unchecked();
    
        assert_eq!(outflow_entry.amount,50);
        assert_eq!(outflow_entry.time_left,100);
    }

    //
    // Introducing new transactions in existing 
    // flows should afect the existing quota and
    // populate the array
    //
    //
    // B->A 50
    // ledger 60
    advance_ledger(&e, 20);

    ra_client.transfer(&user_b, &user_a, &50);
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,100,100]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,100,100]);
    e.budget().reset_default();


    release_data = ac_client.get_quota_release_time(&user_a);

    // verify user A inflow quotas (first)
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.get_unchecked(0);
    
        assert_eq!(inflow_entry.amount,50);
        assert_eq!(inflow_entry.time_left,80);
    }

    // verify user A inflow quotas (second)
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.get_unchecked(1);
    
        assert_eq!(inflow_entry.amount,50);
        assert_eq!(inflow_entry.time_left,100);
    }

    // verify user A outflow quotas
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.first_unchecked();
    
        assert_eq!(outflow_entry.amount,100);
        assert_eq!(outflow_entry.time_left,40);
    }

    release_data = ac_client.get_quota_release_time(&user_b);

    // verify user B inflow quotas
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.first_unchecked();
    
        assert_eq!(inflow_entry.amount,100);
        assert_eq!(inflow_entry.time_left,40);
    }

    
    // verify user B outflow quotas(first)
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.get_unchecked(0);
    
        assert_eq!(outflow_entry.amount,50);
        assert_eq!(outflow_entry.time_left,80);
    }

    // verify user B outflow quotas (second)
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.get_unchecked(1);
    
        assert_eq!(outflow_entry.amount,50);
        assert_eq!(outflow_entry.time_left,100);
    }

    //
    // As time goes by we drop the oldest transaction
    // from the quota history and release quota
    //
    // 
    // ledger 110
    advance_ledger(&e, 50);

    e.budget().reset_default();


    release_data = ac_client.get_quota_release_time(&user_a);

    // verify user A inflow quotas (first)
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.get_unchecked(0);
    
        assert_eq!(inflow_entry.amount,50);
        assert_eq!(inflow_entry.time_left,30);
    }

    // verify user A inflow quotas (second)
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.get_unchecked(1);
    
        assert_eq!(inflow_entry.amount,50);
        assert_eq!(inflow_entry.time_left,50);
    }

    // verify user A outflow quotas
    assert_eq!(release_data.outflow.len(),0);
    
    release_data = ac_client.get_quota_release_time(&user_b);

    // verify user B inflow quotas
    assert_eq!(release_data.inflow.len(),0);
    
    // verify user B outflow quotas (first)
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.get_unchecked(0);
    
        assert_eq!(outflow_entry.amount,50);
        assert_eq!(outflow_entry.time_left,30);
    }

    // verify user B outflow quotas (second)
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.get_unchecked(1);
    
        assert_eq!(outflow_entry.amount,50);
        assert_eq!(outflow_entry.time_left,50);
    }

    //
    // As time goes by we drop the oldest transaction
    // from the quota history again and release quota
    //
    // 
    // ledger 150
    advance_ledger(&e, 40);

    e.budget().reset_default();


    release_data = ac_client.get_quota_release_time(&user_a);

    // verify user A inflow quotas 
    if !release_data.inflow.is_empty() {
        let inflow_entry = release_data.inflow.first_unchecked();
    
        assert_eq!(inflow_entry.amount,50);
        assert_eq!(inflow_entry.time_left,10);
    }

    // verify user A outflow quotas
    assert_eq!(release_data.outflow.len(),0);
    
    release_data = ac_client.get_quota_release_time(&user_b);

    // verify user B inflow quotas
    assert_eq!(release_data.inflow.len(),0);
    
    // verify user B outflow quotas
    if !release_data.outflow.is_empty() {
        let outflow_entry = release_data.outflow.first_unchecked();
    
        assert_eq!(outflow_entry.amount,50);
        assert_eq!(outflow_entry.time_left,10);
    }


     //
    // As time goes by we drop the last transactions
    // from the quota history and fully reset the quota
    //
    // 
    // ledger 161
    advance_ledger(&e, 11);

    e.budget().reset_default();


    release_data = ac_client.get_quota_release_time(&user_a);
    // verify user A inflow quotas 
    assert_eq!(release_data.inflow.len(),0);
    // verify user A outflow quotas
    assert_eq!(release_data.outflow.len(),0);
    
    release_data = ac_client.get_quota_release_time(&user_b);
    // verify user B inflow quotas
    assert_eq!(release_data.inflow.len(),0);
    
    // verify user B outflow quotas
    assert_eq!(release_data.outflow.len(),0);


    //verify quota total
    assert_eq!(ac_client.get_quota(&user_a), vec![&e,0,0]);
    assert_eq!(ac_client.get_quota(&user_b), vec![&e,0,0]);

}