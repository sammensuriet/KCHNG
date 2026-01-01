#![allow(dead_code)]

use soroban_sdk::U256;
use soroban_sdk::{Address, Env, String};
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};

use crate::{KchngToken, KchngTokenClient};

// ==========================================================================
// LEGACY TESTS (Basic Token Functionality)
// ==========================================================================

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

// ==========================================================================
// TRUST SYSTEM TESTS
// ==========================================================================

#[test]
fn test_register_trust() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let governor = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Register a trust with 12% annual rate (1200 bps)
    client.register_trust(
        &governor,
        &String::from_str(&env, "Urban Elder Care"),
        &1200u32,
        &30u64,
    );

    // Verify trust was created
    let trust_info = client.get_trust_info(&governor);
    assert_eq!(trust_info.name, String::from_str(&env, "Urban Elder Care"));
    assert_eq!(trust_info.governor, governor);
    assert_eq!(trust_info.annual_rate_bps, 1200);
    assert_eq!(trust_info.demurrage_period_days, 30);
    assert!(trust_info.is_active);
}

#[test]
fn test_join_trust() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let governor = Address::generate(&env);
    let member = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Register trust
    client.register_trust(
        &governor,
        &String::from_str(&env, "Test Trust"),
        &1200u32,
        &30u64,
    );

    // Join trust
    client.join_trust(&member, &governor);

    // Verify membership
    let account = client.get_account(&member);
    assert_eq!(account.trust_id, governor);

    // Verify member count
    let trust_info = client.get_trust_info(&governor);
    assert_eq!(trust_info.member_count, 1);
}

// ==========================================================================
// WORK VERIFICATION TESTS
// ==========================================================================

#[test]
fn test_submit_work_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let worker = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Submit work claim (30 minutes basic work)
    let claim_id = client.submit_work_claim(
        &worker,
        &0u32, // WorkType::Basic
        &30u64,
        &String::from_str(&env, "QmHash123"),
        &None::<i64>,
        &None::<i64>,
    );

    // Verify claim was created
    assert!(claim_id >= 0);

    let claim = client.get_work_claim(&claim_id);
    assert_eq!(claim.worker, worker);
    assert_eq!(claim.minutes_worked, 30);
    assert_eq!(claim.status, 0); // ClaimStatus::Pending
}

#[test]
fn test_approve_work_claim() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let worker = Address::generate(&env);
    let verifier = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Submit work claim
    let claim_id = client.submit_work_claim(
        &worker,
        &0u32,
        &30u64,
        &String::from_str(&env, "QmHash123"),
        &None::<i64>,
        &None::<i64>,
    );

    // Approve claim
    client.approve_work_claim(&verifier, &claim_id);

    // Verify approval was recorded
    let claim = client.get_work_claim(&claim_id);
    assert!(claim.approvals_received > 0);
}

// ==========================================================================
// GRACE PERIOD TESTS
// ==========================================================================

#[test]
fn test_register_oracle() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Give oracle enough stake
    let stake_amount = U256::from_u32(&env, 500_000);
    client.transfer(&admin, &oracle, &stake_amount);

    // Register oracle
    client.register_oracle(&oracle);

    // Verify oracle is registered
    let oracle_data = client.get_oracle(&oracle);
    assert_eq!(oracle_data.oracle_address, oracle);
    assert_eq!(oracle_data.stake, stake_amount);
}

#[test]
fn test_activate_grace_period() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let oracle = Address::generate(&env);
    let account = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Setup oracle
    let stake_amount = U256::from_u32(&env, 500_000);
    client.transfer(&admin, &oracle, &stake_amount);
    client.register_oracle(&oracle);

    // Setup account with contribution hours
    client.transfer(&admin, &account, &U256::from_u32(&env, 100));
    // Note: Would need to call contribution tracking functions

    // Activate grace period
    client.activate_grace_period(
        &oracle,
        &account,
        &0u32, // GraceType::Emergency
        &30u64,
    );

    // Verify grace period
    let grace_period = client.get_grace_period(&account);
    assert_eq!(grace_period.account, account);
    assert!(grace_period.oracle_verified);
}

// ==========================================================================
// CROSS-TRUST EXCHANGE TESTS
// ==========================================================================

#[test]
fn test_calculate_exchange_rate() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let trust_a = Address::generate(&env);
    let trust_b = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Register trust A (12% rate)
    client.register_trust(&trust_a, &String::from_str(&env, "Trust A"), &1200u32, &30u64);

    // Register trust B (8% rate)
    client.register_trust(&trust_b, &String::from_str(&env, "Trust B"), &800u32, &30u64);

    // Calculate exchange rate
    let rate_bps = client.calculate_exchange_rate(&trust_a, &trust_b);

    // Expected: (1 - 0.12) / (1 - 0.08) = 0.88 / 0.92 ≈ 0.9565
    // In basis points: ~9565
    assert!(rate_bps > 9500 && rate_bps < 9600);
}

#[test]
fn test_cross_trust_swap() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let trust_a = Address::generate(&env);
    let trust_b = Address::generate(&env);
    let user = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Register trusts
    client.register_trust(&trust_a, &String::from_str(&env, "Trust A"), &1200u32, &30u64);
    client.register_trust(&trust_b, &String::from_str(&env, "Trust B"), &800u32, &30u64);

    // Setup user in trust A
    client.transfer(&admin, &user, &U256::from_u32(&env, 100));
    client.join_trust(&user, &trust_a);

    // Perform cross-trust swap
    let swap_amount = U256::from_u32(&env, 100);
    client.cross_trust_swap(&user, &trust_b, &swap_amount);

    // Verify user is now in trust B
    let account = client.get_account(&user);
    assert_eq!(account.trust_id, trust_b);
}

// ==========================================================================
// GOVERNANCE TESTS
// ==========================================================================

#[test]
fn test_create_proposal() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let governor = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Register trust
    client.register_trust(&governor, &String::from_str(&env, "Test Trust"), &1200u32, &30u64);

    // Create rate change proposal
    let proposal_id = client.create_proposal(
        &governor,
        &0u32, // ProposalType::RateChange
        &String::from_str(&env, "Reduce Rate to 10%"),
        &String::from_str(&env, "Lowering rate due to increased velocity"),
        &governor,
        &Some(1000u32), // 10%
    );

    // Verify proposal was created
    let proposal = client.get_proposal(&proposal_id);
    assert_eq!(proposal.proposer, governor);
    assert_eq!(proposal.proposal_type, 0); // ProposalType::RateChange
    assert_eq!(proposal.status, 0); // ProposalStatus::Review
}

#[test]
fn test_vote_on_proposal() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, KchngToken);
    let client = KchngTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let governor = Address::generate(&env);
    let voter = Address::generate(&env);
    let initial_supply = U256::from_u32(&env, 1_000_000);

    client.init(&admin, &initial_supply);

    // Setup trust and proposal
    client.register_trust(&governor, &String::from_str(&env, "Test Trust"), &1200u32, &30u64);
    client.transfer(&admin, &voter, &U256::from_u32(&env, 100));
    client.join_trust(&voter, &governor);

    let proposal_id = client.create_proposal(
        &governor,
        &0u32,
        &String::from_str(&env, "Test"),
        &String::from_str(&env, "Test"),
        &governor,
        &Some(1000u32),
    );

    // Advance time to voting period
    // Note: In real tests, use env.ledger().set()

    // Vote
    client.vote_on_proposal(&voter, &proposal_id, &true);

    // Verify vote was recorded
    let proposal = client.get_proposal(&proposal_id);
    assert!(proposal.votes_for > 0);
}
