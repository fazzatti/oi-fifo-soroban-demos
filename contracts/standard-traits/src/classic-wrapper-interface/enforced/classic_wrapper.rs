use crate::classic_wrapper::common::WrapperMetadata;
use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "ClassicWrapperClient")]
pub trait EnforcedClassicWrapperInterfaceTrait {
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

    // Get the metadata of the wrapper interface
    fn get_metadata(e: Env) -> WrapperMetadata;

    // Get the admin of the wrapper interface who indirectly controls the asset contract
    fn get_admin(e: Env) -> Address;

    // Check if this wrapper is currently active and controlling the asset contract
    fn is_wrapper_active(e: Env) -> bool;
}
