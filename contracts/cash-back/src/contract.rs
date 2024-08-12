use soroban_sdk::{contract, contractimpl, token, vec, Address, Env};

use crate::storage::{write_admin, write_asset, write_cash_back_amount};

pub trait CashBackTrait {
    fn initialize(
        env: Env,
        admin: Address,
        asset: Address,
        cashback_amount: i128,
        prize: i128,
        end_date: u64,
    );

    fn add_funds(env: Env, amount: i128);
}

#[contract]
pub struct CashBackContract;

#[contractimpl]
impl CashBackTrait for CashBackContract {
    fn initialize(
        env: Env,
        admin: Address,
        asset: Address,
        cashback_amount: i128,
        prize: i128,
        end_date: u64,
    ) {
        admin.require_auth();

        write_admin(&env, &admin);
        write_asset(&env, &asset);
        write_cash_back_amount(&env, &cashback_amount);
        write_prize(&env, &prize);
        write_end_date(&env, &end_date);
    }

    fn add_funds(env: Env, amount: i128) {
        let asset_address = read_asset(&env);
        let asset_admin_client = token::StellarAssetClient::new(&env, &asset_address);

        let action = || {
            asset_admin_client.mint(&env.current_contract_address(), &amount);
        };

        let addresses = vec![&env, env.current_contract_address()];
        execute_with_temporary_authorizations(&env, addresses, action);
    }
}

mod test;
