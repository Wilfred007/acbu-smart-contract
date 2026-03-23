#![cfg(test)]

use acbu_lending_pool::{LendingPool, LendingPoolClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup_env() -> (Env, Address, Address, Address, i128, LendingPoolClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let acbu_token = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let fee_rate = 300; // 3%

    let contract_id = env.register_contract(None, LendingPool);
    let client = LendingPoolClient::new(&env, &contract_id);

    client.initialize(
        &admin,
        &acbu_token,
        &fee_rate,
    );

    let lender = Address::generate(&env);
    (env, admin, acbu_token, lender, fee_rate, client)
}

#[test]
fn test_initialize() {
    let (_env, _admin, _acbu_token, _lender, _fee_rate, _client) = setup_env();
    // Just verify setup works
}

#[test]
fn test_deposit_and_withdraw_with_fee() {
    let (env, admin, acbu_token, lender, fee_rate, client) = setup_env();
    
    let deposit_amount = 10_000_000i128; // 10 ACBU

    // Mock ACBU for lender
    let acbu_admin = soroban_sdk::token::StellarAssetClient::new(&env, &acbu_token);
    acbu_admin.mint(&lender, &deposit_amount);

    // Deposit
    client.deposit(&lender, &deposit_amount);
    assert_eq!(client.get_balance(&lender), deposit_amount);

    // Withdraw all
    client.withdraw(&lender, &deposit_amount);
    
    // Verify tracked balance in contract is 0
    assert_eq!(client.get_balance(&lender), 0);

    // Verify token balances
    let acbu_client = soroban_sdk::token::Client::new(&env, &acbu_token);
    
    let expected_fee = (deposit_amount * fee_rate) / 10_000;
    let expected_net = deposit_amount - expected_fee;

    assert_eq!(acbu_client.balance(&lender), expected_net);
    assert_eq!(acbu_client.balance(&admin), expected_fee);
}

#[test]
fn test_pause_unpause() {
    let (_env, _admin, _acbu_token, _lender, _fee_rate, client) = setup_env();
    client.pause();
    // Verification of pause behavior can be added if needed
}
