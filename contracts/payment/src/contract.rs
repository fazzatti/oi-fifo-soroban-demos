use soroban_sdk::{contract, contractimpl, Address, Env};
use standard_traits::classic_wrapper::enforced::ClassicWrapperClient;
pub trait PaymentTrait {
    fn initialize(env: Env, asset: Address);
    fn pay(env: Env, from: Address, to: Address, amount: i128);
}

#[contract]
pub struct PaymentContract;

#[contractimpl]
impl PaymentTrait for PaymentContract {
    fn initialize(env: Env, asset: Address) {
        // env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"asset", &asset);
    }

    fn pay(env: Env, from: Address, to: Address, amount: i128) {
        // let admin: Address = env.storage().instance().get(&"admin").unwrap();

        from.require_auth();

        let asset: Address = env.storage().instance().get(&"asset").unwrap();
        let asset_client = ClassicWrapperClient::new(&env, &asset);

        asset_client.transfer(&from, &to, &amount);
    }
}
