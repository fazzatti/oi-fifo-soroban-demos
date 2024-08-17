use soroban_sdk::{contract, contractimpl, Address, Env};
use standard_traits::asset_controller::AssetControllerTrait;

use crate::storage::{read_even, read_odd, write_even, write_odd};

pub trait CounterTrait {
    fn get_even_count(env: Env) -> u64;
    fn get_odd_count(env: Env) -> u64;
}

#[contract]
pub struct CounterContract;

#[contractimpl]
impl AssetControllerTrait for CounterContract {
    fn review_transfer(env: Env, _from: Address, _to: Address, amount: i128) {
        if amount % 2 == 0 {
            let mut even_count = read_even(&env);

            even_count += 1;

            write_even(&env, even_count);
        } else {
            let mut odd_count = read_odd(&env);

            odd_count += 1;

            write_odd(&env, odd_count);
        }
    }
}

#[contractimpl]
impl CounterTrait for CounterContract {
    fn get_even_count(env: Env) -> u64 {
        read_even(&env)
    }

    fn get_odd_count(env: Env) -> u64 {
        read_odd(&env)
    }
}
