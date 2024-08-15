use soroban_sdk::{contract, contractimpl, token, Address, Env};

use crate::{
    campaign_rules::{is_campaign_not_over, proccess_user_transfer},
    storage::{
        read_admin, read_asset, read_user_data, read_wrapper, write_admin, write_asset,
        write_end_date, write_inflow_points, write_outflow_points, write_prize_amount,
        write_target_points, write_wait_interval, write_wrapper, UserData,
    },
};

pub trait CampaignTrait {
    fn initialize(
        env: Env,
        admin: Address,
        asset: Address,
        wrapper: Address,
        prize_amount: i128,
        inflow_points: i128,
        outflow_points: i128,
        target_points: i128,
        wait_interval: u64,
        end_date: u64,
    );

    fn add_funds(env: Env, amount: i128);

    fn review_transfer(env: Env, from: Address, to: Address, amount: i128);

    fn get_user(env: Env, user: Address) -> UserData;
}

#[contract]
pub struct CampaignContract;

#[contractimpl]
impl CampaignTrait for CampaignContract {
    fn initialize(
        env: Env,
        admin: Address,
        asset: Address,
        wrapper: Address,
        prize_amount: i128,
        inflow_points: i128, // 1 = 0.01 multiplier
        outflow_points: i128,
        target_points: i128,
        wait_interval: u64,
        end_date: u64,
    ) {
        admin.require_auth();

        write_admin(&env, &admin);
        write_asset(&env, &asset);
        write_wrapper(&env, &wrapper);
        write_prize_amount(&env, &prize_amount);
        write_target_points(&env, &target_points);
        write_inflow_points(&env, &inflow_points);
        write_outflow_points(&env, &outflow_points);
        write_wait_interval(&env, &wait_interval);
        write_end_date(&env, &end_date);
    }

    fn add_funds(env: Env, amount: i128) {
        let admin = read_admin(&env);
        admin.require_auth();

        let asset_address = read_asset(&env);
        let asset_admin_client = token::TokenClient::new(&env, &asset_address);

        asset_admin_client.transfer(&admin, &env.current_contract_address(), &amount);
    }

    fn review_transfer(env: Env, from: Address, to: Address, amount: i128) {
        assert!(is_campaign_not_over(&env), "Campaign is over");

        read_wrapper(&env).require_auth();

        proccess_user_transfer(&env, from, amount, true);
        proccess_user_transfer(&env, to, amount, false);
    }

    fn get_user(env: Env, user: Address) -> UserData {
        read_user_data(&env, user)
    }
}
