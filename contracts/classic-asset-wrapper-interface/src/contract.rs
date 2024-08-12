use soroban_sdk::{contract, contractimpl, token, vec, Address, Env};

use crate::account_authorization::{
    execute_with_temporary_authorizations, set_account_authorization,
};
use crate::admin::set_asset_admin;
use crate::asset_controller::review_transfer;
use crate::data::{read_admin, read_asset, write_admin, write_asset, write_asset_controller};
use crate::validations::{
    is_admin_validation, is_contract_initialized_validation,
    is_contract_not_initialized_validation, is_wrapper_active_validation, is_wrapper_admin,
};

pub trait WrapperInterfaceTrait {
    //
    // Important: Different from the pure soroban regulated asset,
    // when initializing the asset controller for this asset, it is
    // necessary to set the asset contract as this wrapper's address
    // instead of the stellar asset contract. This is necessary because
    // the asset controller validates who is the contract invoking it
    // before allowing the functions to be executed and for the classic
    // asset, the wrapper will perform these invokations.
    //

    // --------------------------------------------------------------------------------
    // Admin interface – privileged functions.
    // --------------------------------------------------------------------------------
    //
    // All the admin functions have to be authorized by the admin with all input
    // arguments, i.e. they have to call `admin.require_auth()`.

    // Inititalize Parameters
    //-------------------------
    // admin:            Address that has managing rights over the contract
    // asset:            Address of the classic asset contract
    // asset_controller: Address of the Asset controller contract
    //
    fn initialize(e: Env, admin: Address, asset: Address, asset_controller: Address);

    // Set the admin that controls the wrapper interface
    fn set_admin(e: Env, new_admin: Address);

    // Activate the wrapper interface contract as the
    // admin and controller of the classic asset contract
    fn activate_wrapper(e: Env);

    // Deactivate the wrapper interface contract as the
    // admin and controller of the classic asset contract
    // and set the current admin of the wrapper as the
    // admin of the classic asset contract.
    fn deactivate_wrapper(e: Env);

    // Authorize or deauthorize the 'id' account trustline to
    // hold balance and transact with the standard asset.
    // The trustline must have been created already through
    // stellar classic.
    fn set_authorized(e: Env, id: Address, authorize: bool);

    // Mint an arbitrary amount of asset units directly to
    // the 'to' address.
    fn mint(e: Env, to: Address, amount: i128);

    // --------------------------------------------------------------------------------
    // Asset interface
    // --------------------------------------------------------------------------------
    //
    // All the functions here have to be authorized by the asset spender
    // (usually named `from` here) using all the input arguments, i.e. they have
    // to call `from.require_auth()`.

    fn transfer(e: Env, from: Address, to: Address, amount: i128);

    // --------------------------------------------------------------------------------
    // Read-only
    // --------------------------------------------------------------------------------
    //
    // The functions here don't need any authorization and don't emit any
    // events.

    // Get the admin of the wrapper interface who indirectly controls the asset contract
    fn get_admin(e: Env) -> Address;

    // Check if this wrapper is currently active and controlling the asset contract
    fn is_wrapper_active(e: Env) -> bool;
}

#[contract]
pub struct WrapperInterface;

#[contractimpl]
impl WrapperInterfaceTrait for WrapperInterface {
    // --------------------------------------------------------------------------------
    // Admin interface – privileged functions.
    // --------------------------------------------------------------------------------
    //
    fn initialize(e: Env, admin: Address, asset: Address, asset_controller: Address) {
        is_contract_not_initialized_validation(&e);

        admin.require_auth();

        write_admin(&e, &admin);
        write_asset(&e, &asset);
        write_asset_controller(&e, &asset_controller);

        set_asset_admin(&e, &e.current_contract_address());
    }

    fn activate_wrapper(e: Env) {
        is_admin_validation(&e); // When checking for admin auth, it is not necessary to check for contract initialization
        set_asset_admin(&e, &e.current_contract_address());
    }

    fn deactivate_wrapper(e: Env) {
        is_admin_validation(&e); // When checking for admin auth, it is not necessary to check for contract initialization
        let admin = read_admin(&e);
        set_asset_admin(&e, &admin);
    }

    fn set_admin(e: Env, new_admin: Address) {
        is_admin_validation(&e); // When checking for admin auth, it is not necessary to check for contract initialization
        write_admin(&e, &new_admin);
    }

    fn set_authorized(e: Env, id: Address, authorize: bool) {
        is_admin_validation(&e); // When checking for admin auth, it is not necessary to check for contract initialization
        set_account_authorization(&e, id, authorize);
    }

    fn mint(e: Env, to: Address, amount: i128) {
        is_admin_validation(&e); // When checking for admin auth, it is not necessary to check for contract initialization

        let asset_address = read_asset(&e);
        let asset_admin_client = token::StellarAssetClient::new(&e, &asset_address);

        let action = || {
            asset_admin_client.mint(&to, &amount);
        };

        let addresses = vec![&e, to.clone()];
        execute_with_temporary_authorizations(&e, addresses, action);
    }

    // --------------------------------------------------------------------------------
    // Asset interface
    // --------------------------------------------------------------------------------
    //
    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        is_contract_initialized_validation(&e);
        is_wrapper_active_validation(&e);

        from.require_auth();

        // invoke asset controller to validate transaction
        review_transfer(&e, &from, &to, &amount);

        let asset_address = read_asset(&e);
        let asset_client = token::Client::new(&e, &asset_address);

        let action = || {
            asset_client.transfer(&from, &to, &amount);
        };

        let addresses = vec![&e, from.clone(), to.clone()];
        execute_with_temporary_authorizations(&e, addresses, action);
    }

    // --------------------------------------------------------------------------------
    // Read-only
    // --------------------------------------------------------------------------------
    fn is_wrapper_active(e: Env) -> bool {
        is_wrapper_admin(&e)
    }

    fn get_admin(e: Env) -> Address {
        read_admin(&e)
    }
}
