use soroban_sdk::{token, Address, Env};

use crate::storage::{
    read_admin, read_asset, read_end_date, read_inflow_points, read_outflow_points,
    read_prize_amount, read_target_points, read_user_data, read_wait_interval, write_user_data,
    UserData,
};

pub fn proccess_user_transfer(env: &Env, user: Address, amount: i128, is_sender: bool) {
    if user == read_admin(env) {
        return;
    }

    let user_data = read_user_data(&env, user.clone());

    if is_elligible_for_transfer(&env, user_data.clone()) && is_there_enough_balance_for_prize(&env)
    {
        let new_points = if is_sender {
            user_data.points + convert_points(read_outflow_points(&env), amount)
        } else {
            user_data.points + convert_points(read_inflow_points(&env), amount)
        };

        let mut new_user_data = UserData {
            points: new_points,
            wait_until: user_data.wait_until,
        };

        if new_user_data.points >= read_target_points(&env) {
            distribute_prize(env, user.clone());
            new_user_data.points = 0;
            new_user_data.wait_until = env.ledger().timestamp() + read_wait_interval(&env);
        }

        write_user_data(&env, user, &new_user_data);
    }
}

fn is_elligible_for_transfer(env: &Env, user_data: UserData) -> bool {
    user_data.wait_until <= env.ledger().timestamp()
}

fn is_there_enough_balance_for_prize(env: &Env) -> bool {
    let asset_address = read_asset(&env);
    let asset_client = token::TokenClient::new(&env, &asset_address);
    asset_client.balance(&env.current_contract_address()) >= read_prize_amount(&env)
}

// 1 = 0.01 multiplier
fn convert_points(multiplier: i128, amount: i128) -> i128 {
    (amount * multiplier) / 100
}

fn distribute_prize(env: &Env, user: Address) {
    let prize_amount = read_prize_amount(&env);
    let asset_address = read_asset(&env);

    let asset_client = token::TokenClient::new(&env, &asset_address);

    asset_client.transfer(&env.current_contract_address(), &user, &prize_amount);
}

pub fn is_campaign_not_over(env: &Env) -> bool {
    env.ledger().timestamp() <= read_end_date(&env)
}
