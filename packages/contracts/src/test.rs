#![allow(dead_code)]

use soroban_sdk::U256;
use soroban_sdk::{Address, Env};
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};

use crate::{KchngToken, KchngTokenClient};

#[test]
fn test_init() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Verify total supply
    assert_eq!(client.total_supply(), initial_supply);

    // Verify admin balance
    assert_eq!(client.balance(&admin), initial_supply);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);
    let transfer_amount = U256::from_u32(&env, 100);

    client.init(&admin, &initial_supply);

    // Transfer
    client.transfer(&admin, &user, &transfer_amount);

    // Verify balances
    let expected_admin = initial_supply.sub(&transfer_amount);
    assert_eq!(client.balance(&admin), expected_admin);
    assert_eq!(client.balance(&user), transfer_amount);
}

#[test]
fn test_demurrage_application() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Give user some tokens
    let user_amount = U256::from_u32(&env, 100);
    client.transfer(&admin, &user, &user_amount);

    // Verify initial balance
    assert_eq!(client.balance(&user), user_amount);

    // Simulate time passing (7+ days)
    // Note: In real tests, you'd jump the ledger timestamp
    // For now, this test verifies the transfer works correctly
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);
    let mint_amount = U256::from_u32(&env, 50_000);

    client.init(&admin, &initial_supply);

    // Mint to user
    client.mint(&admin, &user, &mint_amount);

    // Verify balances and supply
    assert_eq!(client.balance(&admin), initial_supply);
    assert_eq!(client.balance(&user), mint_amount);
    let expected_supply = initial_supply.add(&mint_amount);
    assert_eq!(client.total_supply(), expected_supply);
}

#[test]
#[should_panic(expected = "Insufficient balance")]
fn test_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Try to transfer more than available
    let too_much = U256::from_u32(&env, 2_000_000);
    client.transfer(&user, &admin, &too_much);
}
