use soroban_sdk::{token, Address, Env};
use standard_traits::classic_wrapper::common::read_asset;

pub fn set_asset_admin(e: &Env, new_admin: &Address) {
    let asset_address = read_asset(&e);
    let asset_admin_client = token::StellarAssetClient::new(&e, &asset_address);

    asset_admin_client.set_admin(&new_admin);
}
