#![cfg(test)]
extern crate std;

use crate::contract::WrapperInterfaceClient;

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, vec, Address, Env, String,
};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

//
// TESTUTILS
//

fn advance_ledger(e: &Env, delta: u64) {
    e.ledger().with_mut(|l| {
        l.timestamp += delta;
    });
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin_address = Address::generate(&e);

    // Register the classic asset
    let classic_asset_address = e.register_stellar_asset_contract(admin_address.clone());

    let asset_client = TokenClient::new(&e, &classic_asset_address);
    let asset_admin_client = TokenAdminClient::new(&e, &classic_asset_address);
    // asset_admin_client.env.

    //register the asset controller
    let asset_controler_address = Address::generate(&e);

    //register the wrapper interface
    let wrapper_interface_contract_id = e.register_contract(None, crate::WrapperInterface {});
    let wi_client = WrapperInterfaceClient::new(&e, &wrapper_interface_contract_id);

    // Verify if the issuer/admin is the default prior to initialization
    assert_eq!(asset_admin_client.admin(), admin_address);

    wi_client.initialize(
        &admin_address,
        &classic_asset_address,
        &asset_controler_address,
    );

    // Check if Wrapper was set as the new asset admin after initialization
    assert_eq!(asset_admin_client.admin(), wrapper_interface_contract_id);

    // Change admin through Wrapper Interface
    assert_eq!(wi_client.get_admin(), admin_address);
    let new_admin = Address::generate(&e);
    wi_client.set_admin(&new_admin);
    assert_eq!(wi_client.get_admin(), new_admin);
    wi_client.set_admin(&admin_address);
    assert_eq!(wi_client.get_admin(), admin_address);

    // Mint to random user through the Wrapper
    let user_a = Address::generate(&e);
    // asset_admin_client.set_authorized(&user_a, &false); // simulating an unauthorized trustline
    wi_client.mint(&user_a, &100);
    assert_eq!(asset_client.balance(&user_a), 100);

    // Authorize and deauthorize an account
    //
    // TODO: Add this test once it is supported to simulate different flag configurations
    // reference thread: https://discord.com/channels/897514728459468821/1163144469722431579/1163144469722431579

    // let user_b = Address::random(&e);
    // wi_client.set_authorized(&user_b, &false);
    // assert_eq!(asset_admin_client.authorized(&user_b), false);
    // wi_client.set_authorized(&user_b, &true);
    // assert_eq!(asset_admin_client.authorized(&user_b), true);
    // wi_client.set_authorized(&user_b, &true);
    // wi_client.mint(&user_a, &100);
    // assert_eq!(asset_client.balance(&user_a),100);
}
