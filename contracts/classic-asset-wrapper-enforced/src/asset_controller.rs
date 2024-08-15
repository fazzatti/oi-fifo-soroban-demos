use soroban_sdk::{Address, Env};
use standard_traits::{
    asset_controller::AssetControllerClient, classic_wrapper::common::read_asset_controller,
};

pub fn review_transfer(e: &Env, from: &Address, to: &Address, amount: &i128) {
    let asset_controller = read_asset_controller(&e);
    let asset_controller_client = AssetControllerClient::new(&e, &asset_controller);

    asset_controller_client.review_transfer(from, to, amount);
}
