#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Map, U256, Vec, String, Bytes,
};
use core::cmp::min;

// ============================================================================
// CONSTANTS
// ============================================================================

// Time Standard
const MINUTES_PER_KCHNG: u64 = 30;
const MIN_WORK_MINUTES: u64 = 15;

// Time
const SECONDS_PER_DAY: u64 = 86_400;
const SECONDS_PER_HOUR: u64 = 3_600;

// Demurrage (Wörgl model: 1% monthly = ~12.7% annual)
const DEFAULT_ANNUAL_RATE_BPS: u32 = 1200; // 12% in basis points (100 = 1%)
const DEFAULT_PERIOD_DAYS: u64 = 30; // Monthly demurrage

// Protocol constraints
const MIN_ANNUAL_RATE_BPS: u32 = 500; // 5% minimum
const MAX_ANNUAL_RATE_BPS: u32 = 1500; // 15% maximum

// Verification
const MIN_VERIFIERS: u32 = 2;
const MAX_VERIFIERS: u32 = 5;
const VERIFIER_STAKE: u64 = 100 * 10_000_000_000_000_000; // 100 KCHNG (18 decimals)

// Grace Periods
const MAX_GRACE_PERIODS_PER_YEAR: u32 = 3;
const MIN_CONTRIBUTION_HOURS: u64 = 30;

// Governance
const PROPOSAL_STAKE: u64 = 100 * 10_000_000_000_000_000; // 100 KCHNG
const REVIEW_PERIOD_DAYS: u64 = 7;
const VOTE_PERIOD_DAYS: u64 = 3;
const IMPLEMENTATION_NOTICE_DAYS: u64 = 30;

// ============================================================================
// STORAGE KEYS
// ============================================================================

// Storage keys as u32 constants (standard Soroban pattern)
const KEY_ADMIN: u32 = 0;
const KEY_PROTOCOL_VERSION: u32 = 1;
const KEY_TOTAL_SUPPLY: u32 = 2;
const KEY_NEXT_CLAIM_ID: u32 = 3;
const KEY_NEXT_PROPOSAL_ID: u32 = 4;
const KEY_ACCOUNTS: u32 = 100;
const KEY_TRUSTS: u32 = 200;
const KEY_VERIFIERS: u32 = 300;
const KEY_WORK_CLAIMS: u32 = 400;
const KEY_GRACE_PERIODS: u32 = 500;
const KEY_PROPOSALS: u32 = 600;
const KEY_ORACLES: u32 = 700;
const KEY_VERIFIER_ASSIGNMENTS: u32 = 800;

// ============================================================================
// ENUMS
// ============================================================================

/// Type of work being claimed
#[derive(Clone, PartialEq)]
#[contracttype]
pub enum WorkType {
    BasicCare = 0,      // Basic care or agriculture work (1.0× multiplier)
    SkilledCare = 1,    // Skilled care or heavy labor (1.3× multiplier)
    Training = 2,       // Teaching or training (1.5× multiplier)
    EmergencyCare = 3,  // Emergency response (2.0× multiplier)
}

impl WorkType {
    pub fn multiplier(&self) -> u32 {
        match self {
            WorkType::BasicCare => 100,     // 1.0×
            WorkType::SkilledCare => 130,   // 1.3×
            WorkType::Training => 150,      // 1.5×
            WorkType::EmergencyCare => 200, // 2.0×
        }
    }
}

/// Type of grace period
#[derive(Clone, PartialEq)]
#[contracttype]
pub enum GraceType {
    Emergency = 0,     // Emergency pause (14-90 days, oracle-activated)
    Illness = 1,       // Illness or injury (30+ days automatic)
    Community = 2,     // Community voted (30-180 days)
}

/// Status of a work claim
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ClaimStatus {
    Pending = 0,       // Waiting for verification
    Approved = 1,      // Approved and tokens minted
    Rejected = 2,      // Rejected by verifiers
    Expired = 3,       // Verification window expired
}

/// Status of a governance proposal
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ProposalStatus {
    Review = 0,        // In review period (7 days)
    Voting = 1,        // In voting period (3 days)
    Approved = 2,      // Approved, awaiting implementation
    Rejected = 3,      // Rejected by community
    Implemented = 4,   // Successfully implemented
    Expired = 5,       // Expired without passing
}

/// Type of proposal
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ProposalType {
    RateChange = 0,           // Change trust demurrage rate
    TrustParameters = 1,      // Adjust trust parameters
    ProtocolUpgrade = 2,      // Protocol-level upgrade
    Emergency = 3,            // Emergency measure (crisis exception)
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Account data including demurrage tracking
#[contracttype]
pub struct AccountData {
    pub balance: U256,
    pub last_activity: u64,
    pub grace_period_end: u64,     // Timestamp when grace ends (0 if not in grace)
    pub trust_id: Address,         // Trust membership (zero address if none)
    pub contribution_hours: u64,   // Total hours contributed
    pub grace_periods_used: u32,   // Grace periods used this year
    pub last_grace_year: u32,      // Year of last grace period
}

/// Trust (community organization) data
#[derive(Clone)]
#[contracttype]
pub struct TrustData {
    pub name: String,
    pub governor: Address,
    pub annual_rate_bps: u32,      // Annual demurrage rate in basis points
    pub demurrage_period_days: u64,
    pub member_count: u32,
    pub is_active: bool,
    pub created_at: u64,
}

/// Verifier data for work verification
#[derive(Clone)]
#[contracttype]
pub struct VerifierData {
    pub trust_id: Address,
    pub stake: U256,
    pub reputation_score: u32,     // 0-1000
    pub verified_claims: u32,
    pub rejected_claims: u32,
    pub fraud_reports: u32,
}

/// Work claim for time-based token issuance
#[derive(Clone)]
#[contracttype]
pub struct WorkClaim {
    pub claim_id: u64,
    pub worker: Address,
    pub work_type: WorkType,
    pub minutes_worked: u64,
    pub evidence_hash: Bytes,   // Hash of evidence (photo, GPS, notes)
    pub gps_lat: Option<i64>,
    pub gps_lon: Option<i64>,
    pub submitted_at: u64,
    pub verifiers_assigned: Vec<Address>,
    pub approvals_received: u32,
    pub rejections_received: u32,
    pub status: ClaimStatus,
    pub multiplier: u32,
}

/// Grace period data
#[derive(Clone)]
#[contracttype]
pub struct GracePeriod {
    pub account: Address,
    pub grace_type: GraceType,
    pub start_time: u64,
    pub end_time: u64,
    pub oracle_verified: bool,
    pub extension_votes: u32,
}

/// Governance proposal
#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub proposal_id: u64,
    pub proposer: Address,
    pub proposal_type: ProposalType,
    pub title: String,
    pub description: String,
    pub trust_id: Address,             // Zero address for protocol-level
    pub new_rate_bps: Option<u32>,     // For rate change proposals
    pub created_at: u64,
    pub review_end: u64,
    pub vote_end: u64,
    pub implementation_date: u64,
    pub status: ProposalStatus,
    pub votes_for: u32,
    pub votes_against: u32,
    pub voters: Vec<Address>,          // To prevent double voting
}

