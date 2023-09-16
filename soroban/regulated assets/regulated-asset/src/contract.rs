

use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::asset_control::{write_asset_controller, read_asset_controller};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{is_authorized, write_authorization};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::event;
use crate::validations::check_nonnegative_amount;
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::storage_types::{INSTANCE_BUMP_AMOUNT,INSTANCE_BUMP_THREASHOLD};
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;


mod asset_controller_contract { 
    soroban_sdk::contractimport!( 
        file = "../../../target/wasm32-unknown-unknown/release/asset_controller.wasm"
    );
}


    
pub trait RegulatedAssetTrait {


    fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String, asset_controller: Address);

    // --------------------------------------------------------------------------------
    // Admin interface – privileged functions.
    // --------------------------------------------------------------------------------
    //
    // All the admin functions have to be authorized by the admin with all input
    // arguments, i.e. they have to call `admin.require_auth()`.

    /// Clawback "amount" from "from" account. "amount" is burned.
    /// Emit event with topics = ["clawback", admin: Address, to: Address], data = [amount: i128]
    fn clawback(
        env: Env,
        from: Address,
        amount: i128,
    );

    /// Mints "amount" to "to".
    /// Emit event with topics = ["mint", admin: Address, to: Address], data = [amount: i128]
    fn mint(
        env: Env,
        to: Address,
        amount: i128,
    );

    /// Sets the administrator to the specified address "new_admin".
    /// Emit event with topics = ["set_admin", admin: Address], data = [new_admin: Address]
    fn set_admin(
        env: Env,
        new_admin: Address,
    );

    /// Sets whether the account is authorized to use its balance freely.
    /// If "authorized" is true, "id" should be able to use its balance.
    /// Emit event with topics = ["set_authorized", id: Address], data = [authorize: bool]
    fn set_authorized(
        env: Env,
        id: Address,
        authorized: bool,
    );

    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------
    //
    // All the functions here have to be authorized by the token spender
    // (usually named `from` here) using all the input arguments, i.e. they have
    // to call `from.require_auth()`.

    /// Set the allowance by "amount" for "spender" to transfer/burn from "from".
    /// "expiration_ledger" is the ledger number where this allowance expires. It cannot
    /// be less than the current ledger number unless the amount is being set to 0.
    /// An expired entry (where "expiration_ledger" < the current ledger number)
    /// should be treated as a 0 amount allowance.
    /// Emit event with topics = ["approve", from: Address, spender: Address], data = [amount: i128, expiration_ledger: u32]
    fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    );

    /// Transfer "amount" from "from" to "to".
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [amount: i128]
    fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    );

    /// Transfer "amount" from "from" to "to", consuming the allowance of "spender".
    /// Authorized by spender (`spender.require_auth()`).
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [amount: i128]
    fn transfer_from(
        env: Env,
        spender: Address,
        from: Address,
        to: Address,
        amount: i128,
    );

    /// Burn "amount" from "from".
    /// Emit event with topics = ["burn", from: Address], data = [amount: i128]
    fn burn(
        env: Env,
        from: Address,
        amount: i128,
    );

    /// Burn "amount" from "from", consuming the allowance of "spender".
    /// Emit event with topics = ["burn", from: Address], data = [amount: i128]
    fn burn_from(
        env: Env,
        spender: Address,
        from: Address,
        amount: i128,
    );

    // --------------------------------------------------------------------------------
    // Read-only Token interface
    // --------------------------------------------------------------------------------
    //
    // The functions here don't need any authorization and don't emit any
    // events.

    /// Get the balance of "id".
    fn balance(env: Env, id: Address) -> i128;

    /// Get the spendable balance of "id". This will return the same value as balance()
    /// unless this is called on the Stellar Asset Contract, in which case this can
    /// be less due to reserves/liabilities.
    fn spendable_balance(env: Env, id: Address) -> i128;

    // Returns true if "id" is authorized to use its balance.
    // By default, starts as true as account is not frozen.
    fn authorized(env: Env, id: Address) -> bool;

    /// Get the allowance for "spender" to transfer from "from".
    fn allowance(
        env: Env,
        from: Address,
        spender: Address,
    ) -> i128;

    // --------------------------------------------------------------------------------
    // Descriptive Interface
    // --------------------------------------------------------------------------------

    // Get the number of decimals used to represent amounts of this token.
    fn decimals(env: Env) -> u32;

    // Get the name for this token.
    fn name(env: Env) -> String;

    // Get the symbol for this token.
    fn symbol(env: Env) -> String;


}


#[contract]
pub struct RegulatedAsset;

#[contractimpl]
impl RegulatedAssetTrait for RegulatedAsset {
    
    
    fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String, asset_controller: Address) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        write_asset_controller(&e, &asset_controller);

        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        event::approve(&e, from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn spendable_balance(e: Env, id: Address) -> i128 {
        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn authorized(e: Env, id: Address) -> bool {
        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);
        is_authorized(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        check_nonnegative_amount(amount);
        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        let asset_controller = read_asset_controller(&e);
        let asset_controller_client = asset_controller_contract::Client::new(&e, &asset_controller);

        asset_controller_client.review_transfer(&from,&to, &amount);


        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        event::transfer(&e, from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        event::transfer(&e, from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        event::burn(&e, from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        event::burn(&e, from, amount)
    }

    fn clawback(e: Env, from: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        spend_balance(&e, from.clone(), amount);
        event::clawback(&e, admin, from, amount);
    }

    fn set_authorized(e: Env, id: Address, authorize: bool) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        write_authorization(&e, id.clone(), authorize);
        event::set_authorized(&e, admin, id, authorize);
    }

    fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        receive_balance(&e, to.clone(), amount);
        event::mint(&e, admin, to, amount);
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage().instance().bump(INSTANCE_BUMP_THREASHOLD,INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin);
        event::set_admin(&e, admin, new_admin);
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }

}