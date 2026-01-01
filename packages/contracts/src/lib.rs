#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Map, U256, Vec, String, Bytes,
};

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
#[contracttype]
pub enum GraceType {
    Emergency = 0,     // Emergency pause (14-90 days, oracle-activated)
    Illness = 1,       // Illness or injury (30+ days automatic)
    Community = 2,     // Community voted (30-180 days)
}

/// Status of a work claim
#[contracttype]
pub enum ClaimStatus {
    Pending = 0,       // Waiting for verification
    Approved = 1,      // Approved and tokens minted
    Rejected = 2,      // Rejected by verifiers
    Expired = 3,       // Verification window expired
}

/// Status of a governance proposal
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
    // Parameters should be passed via soroban contract deploy -- --creator ADDRESS --initial_supply VALUE
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
}

#[cfg(test)]
mod test;