/// Oracle for grace period verification
#[derive(Clone)]
#[contracttype]
pub struct OracleData {
    pub oracle_address: Address,
    pub stake: U256,
    pub reputation_score: u32,
    pub grace_periods_granted: u32,
}

/// Legacy app demurrage entry (for backward compatibility)
#[contracttype]
pub struct AppDemurrageEntry {
    pub app_id: Address,
    pub additional_rate: u64,
}

/// KCHNG Token Contract with native on-chain demurrage
#[contract]
pub struct KchngToken;

#[contractimpl]
impl KchngToken {
    // Contract constructor - called automatically during deployment
    // Parameters are passed via soroban contract deploy or in tests via env.register()
    pub fn __constructor(env: Env, creator: Address, initial_supply: U256) {
        // Store the creator as admin
        env.storage().instance().set(&KEY_ADMIN, &creator);

        // Set protocol version
        env.storage().instance().set(&KEY_PROTOCOL_VERSION, &U256::from_u32(&env, 1));

        // Set initial balance for creator
        let account_data = AccountData {
            balance: initial_supply.clone(),
            last_activity: env.ledger().timestamp(),
            grace_period_end: 0,
            trust_id: Address::generate(&env), // Use generated address as placeholder
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        };

        let mut accounts: Map<Address, AccountData> = Map::new(&env);
        accounts.set(creator.clone(), account_data);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Track total supply
        env.storage().instance().set(&KEY_TOTAL_SUPPLY, &initial_supply);

        // Initialize counters
        env.storage().instance().set(&KEY_NEXT_CLAIM_ID, &U256::from_u32(&env, 1));
        env.storage().instance().set(&KEY_NEXT_PROPOSAL_ID, &U256::from_u32(&env, 1));
    }

    /// Initialize the token with initial supply to the creator (legacy method)
    pub fn init(env: Env, creator: Address, initial_supply: U256) {
        // Check if already initialized
        if env.storage().persistent().has(&KEY_ACCOUNTS) {
            let admin_result: Option<Address> = env.storage().instance().get(&KEY_ADMIN);
            if let Some(admin) = admin_result {
                if admin != creator {
                    panic!("Already initialized with different admin");
                }
                return;
            }
        }

        // Store the creator as admin
        env.storage().instance().set(&KEY_ADMIN, &creator);

        // Set protocol version
        env.storage().instance().set(&KEY_PROTOCOL_VERSION, &U256::from_u32(&env, 1));

        // Set initial balance for creator
        let account_data = AccountData {
            balance: initial_supply.clone(),
            last_activity: env.ledger().timestamp(),
            grace_period_end: 0,
            trust_id: Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        };

        let mut accounts: Map<Address, AccountData> = Map::new(&env);
        accounts.set(creator.clone(), account_data);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Track total supply
        env.storage().instance().set(&KEY_TOTAL_SUPPLY, &initial_supply);

        // Initialize counters
        env.storage().instance().set(&KEY_NEXT_CLAIM_ID, &U256::from_u32(&env, 1));
        env.storage().instance().set(&KEY_NEXT_PROPOSAL_ID, &U256::from_u32(&env, 1));
    }

    /// Get the current balance of an account (after applying demurrage)
    pub fn balance(env: Env, account: Address) -> U256 {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        if let Some(data) = accounts.get(account.clone()) {
            Self::calculate_balance_with_demurrage(&env, account, &data)
        } else {
            U256::from_u32(&env, 0)
        }
    }

