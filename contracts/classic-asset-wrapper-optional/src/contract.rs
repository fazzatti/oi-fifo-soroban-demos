use soroban_sdk::{contract, contractimpl, token, Address, Env};
use standard_traits::classic_wrapper::common::{
    read_asset, read_metadata, write_metadata, WrapperMetadata,
};

use crate::asset_controller::review_transfer;
use crate::validations::{
    is_contract_initialized_validation, is_contract_not_initialized_validation,
};

use standard_traits::classic_wrapper::optional::OptionalClassicWrapperInterfaceTrait;

pub trait SpecificFeaturesTrait {
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
}

#[contract]
pub struct WrapperInterface;

#[contractimpl]
impl OptionalClassicWrapperInterfaceTrait for WrapperInterface {
    // --------------------------------------------------------------------------------
    // Asset interface
    // --------------------------------------------------------------------------------
    //
    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        is_contract_initialized_validation(&e);

        from.require_auth();

        // invoke asset controller to validate transaction
        review_transfer(&e, &from, &to, &amount);

        let asset_address = read_asset(&e);
        let asset_client = token::Client::new(&e, &asset_address);

        asset_client.transfer(&from, &to, &amount);
    }

    fn get_metadata(e: Env) -> WrapperMetadata {
        is_contract_initialized_validation(&e);

        read_metadata(&e)
    }

    fn is_wrapper_active(e: Env) -> bool {
        is_contract_initialized_validation(&e);

        let metadata = read_metadata(&e);
        metadata.is_active
    }
}

#[contractimpl]
impl SpecificFeaturesTrait for WrapperInterface {
    // --------------------------------------------------------------------------------
    // Admin interface – privileged functions.
    // --------------------------------------------------------------------------------
    //
    fn initialize(e: Env, admin: Address, asset: Address, asset_controller: Address) {
        is_contract_not_initialized_validation(&e);

        admin.require_auth();

        let metadata = WrapperMetadata {
            enforced: false,
            is_active: true,
            admin,
            asset,
            asset_controller,
        };

        write_metadata(&e, &metadata)
    }
}
