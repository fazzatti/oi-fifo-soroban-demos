use crate::data::read_asset;
use soroban_sdk::{token, Address, Env};

pub fn set_asset_admin(e: &Env, new_admin: &Address) {
    let asset_address = read_asset(&e);
    let asset_admin_client = token::StellarAssetClient::new(&e, &asset_address);

    asset_admin_client.set_admin(&new_admin);
}
