#![cfg(test)]
extern crate std;

use crate::contract::{CampaignContract, CampaignContractClient};
// use standard_traits::classic_wrapper::enforced::{
//     ClassicWrapperClient, EnforcedClassicWrapperInterfaceTrait,
// };

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{self},
    Address, Env,
};

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

#[test]
fn test() {
    // INITIALIZATION
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user_a = Address::generate(&e);
    let user_b = Address::generate(&e);
    let user_c = Address::generate(&e);

    // Deploys Asset Controller and Regulated Asset
    let campaign_id = e.register_contract(None, CampaignContract);
    let campaign_client = CampaignContractClient::new(&e, &campaign_id);

    let wrapper_id = e.register_contract_wasm(None, wrapper_contract::WASM);
    let wr_client = wrapper_contract::Client::new(&e, &wrapper_id);

    let asset_id = e.register_stellar_asset_contract(admin.clone());
    let sac_client = token::StellarAssetClient::new(&e, &asset_id);
    let a_client = token::TokenClient::new(&e, &asset_id);

    // Initializes AC and RA with crossed reference.
    campaign_client.initialize(
        &admin,
        &a_client.address,
        &wr_client.address,
        &1000,
        &100,
        &150,
        &10000,
        &600,
        &(e.ledger().timestamp() + 600_000),
    );

    wr_client.initialize(&admin, &sac_client.address, &campaign_client.address);

    sac_client.mint(&user_a, &10_000);
    sac_client.mint(&user_b, &10_000);
    sac_client.mint(&user_c, &10_000);

    e.budget().reset_default();

    sac_client.mint(&admin, &10000);
    campaign_client.add_funds(&10000);

    assert_eq!(a_client.balance(&campaign_id), 10000);

    wr_client.transfer(&user_a, &user_b, &23);
    assert_eq!(a_client.balance(&user_b), 10023);
    assert_eq!(a_client.balance(&user_a), 9977);

    assert_eq!(campaign_client.get_user(&user_a).points, 34);
    assert_eq!(campaign_client.get_user(&user_b).points, 23);
}
