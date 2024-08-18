use crate::classic_wrapper::common::WrapperMetadata;
use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "ClassicWrapperClient")]
pub trait OptionalClassicWrapperInterfaceTrait {
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

    // Check if this wrapper is currently active and controlling the asset contract
    fn is_wrapper_active(e: Env) -> bool;
}
