use soroban_sdk::{token, Address, Env, Vec};
use standard_traits::classic_wrapper::common::read_asset;

// Verifies if the accounts are already authorized before executing
// the 'action' passed as arg. All accounts that aren't authorized
// will then be temporarily authorized just for this execution and then
// reverted back to their original authorization status once the 'action'
// is finished executing.
//
pub fn execute_with_temporary_authorizations<F, I>(e: &Env, addresses: I, action: F)
where
    F: FnOnce(),
    I: IntoIterator<Item = Address>,
{
    let mut temp_authorized_addresses = Vec::<Address>::new(&e);

    for address in addresses {
        if !is_authorized(&e, &address) {
            set_account_authorization(&e, address.clone(), true);
            temp_authorized_addresses.push_back(address);
        }
    }

    action();

    for address in temp_authorized_addresses.iter() {
        set_account_authorization(&e, address, false);
    }
}

fn is_authorized(e: &Env, id: &Address) -> bool {
    let asset_address = read_asset(&e);
    let asset_admin_client = token::StellarAssetClient::new(&e, &asset_address);

    asset_admin_client.authorized(&id)
}

pub fn set_account_authorization(e: &Env, id: Address, authorize: bool) {
    let asset_address = read_asset(&e);
    let asset_admin_client = token::StellarAssetClient::new(&e, &asset_address);

    asset_admin_client.set_authorized(&id, &authorize);
}
