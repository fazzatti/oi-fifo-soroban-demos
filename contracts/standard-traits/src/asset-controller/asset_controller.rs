use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "AssetControllerClient")]
pub trait AssetControllerTrait {
    // --------------------------------------------------------------------------------
    // Transaction Auditing Functions
    // --------------------------------------------------------------------------------
    // These transactions apply the validation rules to the
    // transactions triggered by the regulated asset contract.
    // They can only be invoked by the registered asset contract
    // which can be verified with function `is_invoker_the_asset_contract()`

    /// Process a simple transfer transaction
    /// enforcing inflow and outflow rules
    ///
    fn review_transfer(env: Env, from: Address, to: Address, amount: i128);
}