    /// Transfer tokens from one account to another
    pub fn transfer(env: Env, from: Address, to: Address, amount: U256) {
        from.require_auth();

        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let from_data = match accounts.get(from.clone()) {
            Some(data) => data,
            None => panic!("Insufficient balance"),
        };

        let balance_after_demurrage =
            Self::calculate_balance_with_demurrage(&env, from.clone(), &from_data);

        if balance_after_demurrage < amount {
            panic!("Insufficient balance");
        }

        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let current_time = env.ledger().timestamp();
        let zero_address = Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");

        // Update sender
        let mut updated_from = from_data;
        updated_from.balance = balance_after_demurrage.sub(&amount);
        updated_from.last_activity = current_time;
        accounts.set(from.clone(), updated_from);

        // Get and update recipient
        let to_data = accounts.get(to.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: current_time,
            grace_period_end: 0,
            trust_id: zero_address.clone(),
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        let mut updated_to = to_data;
        updated_to.balance = updated_to.balance.add(&amount);
        updated_to.last_activity = current_time;
        accounts.set(to.clone(), updated_to);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
    }

    /// Mint new tokens (admin only)
    pub fn mint(env: Env, admin: Address, to: Address, amount: U256) {
        let stored_admin: Address = env.storage().instance().get(&KEY_ADMIN).unwrap();
        admin.require_auth();
        if admin != stored_admin {
            panic!("Not authorized");
        }

        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let current_time = env.ledger().timestamp();
        let zero_address = Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        let to_data = accounts.get(to.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: current_time,
            grace_period_end: 0,
            trust_id: zero_address.clone(),
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        let mut updated_to = to_data;
        updated_to.balance = updated_to.balance.add(&amount);
        updated_to.last_activity = current_time;
        accounts.set(to.clone(), updated_to);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Update total supply
        let mut total_supply: U256 = env.storage().instance().get(&KEY_TOTAL_SUPPLY).unwrap();
        total_supply = total_supply.add(&amount);
        env.storage().instance().set(&KEY_TOTAL_SUPPLY, &total_supply);
    }

    /// Get the total supply
    pub fn total_supply(env: Env) -> U256 {
        env.storage().instance().get(&KEY_TOTAL_SUPPLY).unwrap()
    }

    /// Register an app for additional demurrage logic
    pub fn register_app(env: Env, admin: Address, app_id: Address, additional_rate: u64) {
        let stored_admin: Address = env.storage().instance().get(&KEY_ADMIN).unwrap();
        admin.require_auth();
        if admin != stored_admin {
            panic!("Not authorized");
        }

        let mut apps: Map<Address, AppDemurrageEntry> =
            env.storage().persistent().get(&U256::from_u32(&env, 4)).unwrap();

        let entry = AppDemurrageEntry {
            app_id: app_id.clone(),
            additional_rate,
        };
        apps.set(app_id, entry);

        env.storage().persistent().set(&U256::from_u32(&env, 4), &apps);
    }

    // ============================================================================
    // TRUST SYSTEM (Phase 2)
    // ============================================================================

    /// Register a new community trust
    /// Parameters:
    /// - governor: Address that will govern this trust
    /// - name: Human-readable name for the trust
    /// - annual_rate_bps: Annual demurrage rate in basis points (500-1500 = 5-15%)
    /// - demurrage_period_days: How often to apply demurrage (default: 30 days)
    pub fn register_trust(
        env: Env,
        governor: Address,
        name: String,
        annual_rate_bps: u32,
        demurrage_period_days: u64,
    ) {
        governor.require_auth();

        // Validate rate is within protocol constraints
        if annual_rate_bps < MIN_ANNUAL_RATE_BPS || annual_rate_bps > MAX_ANNUAL_RATE_BPS {
            panic!("Rate must be between 5% and 15% annually");
        }

        // Validate period is reasonable (7-365 days)
        if demurrage_period_days < 7 || demurrage_period_days > 365 {
            panic!("Period must be between 7 and 365 days");
        }

        // Use governor address as the trust ID for simplicity
        let trust_id = governor.clone();

        // Check if trust already exists
        let trusts: Map<Address, TrustData> = env
            .storage()
            .persistent()
            .get(&KEY_TRUSTS)
            .unwrap_or(Map::new(&env));

        if trusts.contains_key(trust_id.clone()) {
            panic!("Trust already exists for this governor");
        }

        // Create trust data
        let trust = TrustData {
            name: name.clone(),
            governor: governor.clone(),
            annual_rate_bps,
            demurrage_period_days,
            member_count: 1, // Governor counts as first member
            is_active: true,
            created_at: env.ledger().timestamp(),
        };

        // Store trust
        let mut trusts: Map<Address, TrustData> = env
            .storage()
            .persistent()
            .get(&KEY_TRUSTS)
            .unwrap_or(Map::new(&env));
        trusts.set(trust_id, trust);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);

        // Update governor's account to be part of this trust
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
        let governor_data = accounts.get(governor.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: env.ledger().timestamp(),
            grace_period_end: 0,
            trust_id: Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        let mut updated_governor = governor_data;
        updated_governor.trust_id = governor.clone();
        accounts.set(governor, updated_governor);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
    }

    /// Join an existing trust
    pub fn join_trust(env: Env, member: Address, trust_id: Address) {
        member.require_auth();

        // Get trust data
        let trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        let trust = match trusts.get(trust_id.clone()) {
            Some(t) => t,
            None => panic!("Trust not found"),
        };

        if !trust.is_active {
            panic!("Trust is not active");
        }

        // Get and update member's account
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let member_data = match accounts.get(member.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Check if already in a trust
        let zero_address = Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        if member_data.trust_id != zero_address {
            panic!("Already a member of a trust");
        }

        // Update member's trust membership
        let mut updated_member = member_data;
        updated_member.trust_id = trust_id.clone();
        accounts.set(member.clone(), updated_member);

        // Update trust member count
        let mut trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        let mut updated_trust = trust;
        updated_trust.member_count += 1;
        trusts.set(trust_id, updated_trust);

        // Save changes
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);
    }

    /// Get information about a specific trust
    pub fn get_trust_info(env: Env, trust_id: Address) -> TrustData {
        let trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        match trusts.get(trust_id) {
            Some(trust) => trust,
            None => panic!("Trust not found"),
        }
    }

    /// Get list of all registered trust IDs
    pub fn get_all_trusts(env: Env) -> Vec<Address> {
        let trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        let mut trust_ids = Vec::new(&env);
        for (trust_id, _) in trusts.iter() {
            trust_ids.push_back(trust_id);
        }
        trust_ids
    }

    /// Get the trust ID for an account
    pub fn get_account_trust(env: Env, account: Address) -> Address {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        match accounts.get(account) {
            Some(data) => data.trust_id,
            None => {
                // Return zero address for accounts not in a trust
                Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=")
            }
        }
    }

    /// Get full account data
    pub fn get_account(env: Env, account: Address) -> AccountData {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        match accounts.get(account) {
            Some(data) => data,
            None => {
                // Return default account data for new accounts
                AccountData {
                    balance: U256::from_u32(&env, 0),
                    last_activity: env.ledger().timestamp(),
                    grace_period_end: 0,
                    trust_id: Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="),
                    contribution_hours: 0,
                    grace_periods_used: 0,
                    last_grace_year: 0,
                }
            }
        }
    }

    /// Get oracle data
    pub fn get_oracle(env: Env, oracle: Address) -> OracleData {
        let oracles: Map<Address, OracleData> =
            env.storage().persistent().get(&KEY_ORACLES).unwrap();

        match oracles.get(oracle) {
            Some(data) => data,
            None => {
                panic!("Oracle not found");
            }
        }
    }

    // ============================================================================
    // ENHANCED DEMURRAGE (Phase 3)
    // ============================================================================

    /// Calculate balance after applying trust-specific percentage-based demurrage
    /// Uses the Wörgl model: ~12.7% annual (1% monthly) with trust-specific rates
    fn calculate_balance_with_demurrage(
        env: &Env,
        _account: Address,
        data: &AccountData,
    ) -> U256 {
        let current_timestamp = env.ledger().timestamp();

        // Check if account is in a grace period
        if data.grace_period_end > 0 && current_timestamp < data.grace_period_end {
            // Demurrage is paused during grace period
            return data.balance.clone();
        }

        let last_activity: u64 = data.last_activity;

        if current_timestamp <= last_activity {
            return data.balance.clone();
        }

        // Get trust-specific demurrage parameters
        let zero_address = Address::from_str(env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        let (annual_rate_bps, period_days) = if data.trust_id == zero_address {
            // Not in a trust, use default rate (12% annual, 30 day period)
            (DEFAULT_ANNUAL_RATE_BPS, DEFAULT_PERIOD_DAYS)
        } else {
            // Get trust data
            let trusts: Map<Address, TrustData> =
                env.storage().persistent().get(&KEY_TRUSTS).unwrap();

            match trusts.get(data.trust_id.clone()) {
                Some(trust) => (trust.annual_rate_bps, trust.demurrage_period_days),
                None => {
                    // Trust not found, use default
                    (DEFAULT_ANNUAL_RATE_BPS, DEFAULT_PERIOD_DAYS)
                }
            }
        };

        let inactive_seconds = current_timestamp - last_activity;
        let inactive_days = inactive_seconds / SECONDS_PER_DAY;

        // Calculate how many complete demurrage periods have passed
        if inactive_days < period_days {
            return data.balance.clone();
        }

        let periods = inactive_days / period_days;

        // Calculate percentage-based demurrage
        // Formula: balance * (1 - (annual_rate / 100) / (365 / period_days))^periods
        // Simplified: For 1% monthly (12% annual), each period reduces by 1%
        // period_rate_bps = annual_rate_bps * period_days / 36500

        // Calculate the per-period rate in basis points
        // Example: 1200 bps annual (12%), 30 day period
        // period_rate = 1200 * 30 / 36500 ≈ 0.986% per period (roughly 1%)
        let period_rate_bps = (annual_rate_bps as u64) * period_days / 36500;

        // Calculate total burn amount across all periods
        let mut balance = data.balance.clone();

        for _ in 0..periods {
            // Calculate burn for this period: balance * period_rate_bps / 10000
            let burn_amount = {
                // balance * period_rate_bps / 10000
                let rate_factor = U256::from_u128(env, period_rate_bps as u128);
                let tmp = balance.mul(&rate_factor);
                tmp.div(&U256::from_u128(env, 10000))
            };

            balance = if balance > burn_amount {
                balance.sub(&burn_amount)
            } else {
                U256::from_u32(env, 0)
            };

            // Early exit if balance is zero
            if balance == U256::from_u32(env, 0) {
                break;
            }
        }

        balance
    }

    /// Get the effective demurrage rate for an account
    pub fn get_account_demurrage_rate(env: Env, account: Address) -> (u32, u64) {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let account_data = match accounts.get(account) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        let zero_address = Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");

        if account_data.trust_id == zero_address {
            (DEFAULT_ANNUAL_RATE_BPS, DEFAULT_PERIOD_DAYS)
        } else {
            let trust = Self::get_trust_info(env, account_data.trust_id);
            (trust.annual_rate_bps, trust.demurrage_period_days)
        }
    }

    // ============================================================================
    // WORK VERIFICATION SYSTEM (Phase 4)
    // ============================================================================

    /// Register as a verifier for a trust
    /// Must stake 100 KCHNG to become a verifier
    pub fn register_verifier(env: Env, verifier: Address, trust_id: Address) {
        verifier.require_auth();

        // Verify trust exists
        let _trust = Self::get_trust_info(env.clone(), trust_id.clone());

        // Get verifier's account
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let verifier_account = match accounts.get(verifier.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Check verifier has enough balance to stake
        let stake_amount = U256::from_u128(&env, VERIFIER_STAKE as u128);
        let balance_after_demurrage =
            Self::calculate_balance_with_demurrage(&env, verifier.clone(), &verifier_account);

        if balance_after_demurrage < stake_amount {
            panic!("Insufficient balance to stake");
        }

        // Register verifier
        let mut verifiers: Map<Address, VerifierData> =
            env.storage().persistent().get(&KEY_VERIFIERS).unwrap_or(Map::new(&env));

        if verifiers.contains_key(verifier.clone()) {
            panic!("Already registered as verifier");
        }

        let verifier_data = VerifierData {
            trust_id: trust_id.clone(),
            stake: stake_amount.clone(),
            reputation_score: 500, // Start at neutral reputation
            verified_claims: 0,
            rejected_claims: 0,
            fraud_reports: 0,
        };

        verifiers.set(verifier.clone(), verifier_data);
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

        // Deduct stake from verifier's balance (transfer to contract)
        // For simplicity, we'll just track the stake - in production you'd escrow it
        let mut updated_verifier = verifier_account;
        updated_verifier.balance = balance_after_demurrage.sub(&stake_amount);
        updated_verifier.last_activity = env.ledger().timestamp();
        accounts.set(verifier, updated_verifier);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
    }

    /// Submit a work claim for verification
    /// Returns the claim ID
    pub fn submit_work_claim(
        env: Env,
        worker: Address,
        work_type: WorkType,
        minutes_worked: u64,
        evidence_hash: Bytes,
        gps_lat: Option<i64>,
        gps_lon: Option<i64>,
    ) -> u64 {
        worker.require_auth();

        // Validate minimum work time
        if minutes_worked < MIN_WORK_MINUTES {
            panic!("Work must be at least 15 minutes");
        }

        // Get worker's account
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let worker_account = match accounts.get(worker.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Worker must be in a trust
        let zero_address = Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        if worker_account.trust_id == zero_address {
            panic!("Must join a trust before submitting work claims");
        }

        // Get next claim ID
        let mut claim_id_u256: U256 = env.storage().instance().get(&KEY_NEXT_CLAIM_ID).unwrap();
        let claim_id = claim_id_u256.to_u128().unwrap() as u64;

        // Assign verifiers (2-5 random verifiers from the same trust)
        let verifiers: Map<Address, VerifierData> =
            env.storage().persistent().get(&KEY_VERIFIERS).unwrap_or(Map::new(&env));

        let mut trust_verifiers: Vec<Address> = Vec::new(&env);
        for (verifier_addr, verifier_data) in verifiers.iter() {
            if verifier_data.trust_id == worker_account.trust_id {
                trust_verifiers.push_back(verifier_addr);
            }
        }

        if trust_verifiers.len() < MIN_VERIFIERS {
            panic!("Not enough verifiers in trust");
        }

        // Assign MIN_VERIFIERS verifiers (in production, would select randomly)
        let mut assigned_verifiers: Vec<Address> = Vec::new(&env);
        let count = min(MIN_VERIFIERS, trust_verifiers.len());
        for i in 0..count {
            if i < trust_verifiers.len() {
                assigned_verifiers.push_back(trust_verifiers.get(i).unwrap());
            }
        }

        // Calculate multiplier and tokens to mint
        let multiplier = work_type.multiplier();
        let base_kchng = minutes_worked / MINUTES_PER_KCHNG; // 30 min = 1 KCHNG
        let _kchng_to_mint = (base_kchng * multiplier as u64) / 100;

        // Create work claim
        let claim = WorkClaim {
            claim_id,
            worker: worker.clone(),
            work_type,
            minutes_worked,
            evidence_hash,
            gps_lat,
            gps_lon,
            submitted_at: env.ledger().timestamp(),
            verifiers_assigned: assigned_verifiers.clone(),
            approvals_received: 0,
            rejections_received: 0,
            status: ClaimStatus::Pending,
            multiplier,
        };

        // Store claim
        let mut claims: Map<u64, WorkClaim> =
            env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap_or(Map::new(&env));
        claims.set(claim_id, claim);
        env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

        // Store verifier assignments for lookup
        let mut assignments: Map<u64, Vec<Address>> =
            env.storage().persistent().get(&KEY_VERIFIER_ASSIGNMENTS).unwrap_or(Map::new(&env));
        assignments.set(claim_id, assigned_verifiers);
        env.storage().persistent().set(&KEY_VERIFIER_ASSIGNMENTS, &assignments);

        // Increment claim ID counter
        claim_id_u256 = U256::from_u128(&env, (claim_id + 1) as u128);
        env.storage().instance().set(&KEY_NEXT_CLAIM_ID, &claim_id_u256);

        claim_id
    }

    /// Approve a work claim (verifier only)
    pub fn approve_work_claim(env: Env, verifier: Address, claim_id: u64) {
        verifier.require_auth();

        let mut claims: Map<u64, WorkClaim> =
            env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap();
        let mut claim = match claims.get(claim_id) {
            Some(c) => c,
            None => panic!("Claim not found"),
        };

        if claim.status != ClaimStatus::Pending {
            panic!("Claim is not pending");
        }

        // Verify this verifier was assigned to this claim
        let assignments: Map<u64, Vec<Address>> =
            env.storage().persistent().get(&KEY_VERIFIER_ASSIGNMENTS).unwrap_or(Map::new(&env));
        let assigned_verifiers = match assignments.get(claim_id) {
            Some(v) => v,
            None => panic!("No verifiers assigned"),
        };

        let mut is_assigned = false;
        for i in 0..assigned_verifiers.len() {
            if assigned_verifiers.get(i).unwrap() == verifier {
                is_assigned = true;
                break;
            }
        }

        if !is_assigned {
            panic!("Verifier not assigned to this claim");
        }

        // Record approval
        claim.approvals_received += 1;
        claims.set(claim_id, claim.clone());
        env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

        // Update verifier stats
        let mut verifiers: Map<Address, VerifierData> =
            env.storage().persistent().get(&KEY_VERIFIERS).unwrap();
        let mut verifier_data = verifiers.get(verifier.clone()).unwrap();
        verifier_data.verified_claims += 1;
        verifiers.set(verifier, verifier_data);
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

        // Check if we have enough approvals (simple majority)
        // Need more than half of assigned verifiers to approve
        let total_verifiers = assigned_verifiers.len() as u32;
        let required = (total_verifiers / 2) + 1;

        if claim.approvals_received >= required {
            // Mint tokens to worker
            let base_kchng = claim.minutes_worked / MINUTES_PER_KCHNG;
            let kchng_to_mint = (base_kchng * claim.multiplier as u64) / 100;
            let amount = U256::from_u128(&env, kchng_to_mint as u128);

            let mut accounts: Map<Address, AccountData> =
                env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

            let worker_data = accounts.get(claim.worker.clone()).unwrap();
            let mut updated_worker = worker_data;
            updated_worker.balance = updated_worker.balance.add(&amount);
            updated_worker.last_activity = env.ledger().timestamp();
            updated_worker.contribution_hours += claim.minutes_worked / 60;
            accounts.set(claim.worker.clone(), updated_worker);
            env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

            // Update total supply
            let mut total_supply: U256 = env.storage().instance().get(&KEY_TOTAL_SUPPLY).unwrap();
            total_supply = total_supply.add(&amount);
            env.storage().instance().set(&KEY_TOTAL_SUPPLY, &total_supply);

            // Mark claim as approved
            claim.status = ClaimStatus::Approved;
            claims.set(claim_id, claim);
            env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);
        }
    }

    /// Reject a work claim (verifier only)
    pub fn reject_work_claim(env: Env, verifier: Address, claim_id: u64) {
        verifier.require_auth();

        let mut claims: Map<u64, WorkClaim> =
            env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap();
        let mut claim = match claims.get(claim_id) {
            Some(c) => c,
            None => panic!("Claim not found"),
        };

        if claim.status != ClaimStatus::Pending {
            panic!("Claim is not pending");
        }

        // Verify this verifier was assigned to this claim
        let assignments: Map<u64, Vec<Address>> =
            env.storage().persistent().get(&KEY_VERIFIER_ASSIGNMENTS).unwrap_or(Map::new(&env));
        let assigned_verifiers = match assignments.get(claim_id) {
            Some(v) => v,
            None => panic!("No verifiers assigned"),
        };

        let mut is_assigned = false;
        for i in 0..assigned_verifiers.len() {
            if assigned_verifiers.get(i).unwrap() == verifier {
                is_assigned = true;
                break;
            }
        }

        if !is_assigned {
            panic!("Verifier not assigned to this claim");
        }

        // Record rejection
        claim.rejections_received += 1;
        claims.set(claim_id, claim.clone());
        env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

        // Update verifier stats
        let mut verifiers: Map<Address, VerifierData> =
            env.storage().persistent().get(&KEY_VERIFIERS).unwrap();
        let mut verifier_data = verifiers.get(verifier.clone()).unwrap();
        verifier_data.rejected_claims += 1;
        verifiers.set(verifier, verifier_data);
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

        // Check if we have enough rejections to reject the claim
        let total_verifiers = assigned_verifiers.len() as u32;
        let required = (total_verifiers / 2) + 1;

        if claim.rejections_received >= required {
            // Mark claim as rejected
            claim.status = ClaimStatus::Rejected;
            claims.set(claim_id, claim);
            env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);
        }
    }

    /// Get work claim details
    pub fn get_work_claim(env: Env, claim_id: u64) -> WorkClaim {
        let claims: Map<u64, WorkClaim> =
            env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap();

        match claims.get(claim_id) {
            Some(claim) => claim,
            None => panic!("Claim not found"),
        }
    }

    /// Get pending claims for a verifier
    pub fn get_verifier_pending_claims(env: Env, verifier: Address) -> Vec<u64> {
        let claims: Map<u64, WorkClaim> =
            env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap_or(Map::new(&env));

        let mut pending_claims = Vec::new(&env);
        for (claim_id, claim) in claims.iter() {
            if claim.status == ClaimStatus::Pending {
                // Check if verifier is assigned
                for i in 0..claim.verifiers_assigned.len() {
                    if claim.verifiers_assigned.get(i).unwrap() == verifier {
                        pending_claims.push_back(claim_id);
                        break;
                    }
                }
            }
        }
        pending_claims
    }

    // ============================================================================
    // GRACE PERIOD SYSTEM (Phase 5)
    // ============================================================================

    /// Register as a grace period oracle
    /// Oracles can activate grace periods for accounts in hardship
    pub fn register_oracle(env: Env, oracle: Address) {
        oracle.require_auth();

        let mut oracles: Map<Address, OracleData> =
            env.storage().persistent().get(&KEY_ORACLES).unwrap_or(Map::new(&env));

        if oracles.contains_key(oracle.clone()) {
            panic!("Already registered as oracle");
        }

        // Verify oracle has enough balance to stake
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let oracle_account = match accounts.get(oracle.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Minimum stake for oracles
        let oracle_stake = U256::from_u128(&env, 500 * 10_000_000_000_000_000); // 500 KCHNG

        // Simple balance check (without demurrage for oracle registration)
        if oracle_account.balance < oracle_stake {
            panic!("Insufficient balance to register as oracle");
        }

        let oracle_data = OracleData {
            oracle_address: oracle.clone(),
            stake: oracle_stake,
            reputation_score: 500,
            grace_periods_granted: 0,
        };

        oracles.set(oracle, oracle_data);
        env.storage().persistent().set(&KEY_ORACLES, &oracles);
    }

    /// Activate a grace period for an account
    /// Parameters:
    /// - oracle: Address of the oracle activating the grace period
    /// - account: Account to activate grace period for
    /// - grace_type: Type of grace period
    /// - duration_days: Length of grace period
    pub fn activate_grace_period(
        env: Env,
        oracle: Address,
        account: Address,
        grace_type: GraceType,
        duration_days: u64,
    ) {
        oracle.require_auth();

        // Verify oracle is registered
        let oracles: Map<Address, OracleData> =
            env.storage().persistent().get(&KEY_ORACLES).unwrap_or(Map::new(&env));

        if !oracles.contains_key(oracle.clone()) {
            panic!("Not a registered oracle");
        }

        // Get account data
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let account_data = match accounts.get(account.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Check anti-abuse: max 3 grace periods per year, requires 30+ contribution hours
        let current_year = env.ledger().timestamp() / (365 * SECONDS_PER_DAY);

        if account_data.last_grace_year == current_year as u32 {
            if account_data.grace_periods_used >= MAX_GRACE_PERIODS_PER_YEAR {
                panic!("Maximum grace periods used for this year");
            }
        }

        if account_data.contribution_hours < MIN_CONTRIBUTION_HOURS {
            panic!("Must have at least 30 contribution hours to qualify for grace period");
        }

        // Validate duration based on grace type
        let max_days = match grace_type {
            GraceType::Emergency => 90,   // Emergency: up to 90 days
            GraceType::Illness => 60,     // Illness: up to 60 days
            GraceType::Community => 180,  // Community: up to 180 days
        };

        if duration_days > max_days {
            panic!("Duration exceeds maximum for this grace type");
        }

        let current_time = env.ledger().timestamp();
        let end_time = current_time + (duration_days * SECONDS_PER_DAY);

        // Create grace period
        let grace_period = GracePeriod {
            account: account.clone(),
            grace_type,
            start_time: current_time,
            end_time,
            oracle_verified: true,
            extension_votes: 0,
        };

        // Store grace period
        let mut grace_periods: Map<Address, GracePeriod> =
            env.storage().persistent().get(&KEY_GRACE_PERIODS).unwrap_or(Map::new(&env));
        grace_periods.set(account.clone(), grace_period);
        env.storage().persistent().set(&KEY_GRACE_PERIODS, &grace_periods);

        // Update account data
        let mut updated_account = account_data;
        updated_account.grace_period_end = end_time;

        if updated_account.last_grace_year != current_year as u32 {
            updated_account.last_grace_year = current_year as u32;
            updated_account.grace_periods_used = 1;
        } else {
            updated_account.grace_periods_used += 1;
        }

        accounts.set(account.clone(), updated_account);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Update oracle stats
        let mut oracles: Map<Address, OracleData> =
            env.storage().persistent().get(&KEY_ORACLES).unwrap();
        let mut oracle_data = oracles.get(oracle.clone()).unwrap();
        oracle_data.grace_periods_granted += 1;
        oracles.set(oracle, oracle_data);
        env.storage().persistent().set(&KEY_ORACLES, &oracles);
    }

    /// Check if an account is currently in a grace period
    pub fn is_in_grace_period(env: Env, account: Address) -> bool {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        match accounts.get(account) {
            Some(data) => {
                let current_time = env.ledger().timestamp();
                data.grace_period_end > 0 && current_time < data.grace_period_end
            }
            None => false,
        }
    }

    /// Get grace period details for an account
    pub fn get_grace_period(env: Env, account: Address) -> Option<GracePeriod> {
        let grace_periods: Map<Address, GracePeriod> =
            env.storage().persistent().get(&KEY_GRACE_PERIODS).unwrap_or(Map::new(&env));

        grace_periods.get(account)
    }

    /// Extend an existing grace period (requires community voting)
    pub fn extend_grace_period(env: Env, account: Address, additional_days: u64) {
        // Check if account has an active grace period
        let mut grace_periods: Map<Address, GracePeriod> =
            env.storage().persistent().get(&KEY_GRACE_PERIODS).unwrap_or(Map::new(&env));

        let mut grace_period = match grace_periods.get(account.clone()) {
            Some(gp) => gp,
            None => panic!("No active grace period found"),
        };

        // Can only extend community-voted grace periods
        if grace_period.grace_type != GraceType::Community {
            panic!("Only community grace periods can be extended");
        }

        // Maximum total duration for community grace periods is 180 days
        let current_duration = (grace_period.end_time - grace_period.start_time) / SECONDS_PER_DAY;
        let new_duration = current_duration + additional_days;

        if new_duration > 180 {
            panic!("Extended grace period would exceed maximum of 180 days");
        }

        // Extend the grace period
        grace_period.end_time += additional_days * SECONDS_PER_DAY;
        grace_period.extension_votes += 1;

        let new_end_time = grace_period.end_time;

        grace_periods.set(account.clone(), grace_period);
        env.storage().persistent().set(&KEY_GRACE_PERIODS, &grace_periods);

        // Update account's grace_period_end
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let account_data = match accounts.get(account.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        let mut updated_account = account_data;
        updated_account.grace_period_end = new_end_time;
        accounts.set(account, updated_account);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
    }

    // ============================================================================
    // CROSS-TRUST EXCHANGE (Phase 6)
    // ============================================================================

    /// Calculate exchange rate between two trusts
    /// Returns the multiplier for converting from source to destination trust
    /// Formula: (1 - r_source) / (1 - r_dest)
    /// Example: Trust A (12%) → Trust B (8%) = (1 - 0.12) / (1 - 0.08) = 0.957
    pub fn calculate_exchange_rate(env: Env, source_trust: Address, dest_trust: Address) -> u64 {
        let source_trust_data = Self::get_trust_info(env.clone(), source_trust);
        let dest_trust_data = Self::get_trust_info(env, dest_trust);

        // Convert basis points to rate (e.g., 1200 bps = 0.12)
        let source_rate = source_trust_data.annual_rate_bps as u64;
        let dest_rate = dest_trust_data.annual_rate_bps as u64;

        // Formula: (1 - r_source) / (1 - r_dest)
        // Using basis points: (10000 - source_rate) / (10000 - dest_rate)
        let numerator = 10000 - source_rate;
        let denominator = 10000 - dest_rate;

        // Return as basis points (multiply by 10000 for precision)
        (numerator * 10000) / denominator
    }

    /// Swap tokens from source trust to destination trust
    /// Uses rate-adjusted calculation to account for different demurrage rates
    pub fn cross_trust_swap(
        env: Env,
        from: Address,
        dest_trust: Address,
        amount: U256,
    ) {
        from.require_auth();

        // Get accounts
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let from_data = match accounts.get(from.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Get from trust
        let from_trust = from_data.trust_id.clone();

        let zero_address = Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        if from_trust == zero_address {
            panic!("Must be in a trust to perform cross-trust swap");
        }

        // Calculate exchange rate
        let exchange_rate_bps = Self::calculate_exchange_rate(env.clone(), from_trust.clone(), dest_trust.clone());

        // Calculate destination amount: amount * exchange_rate / 10000
        let _dest_amount = {
            let rate_factor = U256::from_u128(&env, exchange_rate_bps as u128);
            let tmp = amount.mul(&rate_factor);
            tmp.div(&U256::from_u128(&env, 10000))
        };

        // Check from account has enough balance after demurrage
        let balance_after_demurrage =
            Self::calculate_balance_with_demurrage(&env, from.clone(), &from_data);

        if balance_after_demurrage < amount {
            panic!("Insufficient balance");
        }

        // Update from account (deduct amount, update trust membership)
        let mut updated_from = from_data;
        updated_from.balance = balance_after_demurrage.sub(&amount);
        updated_from.last_activity = env.ledger().timestamp();
        updated_from.trust_id = dest_trust.clone(); // Move to destination trust
        accounts.set(from.clone(), updated_from);

        // Update trust member counts
        let mut trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        // Decrement source trust count
        if let Some(mut source_trust_data) = trusts.get(from_trust.clone()) {
            source_trust_data.member_count -= 1;
            trusts.set(from_trust, source_trust_data);
        }

        // Increment destination trust count
        if let Some(mut dest_trust_data) = trusts.get(dest_trust.clone()) {
            dest_trust_data.member_count += 1;
            trusts.set(dest_trust, dest_trust_data);
        }

        env.storage().persistent().set(&KEY_TRUSTS, &trusts);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
    }

    /// Simulate a cross-trust swap without executing it
    /// Returns the amount that would be received in the destination trust
    pub fn simulate_cross_trust_swap(
        env: Env,
        source_trust: Address,
        dest_trust: Address,
        amount: U256,
    ) -> U256 {
        let exchange_rate_bps = Self::calculate_exchange_rate(env.clone(), source_trust, dest_trust);

        let rate_factor = U256::from_u128(&env, exchange_rate_bps as u128);
        let tmp = amount.mul(&rate_factor);
        tmp.div(&U256::from_u128(&env, 10000))
    }

    // =========================================================================
    // GOVERNANCE SYSTEM
    // =========================================================================

    /// Create a governance proposal
    /// Proposer must be a trust governor for trust-specific proposals
    /// or admin for protocol-level proposals
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        proposal_type: ProposalType,
        title: String,
        description: String,
        trust_id: Address,
        new_rate_bps: Option<u32>,
    ) -> u64 {
        proposer.require_auth();

        // Validate proposer authority
        match proposal_type {
            ProposalType::RateChange | ProposalType::TrustParameters => {
                // Must be governor of the trust
                let trusts: Map<Address, TrustData> =
                    env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                let trust_data = match trusts.get(trust_id.clone()) {
                    Some(data) => data,
                    None => panic!("Trust not found"),
                };

                if trust_data.governor != proposer {
                    panic!("Only trust governors can propose trust changes");
                }
            }
            ProposalType::ProtocolUpgrade | ProposalType::Emergency => {
                // Must be admin
                let admin: Address = env.storage().instance().get(&KEY_ADMIN).unwrap();
                if admin != proposer {
                    panic!("Only admin can propose protocol changes");
                }
            }
        }

        // Validate rate change if provided
        if let Some(rate) = new_rate_bps {
            if rate < MIN_ANNUAL_RATE_BPS || rate > MAX_ANNUAL_RATE_BPS {
                panic!("Rate must be within protocol bounds (5-15%)");
            }
        }

        // Calculate proposal timeline
        let current_timestamp = env.ledger().timestamp();
        let review_end = current_timestamp + (REVIEW_PERIOD_DAYS * SECONDS_PER_DAY);
        let vote_end = review_end + (VOTE_PERIOD_DAYS * SECONDS_PER_DAY);
        let implementation_date = vote_end + (IMPLEMENTATION_NOTICE_DAYS * SECONDS_PER_DAY);

        // Generate proposal ID
        let mut proposals: Map<u64, Proposal> =
            env.storage().persistent().get(&KEY_PROPOSALS).unwrap_or(Map::new(&env));
        let proposal_id: u64 = proposals.len().into();

        // Create proposal
        let proposal = Proposal {
            proposal_id,
            proposer: proposer.clone(),
            proposal_type,
            title,
            description,
            trust_id,
            new_rate_bps,
            created_at: current_timestamp,
            review_end,
            vote_end,
            implementation_date,
            status: ProposalStatus::Review,
            votes_for: 0,
            votes_against: 0,
            voters: Vec::new(&env),
        };

        proposals.set(proposal_id, proposal);
        env.storage().persistent().set(&KEY_PROPOSALS, &proposals);

        proposal_id
    }

    /// Vote on a governance proposal
    /// Only trust members can vote on trust-specific proposals
    /// Admin can vote on protocol proposals
    pub fn vote_on_proposal(env: Env, voter: Address, proposal_id: u64, support: bool) {
        voter.require_auth();

        let mut proposals: Map<u64, Proposal> =
            env.storage().persistent().get(&KEY_PROPOSALS).unwrap();

        let mut proposal = match proposals.get(proposal_id) {
            Some(p) => p,
            None => panic!("Proposal not found"),
        };

        // Check if proposal is in voting period
        if proposal.status != ProposalStatus::Voting {
            panic!("Proposal is not in voting period");
        }

        let current_timestamp = env.ledger().timestamp();

        // Check if voting period has ended
        if current_timestamp > proposal.vote_end {
            proposal.status = ProposalStatus::Expired;
            proposals.set(proposal_id, proposal);
            env.storage().persistent().set(&KEY_PROPOSALS, &proposals);
            panic!("Voting period has ended");
        }

        // Check if already voted
        if proposal.voters.contains(voter.clone()) {
            panic!("Already voted on this proposal");
        }

        // Verify voter is a trust member for trust-specific proposals
        let zero_address = Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
        if proposal.trust_id != zero_address {
            let accounts: Map<Address, AccountData> =
                env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

            let account = match accounts.get(voter.clone()) {
                Some(a) => a,
                None => panic!("Not a trust member"),
            };

            if account.trust_id != proposal.trust_id {
                panic!("Not a member of this trust");
            }
        }

        // Record vote
        if support {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }

        // Add to voters list
        proposal.voters.push_back(voter);

        proposals.set(proposal_id, proposal);
        env.storage().persistent().set(&KEY_PROPOSALS, &proposals);
    }

    /// Process a proposal and update its status
    /// Transitions from Review to Voting, or Voting to Approved/Rejected
    pub fn process_proposal(env: Env, proposal_id: u64) {
        let mut proposals: Map<u64, Proposal> =
            env.storage().persistent().get(&KEY_PROPOSALS).unwrap();

        let mut proposal = match proposals.get(proposal_id) {
            Some(p) => p,
            None => panic!("Proposal not found"),
        };

        let current_timestamp = env.ledger().timestamp();

        match proposal.status {
            ProposalStatus::Review => {
                // Transition to voting if review period ended
                if current_timestamp >= proposal.review_end {
                    proposal.status = ProposalStatus::Voting;
                    proposals.set(proposal_id, proposal);
                    env.storage().persistent().set(&KEY_PROPOSALS, &proposals);
                }
            }
            ProposalStatus::Voting => {
                // Check if voting period ended
                if current_timestamp >= proposal.vote_end {
                    // Calculate quorum and results
                    let total_votes = proposal.votes_for + proposal.votes_against;

                    // Get member count for quorum calculation
                    let member_count = if proposal.trust_id
                        == Address::from_str(&env, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=")
                    {
                        // Protocol proposal: use total accounts as quorum base
                        let accounts: Map<Address, AccountData> =
                            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
                        accounts.len() as u32
                    } else {
                        // Trust proposal: use trust member count
                        let trusts: Map<Address, TrustData> =
                            env.storage().persistent().get(&KEY_TRUSTS).unwrap();
                        let trust_data = trusts.get(proposal.trust_id.clone()).unwrap();
                        trust_data.member_count
                    };

                    // Check quorum (40% participation required)
                    let quorum_met = total_votes >= (member_count * 40 / 100);

                    if !quorum_met {
                        proposal.status = ProposalStatus::Expired;
                    } else {
                        // Check approval (60% support required, or 80% for emergency)
                        let approval_threshold = match proposal.proposal_type {
                            ProposalType::Emergency => 80,
                            _ => 60,
                        };

                        let approval_percentage = (proposal.votes_for * 100) / total_votes;

                        if approval_percentage >= approval_threshold {
                            proposal.status = ProposalStatus::Approved;
                        } else {
                            proposal.status = ProposalStatus::Rejected;
                        }
                    }

                    proposals.set(proposal_id, proposal);
                    env.storage().persistent().set(&KEY_PROPOSALS, &proposals);
                }
            }
            _ => {
                panic!("Proposal cannot be processed in current state");
            }
        }
    }

    /// Implement an approved proposal
    /// Can only be called after implementation date has passed
    pub fn implement_proposal(env: Env, proposal_id: u64) {
        let mut proposals: Map<u64, Proposal> =
            env.storage().persistent().get(&KEY_PROPOSALS).unwrap();

        let proposal = match proposals.get(proposal_id) {
            Some(p) => p,
            None => panic!("Proposal not found"),
        };

        // Check proposal is approved
        if proposal.status != ProposalStatus::Approved {
            panic!("Proposal is not approved");
        }

        // Check implementation date has passed
        let current_timestamp = env.ledger().timestamp();
        if current_timestamp < proposal.implementation_date {
            panic!("Implementation date has not passed");
        }

        // Clone trust_id before match to avoid partial move
        let trust_id = proposal.trust_id.clone();

        // Execute proposal based on type
        match proposal.proposal_type {
            ProposalType::RateChange => {
                if let Some(new_rate) = proposal.new_rate_bps {
                    let mut trusts: Map<Address, TrustData> =
                        env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                    let mut trust_data = match trusts.get(trust_id.clone()) {
                        Some(t) => t,
                        None => panic!("Trust not found"),
                    };

                    trust_data.annual_rate_bps = new_rate;
                    trusts.set(trust_id, trust_data);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                }
            }
            ProposalType::Emergency => {
                if let Some(new_rate) = proposal.new_rate_bps {
                    // Emergency rate can exceed MAX_ANNUAL_RATE_BPS temporarily
                    let mut trusts: Map<Address, TrustData> =
                        env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                    let mut trust_data = match trusts.get(trust_id.clone()) {
                        Some(t) => t,
                        None => panic!("Trust not found"),
                    };

                    trust_data.annual_rate_bps = new_rate;
                    trusts.set(trust_id, trust_data);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                }
            }
            ProposalType::TrustParameters => {
                // Handle trust parameter changes
                // For now, only rate changes are supported
                if let Some(new_rate) = proposal.new_rate_bps {
                    let mut trusts: Map<Address, TrustData> =
                        env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                    let mut trust_data = match trusts.get(trust_id.clone()) {
                        Some(t) => t,
                        None => panic!("Trust not found"),
                    };

                    trust_data.annual_rate_bps = new_rate;
                    trusts.set(trust_id, trust_data);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                }
            }
            ProposalType::ProtocolUpgrade => {
                // Protocol upgrades require contract upgrade
                // This is a placeholder for future implementation
                panic!("Protocol upgrades must be executed via contract upgrade");
            }
        }

        // Mark proposal as implemented
        let mut proposal = proposal;
        proposal.status = ProposalStatus::Implemented;
        proposals.set(proposal_id, proposal);
        env.storage().persistent().set(&KEY_PROPOSALS, &proposals);
    }

    /// Get proposal details
    pub fn get_proposal(env: Env, proposal_id: u64) -> Proposal {
        let proposals: Map<u64, Proposal> =
            env.storage().persistent().get(&KEY_PROPOSALS).unwrap();

        match proposals.get(proposal_id) {
            Some(p) => p,
            None => panic!("Proposal not found"),
        }
    }

    /// Get all proposals
    pub fn get_all_proposals(env: Env) -> Vec<u64> {
        let proposals: Map<u64, Proposal> =
            env.storage().persistent().get(&KEY_PROPOSALS).unwrap_or(Map::new(&env));

        let mut keys = Vec::new(&env);
        for (k, _) in proposals.iter() {
            keys.push_back(k);
        }
        keys
    }
}

#[cfg(test)]
mod test;
