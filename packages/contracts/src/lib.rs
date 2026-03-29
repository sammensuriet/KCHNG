#![no_std]
use core::cmp::min;
use soroban_sdk::{
    Address, Bytes, Env, Map, String, U256, Vec, contract, contractevent, contractimpl,
    contracttype,
};

// ============================================================================
// CONSTANTS
// ============================================================================

// Time Standard
const KCHNG_PER_30MINUTES: u64 = 1000; // 1000 KCHNG per 30 minutes of work (base rate)
const MIN_WORK_MINUTES: u64 = 15;

// Transfer protections (anti-gaming: Part 1)
const MIN_TRANSFER_AMOUNT: u64 = 100; // 100 KCHNG (~1/10 of 1 meal @ 30min/1000KCHNG)

// Time
const SECONDS_PER_DAY: u64 = 86_400;

// Demurrage (Wörgl model: 1% monthly = ~12.7% annual)
const DEFAULT_ANNUAL_RATE_BPS: u32 = 1200; // 12% in basis points (100 = 1%)
const DEFAULT_PERIOD_DAYS: u64 = 28; // 4-week demurrage period

// Protocol constraints
const MIN_ANNUAL_RATE_BPS: u32 = 500; // 5% minimum
const MAX_ANNUAL_RATE_BPS: u32 = 1500; // 15% maximum

// Supply cap (anti-gaming: Part 6)
const MAX_SUPPLY: u128 = 1_000_000_000_000_000_000_000; // 1 quintillion

// Verification
const MIN_VERIFIERS: u32 = 2;
const VERIFIER_STAKE: u64 = 100_000; // 100,000 KCHNG
const VERIFIER_BASE_FEE: u64 = 500; // 500 KCHNG per verification
const VERIFIER_CLAIM_PERCENTAGE_BPS: u32 = 200; // 2% of claim value for approved claims

// Governor stake (community creation)
const GOVERNOR_STAKE_AMOUNT: u64 = 500_000; // 500K KCHNG total
const GOVERNOR_COLLATERAL: u64 = 200_000; // 200K collateral (at risk)
const COMMUNITY_VERIFIER_FUND_SEED: u64 = 300_000; // 300K seeds verifier fund

// Demurrage split (basis points)
const DEMURRAGE_GENESIS_SHARE_BPS: u32 = 3000; // 30% to genesis pool
const DEMURRAGE_LOCAL_SHARE_BPS: u32 = 7000; // 70% to local verifier fund

// Verifier elections
const ELECTION_VOTE_PERIOD_DAYS: u64 = 3;
const MIN_ELECTION_QUORUM_PERCENT: u32 = 40; // 40% of members must vote
const MIN_ELECTION_APPROVAL_PERCENT: u32 = 60; // 60% must approve

// Grace Periods
const MAX_GRACE_PERIODS_PER_YEAR: u32 = 3;
const MIN_CONTRIBUTION_HOURS: u64 = 100; // Increased from 30 to 100 (anti-gaming: Part 5.2)
const GRACE_COOLDOWN_DAYS: u64 = 90; // 90 days between grace periods (anti-gaming: Part 5.3)

// Governance
const PROPOSAL_STAKE: u64 = 100; // 100 KCHNG (whole units, matches MIN_TRANSFER_AMOUNT)
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
const KEY_MIGRATION_STATUS: u32 = 5;
const KEY_ACCOUNTS: u32 = 100;
const KEY_TRUSTS: u32 = 200;
const KEY_VERIFIERS: u32 = 300;
const KEY_WORK_CLAIMS: u32 = 400;
const KEY_GRACE_PERIODS: u32 = 500;
const KEY_PROPOSALS: u32 = 600;
const KEY_ORACLES: u32 = 700;
const KEY_VERIFIER_ASSIGNMENTS: u32 = 800;
const KEY_GOVERNOR_TRUSTS: u32 = 201; // Governor-to-trust mapping (anti-gaming: Part 2)
const KEY_LAST_GRACE_TIMES: u32 = 501; // Last grace period activation times (anti-gaming: Part 5.3)
const KEY_REPUTATIONS: u32 = 900; // Map<Address, Map<RoleType, ReputationData>>

// Verifier ecosystem
const KEY_GENESIS_POOL: u32 = 10; // GenesisPoolData (singleton instance)
const KEY_GENESIS_TRUST_ID: u32 = 11; // Address - genesis trust ID
const KEY_NEXT_ELECTION_ID: u32 = 12; // U256 counter
const KEY_TRUST_FUNDS: u32 = 1000; // Map<Address, VerifierFundData>
const KEY_VERIFIER_ELECTIONS: u32 = 1100; // Map<u64, VerifierElection>
const KEY_CANDIDATE_ELECTIONS: u32 = 1200; // Map<Address, u64> active election per candidate
const KEY_COMPENSATION_CLAIMED: u32 = 1300; // Map<(Address, u64), bool> prevent double-claiming

// ============================================================================
// ENUMS
// ============================================================================

/// Type of work being claimed
#[derive(Clone, PartialEq)]
#[contracttype]
pub enum WorkType {
    BasicCare = 0,     // Basic care or agriculture work (1.0× multiplier)
    SkilledCare = 1,   // Skilled care or heavy labor (1.3× multiplier)
    Training = 2,      // Teaching or training (1.5× multiplier)
    EmergencyCare = 3, // Emergency response (2.0× multiplier)
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
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum GraceType {
    Emergency = 0, // Emergency pause (14-90 days, oracle-activated)
    Illness = 1,   // Illness or injury (30+ days automatic)
    Community = 2, // Community voted (30-180 days)
}

/// Status of a work claim
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ClaimStatus {
    Pending = 0,  // Waiting for verification
    Approved = 1, // Approved and tokens minted
    Rejected = 2, // Rejected by verifiers
    Expired = 3,  // Verification window expired
}

/// Status of a governance proposal
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ProposalStatus {
    Review = 0,      // In review period (7 days)
    Voting = 1,      // In voting period (3 days)
    Approved = 2,    // Approved, awaiting implementation
    Rejected = 3,    // Rejected by community
    Implemented = 4, // Successfully implemented
    Expired = 5,     // Expired without passing
}

/// Type of proposal
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ProposalType {
    RateChange = 0,      // Change trust demurrage rate
    TrustParameters = 1, // Adjust trust parameters
    ProtocolUpgrade = 2, // Protocol-level upgrade
    Emergency = 3,       // Emergency measure (crisis exception)
    RemoveVerifier = 4,  // Vote to remove low-rep verifier
    RemoveGovernor = 5,  // Vote to replace governor
    RemoveOracle = 6,    // Vote to remove oracle
    RoleProbation = 7,   // Put a role in probation status
}

/// Status of a verifier election
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum ElectionStatus {
    Pending = 0,  // Voting period active
    Approved = 1, // Elected as verifier
    Rejected = 2, // Not enough support
    Expired = 3,  // Quorum not met
}

/// Source of a verifier's stake
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum StakeSource {
    CommunityFund = 0, // Stake covered by community verifier fund
    SelfFunded = 1,    // Stake from verifier's own balance
}

/// Role type for reputation tracking
#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum RoleType {
    Governor = 0,
    Verifier = 1,
    Oracle = 2,
    Worker = 3,
    Member = 4,
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Account data including demurrage tracking
#[derive(Clone)]
#[contracttype]
pub struct AccountData {
    pub balance: U256,
    pub last_activity: u64,
    pub grace_period_end: u64, // Timestamp when grace ends (0 if not in grace)
    pub trust_id: Option<Address>, // Trust membership (None if none)
    pub contribution_hours: u64, // Total hours contributed
    pub grace_periods_used: u32, // Grace periods used this year
    pub last_grace_year: u32,  // Year of last grace period
}

/// Trust (community organization) data
#[derive(Clone)]
#[contracttype]
pub struct TrustData {
    pub name: String,
    pub governor: Address,
    pub successor: Option<Address>, // Designated successor for governor role
    pub annual_rate_bps: u32,       // Annual demurrage rate in basis points
    pub demurrage_period_days: u64,
    pub member_count: u32,
    pub is_active: bool,
    pub created_at: u64,
    pub governor_stake: U256,              // Collateral staked by governor
    pub governor_collateral_at_risk: U256, // Accumulated slashing on collateral
}

/// Verifier data for work verification
#[derive(Clone)]
#[contracttype]
pub struct VerifierData {
    pub trust_id: Option<Address>,
    pub stake: U256,
    pub reputation_score: u32, // 0-1000 (general trust, independent of roles)
    pub verified_claims: u32,
    pub rejected_claims: u32,
    pub fraud_reports: u32,
    /// Role-based scores (compound key "aspect:role" → score 0-1000)
    /// Default for new roles is 500 (neutral)
    /// Examples: "dining:guest" → 850, "ride_sharing:driver" → 920
    pub aspect_scores: Map<Bytes, u32>,
    /// Whether stake is from community fund or verifier's own balance
    pub stake_source: StakeSource,
    /// Timestamp of last compensation claim (cooldown tracking)
    pub last_compensation_claim: u64,
}

/// Work claim for time-based token issuance
#[derive(Clone)]
#[contracttype]
pub struct WorkClaim {
    pub claim_id: u64,
    pub worker: Address,
    pub work_type: WorkType,
    pub minutes_worked: u64,
    pub evidence_hash: Bytes, // Hash of evidence (photo, GPS, notes)
    pub gps_lat: Option<i64>,
    pub gps_lon: Option<i64>,
    pub submitted_at: u64,
    pub verifiers_assigned: Vec<Address>,
    pub approvers: Vec<Address>, // Track who approved for TF2T
    pub rejecters: Vec<Address>, // Track who rejected for TF2T penalty
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
    pub trust_id: Option<Address>,       // None for protocol-level
    pub new_rate_bps: Option<u32>,       // For rate change proposals
    pub target_address: Option<Address>, // For removal/probation proposals
    pub stake: u64,                      // Staked amount (returned after resolution)
    pub created_at: u64,
    pub review_end: u64,
    pub vote_end: u64,
    pub implementation_date: u64,
    pub status: ProposalStatus,
    pub votes_for: u32,
    pub votes_against: u32,
    pub voters: Vec<Address>, // To prevent double voting
}

/// Oracle for grace period verification
#[derive(Clone)]
#[contracttype]
pub struct OracleData {
    pub oracle_address: Address,
    pub stake: U256,
    pub reputation_score: u32,
    pub grace_periods_granted: u32,
    pub grants_this_year: u32, // Track yearly grants for low-rep limits
    pub last_grant_year: u32,  // Year of last grant (for reset)
    pub abuse_reports: u32,    // Count of abuse reports filed
}

/// Reputation event for tracking history
#[derive(Clone)]
#[contracttype]
pub struct ReputationEvent {
    pub timestamp: u64,
    pub event_type: u32, // Encoded event type
    pub change: i32,     // Can be negative
    pub new_score: u32,
}

/// Reputation data for role-specific tracking
#[derive(Clone)]
#[contracttype]
pub struct ReputationData {
    pub role_type: RoleType,
    pub score: u32,
    pub last_change: u64,
    pub consecutive_negatives: u32, // TF2T pattern tracking
    pub probation_until: Option<u64>,
    pub recent_events: Vec<ReputationEvent>, // Last 10 events
}

/// Migration status tracking for contract upgrades
#[derive(Clone)]
#[contracttype]
pub struct MigrationStatus {
    pub instance_migrated: bool,
    pub admin_migrated: bool,
    pub protocol_version_migrated: bool,
    pub total_supply_migrated: bool,
    pub counters_migrated: bool,
    pub persistent_validated: bool,
    pub migrated_at: u64,
    pub source_contract: Address,
}

/// Result of a migration operation
#[derive(Clone)]
#[contracttype]
pub struct MigrationResult {
    pub success: bool,
    pub accounts_validated: u32,
    pub trusts_validated: u32,
    pub verifiers_validated: u32,
    pub oracles_validated: u32,
    pub work_claims_validated: u32,
    pub proposals_validated: u32,
    pub grace_periods_validated: u32,
    pub reputations_validated: u32,
    pub errors: Vec<String>,
}

// ============================================================================
// VERIFIER ECOSYSTEM DATA STRUCTURES
// ============================================================================

/// Genesis pool - the perpetual verifier funding pool for the genesis trust
/// Funded by demurrage from ALL communities (30% share)
#[derive(Clone)]
#[contracttype]
pub struct GenesisPoolData {
    pub trust_id: Address,      // Genesis trust ID (contract address)
    pub pool_balance: U256,     // Accumulated demurrage for genesis verifiers
    pub total_compensed: U256,  // Lifetime total paid out
}

/// Verifier fund per trust - funded by governor stake (300K seed) + 70% of local demurrage
#[derive(Clone)]
#[contracttype]
pub struct VerifierFundData {
    pub trust_id: Address,
    pub pool_balance: U256,     // Available balance for verifier stakes + compensation
    pub total_compensed: U256,  // Lifetime total paid out to verifiers
    pub total_stakes_covered: U256, // Total stake amount covering community-funded verifiers
}

/// Verifier election - community members vote to elect verifiers
#[derive(Clone)]
#[contracttype]
pub struct VerifierElection {
    pub election_id: u64,
    pub candidate: Address,
    pub trust_id: Address,
    pub created_at: u64,
    pub vote_end: u64,
    pub votes_for: u32,
    pub votes_against: u32,
    pub voters: Vec<Address>,
    pub status: ElectionStatus,
}

/// Result of a verifier compensation claim
#[derive(Clone)]
#[contracttype]
pub struct VerifierCompensation {
    pub verifier: Address,
    pub claim_id: u64,
    pub base_fee: U256,
    pub claim_percentage: U256,
    pub total_compensation: U256,
    pub paid_from: Address, // Trust fund address or genesis trust
}

// Reputation event type codes - track history in ReputationData.recent_events
// Used for TF2T (Tit-for-2-Tats) pattern detection and audit trail
// See docs/2026-01-02_game_theory_simulation_results.md for TF2T strategy analysis
// NOTE: BAD_JUDGMENT, FRAUD_DETECTED, INACTIVITY removed - redundant with PATTERN_PENALTY and DECAY
const REP_EVENT_CLAIM_APPROVED: u32 = 1; // Verifier approves work claim (+5 rep)
const REP_EVENT_CLAIM_REJECTED: u32 = 2; // Verifier rejects work claim (+10 rep)
// 3, 4, 8 removed - were: BAD_JUDGMENT, FRAUD_DETECTED, INACTIVITY
const REP_EVENT_MEMBER_JOIN: u32 = 5; // Member joins trust (+2 gov, +5 member)
const REP_EVENT_MEMBER_LEAVE: u32 = 6; // Member leaves trust (-5 gov, -50 if empty)
const REP_EVENT_PATTERN_PENALTY: u32 = 7; // TF2T: 2+ consecutive negatives (-25 bonus)
const REP_EVENT_GRACE_GRANTED: u32 = 9; // Oracle grants grace period (+5 oracle)
const REP_EVENT_GRACE_ABUSED: u32 = 10; // Grace period conditions violated (after warning)
const REP_EVENT_PROPOSAL_PASS: u32 = 11; // Governance proposal passes (+5 proposer)
const REP_EVENT_PROPOSAL_FAIL: u32 = 12; // Governance proposal fails quorum (-3 proposer)
const REP_EVENT_VOTE_PARTICIPATE: u32 = 13; // Member votes on proposal (+2)
const REP_EVENT_DECAY: u32 = 14; // High reputation decays toward 500 (30+ days)
const REP_EVENT_RECOVERY: u32 = 15; // Low reputation recovers toward 500 (90+ days)
const REP_EVENT_ROLE_RELEASE: u32 = 16; // Voluntary step-down from role (neutral)

// Maximum number of approvers/rejecters to track per claim (prevent storage bloat)
const MAX_VOTERS_PER_CLAIM: u32 = 10;

// Reputation thresholds
const REP_NEUTRAL: u32 = 500;
const REP_RESTRICTED: u32 = 200;
const REMOVAL_THRESHOLD: u32 = 100;
const REP_HIGH: u32 = 700; // Multi-trust verifier threshold

// Slashing percentages (in basis points)
const VERIFIER_SLASH_BPS: u32 = 1000; // 10%
const ORACLE_SLASH_BPS: u32 = 2500; // 25%

// Decay thresholds
const DECAY_START_DAYS: u64 = 30; // High scores start decaying after 30 days
const RECOVERY_START_DAYS: u64 = 90; // Low scores start recovering after 90 days
const MAX_HISTORY_EVENTS: u32 = 10;

// ============================================================================
// CONTRACT EVENTS
// ============================================================================

/// Emitted when tokens are transferred between accounts
#[contractevent]
pub struct Transfer {
    #[topic]
    pub from: Address,
    pub to: Address,
    pub amount: U256,
}

/// Emitted when a new trust is registered
#[contractevent]
pub struct TrustNew {
    #[topic]
    pub governor: Address,
    pub name: String,
    pub annual_rate_bps: u32,
    pub demurrage_period_days: u64,
}

/// Emitted when a member joins a trust
#[contractevent]
pub struct MemberJoin {
    #[topic]
    pub member: Address,
    pub trust_id: Address,
}

/// Emitted when a member leaves a trust
#[contractevent]
pub struct MemberLeave {
    #[topic]
    pub member: Address,
    pub trust_id: Address,
}

/// Emitted when a work claim is submitted
#[contractevent]
pub struct ClaimSubmitted {
    #[topic]
    pub worker: Address,
    pub claim_id: u64,
    pub work_type: u32,
    pub minutes_worked: u64,
}

/// Emitted when a work claim is approved
#[contractevent]
pub struct ClaimApproved {
    #[topic]
    pub worker: Address,
    pub claim_id: u64,
    pub amount: U256,
}

/// Emitted when a work claim is rejected
#[contractevent]
pub struct ClaimRejected {
    #[topic]
    pub worker: Address,
    pub claim_id: u64,
}

/// Emitted when a grace period is activated
#[contractevent]
pub struct GraceActivated {
    #[topic]
    pub account: Address,
    pub grace_type: u32,
    pub duration_days: u64,
    pub end_time: u64,
}

/// Emitted when a grace period is revoked due to abuse
#[contractevent]
pub struct GraceRevoked {
    #[topic]
    pub account: Address,
    #[topic]
    pub reporter: Address,
    pub grace_type: u32,
    pub reason: String,
}

/// Emitted when a governance proposal is created
#[contractevent]
pub struct ProposalCreated {
    #[topic]
    pub proposer: Address,
    pub proposal_id: u64,
    pub proposal_type: u32,
}

/// Emitted when a vote is cast on a proposal
#[contractevent]
pub struct VoteCast {
    #[topic]
    pub voter: Address,
    pub proposal_id: u64,
    pub support: bool,
}

/// Emitted when reputation changes for a role
#[contractevent]
pub struct ReputationChanged {
    #[topic]
    pub address: Address,
    pub role: u32,
    pub change: i32,
    pub new_score: u32,
}

/// Emitted when reputation drops below a threshold (notification for circle review)
#[contractevent]
pub struct ReputationThreshold {
    #[topic]
    pub address: Address,
    pub role: u32,
    pub score: u32,
    pub threshold: u32, // Which threshold was crossed (200 = probation, 100 = removal)
}

/// Emitted when a role holder voluntarily steps down
#[contractevent]
pub struct RoleReleased {
    #[topic]
    pub address: Address,
    pub role: u32,
    pub successor: Option<Address>,
}

/// Emitted when contract migration is completed
#[contractevent]
pub struct MigrationCompleted {
    #[topic]
    pub admin: Address,
    pub source_contract: Address,
    pub old_version: u32,
    pub new_version: u32,
    pub timestamp: u64,
}

/// Emitted when the genesis trust is created
#[contractevent]
pub struct GenesisTrustCreated {
    #[topic]
    pub trust_id: Address,
    pub timestamp: u64,
}

/// Emitted when a member joins the genesis trust
#[contractevent]
pub struct GenesisMemberJoined {
    #[topic]
    pub member: Address,
    pub member_count: u32,
}

/// Emitted when demurrage is split between genesis and local pools
#[contractevent]
pub struct DemurrageSplit {
    #[topic]
    pub account: Address,
    pub total_burned: U256,
    pub genesis_share: U256,
    pub local_share: U256,
}

/// Emitted when a verifier election is created
#[contractevent]
pub struct VerifierElectionCreated {
    #[topic]
    pub candidate: Address,
    pub election_id: u64,
    pub trust_id: Address,
}

/// Emitted when a vote is cast on a verifier election
#[contractevent]
pub struct VerifierElectionVote {
    #[topic]
    pub voter: Address,
    pub election_id: u64,
    pub support: bool,
}

/// Emitted when a verifier election is finalized
#[contractevent]
pub struct VerifierElectionFinalized {
    #[topic]
    pub candidate: Address,
    pub election_id: u64,
    pub approved: bool,
}

/// Emitted when a verifier claims compensation
#[contractevent]
pub struct VerifierCompensationClaimed {
    #[topic]
    pub verifier: Address,
    pub claim_id: u64,
    pub amount: U256,
}

/// Emitted when a trust's verifier fund is updated
#[contractevent]
pub struct VerifierFundUpdated {
    #[topic]
    pub trust_id: Address,
    pub new_balance: U256,
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
        env.storage()
            .instance()
            .set(&KEY_PROTOCOL_VERSION, &U256::from_u32(&env, 1));

        // Set initial balance for creator
        let account_data = AccountData {
            balance: initial_supply.clone(),
            last_activity: u64::MAX, // u64::MAX = no previous transfer, first transfer allowed immediately
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        };

        let mut accounts: Map<Address, AccountData> = Map::new(&env);
        accounts.set(creator.clone(), account_data);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Track total supply
        env.storage()
            .instance()
            .set(&KEY_TOTAL_SUPPLY, &initial_supply);

        // Initialize counters
        env.storage()
            .instance()
            .set(&KEY_NEXT_CLAIM_ID, &U256::from_u32(&env, 1));
        env.storage()
            .instance()
            .set(&KEY_NEXT_PROPOSAL_ID, &U256::from_u32(&env, 1));

        // Create genesis trust (contract-governed community)
        let contract_address = env.current_contract_address();
        let genesis_trust = TrustData {
            name: String::from_str(&env, "KCHNG Genesis Community"),
            governor: contract_address.clone(),
            successor: None,
            annual_rate_bps: DEFAULT_ANNUAL_RATE_BPS,
            demurrage_period_days: DEFAULT_PERIOD_DAYS,
            member_count: 0, // Members join explicitly
            is_active: true,
            created_at: env.ledger().timestamp(),
            governor_stake: U256::from_u32(&env, 0),
            governor_collateral_at_risk: U256::from_u32(&env, 0),
        };

        let mut trusts: Map<Address, TrustData> = Map::new(&env);
        trusts.set(contract_address.clone(), genesis_trust);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);

        // Store genesis trust ID
        env.storage().instance().set(&KEY_GENESIS_TRUST_ID, &contract_address);

        // Initialize genesis pool
        let genesis_pool = GenesisPoolData {
            trust_id: contract_address.clone(),
            pool_balance: U256::from_u32(&env, 0),
            total_compensed: U256::from_u32(&env, 0),
        };
        env.storage().instance().set(&KEY_GENESIS_POOL, &genesis_pool);

        // Initialize election counter
        env.storage()
            .instance()
            .set(&KEY_NEXT_ELECTION_ID, &U256::from_u32(&env, 1));

        // Emit genesis trust created event
        GenesisTrustCreated {
            trust_id: contract_address,
            timestamp: env.ledger().timestamp(),
        }
        .publish(&env);
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
        env.storage()
            .instance()
            .set(&KEY_PROTOCOL_VERSION, &U256::from_u32(&env, 1));

        // Set initial balance for creator
        let account_data = AccountData {
            balance: initial_supply.clone(),
            last_activity: u64::MAX, // u64::MAX = no previous transfer, first transfer allowed immediately
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        };

        let mut accounts: Map<Address, AccountData> = Map::new(&env);
        accounts.set(creator.clone(), account_data);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Track total supply
        env.storage()
            .instance()
            .set(&KEY_TOTAL_SUPPLY, &initial_supply);

        // Initialize counters
        env.storage()
            .instance()
            .set(&KEY_NEXT_CLAIM_ID, &U256::from_u32(&env, 1));
        env.storage()
            .instance()
            .set(&KEY_NEXT_PROPOSAL_ID, &U256::from_u32(&env, 1));

        // Create genesis trust (same as __constructor)
        let contract_address = env.current_contract_address();
        let genesis_trust = TrustData {
            name: String::from_str(&env, "KCHNG Genesis Community"),
            governor: contract_address.clone(),
            successor: None,
            annual_rate_bps: DEFAULT_ANNUAL_RATE_BPS,
            demurrage_period_days: DEFAULT_PERIOD_DAYS,
            member_count: 0,
            is_active: true,
            created_at: env.ledger().timestamp(),
            governor_stake: U256::from_u32(&env, 0),
            governor_collateral_at_risk: U256::from_u32(&env, 0),
        };

        let mut trusts: Map<Address, TrustData> = Map::new(&env);
        trusts.set(contract_address.clone(), genesis_trust);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);

        env.storage().instance().set(&KEY_GENESIS_TRUST_ID, &contract_address);

        let genesis_pool = GenesisPoolData {
            trust_id: contract_address.clone(),
            pool_balance: U256::from_u32(&env, 0),
            total_compensed: U256::from_u32(&env, 0),
        };
        env.storage().instance().set(&KEY_GENESIS_POOL, &genesis_pool);

        env.storage()
            .instance()
            .set(&KEY_NEXT_ELECTION_ID, &U256::from_u32(&env, 1));
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

        // Prevent self-transfers (anti-gaming: Part 1.1)
        if from == to {
            panic!("Cannot transfer to self");
        }

        // Enforce minimum transfer amount (anti-gaming: Part 1.2)
        let amount_u128 = match amount.to_u128() {
            Some(v) => v,
            None => panic!("Amount conversion failed"),
        };
        if amount_u128 < MIN_TRANSFER_AMOUNT as u128 {
            panic!("Transfer amount below minimum (10 KCHNG)");
        }

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
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        let mut updated_to = to_data;
        updated_to.balance = updated_to.balance.add(&amount);
        updated_to.last_activity = current_time;
        accounts.set(to.clone(), updated_to);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Emit transfer event for PWA integration
        Transfer {
            from: from.clone(),
            to: to.clone(),
            amount: amount.clone(),
        }
        .publish(&env);
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
        let to_data = accounts.get(to.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: current_time,
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        let mut updated_to = to_data;
        updated_to.balance = updated_to.balance.add(&amount);
        updated_to.last_activity = current_time;
        accounts.set(to.clone(), updated_to);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Update total supply with cap check (anti-gaming: Part 6)
        let mut total_supply: U256 = env.storage().instance().get(&KEY_TOTAL_SUPPLY).unwrap();
        let new_total = total_supply.add(&amount);

        let new_total_u128 = match new_total.to_u128() {
            Some(v) => v,
            None => panic!("Supply conversion failed"),
        };
        if new_total_u128 > MAX_SUPPLY {
            panic!("Maximum supply reached");
        }

        total_supply = new_total;
        env.storage()
            .instance()
            .set(&KEY_TOTAL_SUPPLY, &total_supply);
    }

    /// Get the total supply
    pub fn total_supply(env: Env) -> U256 {
        env.storage().instance().get(&KEY_TOTAL_SUPPLY).unwrap()
    }

    // ============================================================================
    // INSTANCE STORAGE GETTERS (for migration)
    // ============================================================================

    /// Get the admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&KEY_ADMIN).unwrap()
    }

    /// Get the protocol version
    pub fn get_protocol_version(env: Env) -> u32 {
        let version: U256 = env.storage().instance().get(&KEY_PROTOCOL_VERSION).unwrap();
        match version.to_u128() {
            Some(v) => v as u32,
            None => panic!("Protocol version conversion failed"),
        }
    }

    /// Get the total supply (raw U256)
    pub fn get_total_supply_raw(env: Env) -> U256 {
        env.storage().instance().get(&KEY_TOTAL_SUPPLY).unwrap()
    }

    /// Get the next claim ID
    pub fn get_next_claim_id(env: Env) -> u64 {
        let id: U256 = env.storage().instance().get(&KEY_NEXT_CLAIM_ID).unwrap();
        match id.to_u128() {
            Some(v) => v as u64,
            None => panic!("Claim ID conversion failed"),
        }
    }

    /// Get the next proposal ID
    pub fn get_next_proposal_id(env: Env) -> u64 {
        let id: U256 = env.storage().instance().get(&KEY_NEXT_PROPOSAL_ID).unwrap();
        match id.to_u128() {
            Some(v) => v as u64,
            None => panic!("Proposal ID conversion failed"),
        }
    }

    /// Get the migration status
    pub fn get_migration_status(env: Env) -> Option<MigrationStatus> {
        env.storage().instance().get(&KEY_MIGRATION_STATUS)
    }

    // ============================================================================
    // TRUST SYSTEM (Phase 2)
    // ============================================================================

    /// Register a new community trust
    /// Parameters:
    /// - governor: Address that will govern this trust
    /// - name: Human-readable name for the trust
    /// - annual_rate_bps: Annual demurrage rate in basis points (500-1500 = 5-15%)
    /// - demurrage_period_days: How often to apply demurrage (default: 28 days)
    pub fn register_trust(
        env: Env,
        governor: Address,
        name: String,
        annual_rate_bps: u32,
        demurrage_period_days: u64,
    ) {
        governor.require_auth();

        // Check if governor has high enough reputation to create second trust
        let governor_rep = Self::get_reputation(env.clone(), governor.clone(), RoleType::Governor);
        let governor_trusts_map: Map<Address, Address> = env
            .storage()
            .persistent()
            .get(&KEY_GOVERNOR_TRUSTS)
            .unwrap_or(Map::new(&env));

        if governor_trusts_map.contains_key(governor.clone()) {
            // Governor already has a trust - check if they can create a second
            if governor_rep < REP_HIGH {
                panic!("Governor needs 700+ reputation to create a second trust");
            }
        }

        // Validate rate is within protocol constraints
        if !(MIN_ANNUAL_RATE_BPS..=MAX_ANNUAL_RATE_BPS).contains(&annual_rate_bps) {
            panic!("Rate must be between 5% and 15% annually");
        }

        // Validate period is reasonable (7-365 days)
        if !(7..=365).contains(&demurrage_period_days) {
            panic!("Period must be between 7 and 365 days");
        }

        // Use governor address as the trust ID for simplicity
        let trust_id = governor.clone();

        // Check if governor already has a trust (anti-gaming: Part 2.1)
        let governor_trusts: Map<Address, Address> = env
            .storage()
            .persistent()
            .get(&KEY_GOVERNOR_TRUSTS)
            .unwrap_or(Map::new(&env));

        if governor_trusts.contains_key(governor.clone()) {
            panic!("Governor can only register one trust");
        }

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
        let stake_amount = U256::from_u128(&env, GOVERNOR_STAKE_AMOUNT as u128);
        let collateral = U256::from_u128(&env, GOVERNOR_COLLATERAL as u128);

        let trust = TrustData {
            name: name.clone(),
            governor: governor.clone(),
            successor: None, // No successor designated initially
            annual_rate_bps,
            demurrage_period_days,
            member_count: 1, // Governor counts as first member
            is_active: true,
            created_at: env.ledger().timestamp(),
            governor_stake: collateral.clone(),
            governor_collateral_at_risk: U256::from_u32(&env, 0),
        };

        // --- Governor stake: deduct 500K from governor's balance ---
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let governor_account = accounts.get(governor.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: u64::MAX,
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        // Apply demurrage before checking balance
        let (balance_after_demo, burned) =
            Self::calculate_demurrage_amount(&env, governor.clone(), &governor_account);
        Self::apply_demurrage_split(&env, &governor_account, burned);

        if balance_after_demo < stake_amount {
            panic!("Insufficient balance for governor stake (500,000 KCHNG required)");
        }

        // Deduct 500K from governor
        let mut updated_governor = governor_account.clone();
        updated_governor.balance = balance_after_demo.sub(&stake_amount);
        updated_governor.last_activity = env.ledger().timestamp();
        updated_governor.trust_id = Some(governor.clone());
        accounts.set(governor.clone(), updated_governor);

        // Seed the trust's verifier fund with 300K
        let fund_seed = U256::from_u128(&env, COMMUNITY_VERIFIER_FUND_SEED as u128);
        let trust_fund = VerifierFundData {
            trust_id: governor.clone(),
            pool_balance: fund_seed,
            total_compensed: U256::from_u32(&env, 0),
            total_stakes_covered: U256::from_u32(&env, 0),
        };
        let mut funds: Map<Address, VerifierFundData> = env
            .storage()
            .persistent()
            .get(&KEY_TRUST_FUNDS)
            .unwrap_or(Map::new(&env));
        funds.set(governor.clone(), trust_fund);
        env.storage().persistent().set(&KEY_TRUST_FUNDS, &funds);

        // Store trust
        let mut trusts: Map<Address, TrustData> = env
            .storage()
            .persistent()
            .get(&KEY_TRUSTS)
            .unwrap_or(Map::new(&env));
        trusts.set(trust_id.clone(), trust);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);

        // Record governor-to-trust mapping (anti-gaming: Part 2.1)
        let mut governor_trusts: Map<Address, Address> = env
            .storage()
            .persistent()
            .get(&KEY_GOVERNOR_TRUSTS)
            .unwrap_or(Map::new(&env));
        governor_trusts.set(governor.clone(), trust_id);
        env.storage()
            .persistent()
            .set(&KEY_GOVERNOR_TRUSTS, &governor_trusts);

        // Save updated accounts (governor's balance already deducted above)
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Initialize governor reputation (starts at 500 = neutral)
        let _ = Self::get_reputation_data(env.clone(), governor.clone(), RoleType::Governor);

        // Emit trust registered event for PWA integration
        TrustNew {
            governor: governor.clone(),
            name,
            annual_rate_bps,
            demurrage_period_days,
        }
        .publish(&env);
    }

    /// Join an existing trust
    pub fn join_trust(env: Env, member: Address, trust_id: Address) {
        member.require_auth();

        // Get trust data
        let trusts: Map<Address, TrustData> = env.storage().persistent().get(&KEY_TRUSTS).unwrap();

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

        let member_data = accounts.get(member.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: u64::MAX, // u64::MAX = no previous transfer, first transfer allowed immediately
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        // Check if already in a trust
        if member_data.trust_id.is_some() {
            panic!("Already a member of a trust");
        }

        // Update member's trust membership
        let mut updated_member = member_data;
        updated_member.trust_id = Some(trust_id.clone());
        accounts.set(member.clone(), updated_member);

        // Update trust member count
        let mut trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        let mut updated_trust = trust;
        updated_trust.member_count += 1;
        trusts.set(trust_id.clone(), updated_trust.clone());

        // Save changes
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);

        // Update governor reputation (+2 for member joining)
        Self::update_reputation(
            &env,
            &updated_trust.governor,
            &RoleType::Governor,
            2,
            REP_EVENT_MEMBER_JOIN,
        );

        // Update member reputation (+5 for joining trust)
        Self::update_reputation(&env, &member, &RoleType::Member, 5, REP_EVENT_MEMBER_JOIN);

        // Emit member joined event for PWA integration
        MemberJoin {
            member: member.clone(),
            trust_id: trust_id.clone(),
        }
        .publish(&env);
    }

    /// Leave current trust (anti-gaming: Part 2.2 - allows escaping bad governors)
    pub fn leave_trust(env: Env, member: Address) {
        member.require_auth();

        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let account_data = match accounts.get(member.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        if account_data.trust_id.is_none() {
            panic!("Not a member of any trust");
        }

        let trust_id = account_data.trust_id.clone().unwrap();

        // Update trust member count
        let mut trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();
        let mut trust = trusts.get(trust_id.clone()).unwrap();
        trust.member_count -= 1;
        trusts.set(trust_id.clone(), trust.clone());

        // Remove trust membership from account
        let mut updated_member = account_data;
        updated_member.trust_id = None;
        accounts.set(member.clone(), updated_member);

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);

        // Update governor reputation (-5 for member leaving)
        Self::update_reputation(
            &env,
            &trust.governor,
            &RoleType::Governor,
            -5,
            REP_EVENT_MEMBER_LEAVE,
        );

        // Apply TF2T pattern penalty if needed (2+ members leaving in short period)
        Self::apply_pattern_penalty_if_needed(&env, &trust.governor, &RoleType::Governor);

        // Check if trust is now empty - severe penalty
        if trust.member_count == 0 {
            let _ = Self::update_reputation(
                &env,
                &trust.governor,
                &RoleType::Governor,
                -50,
                REP_EVENT_MEMBER_LEAVE,
            );
        }

        // Emit member left event for PWA integration
        MemberLeave {
            member: member.clone(),
            trust_id: trust_id.clone(),
        }
        .publish(&env);
    }

    /// Get information about a specific trust
    pub fn get_trust_info(env: Env, trust_id: Address) -> TrustData {
        let trusts: Map<Address, TrustData> = env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        match trusts.get(trust_id) {
            Some(trust) => trust,
            None => panic!("Trust not found"),
        }
    }

    /// Get list of all registered trust IDs
    pub fn get_all_trusts(env: Env) -> Vec<Address> {
        let trusts: Map<Address, TrustData> = env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        let mut trust_ids = Vec::new(&env);
        for (trust_id, _) in trusts.iter() {
            trust_ids.push_back(trust_id);
        }
        trust_ids
    }

    /// Designate a successor for the governor role (sociocratic succession)
    /// Only the current governor can call this
    /// The successor must be a member of the trust
    pub fn designate_successor(env: Env, governor: Address, successor: Address) {
        governor.require_auth();

        // Get governor's trust
        let governor_trusts: Map<Address, Address> = env
            .storage()
            .persistent()
            .get(&KEY_GOVERNOR_TRUSTS)
            .unwrap_or(Map::new(&env));

        let trust_id = match governor_trusts.get(governor.clone()) {
            Some(tid) => tid,
            None => panic!("Governor has no trust"),
        };

        // Verify successor is a member of the trust
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let successor_account = match accounts.get(successor.clone()) {
            Some(a) => a,
            None => panic!("Successor not found"),
        };

        if successor_account.trust_id.as_ref() != Some(&trust_id) {
            panic!("Successor must be a member of the trust");
        }

        // Update trust with successor
        let mut trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        if let Some(mut trust) = trusts.get(trust_id.clone()) {
            if trust.governor != governor {
                panic!("Only current governor can designate successor");
            }
            trust.successor = Some(successor.clone());
            trusts.set(trust_id, trust);
            env.storage().persistent().set(&KEY_TRUSTS, &trusts);
        }
    }

    /// Voluntarily step down from a role (sociocratic role release)
    /// For governors: transfers to successor if designated, otherwise disables trust
    /// For verifiers/oracles: returns full stake (no slashing for voluntary release)
    pub fn step_down(env: Env, address: Address, role: RoleType) {
        address.require_auth();

        // Track successor for event emission
        let successor_for_event: Option<Address>;

        match role {
            RoleType::Governor => {
                // Get governor's trust
                let governor_trusts: Map<Address, Address> = env
                    .storage()
                    .persistent()
                    .get(&KEY_GOVERNOR_TRUSTS)
                    .unwrap_or(Map::new(&env));

                let trust_id = match governor_trusts.get(address.clone()) {
                    Some(tid) => tid,
                    None => panic!("Not a governor"),
                };

                let mut trusts: Map<Address, TrustData> =
                    env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                if let Some(mut trust) = trusts.get(trust_id.clone()) {
                    if trust.governor != address {
                        panic!("Not the governor of this trust");
                    }

                    if let Some(successor) = trust.successor.clone() {
                        // Transfer to successor
                        trust.governor = successor.clone();
                        trust.successor = None;
                        successor_for_event = Some(successor.clone());

                        // Update governor trusts mapping
                        let mut gov_trusts: Map<Address, Address> = env
                            .storage()
                            .persistent()
                            .get(&KEY_GOVERNOR_TRUSTS)
                            .unwrap_or(Map::new(&env));
                        gov_trusts.remove(address.clone());
                        gov_trusts.set(successor, trust_id.clone());
                        env.storage()
                            .persistent()
                            .set(&KEY_GOVERNOR_TRUSTS, &gov_trusts);
                    } else {
                        // No successor - disable trust
                        trust.is_active = false;
                        successor_for_event = None;
                    }
                    trusts.set(trust_id, trust);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                } else {
                    successor_for_event = None;
                }

                // Update governor reputation (neutral - voluntary release is not punitive)
                let _ = Self::update_reputation(
                    &env,
                    &address,
                    &RoleType::Governor,
                    0,
                    REP_EVENT_ROLE_RELEASE,
                );
            }
            RoleType::Verifier => {
                successor_for_event = None;

                let mut verifiers: Map<Address, VerifierData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_VERIFIERS)
                    .unwrap_or(Map::new(&env));

                if let Some(verifier_data) = verifiers.get(address.clone()) {
                    match verifier_data.stake_source {
                        StakeSource::SelfFunded => {
                            // Return full stake to verifier's account
                            let mut accounts: Map<Address, AccountData> =
                                env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
                            if let Some(mut account) = accounts.get(address.clone()) {
                                account.balance = account.balance.add(&verifier_data.stake);
                                accounts.set(address.clone(), account);
                                env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
                            }
                        }
                        StakeSource::CommunityFund => {
                            // Return stake to trust's verifier fund
                            if let Some(trust_id) = verifier_data.trust_id.clone() {
                                let mut funds: Map<Address, VerifierFundData> = env
                                    .storage()
                                    .persistent()
                                    .get(&KEY_TRUST_FUNDS)
                                    .unwrap_or(Map::new(&env));
                                if let Some(mut fund) = funds.get(trust_id.clone()) {
                                    fund.pool_balance =
                                        fund.pool_balance.add(&verifier_data.stake);
                                    fund.total_stakes_covered = fund
                                        .total_stakes_covered
                                        .sub(&verifier_data.stake);
                                    funds.set(trust_id, fund);
                                    env.storage().persistent().set(&KEY_TRUST_FUNDS, &funds);
                                }
                            }
                        }
                    }

                    verifiers.remove(address.clone());
                    env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);
                } else {
                    panic!("Not a verifier");
                }

                // Update reputation (neutral)
                let _ = Self::update_reputation(
                    &env,
                    &address,
                    &RoleType::Verifier,
                    0,
                    REP_EVENT_ROLE_RELEASE,
                );
            }
            RoleType::Oracle => {
                successor_for_event = None;

                // Return full stake (no slashing for voluntary release)
                let mut oracles: Map<Address, OracleData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_ORACLES)
                    .unwrap_or(Map::new(&env));

                if let Some(oracle_data) = oracles.get(address.clone()) {
                    // Return full stake to account
                    let mut accounts: Map<Address, AccountData> =
                        env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
                    if let Some(mut account) = accounts.get(address.clone()) {
                        account.balance = account.balance.add(&oracle_data.stake);
                        accounts.set(address.clone(), account);
                        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
                    }

                    oracles.remove(address.clone());
                    env.storage().persistent().set(&KEY_ORACLES, &oracles);
                } else {
                    panic!("Not an oracle");
                }

                // Update reputation (neutral)
                let _ = Self::update_reputation(
                    &env,
                    &address,
                    &RoleType::Oracle,
                    0,
                    REP_EVENT_ROLE_RELEASE,
                );
            }
            _ => panic!("Role type does not support step_down"),
        }

        // Emit role released event with successor info
        RoleReleased {
            address: address.clone(),
            role: role as u32,
            successor: successor_for_event,
        }
        .publish(&env);
    }

    /// Get the trust ID for an account
    pub fn get_account_trust(env: Env, account: Address) -> Option<Address> {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        match accounts.get(account) {
            Some(data) => data.trust_id.clone(),
            None => None,
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
                    last_activity: u64::MAX, // u64::MAX = no previous transfer, first transfer allowed immediately
                    grace_period_end: 0,
                    trust_id: None,
                    contribution_hours: 0,
                    grace_periods_used: 0,
                    last_grace_year: 0,
                }
            }
        }
    }

    // ============================================================================
    // GENESIS TRUST SYSTEM
    // ============================================================================

    /// Get the genesis trust ID (contract address)
    pub fn get_genesis_trust_id(env: Env) -> Address {
        env.storage().instance().get(&KEY_GENESIS_TRUST_ID).unwrap()
    }

    /// Join the genesis trust (open to all, no governor stake required)
    /// This is the bootstrap entry point for new users
    pub fn join_genesis_trust(env: Env, member: Address) {
        member.require_auth();

        let genesis_trust_id: Address = env.storage().instance().get(&KEY_GENESIS_TRUST_ID).unwrap();

        // Get member's account
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let member_data = accounts.get(member.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: u64::MAX,
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        // Check if already in a trust
        if member_data.trust_id.is_some() {
            panic!("Already a member of a trust");
        }

        // Update member's trust membership
        let mut updated_member = member_data;
        updated_member.trust_id = Some(genesis_trust_id.clone());
        accounts.set(member.clone(), updated_member);

        // Update genesis trust member count
        let mut trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();
        let mut genesis_trust = trusts.get(genesis_trust_id.clone()).unwrap();
        genesis_trust.member_count += 1;
        trusts.set(genesis_trust_id.clone(), genesis_trust.clone());

        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
        env.storage().persistent().set(&KEY_TRUSTS, &trusts);

        // Update member reputation (+5 for joining)
        Self::update_reputation(&env, &member, &RoleType::Member, 5, REP_EVENT_MEMBER_JOIN);

        // Emit member joined event
        MemberJoin {
            member: member.clone(),
            trust_id: genesis_trust_id,
        }
        .publish(&env);
    }

    /// Get genesis pool data
    pub fn get_genesis_pool(env: Env) -> GenesisPoolData {
        env.storage().instance().get(&KEY_GENESIS_POOL).unwrap()
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
    /// Returns (new_balance, total_burned) for demurrage split routing
    fn calculate_demurrage_amount(
        env: &Env,
        _account: Address,
        data: &AccountData,
    ) -> (U256, U256) {
        let current_timestamp = env.ledger().timestamp();

        // Check if account is in a grace period
        if data.grace_period_end > 0 && current_timestamp < data.grace_period_end {
            return (data.balance.clone(), U256::from_u32(env, 0));
        }

        let last_activity: u64 = data.last_activity;

        // Timestamp validation (anti-gaming: Part 8) - prevent future timestamps
        if last_activity > current_timestamp {
            return (data.balance.clone(), U256::from_u32(env, 0));
        }

        if current_timestamp <= last_activity {
            return (data.balance.clone(), U256::from_u32(env, 0));
        }

        // Calculate full inactive period
        let inactive_seconds = current_timestamp.saturating_sub(last_activity);
        let inactive_days = inactive_seconds / SECONDS_PER_DAY;

        // Get trust-specific demurrage parameters
        let (annual_rate_bps, period_days) = match &data.trust_id {
            None => (DEFAULT_ANNUAL_RATE_BPS, DEFAULT_PERIOD_DAYS),
            Some(trust_id) => {
                let trusts: Map<Address, TrustData> =
                    env.storage().persistent().get(&KEY_TRUSTS).unwrap();
                match trusts.get(trust_id.clone()) {
                    Some(trust) => (trust.annual_rate_bps, trust.demurrage_period_days),
                    None => (DEFAULT_ANNUAL_RATE_BPS, DEFAULT_PERIOD_DAYS),
                }
            }
        };

        // Calculate how many complete demurrage periods have passed
        if inactive_days < period_days {
            return (data.balance.clone(), U256::from_u32(env, 0));
        }

        let periods = inactive_days / period_days;

        // Calculate the per-period rate in basis points
        let period_rate_bps = (annual_rate_bps as u64) * 10000 * period_days / 365 / 10000;

        // Calculate balance reduction and track total burned
        let mut balance = data.balance.clone();
        let mut total_burned = U256::from_u32(env, 0);

        for _ in 0..periods {
            let burn_amount = {
                let rate_factor = U256::from_u128(env, period_rate_bps as u128);
                let tmp = balance.mul(&rate_factor);
                tmp.div(&U256::from_u128(env, 10000))
            };

            if balance > burn_amount {
                total_burned = total_burned.add(&burn_amount);
                balance = balance.sub(&burn_amount);
            } else {
                total_burned = total_burned.add(&balance);
                balance = U256::from_u32(env, 0);
                break;
            }
        }

        (balance, total_burned)
    }

    /// Backward-compatible wrapper that returns only the new balance
    fn calculate_balance_with_demurrage(env: &Env, account: Address, data: &AccountData) -> U256 {
        let (balance, _burned) = Self::calculate_demurrage_amount(env, account, data);
        balance
    }

    /// Route burned demurrage to genesis pool and/or local verifier fund
    /// 30% → genesis pool, 70% → local trust verifier fund (100% genesis if no trust)
    fn apply_demurrage_split(env: &Env, account: &AccountData, burned: U256) {
        if burned == U256::from_u32(env, 0) {
            return;
        }

        // Calculate shares
        let genesis_share = burned
            .mul(&U256::from_u32(env, DEMURRAGE_GENESIS_SHARE_BPS))
            .div(&U256::from_u32(env, 10000));
        let local_share = burned.sub(&genesis_share);

        // Always credit genesis pool
        let mut genesis_pool: GenesisPoolData = env
            .storage()
            .instance()
            .get(&KEY_GENESIS_POOL)
            .unwrap();
        genesis_pool.pool_balance = genesis_pool.pool_balance.add(&genesis_share);
        env.storage().instance().set(&KEY_GENESIS_POOL, &genesis_pool);

        // Credit local verifier fund if account is in a trust
        if let Some(trust_id) = &account.trust_id {
            let mut funds: Map<Address, VerifierFundData> = env
                .storage()
                .persistent()
                .get(&KEY_TRUST_FUNDS)
                .unwrap_or(Map::new(env));
            let mut fund = funds.get(trust_id.clone()).unwrap_or(VerifierFundData {
                trust_id: trust_id.clone(),
                pool_balance: U256::from_u32(env, 0),
                total_compensed: U256::from_u32(env, 0),
                total_stakes_covered: U256::from_u32(env, 0),
            });
            fund.pool_balance = fund.pool_balance.add(&local_share);
            funds.set(trust_id.clone(), fund);
            env.storage().persistent().set(&KEY_TRUST_FUNDS, &funds);
        } else {
            // Not in any trust: local share goes to genesis pool too
            let mut genesis_pool: GenesisPoolData = env
                .storage()
                .instance()
                .get(&KEY_GENESIS_POOL)
                .unwrap();
            genesis_pool.pool_balance = genesis_pool.pool_balance.add(&local_share);
            env.storage().instance().set(&KEY_GENESIS_POOL, &genesis_pool);
        }
    }

    /// Get the effective demurrage rate for an account
    pub fn get_account_demurrage_rate(env: Env, account: Address) -> (u32, u64) {
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let account_data = match accounts.get(account) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        match &account_data.trust_id {
            None => (DEFAULT_ANNUAL_RATE_BPS, DEFAULT_PERIOD_DAYS),
            Some(trust_id) => {
                let trust = Self::get_trust_info(env, trust_id.clone());
                (trust.annual_rate_bps, trust.demurrage_period_days)
            }
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
        let mut verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap_or(Map::new(&env));

        if verifiers.contains_key(verifier.clone()) {
            panic!("Already registered as verifier");
        }

        let verifier_data = VerifierData {
            trust_id: Some(trust_id.clone()),
            stake: stake_amount.clone(),
            reputation_score: REP_NEUTRAL, // Start at neutral reputation
            verified_claims: 0,
            rejected_claims: 0,
            fraud_reports: 0,
            aspect_scores: Map::new(&env), // Empty aspects map
            stake_source: StakeSource::SelfFunded,
            last_compensation_claim: 0,
        };

        verifiers.set(verifier.clone(), verifier_data);
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

        // Initialize verifier reputation (starts at 500 = neutral)
        let _ = Self::get_reputation_data(env.clone(), verifier.clone(), RoleType::Verifier);

        // Deduct stake from verifier's balance (transfer to contract)
        // For simplicity, we'll just track the stake - in production you'd escrow it
        let mut updated_verifier = verifier_account;
        updated_verifier.balance = balance_after_demurrage.sub(&stake_amount);
        updated_verifier.last_activity = env.ledger().timestamp();
        accounts.set(verifier, updated_verifier);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
    }

    /// Get verifier data
    pub fn get_verifier(env: Env, verifier: Address) -> VerifierData {
        let verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap_or(Map::new(&env));

        match verifiers.get(verifier) {
            Some(data) => data,
            None => panic!("Verifier not found"),
        }
    }

    /// Unregister as a verifier and return stake
    /// If reputation is below 200, 10% of stake is slashed
    pub fn unregister_verifier(env: Env, verifier: Address) {
        verifier.require_auth();

        let mut verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap_or(Map::new(&env));

        let verifier_data = match verifiers.get(verifier.clone()) {
            Some(data) => data,
            None => panic!("Verifier not found"),
        };

        // Check for pending claims
        let claims: Map<u64, WorkClaim> = env
            .storage()
            .persistent()
            .get(&KEY_WORK_CLAIMS)
            .unwrap_or(Map::new(&env));

        for (_, claim) in claims.iter() {
            if claim.status == ClaimStatus::Pending {
                for i in 0..claim.verifiers_assigned.len() {
                    if claim.verifiers_assigned.get(i).unwrap() == verifier {
                        panic!("Cannot unregister while pending claims exist");
                    }
                }
            }
        }

        // Get verifier reputation
        let reputation = Self::get_reputation(env.clone(), verifier.clone(), RoleType::Verifier);

        // Calculate stake to return based on reputation
        let stake_to_return = if reputation < REMOVAL_THRESHOLD {
            // Severe penalty: Slash 20% for auto-removal threshold (< 100)
            let slash_bps: u32 = VERIFIER_SLASH_BPS * 2; // 20%
            let slash_amount = verifier_data
                .stake
                .mul(&U256::from_u32(&env, slash_bps))
                .div(&U256::from_u32(&env, 10000));
            verifier_data.stake.sub(&slash_amount)
        } else if reputation < REP_RESTRICTED {
            // Slash 10% for probation threshold (< 200)
            let slash_amount = verifier_data
                .stake
                .mul(&U256::from_u32(&env, VERIFIER_SLASH_BPS))
                .div(&U256::from_u32(&env, 10000));
            verifier_data.stake.sub(&slash_amount)
        } else {
            verifier_data.stake.clone()
        };

        // Return stake to verifier's account
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let mut account_data = accounts.get(verifier.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: u64::MAX, // u64::MAX = no previous transfer, first transfer allowed immediately
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        account_data.balance = account_data.balance.add(&stake_to_return);
        account_data.last_activity = env.ledger().timestamp();
        accounts.set(verifier.clone(), account_data);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Remove verifier from registry
        verifiers.remove(verifier.clone());
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);
    }

    // ============================================================================
    // VERIFIER ELECTION SYSTEM
    // ============================================================================

    /// Propose oneself as a verifier for the community
    /// Creates an election that other members vote on
    /// The community's verifier fund covers the 100K stake
    pub fn propose_verifier_election(env: Env, candidate: Address) -> u64 {
        candidate.require_auth();

        // Candidate must be in a trust
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
        let account = accounts
            .get(candidate.clone())
            .expect("Account not found");
        let trust_id = account
            .trust_id
            .clone()
            .expect("Must be in a community to become a verifier");

        // Cannot be genesis trust (genesis verifiers are appointed differently)
        let genesis_trust_id: Address =
            env.storage().instance().get(&KEY_GENESIS_TRUST_ID).unwrap();
        if trust_id == genesis_trust_id {
            panic!("Genesis trust uses appointed verifiers, not elections");
        }

        // Check not already a verifier
        let verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap_or(Map::new(&env));
        if verifiers.contains_key(candidate.clone()) {
            panic!("Already a verifier");
        }

        // Check no active election
        let active_elections: Map<Address, u64> = env
            .storage()
            .persistent()
            .get(&KEY_CANDIDATE_ELECTIONS)
            .unwrap_or(Map::new(&env));
        if let Some(election_id) = active_elections.get(candidate.clone()) {
            let elections: Map<u64, VerifierElection> = env
                .storage()
                .persistent()
                .get(&KEY_VERIFIER_ELECTIONS)
                .unwrap_or(Map::new(&env));
            if let Some(election) = elections.get(election_id) {
                if election.status == ElectionStatus::Pending
                    && env.ledger().timestamp() < election.vote_end
                {
                    panic!("Already have an active election");
                }
            }
        }

        // Check trust fund has enough to cover stake (100K)
        let funds: Map<Address, VerifierFundData> = env
            .storage()
            .persistent()
            .get(&KEY_TRUST_FUNDS)
            .unwrap_or(Map::new(&env));
        let fund = funds.get(trust_id.clone()).expect("Community has no verifier fund");
        let stake_amount = U256::from_u128(&env, VERIFIER_STAKE as u128);
        if fund.pool_balance < stake_amount {
            panic!("Community verifier fund has insufficient balance");
        }

        // Generate election ID
        let mut election_id_u256: U256 = env
            .storage()
            .instance()
            .get(&KEY_NEXT_ELECTION_ID)
            .unwrap();
        let election_id = election_id_u256.to_u128().unwrap() as u64;

        let current_time = env.ledger().timestamp();

        // Create election
        let election = VerifierElection {
            election_id,
            candidate: candidate.clone(),
            trust_id: trust_id.clone(),
            created_at: current_time,
            vote_end: current_time + (ELECTION_VOTE_PERIOD_DAYS * SECONDS_PER_DAY),
            votes_for: 0,
            votes_against: 0,
            voters: Vec::new(&env),
            status: ElectionStatus::Pending,
        };

        // Store election
        let mut elections: Map<u64, VerifierElection> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ELECTIONS)
            .unwrap_or(Map::new(&env));
        elections.set(election_id, election);
        env.storage().persistent().set(&KEY_VERIFIER_ELECTIONS, &elections);

        // Track candidate's active election
        let mut active: Map<Address, u64> = env
            .storage()
            .persistent()
            .get(&KEY_CANDIDATE_ELECTIONS)
            .unwrap_or(Map::new(&env));
        active.set(candidate.clone(), election_id);
        env.storage().persistent().set(&KEY_CANDIDATE_ELECTIONS, &active);

        // Increment counter
        election_id_u256 = U256::from_u128(&env, (election_id + 1) as u128);
        env.storage().instance().set(&KEY_NEXT_ELECTION_ID, &election_id_u256);

        // Emit event
        VerifierElectionCreated {
            candidate: candidate.clone(),
            election_id,
            trust_id,
        }
        .publish(&env);

        election_id
    }

    /// Vote on a verifier election
    /// Only members of the same trust can vote
    pub fn vote_verifier_election(env: Env, voter: Address, election_id: u64, support: bool) {
        voter.require_auth();

        let mut elections: Map<u64, VerifierElection> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ELECTIONS)
            .unwrap();
        let mut election = elections
            .get(election_id)
            .expect("Election not found");

        if election.status != ElectionStatus::Pending {
            panic!("Election is not active");
        }
        if env.ledger().timestamp() > election.vote_end {
            panic!("Voting period has ended");
        }
        if election.voters.contains(voter.clone()) {
            panic!("Already voted");
        }

        // Voter must be in the same trust
        let accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
        let voter_account = accounts
            .get(voter.clone())
            .expect("Voter account not found");
        if voter_account.trust_id.as_ref() != Some(&election.trust_id) {
            panic!("Only members of the same community can vote");
        }

        // Record vote
        if support {
            election.votes_for += 1;
        } else {
            election.votes_against += 1;
        }
        election.voters.push_back(voter.clone());
        elections.set(election_id, election.clone());
        env.storage().persistent().set(&KEY_VERIFIER_ELECTIONS, &elections);

        // Emit event
        VerifierElectionVote {
            voter,
            election_id,
            support,
        }
        .publish(&env);
    }

    /// Finalize a verifier election and register the verifier if approved
    pub fn finalize_verifier_election(env: Env, election_id: u64) {
        let mut elections: Map<u64, VerifierElection> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ELECTIONS)
            .unwrap();
        let mut election = elections
            .get(election_id)
            .expect("Election not found");

        if election.status != ElectionStatus::Pending {
            panic!("Election already finalized");
        }
        if env.ledger().timestamp() <= election.vote_end {
            panic!("Voting period has not ended");
        }

        // Get trust member count for quorum
        let trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();
        let trust = trusts.get(election.trust_id.clone()).unwrap();

        let total_votes = election.votes_for + election.votes_against;
        let quorum_required = (trust.member_count * MIN_ELECTION_QUORUM_PERCENT / 100).max(1);

        if total_votes < quorum_required || total_votes == 0 {
            election.status = ElectionStatus::Expired;
        } else {
            let approval_pct = (election.votes_for * 100) / total_votes;
            if approval_pct >= MIN_ELECTION_APPROVAL_PERCENT {
                election.status = ElectionStatus::Approved;

                // Deduct stake from trust fund
                let stake_amount = U256::from_u128(&env, VERIFIER_STAKE as u128);
                let mut funds: Map<Address, VerifierFundData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_TRUST_FUNDS)
                    .unwrap_or(Map::new(&env));
                let mut fund = funds.get(election.trust_id.clone()).unwrap();
                fund.pool_balance = fund.pool_balance.sub(&stake_amount);
                fund.total_stakes_covered =
                    fund.total_stakes_covered.add(&stake_amount);
                funds.set(election.trust_id.clone(), fund);
                env.storage().persistent().set(&KEY_TRUST_FUNDS, &funds);

                // Register verifier with community-funded stake
                let verifier_data = VerifierData {
                    trust_id: Some(election.trust_id.clone()),
                    stake: stake_amount,
                    reputation_score: REP_NEUTRAL,
                    verified_claims: 0,
                    rejected_claims: 0,
                    fraud_reports: 0,
                    aspect_scores: Map::new(&env),
                    stake_source: StakeSource::CommunityFund,
                    last_compensation_claim: 0,
                };
                let mut verifiers: Map<Address, VerifierData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_VERIFIERS)
                    .unwrap_or(Map::new(&env));
                verifiers.set(election.candidate.clone(), verifier_data);
                env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

                // Initialize reputation
                let _ = Self::get_reputation_data(
                    env.clone(),
                    election.candidate.clone(),
                    RoleType::Verifier,
                );
            } else {
                election.status = ElectionStatus::Rejected;
            }
        }

        let approved = election.status == ElectionStatus::Approved;
        let candidate_addr = election.candidate.clone();
        elections.set(election_id, election);
        env.storage().persistent().set(&KEY_VERIFIER_ELECTIONS, &elections);

        // Clear active election for candidate
        let mut active: Map<Address, u64> = env
            .storage()
            .persistent()
            .get(&KEY_CANDIDATE_ELECTIONS)
            .unwrap_or(Map::new(&env));
        active.remove(candidate_addr.clone());
        env.storage().persistent().set(&KEY_CANDIDATE_ELECTIONS, &active);

        // Emit event
        VerifierElectionFinalized {
            candidate: candidate_addr,
            election_id,
            approved,
        }
        .publish(&env);
    }

    /// Get election details
    pub fn get_election(env: Env, election_id: u64) -> VerifierElection {
        let elections: Map<u64, VerifierElection> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ELECTIONS)
            .unwrap_or(Map::new(&env));
        elections.get(election_id).expect("Election not found")
    }

    /// Get all active elections for a trust
    pub fn get_trust_elections(env: Env, trust_id: Address) -> Vec<u64> {
        let elections: Map<u64, VerifierElection> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ELECTIONS)
            .unwrap_or(Map::new(&env));

        let mut result = Vec::new(&env);
        for (id, election) in elections.iter() {
            if election.trust_id == trust_id && election.status == ElectionStatus::Pending {
                result.push_back(id);
            }
        }
        result
    }

    /// Get verifier fund data for a trust
    pub fn get_verifier_fund(env: Env, trust_id: Address) -> VerifierFundData {
        let funds: Map<Address, VerifierFundData> = env
            .storage()
            .persistent()
            .get(&KEY_TRUST_FUNDS)
            .unwrap_or(Map::new(&env));
        funds.get(trust_id).expect("No verifier fund for this trust")
    }

    // ============================================================================
    // VERIFIER COMPENSATION
    // ============================================================================

    /// Claim compensation for verification work
    /// Base fee for any participation + 2% of claim value for approved claims
    pub fn claim_verifier_compensation(env: Env, verifier: Address, claim_id: u64) -> VerifierCompensation {
        verifier.require_auth();

        // Get the claim
        let claims: Map<u64, WorkClaim> =
            env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap();
        let claim = claims.get(claim_id).expect("Claim not found");

        // Must be Approved or Rejected (verifier did work either way)
        if claim.status != ClaimStatus::Approved && claim.status != ClaimStatus::Rejected {
            panic!("Claim must be resolved to claim compensation");
        }

        // Verify this verifier participated in the claim
        let mut participated = false;
        for i in 0..claim.verifiers_assigned.len() {
            if claim.verifiers_assigned.get(i).unwrap() == verifier {
                participated = true;
                break;
            }
        }
        if !participated {
            panic!("Did not participate in this claim");
        }

        // Check not already claimed compensation for this claim
        let claimed: Map<(Address, u64), bool> = env
            .storage()
            .persistent()
            .get(&KEY_COMPENSATION_CLAIMED)
            .unwrap_or(Map::new(&env));
        let claim_key = (verifier.clone(), claim_id);
        if claimed.get(claim_key.clone()) == Some(true) {
            panic!("Already claimed compensation for this claim");
        }

        // Calculate compensation
        let base_fee = U256::from_u128(&env, VERIFIER_BASE_FEE as u128);

        // Claim value percentage: 2% of minted amount (for approved claims only)
        let claim_percentage = if claim.status == ClaimStatus::Approved {
            let base_kchng = claim.minutes_worked * KCHNG_PER_30MINUTES / 30;
            let kchng_to_mint = (base_kchng * claim.multiplier as u64) / 100;
            let claim_amount = U256::from_u128(&env, kchng_to_mint as u128);
            claim_amount
                .mul(&U256::from_u32(&env, VERIFIER_CLAIM_PERCENTAGE_BPS))
                .div(&U256::from_u32(&env, 10000))
        } else {
            U256::from_u32(&env, 0)
        };

        let total_compensation = base_fee.add(&claim_percentage);

        // Determine funding source
        let verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap();
        let verifier_data = verifiers.get(verifier.clone()).expect("Verifier not found");
        let trust_id = verifier_data.trust_id.clone().expect("Verifier has no trust");

        let genesis_trust_id: Address = env.storage().instance().get(&KEY_GENESIS_TRUST_ID).unwrap();
        let paid_from;

        if trust_id == genesis_trust_id {
            // Genesis trust verifier: pay from genesis pool
            let mut pool: GenesisPoolData =
                env.storage().instance().get(&KEY_GENESIS_POOL).unwrap();
            if pool.pool_balance < total_compensation {
                panic!("Genesis pool has insufficient funds for compensation");
            }
            pool.pool_balance = pool.pool_balance.sub(&total_compensation);
            pool.total_compensed = pool.total_compensed.add(&total_compensation);
            env.storage().instance().set(&KEY_GENESIS_POOL, &pool);
            paid_from = genesis_trust_id;
        } else {
            // Local trust verifier: pay from trust fund
            let mut funds: Map<Address, VerifierFundData> = env
                .storage()
                .persistent()
                .get(&KEY_TRUST_FUNDS)
                .unwrap_or(Map::new(&env));
            let mut fund = funds.get(trust_id.clone()).expect("No verifier fund");
            if fund.pool_balance < total_compensation {
                panic!("Community verifier fund has insufficient funds for compensation");
            }
            fund.pool_balance = fund.pool_balance.sub(&total_compensation);
            fund.total_compensed = fund.total_compensed.add(&total_compensation);
            funds.set(trust_id.clone(), fund);
            env.storage().persistent().set(&KEY_TRUST_FUNDS, &funds);
            paid_from = trust_id;
        }

        // Credit compensation to verifier's account
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
        let mut account = accounts.get(verifier.clone()).unwrap();
        let current_time = env.ledger().timestamp();
        account.balance = account.balance.add(&total_compensation);
        account.last_activity = current_time;
        accounts.set(verifier.clone(), account);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Mark compensation as claimed (prevent double-claiming)
        let mut claimed: Map<(Address, u64), bool> = env
            .storage()
            .persistent()
            .get(&KEY_COMPENSATION_CLAIMED)
            .unwrap_or(Map::new(&env));
        claimed.set(claim_key, true);
        env.storage().persistent().set(&KEY_COMPENSATION_CLAIMED, &claimed);

        // Update verifier tracking
        let mut verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap();
        if let Some(mut vdata) = verifiers.get(verifier.clone()) {
            vdata.last_compensation_claim = current_time;
            verifiers.set(verifier.clone(), vdata);
        }
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

        // Emit event
        VerifierCompensationClaimed {
            verifier: verifier.clone(),
            claim_id,
            amount: total_compensation.clone(),
        }
        .publish(&env);

        VerifierCompensation {
            verifier,
            claim_id,
            base_fee,
            claim_percentage,
            total_compensation,
            paid_from,
        }
    }
    /// If reputation is below 200 (restricted), 25% of stake is slashed
    pub fn unregister_oracle(env: Env, oracle: Address) {
        oracle.require_auth();

        let mut oracles: Map<Address, OracleData> = env
            .storage()
            .persistent()
            .get(&KEY_ORACLES)
            .unwrap_or(Map::new(&env));

        let oracle_data = match oracles.get(oracle.clone()) {
            Some(data) => data,
            None => panic!("Oracle not found"),
        };

        // Get oracle reputation
        let reputation = Self::get_reputation(env.clone(), oracle.clone(), RoleType::Oracle);

        // Calculate stake to return based on reputation
        let stake_to_return = if reputation < REMOVAL_THRESHOLD {
            // Severe penalty: Slash 50% for auto-removal threshold (< 100)
            let slash_bps: u32 = ORACLE_SLASH_BPS * 2; // 50%
            let slash_amount = oracle_data
                .stake
                .mul(&U256::from_u32(&env, slash_bps))
                .div(&U256::from_u32(&env, 10000));
            oracle_data.stake.sub(&slash_amount)
        } else if reputation < REP_RESTRICTED {
            // Slash 25% for probation threshold (< 200)
            let slash_amount = oracle_data
                .stake
                .mul(&U256::from_u32(&env, ORACLE_SLASH_BPS))
                .div(&U256::from_u32(&env, 10000));
            oracle_data.stake.sub(&slash_amount)
        } else {
            oracle_data.stake.clone()
        };

        // Return stake to oracle's account
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let mut account_data = accounts.get(oracle.clone()).unwrap_or(AccountData {
            balance: U256::from_u32(&env, 0),
            last_activity: u64::MAX, // u64::MAX = no previous transfer, first transfer allowed immediately
            grace_period_end: 0,
            trust_id: None,
            contribution_hours: 0,
            grace_periods_used: 0,
            last_grace_year: 0,
        });

        account_data.balance = account_data.balance.add(&stake_to_return);
        account_data.last_activity = env.ledger().timestamp();
        accounts.set(oracle.clone(), account_data);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Remove oracle from registry
        oracles.remove(oracle.clone());
        env.storage().persistent().set(&KEY_ORACLES, &oracles);
    }

    /// Update a role-based score for a verifier
    ///
    /// Role-based reputation allows for context-specific scoring.
    /// Hierarchy: Domain → Aspect → Role
    ///
    /// For example:
    ///   - Domain: Hospitality
    ///     - Aspect: Dining
    ///       - Role: Guest (score: 850)
    ///       - Role: Host (score: 400)
    ///
    /// This allows someone to have high reputation as a dinner guest
    /// but low reputation as a dinner host.
    ///
    /// # Arguments
    /// * `verifier` - The verifier whose role score is being updated
    /// * `role_key` - Compound key "aspect:role" (e.g., "dining:guest", "ride_sharing:driver")
    /// * `delta` - The change to apply (positive or negative, e.g., +30, -50)
    /// * `scorer` - The account submitting this score update (must authenticate)
    ///
    /// # Behavior
    /// - If role doesn't exist, initializes to 500 (neutral) then applies delta
    /// - Caps final score at [0, 1000]
    /// - Requires auth from scorer
    pub fn update_role_score(
        env: Env,
        verifier: Address,
        role_key: Bytes,
        delta: i32,
        scorer: Address,
    ) {
        scorer.require_auth();

        // Prevent self-scoring
        if scorer == verifier {
            panic!("Cannot score yourself");
        }

        // Get existing verifier data - map may not exist yet
        let verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap_or(Map::new(&env));

        if !verifiers.contains_key(verifier.clone()) {
            panic!("Verifier not found");
        }

        let mut verifier_data = verifiers.get(verifier.clone()).unwrap();

        // Get current score, defaulting to neutral (500) if not present
        let current_score = verifier_data
            .aspect_scores
            .get(role_key.clone())
            .unwrap_or(500);

        // Apply delta with bounds checking [0, 1000]
        let new_score = (current_score as i32 + delta).clamp(0, 1000) as u32;

        // Update the role score
        verifier_data.aspect_scores.set(role_key, new_score);

        // Get mutable map and update
        let mut verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap_or(Map::new(&env));
        verifiers.set(verifier, verifier_data);
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);
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
        let worker_trust_id = worker_account
            .trust_id
            .as_ref()
            .expect("Must join a trust before submitting work claims");

        // Get next claim ID
        let mut claim_id_u256: U256 = env.storage().instance().get(&KEY_NEXT_CLAIM_ID).unwrap();
        let claim_id = claim_id_u256.to_u128().unwrap() as u64;

        // Assign verifiers (2-5 random verifiers from the same trust)
        let verifiers: Map<Address, VerifierData> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIERS)
            .unwrap_or(Map::new(&env));

        let mut trust_verifiers: Vec<Address> = Vec::new(&env);
        for (verifier_addr, verifier_data) in verifiers.iter() {
            if verifier_data.trust_id.as_ref() == Some(worker_trust_id) {
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
        let base_kchng = minutes_worked * KCHNG_PER_30MINUTES / 30; // 1000 KCHNG per 30 minutes
        let _kchng_to_mint = (base_kchng * multiplier as u64) / 100;

        // Create work claim
        let claim = WorkClaim {
            claim_id,
            worker: worker.clone(),
            work_type: work_type.clone(),
            minutes_worked,
            evidence_hash,
            gps_lat,
            gps_lon,
            submitted_at: env.ledger().timestamp(),
            verifiers_assigned: assigned_verifiers.clone(),
            approvers: Vec::new(&env),
            rejecters: Vec::new(&env),
            approvals_received: 0,
            rejections_received: 0,
            status: ClaimStatus::Pending,
            multiplier,
        };

        // Store claim
        let mut claims: Map<u64, WorkClaim> = env
            .storage()
            .persistent()
            .get(&KEY_WORK_CLAIMS)
            .unwrap_or(Map::new(&env));
        claims.set(claim_id, claim);
        env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

        // Store verifier assignments for lookup
        let mut assignments: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ASSIGNMENTS)
            .unwrap_or(Map::new(&env));
        assignments.set(claim_id, assigned_verifiers);
        env.storage()
            .persistent()
            .set(&KEY_VERIFIER_ASSIGNMENTS, &assignments);

        // Increment claim ID counter
        claim_id_u256 = U256::from_u128(&env, (claim_id + 1) as u128);
        env.storage()
            .instance()
            .set(&KEY_NEXT_CLAIM_ID, &claim_id_u256);

        // Emit work claim submitted event for PWA integration
        ClaimSubmitted {
            worker: worker.clone(),
            claim_id,
            work_type: work_type.clone() as u32,
            minutes_worked,
        }
        .publish(&env);

        claim_id
    }

    /// Approve a work claim (verifier only)
    pub fn approve_work_claim(env: Env, verifier: Address, claim_id: u64) {
        verifier.require_auth();

        // Check if verifier is on probation
        if Self::is_on_probation(env.clone(), verifier.clone(), RoleType::Verifier) {
            panic!("Verifier is on probation");
        }

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
        let assignments: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ASSIGNMENTS)
            .unwrap_or(Map::new(&env));
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

        // Record approval and track approver (limit stored voters to prevent storage bloat)
        claim.approvals_received += 1;
        if claim.approvers.len() < MAX_VOTERS_PER_CLAIM {
            claim.approvers.push_back(verifier.clone());
        }
        claims.set(claim_id, claim.clone());
        env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

        // Update verifier reputation using new system (+5 for approval)
        // Will be adjusted later if majority disagrees (TF2T)
        let _new_rep = Self::update_reputation(
            &env,
            &verifier,
            &RoleType::Verifier,
            5,
            REP_EVENT_CLAIM_APPROVED,
        );

        // Also update legacy reputation score for backward compatibility
        let mut verifiers: Map<Address, VerifierData> =
            env.storage().persistent().get(&KEY_VERIFIERS).unwrap();
        let mut verifier_data = verifiers.get(verifier.clone()).unwrap();
        verifier_data.verified_claims += 1;
        verifier_data.reputation_score =
            Self::get_reputation(env.clone(), verifier.clone(), RoleType::Verifier);
        verifiers.set(verifier, verifier_data);
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

        // Check if we have enough approvals (simple majority)
        // Need more than half of assigned verifiers to approve
        let total_verifiers = assigned_verifiers.len();
        let required = (total_verifiers / 2) + 1;

        if claim.approvals_received >= required {
            // Mint tokens to worker
            let base_kchng = claim.minutes_worked * KCHNG_PER_30MINUTES / 30; // 1000 KCHNG per 30 minutes
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
            env.storage()
                .instance()
                .set(&KEY_TOTAL_SUPPLY, &total_supply);

            // Update worker reputation for approved claim (+5)
            Self::update_reputation(
                &env,
                &claim.worker,
                &RoleType::Worker,
                5,
                REP_EVENT_CLAIM_APPROVED,
            );

            // Mark claim as approved
            claim.status = ClaimStatus::Approved;
            let worker_addr = claim.worker.clone();

            // Clone rejecters list before storing claim (to avoid borrow issues)
            let rejecters_list = claim.rejecters.clone();

            claims.set(claim_id, claim);
            env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

            // Emit work claim approved event for PWA integration
            ClaimApproved {
                worker: worker_addr,
                claim_id,
                amount: amount.clone(),
            }
            .publish(&env);

            // Penalize verifiers who wrongly rejected this claim (TF2T bad judgment)
            // They voted against the majority and were proven wrong
            for i in 0..rejecters_list.len() {
                let rejecter = rejecters_list.get(i).unwrap();
                // Apply bad judgment penalty (-10 reputation)
                let _ = Self::update_reputation(
                    &env,
                    &rejecter,
                    &RoleType::Verifier,
                    -10,
                    REP_EVENT_PATTERN_PENALTY, // Using pattern penalty for bad judgment
                );

                // Also update legacy verifier data
                let mut verifiers: Map<Address, VerifierData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_VERIFIERS)
                    .unwrap_or(Map::new(&env));
                if let Some(mut vdata) = verifiers.get(rejecter.clone()) {
                    vdata.rejected_claims += 1;
                    vdata.reputation_score =
                        Self::get_reputation(env.clone(), rejecter.clone(), RoleType::Verifier);
                    verifiers.set(rejecter, vdata);
                    env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);
                }
            }
        }
    }

    /// Reject a work claim (verifier only)
    pub fn reject_work_claim(env: Env, verifier: Address, claim_id: u64) {
        verifier.require_auth();

        // Check if verifier is on probation
        if Self::is_on_probation(env.clone(), verifier.clone(), RoleType::Verifier) {
            panic!("Verifier is on probation");
        }

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
        let assignments: Map<u64, Vec<Address>> = env
            .storage()
            .persistent()
            .get(&KEY_VERIFIER_ASSIGNMENTS)
            .unwrap_or(Map::new(&env));
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

        // Record rejection and track rejecter (limit stored voters to prevent storage bloat)
        claim.rejections_received += 1;
        if claim.rejecters.len() < MAX_VOTERS_PER_CLAIM {
            claim.rejecters.push_back(verifier.clone());
        }
        claims.set(claim_id, claim.clone());
        env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

        // Update verifier reputation using new system (+10 for rejection)
        // Will be adjusted later if majority disagrees (TF2T)
        let _new_rep = Self::update_reputation(
            &env,
            &verifier,
            &RoleType::Verifier,
            10,
            REP_EVENT_CLAIM_REJECTED,
        );

        // Also update legacy reputation score for backward compatibility
        let mut verifiers: Map<Address, VerifierData> =
            env.storage().persistent().get(&KEY_VERIFIERS).unwrap();
        let mut verifier_data = verifiers.get(verifier.clone()).unwrap();
        verifier_data.rejected_claims += 1;
        verifier_data.reputation_score =
            Self::get_reputation(env.clone(), verifier.clone(), RoleType::Verifier);
        verifiers.set(verifier, verifier_data);
        env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);

        // Check if we have enough rejections to reject the claim
        let total_verifiers = assigned_verifiers.len();
        let required = (total_verifiers / 2) + 1;

        if claim.rejections_received >= required {
            // Update worker reputation for rejected claim (-10)
            Self::update_reputation(
                &env,
                &claim.worker,
                &RoleType::Worker,
                -10,
                REP_EVENT_CLAIM_REJECTED,
            );

            // Apply TF2T pattern penalty if needed (2+ consecutive negatives)
            Self::apply_pattern_penalty_if_needed(&env, &claim.worker, &RoleType::Worker);

            // Mark claim as rejected
            claim.status = ClaimStatus::Rejected;
            let worker_addr = claim.worker.clone();
            claims.set(claim_id, claim);
            env.storage().persistent().set(&KEY_WORK_CLAIMS, &claims);

            // Emit work claim rejected event for PWA integration
            ClaimRejected {
                worker: worker_addr,
                claim_id,
            }
            .publish(&env);
        }
    }

    /// Get work claim details
    pub fn get_work_claim(env: Env, claim_id: u64) -> WorkClaim {
        let claims: Map<u64, WorkClaim> = env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap();

        match claims.get(claim_id) {
            Some(claim) => claim,
            None => panic!("Claim not found"),
        }
    }

    /// Get pending claims for a verifier
    pub fn get_verifier_pending_claims(env: Env, verifier: Address) -> Vec<u64> {
        let claims: Map<u64, WorkClaim> = env
            .storage()
            .persistent()
            .get(&KEY_WORK_CLAIMS)
            .unwrap_or(Map::new(&env));

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

        let mut oracles: Map<Address, OracleData> = env
            .storage()
            .persistent()
            .get(&KEY_ORACLES)
            .unwrap_or(Map::new(&env));

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

        // Minimum stake for oracles (anti-gaming: Part 5.1 - increased to 5M)
        let oracle_stake = U256::from_u128(&env, 5_000_000); // 5,000,000 KCHNG (increased from 500K)

        // Simple balance check (without demurrage for oracle registration)
        if oracle_account.balance < oracle_stake {
            panic!("Insufficient balance to register as oracle");
        }

        let oracle_data = OracleData {
            oracle_address: oracle.clone(),
            stake: oracle_stake,
            reputation_score: REP_NEUTRAL,
            grace_periods_granted: 0,
            grants_this_year: 0,
            last_grant_year: 0,
            abuse_reports: 0,
        };

        oracles.set(oracle.clone(), oracle_data);
        env.storage().persistent().set(&KEY_ORACLES, &oracles);

        // Initialize oracle reputation (starts at 500 = neutral)
        let _ = Self::get_reputation_data(env.clone(), oracle, RoleType::Oracle);
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

        // Check if oracle is on probation
        if Self::is_on_probation(env.clone(), oracle.clone(), RoleType::Oracle) {
            panic!("Oracle is on probation");
        }

        // Verify oracle is registered
        let mut oracles: Map<Address, OracleData> = env
            .storage()
            .persistent()
            .get(&KEY_ORACLES)
            .unwrap_or(Map::new(&env));

        if !oracles.contains_key(oracle.clone()) {
            panic!("Not a registered oracle");
        }

        // Check oracle reputation for grace period limits
        let oracle_rep = Self::get_reputation(env.clone(), oracle.clone(), RoleType::Oracle);
        let current_year = (env.ledger().timestamp() / (365 * SECONDS_PER_DAY)) as u32;

        if oracle_rep < REP_RESTRICTED {
            // Low reputation oracle can only grant 1 grace period per year
            let mut oracle_data = oracles.get(oracle.clone()).unwrap();

            // Reset yearly counter if new year
            if oracle_data.last_grant_year != current_year {
                oracle_data.grants_this_year = 0;
                oracle_data.last_grant_year = current_year;
            }

            if oracle_data.grants_this_year >= 1 {
                panic!("Low reputation oracle limited to 1 grace period per year");
            }

            // Increment yearly grant counter
            oracle_data.grants_this_year += 1;
            oracle_data.grace_periods_granted += 1;
            oracles.set(oracle.clone(), oracle_data);
            env.storage().persistent().set(&KEY_ORACLES, &oracles);
        } else {
            // High rep oracle - just increment total counter
            if let Some(mut oracle_data) = oracles.get(oracle.clone()) {
                oracle_data.grace_periods_granted += 1;
                oracles.set(oracle.clone(), oracle_data);
                env.storage().persistent().set(&KEY_ORACLES, &oracles);
            }
        }

        // High reputation oracles can grant longer grace periods (1.5x multiplier)
        // Use basis points: 15000 = 1.5x, 10000 = 1.0x
        let max_duration_bps: u64 = if oracle_rep >= REP_HIGH { 15000 } else { 10000 };

        // Get account data
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let account_data = match accounts.get(account.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Check grace period cooldown (anti-gaming: Part 5.3)
        let current_time = env.ledger().timestamp();
        let last_grace_times: Map<Address, u64> = env
            .storage()
            .persistent()
            .get(&KEY_LAST_GRACE_TIMES)
            .unwrap_or(Map::new(&env));

        if let Some(last_time) = last_grace_times.get(account.clone()) {
            let days_since_grace = (current_time - last_time) / SECONDS_PER_DAY;
            if days_since_grace < GRACE_COOLDOWN_DAYS {
                panic!("Must wait 90 days between grace periods");
            }
        }

        // Check anti-abuse: max 3 grace periods per year, requires 30+ contribution hours
        let current_year = env.ledger().timestamp() / (365 * SECONDS_PER_DAY);

        if account_data.last_grace_year == current_year as u32
            && account_data.grace_periods_used >= MAX_GRACE_PERIODS_PER_YEAR
        {
            panic!("Maximum grace periods used for this year");
        }

        if account_data.contribution_hours < MIN_CONTRIBUTION_HOURS {
            panic!(
                "Must have at least {} contribution hours to qualify for grace period",
                MIN_CONTRIBUTION_HOURS
            );
        }

        // Validate duration based on grace type
        let base_max_days = match grace_type {
            GraceType::Emergency => 90,  // Emergency: up to 90 days
            GraceType::Illness => 60,    // Illness: up to 60 days
            GraceType::Community => 180, // Community: up to 180 days
        };

        let max_days = (base_max_days * max_duration_bps) / 10000;

        if duration_days > max_days {
            panic!("Duration exceeds maximum for this grace type");
        }

        let current_time = env.ledger().timestamp();
        let end_time = current_time + (duration_days * SECONDS_PER_DAY);
        let grace_type_code = grace_type.clone() as u32;

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
        let mut grace_periods: Map<Address, GracePeriod> = env
            .storage()
            .persistent()
            .get(&KEY_GRACE_PERIODS)
            .unwrap_or(Map::new(&env));
        grace_periods.set(account.clone(), grace_period);
        env.storage()
            .persistent()
            .set(&KEY_GRACE_PERIODS, &grace_periods);

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

        // Store last grace time for cooldown (anti-gaming: Part 5.3)
        let mut last_grace_times: Map<Address, u64> = env
            .storage()
            .persistent()
            .get(&KEY_LAST_GRACE_TIMES)
            .unwrap_or(Map::new(&env));
        last_grace_times.set(account.clone(), current_time);
        env.storage()
            .persistent()
            .set(&KEY_LAST_GRACE_TIMES, &last_grace_times);

        // Update oracle stats
        let mut oracles: Map<Address, OracleData> =
            env.storage().persistent().get(&KEY_ORACLES).unwrap();
        let mut oracle_data = oracles.get(oracle.clone()).unwrap();
        oracle_data.grace_periods_granted += 1;
        oracle_data.reputation_score =
            Self::get_reputation(env.clone(), oracle.clone(), RoleType::Oracle);
        oracles.set(oracle.clone(), oracle_data);
        env.storage().persistent().set(&KEY_ORACLES, &oracles);

        // Update oracle reputation (+5 for granting grace period appropriately)
        Self::update_reputation(&env, &oracle, &RoleType::Oracle, 5, REP_EVENT_GRACE_GRANTED);

        // Emit grace period activated event for PWA integration
        GraceActivated {
            account: account.clone(),
            grace_type: grace_type_code,
            duration_days,
            end_time,
        }
        .publish(&env);
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
        let grace_periods: Map<Address, GracePeriod> = env
            .storage()
            .persistent()
            .get(&KEY_GRACE_PERIODS)
            .unwrap_or(Map::new(&env));

        grace_periods.get(account)
    }

    /// Extend an existing grace period (requires community voting)
    pub fn extend_grace_period(env: Env, account: Address, additional_days: u64) {
        // Check if account has an active grace period
        let mut grace_periods: Map<Address, GracePeriod> = env
            .storage()
            .persistent()
            .get(&KEY_GRACE_PERIODS)
            .unwrap_or(Map::new(&env));

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
        env.storage()
            .persistent()
            .set(&KEY_GRACE_PERIODS, &grace_periods);

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

    /// Report grace period abuse
    /// Called by an oracle when a member violates grace period conditions
    /// (e.g., claims illness but is seen working elsewhere)
    /// Penalizes the abuser and ends their grace period early
    pub fn report_grace_abuse(env: Env, reporter: Address, account: Address, reason: String) {
        // Only registered oracles can report abuse
        reporter.require_auth();
        let oracles: Map<Address, OracleData> = env
            .storage()
            .persistent()
            .get(&KEY_ORACLES)
            .unwrap_or(Map::new(&env));

        if oracles.get(reporter.clone()).is_none() {
            panic!("Only registered oracles can report grace abuse");
        }

        // Verify account has/had a grace period
        let grace_periods: Map<Address, GracePeriod> = env
            .storage()
            .persistent()
            .get(&KEY_GRACE_PERIODS)
            .unwrap_or(Map::new(&env));

        let grace_period = match grace_periods.get(account.clone()) {
            Some(gp) => gp,
            None => panic!("No grace period found for account"),
        };

        let grace_type_code = grace_period.grace_type.clone() as u32;
        let current_time = env.ledger().timestamp();

        // End grace period early if still active
        if current_time < grace_period.end_time {
            // Update account data to end grace period
            let mut accounts: Map<Address, AccountData> =
                env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

            if let Some(mut account_data) = accounts.get(account.clone()) {
                account_data.grace_period_end = current_time; // End now
                accounts.set(account.clone(), account_data);
                env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
            }

            // Remove grace period from storage
            let mut grace_periods_mut = grace_periods;
            grace_periods_mut.remove(account.clone());
            env.storage()
                .persistent()
                .set(&KEY_GRACE_PERIODS, &grace_periods_mut);
        }

        // Penalize the abuser's reputation (-50 for violating grace terms)
        // This affects both Worker and Member roles
        let _ = Self::update_reputation(
            &env,
            &account,
            &RoleType::Worker,
            -50,
            REP_EVENT_GRACE_ABUSED,
        );

        let _ = Self::update_reputation(
            &env,
            &account,
            &RoleType::Member,
            -25,
            REP_EVENT_GRACE_ABUSED,
        );

        // Update oracle's abuse reports count
        let mut oracles_mut: Map<Address, OracleData> = oracles;
        if let Some(mut oracle_data) = oracles_mut.get(reporter.clone()) {
            oracle_data.abuse_reports += 1;
            oracles_mut.set(reporter.clone(), oracle_data);
            env.storage().persistent().set(&KEY_ORACLES, &oracles_mut);
        }

        // Emit grace revoked event
        GraceRevoked {
            account: account.clone(),
            reporter: reporter.clone(),
            grace_type: grace_type_code,
            reason,
        }
        .publish(&env);
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
    pub fn cross_trust_swap(env: Env, from: Address, dest_trust: Address, amount: U256) {
        from.require_auth();

        // Get accounts
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let from_data = match accounts.get(from.clone()) {
            Some(data) => data,
            None => panic!("Account not found"),
        };

        // Get from trust (clone to avoid borrow issues)
        let from_trust = from_data
            .trust_id
            .clone()
            .expect("Must be in a trust to perform cross-trust swap");

        // Calculate exchange rate
        let exchange_rate_bps =
            Self::calculate_exchange_rate(env.clone(), from_trust.clone(), dest_trust.clone());

        // Calculate destination amount: amount * exchange_rate / 10000
        let dest_amount = {
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

        // Update from account (deduct original amount, credit dest_amount, update trust membership)
        let mut updated_from = from_data;
        updated_from.balance = balance_after_demurrage.sub(&amount);
        updated_from.balance = updated_from.balance.add(&dest_amount); // Credit rate-adjusted amount
        updated_from.last_activity = env.ledger().timestamp();
        updated_from.trust_id = Some(dest_trust.clone()); // Move to destination trust
        accounts.set(from.clone(), updated_from);

        // Update trust member counts
        let mut trusts: Map<Address, TrustData> =
            env.storage().persistent().get(&KEY_TRUSTS).unwrap();

        // Decrement source trust count
        if let Some(mut source_trust_data) = trusts.get(from_trust.clone()) {
            source_trust_data.member_count -= 1;
            trusts.set(from_trust.clone(), source_trust_data);
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
        let exchange_rate_bps =
            Self::calculate_exchange_rate(env.clone(), source_trust, dest_trust);

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
        trust_id: Option<Address>,
        new_rate_bps: Option<u32>,
        target_address: Option<Address>,
    ) -> u64 {
        proposer.require_auth();

        // Require proposal stake (spam prevention)
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        let mut proposer_account = match accounts.get(proposer.clone()) {
            Some(data) => data,
            None => panic!("Proposer account not found"),
        };

        let stake_amount = U256::from_u128(&env, PROPOSAL_STAKE as u128);
        if proposer_account.balance < stake_amount {
            panic!("Insufficient balance for proposal stake (100 KCHNG required)");
        }

        // Deduct stake from proposer's balance
        proposer_account.balance = proposer_account.balance.sub(&stake_amount);
        accounts.set(proposer.clone(), proposer_account);
        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);

        // Validate proposer authority
        match proposal_type.clone() {
            ProposalType::RateChange | ProposalType::TrustParameters => {
                // Must be governor of the trust
                let trust_addr = trust_id
                    .as_ref()
                    .expect("Trust-specific proposals require a trust_id");
                let trusts: Map<Address, TrustData> =
                    env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                let trust_data = match trusts.get(trust_addr.clone()) {
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
            ProposalType::RemoveVerifier => {
                // Governor can propose if verifier reputation < 300
                // Members can propose if verifier reputation < 500 AND 30 days elapsed
                let target = target_address
                    .as_ref()
                    .expect("RemoveVerifier requires target_address");

                // Check if target is a verifier
                let verifiers: Map<Address, VerifierData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_VERIFIERS)
                    .unwrap_or(Map::new(&env));
                if !verifiers.contains_key(target.clone()) {
                    panic!("Target is not a verifier");
                }

                let verifier_rep =
                    Self::get_reputation(env.clone(), target.clone(), RoleType::Verifier);

                // Check if proposer is governor
                if let Some(ref tid) = trust_id {
                    let trusts: Map<Address, TrustData> =
                        env.storage().persistent().get(&KEY_TRUSTS).unwrap();
                    if let Some(trust) = trusts.get(tid.clone()) {
                        if trust.governor == proposer && verifier_rep < 300 {
                            // Governor can propose removal for low rep
                        } else if verifier_rep < 500 {
                            // Member can propose - need to check 30 day elapsed
                            // (simplified check for now)
                        } else {
                            panic!("Verifier reputation too high for removal");
                        }
                    }
                } else {
                    panic!("RemoveVerifier requires a trust_id");
                }
            }
            ProposalType::RemoveGovernor => {
                // Members can propose if governor reputation < 500 AND 30 days elapsed
                let target = target_address
                    .as_ref()
                    .expect("RemoveGovernor requires target_address");
                let governor_rep =
                    Self::get_reputation(env.clone(), target.clone(), RoleType::Governor);

                if governor_rep >= 500 {
                    panic!("Governor reputation too high for removal");
                }
            }
            ProposalType::RemoveOracle => {
                // Any verifier can propose if oracle reputation < 200
                let verifiers: Map<Address, VerifierData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_VERIFIERS)
                    .unwrap_or(Map::new(&env));
                if !verifiers.contains_key(proposer.clone()) {
                    panic!("Only verifiers can propose oracle removal");
                }

                let target = target_address
                    .as_ref()
                    .expect("RemoveOracle requires target_address");
                let oracle_rep =
                    Self::get_reputation(env.clone(), target.clone(), RoleType::Oracle);

                if oracle_rep >= 200 {
                    panic!("Oracle reputation too high for removal");
                }
            }
            ProposalType::RoleProbation => {
                // Must be admin or governor
                let admin: Address = env.storage().instance().get(&KEY_ADMIN).unwrap();
                let is_admin = admin == proposer;

                if !is_admin {
                    if let Some(ref tid) = trust_id {
                        let trusts: Map<Address, TrustData> =
                            env.storage().persistent().get(&KEY_TRUSTS).unwrap();
                        if let Some(trust) = trusts.get(tid.clone())
                            && trust.governor != proposer
                        {
                            panic!("Only admin or trust governor can propose probation");
                        }
                    } else {
                        panic!("Only admin can propose protocol-level probation");
                    }
                }
            }
        }

        // Validate rate change if provided
        if let Some(rate) = new_rate_bps
            && (!(MIN_ANNUAL_RATE_BPS..=MAX_ANNUAL_RATE_BPS).contains(&rate))
        {
            panic!("Rate must be within protocol bounds (5-15%)");
        }

        // Calculate proposal timeline
        let current_timestamp = env.ledger().timestamp();
        let review_end = current_timestamp + (REVIEW_PERIOD_DAYS * SECONDS_PER_DAY);
        let vote_end = review_end + (VOTE_PERIOD_DAYS * SECONDS_PER_DAY);
        let implementation_date = vote_end + (IMPLEMENTATION_NOTICE_DAYS * SECONDS_PER_DAY);

        // Generate proposal ID
        let mut proposals: Map<u64, Proposal> = env
            .storage()
            .persistent()
            .get(&KEY_PROPOSALS)
            .unwrap_or(Map::new(&env));
        let proposal_id: u64 = proposals.len().into();
        let proposal_type_code = proposal_type.clone() as u32;

        // Create proposal
        let proposal = Proposal {
            proposal_id,
            proposer: proposer.clone(),
            proposal_type,
            title,
            description,
            trust_id,
            new_rate_bps,
            target_address,
            stake: PROPOSAL_STAKE,
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

        // Emit proposal created event for PWA integration
        ProposalCreated {
            proposer: proposer.clone(),
            proposal_id,
            proposal_type: proposal_type_code,
        }
        .publish(&env);

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
        if let Some(proposal_trust) = &proposal.trust_id {
            let accounts: Map<Address, AccountData> =
                env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

            let account = match accounts.get(voter.clone()) {
                Some(a) => a,
                None => panic!("Not a trust member"),
            };

            if account.trust_id.as_ref() != Some(proposal_trust) {
                panic!("Not a member of this trust");
            }
        }

        // Record vote
        if support {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }

        // Add to voters list (clone so we can use voter for reputation update)
        proposal.voters.push_back(voter.clone());

        proposals.set(proposal_id, proposal);
        env.storage().persistent().set(&KEY_PROPOSALS, &proposals);

        // Emit vote cast event for PWA integration
        VoteCast {
            voter: voter.clone(),
            proposal_id,
            support,
        }
        .publish(&env);

        // Update member reputation for voting participation (+2)
        Self::update_reputation(
            &env,
            &voter,
            &RoleType::Member,
            2,
            REP_EVENT_VOTE_PARTICIPATE,
        );
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
                    let member_count = match &proposal.trust_id {
                        None => {
                            // Protocol proposal: use total accounts as quorum base
                            let accounts: Map<Address, AccountData> =
                                env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
                            accounts.len()
                        }
                        Some(trust_id) => {
                            // Trust proposal: use trust member count
                            let trusts: Map<Address, TrustData> =
                                env.storage().persistent().get(&KEY_TRUSTS).unwrap();
                            let trust_data = trusts.get(trust_id.clone()).unwrap();
                            trust_data.member_count
                        }
                    };

                    // Check quorum (40% participation required)
                    let quorum_met = total_votes >= (member_count * 40 / 100);

                    if !quorum_met {
                        proposal.status = ProposalStatus::Expired;
                        // Penalty for proposer when quorum not met (-3)
                        Self::update_reputation(
                            &env,
                            &proposal.proposer,
                            &RoleType::Governor,
                            -3,
                            REP_EVENT_PROPOSAL_FAIL,
                        );
                        // Return stake to proposer
                        Self::return_proposal_stake(&env, &proposal.proposer, proposal.stake);
                    } else {
                        // Check approval (60% support required, or 80% for emergency, 60% for removals)
                        let approval_threshold = match proposal.proposal_type {
                            ProposalType::Emergency => 80,
                            ProposalType::RemoveVerifier => 60,
                            ProposalType::RemoveGovernor => 60,
                            ProposalType::RemoveOracle => 60,
                            ProposalType::RoleProbation => 60,
                            _ => 60,
                        };

                        // Fix division by zero: check if total_votes is zero
                        if total_votes == 0 {
                            proposal.status = ProposalStatus::Expired;
                        } else {
                            let approval_percentage = (proposal.votes_for * 100) / total_votes;

                            if approval_percentage >= approval_threshold {
                                proposal.status = ProposalStatus::Approved;
                                // Reward for proposer when proposal passes (+5)
                                Self::update_reputation(
                                    &env,
                                    &proposal.proposer,
                                    &RoleType::Governor,
                                    5,
                                    REP_EVENT_PROPOSAL_PASS,
                                );
                            } else {
                                proposal.status = ProposalStatus::Rejected;
                            }
                            // Return stake to proposer (for both approved and rejected)
                            Self::return_proposal_stake(&env, &proposal.proposer, proposal.stake);
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

        // Execute proposal based on type
        match proposal.proposal_type {
            ProposalType::RateChange => {
                if let Some(new_rate) = proposal.new_rate_bps {
                    let trust_id = proposal
                        .trust_id
                        .as_ref()
                        .expect("Rate change requires a trust");

                    let mut trusts: Map<Address, TrustData> =
                        env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                    let mut trust_data = match trusts.get(trust_id.clone()) {
                        Some(t) => t,
                        None => panic!("Trust not found"),
                    };

                    trust_data.annual_rate_bps = new_rate;
                    trusts.set(trust_id.clone(), trust_data);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                }
            }
            ProposalType::Emergency => {
                if let Some(new_rate) = proposal.new_rate_bps {
                    // Emergency rate can exceed MAX_ANNUAL_RATE_BPS temporarily
                    let trust_id = proposal
                        .trust_id
                        .as_ref()
                        .expect("Emergency rate change requires a trust");

                    let mut trusts: Map<Address, TrustData> =
                        env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                    let mut trust_data = match trusts.get(trust_id.clone()) {
                        Some(t) => t,
                        None => panic!("Trust not found"),
                    };

                    trust_data.annual_rate_bps = new_rate;
                    trusts.set(trust_id.clone(), trust_data);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                }
            }
            ProposalType::TrustParameters => {
                // Handle trust parameter changes
                // For now, only rate changes are supported
                if let Some(new_rate) = proposal.new_rate_bps {
                    let trust_id = proposal
                        .trust_id
                        .as_ref()
                        .expect("Trust parameter change requires a trust");

                    let mut trusts: Map<Address, TrustData> =
                        env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                    let mut trust_data = match trusts.get(trust_id.clone()) {
                        Some(t) => t,
                        None => panic!("Trust not found"),
                    };

                    trust_data.annual_rate_bps = new_rate;
                    trusts.set(trust_id.clone(), trust_data);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                }
            }
            ProposalType::ProtocolUpgrade => {
                // Protocol upgrades require contract upgrade
                // This is a placeholder for future implementation
                panic!("Protocol upgrades must be executed via contract upgrade");
            }
            ProposalType::RemoveVerifier => {
                let target = proposal
                    .target_address
                    .as_ref()
                    .expect("RemoveVerifier requires target_address");

                // Remove verifier with stake slashing
                let mut verifiers: Map<Address, VerifierData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_VERIFIERS)
                    .unwrap_or(Map::new(&env));

                if let Some(verifier_data) = verifiers.get(target.clone()) {
                    // Slash 10% of stake
                    let slash_amount = verifier_data
                        .stake
                        .mul(&U256::from_u32(&env, VERIFIER_SLASH_BPS))
                        .div(&U256::from_u32(&env, 10000));
                    let stake_to_return = verifier_data.stake.sub(&slash_amount);

                    // Return remaining stake
                    let mut accounts: Map<Address, AccountData> =
                        env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
                    if let Some(mut account) = accounts.get(target.clone()) {
                        account.balance = account.balance.add(&stake_to_return);
                        accounts.set(target.clone(), account);
                        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
                    }

                    verifiers.remove(target.clone());
                    env.storage().persistent().set(&KEY_VERIFIERS, &verifiers);
                }
            }
            ProposalType::RemoveGovernor => {
                let target = proposal
                    .target_address
                    .as_ref()
                    .expect("RemoveGovernor requires target_address");

                // Get trust and transfer governance to successor or disable
                let trust_id = proposal
                    .trust_id
                    .as_ref()
                    .expect("RemoveGovernor requires trust_id");

                let mut trusts: Map<Address, TrustData> =
                    env.storage().persistent().get(&KEY_TRUSTS).unwrap();

                if let Some(mut trust) = trusts.get(trust_id.clone())
                    && trust.governor == *target
                {
                    // Check if successor is designated
                    if let Some(successor) = trust.successor.clone() {
                        // Transfer governance to successor
                        trust.governor = successor.clone();
                        trust.successor = None; // Clear successor

                        // Update governor trusts mapping
                        let mut governor_trusts: Map<Address, Address> = env
                            .storage()
                            .persistent()
                            .get(&KEY_GOVERNOR_TRUSTS)
                            .unwrap_or(Map::new(&env));
                        governor_trusts.remove(target.clone());
                        governor_trusts.set(successor, trust_id.clone());
                        env.storage()
                            .persistent()
                            .set(&KEY_GOVERNOR_TRUSTS, &governor_trusts);
                    } else {
                        // No successor - disable trust
                        trust.is_active = false;
                    }
                    trusts.set(trust_id.clone(), trust);
                    env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                }
            }
            ProposalType::RemoveOracle => {
                let target = proposal
                    .target_address
                    .as_ref()
                    .expect("RemoveOracle requires target_address");

                // Remove oracle with stake slashing
                let mut oracles: Map<Address, OracleData> = env
                    .storage()
                    .persistent()
                    .get(&KEY_ORACLES)
                    .unwrap_or(Map::new(&env));

                if let Some(oracle_data) = oracles.get(target.clone()) {
                    // Slash 25% of stake
                    let slash_amount = oracle_data
                        .stake
                        .mul(&U256::from_u32(&env, ORACLE_SLASH_BPS))
                        .div(&U256::from_u32(&env, 10000));
                    let stake_to_return = oracle_data.stake.sub(&slash_amount);

                    // Return remaining stake
                    let mut accounts: Map<Address, AccountData> =
                        env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
                    if let Some(mut account) = accounts.get(target.clone()) {
                        account.balance = account.balance.add(&stake_to_return);
                        accounts.set(target.clone(), account);
                        env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
                    }

                    oracles.remove(target.clone());
                    env.storage().persistent().set(&KEY_ORACLES, &oracles);
                }
            }
            ProposalType::RoleProbation => {
                let target = proposal
                    .target_address
                    .as_ref()
                    .expect("RoleProbation requires target_address");

                // Set probation for 90 days
                let mut rep_data =
                    Self::get_reputation_data_internal(&env, target, &RoleType::Verifier);
                rep_data.probation_until = Some(current_timestamp + (90 * SECONDS_PER_DAY));
                Self::save_reputation_data(&env, target, &rep_data);
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
        let proposals: Map<u64, Proposal> = env.storage().persistent().get(&KEY_PROPOSALS).unwrap();

        match proposals.get(proposal_id) {
            Some(p) => p,
            None => panic!("Proposal not found"),
        }
    }

    /// Get all proposals
    pub fn get_all_proposals(env: Env) -> Vec<u64> {
        let proposals: Map<u64, Proposal> = env
            .storage()
            .persistent()
            .get(&KEY_PROPOSALS)
            .unwrap_or(Map::new(&env));

        let mut keys = Vec::new(&env);
        for (k, _) in proposals.iter() {
            keys.push_back(k);
        }
        keys
    }

    // ============================================================================
    // REPUTATION SYSTEM (Phase 7)
    // ============================================================================

    /// Initialize reputation data for a role (internal helper)
    fn init_reputation_data(env: &Env, role: RoleType) -> ReputationData {
        ReputationData {
            role_type: role,
            score: REP_NEUTRAL,
            last_change: env.ledger().timestamp(),
            consecutive_negatives: 0,
            probation_until: None,
            recent_events: Vec::new(env),
        }
    }

    /// Get reputation data for an address and role
    fn get_reputation_data_internal(
        env: &Env,
        address: &Address,
        role: &RoleType,
    ) -> ReputationData {
        let reputations: Map<Address, Map<u32, ReputationData>> = env
            .storage()
            .persistent()
            .get(&KEY_REPUTATIONS)
            .unwrap_or(Map::new(env));

        match reputations.get(address.clone()) {
            Some(role_map) => {
                let role_key = Self::role_type_to_key(role);
                match role_map.get(role_key) {
                    Some(data) => data,
                    None => Self::init_reputation_data(env, role.clone()),
                }
            }
            None => Self::init_reputation_data(env, role.clone()),
        }
    }

    /// Convert RoleType to storage key
    fn role_type_to_key(role: &RoleType) -> u32 {
        match role {
            RoleType::Governor => 0,
            RoleType::Verifier => 1,
            RoleType::Oracle => 2,
            RoleType::Worker => 3,
            RoleType::Member => 4,
        }
    }

    /// Update reputation for a role (internal)
    /// Returns the new score
    fn update_reputation(
        env: &Env,
        address: &Address,
        role: &RoleType,
        change: i32,
        event_type: u32,
    ) -> u32 {
        let mut data = Self::get_reputation_data_internal(env, address, role);
        let current_time = env.ledger().timestamp();

        // Apply decay before updating
        Self::apply_reputation_decay_internal(env, &mut data);

        // Store old score for threshold detection
        let old_score = data.score;

        // Track consecutive negatives for TF2T pattern detection
        if change < 0 {
            data.consecutive_negatives += 1;
        } else if change > 0 {
            data.consecutive_negatives = 0;
        }

        // Calculate new score with bounds [0, 1000]
        let new_score = (data.score as i32 + change).clamp(0, 1000) as u32;
        data.score = new_score;
        data.last_change = current_time;

        // Add to history (keep last 10 events)
        let event = ReputationEvent {
            timestamp: current_time,
            event_type,
            change,
            new_score,
        };
        data.recent_events.push_back(event);
        if data.recent_events.len() > MAX_HISTORY_EVENTS {
            data.recent_events.pop_front();
        }

        // Store updated reputation
        Self::save_reputation_data(env, address, &data);

        // Emit reputation changed event for PWA integration
        ReputationChanged {
            address: address.clone(),
            role: role.clone() as u32,
            change,
            new_score,
        }
        .publish(env);

        // Emit threshold notification if score dropped below thresholds
        // This enables circle review for sociocratic governance
        if new_score < REMOVAL_THRESHOLD {
            ReputationThreshold {
                address: address.clone(),
                role: role.clone() as u32,
                score: new_score,
                threshold: REMOVAL_THRESHOLD,
            }
            .publish(env);
            Self::enforce_removal_threshold(env, address, role);
        } else if new_score < REP_RESTRICTED && old_score >= REP_RESTRICTED {
            // Crossed into probation territory
            ReputationThreshold {
                address: address.clone(),
                role: role.clone() as u32,
                score: new_score,
                threshold: REP_RESTRICTED,
            }
            .publish(env);
        }

        new_score
    }

    /// Apply TF2T pattern penalty for consecutive negatives
    fn apply_pattern_penalty_if_needed(env: &Env, address: &Address, role: &RoleType) {
        let mut data = Self::get_reputation_data_internal(env, address, role);

        // TF2T: Additional penalty for 2+ consecutive negative events
        if data.consecutive_negatives >= 2 {
            let penalty = -25i32; // Additional penalty
            data.consecutive_negatives = 0; // Reset after applying

            let new_score = (data.score as i32 + penalty).max(0) as u32;
            data.score = new_score;

            // Add pattern penalty event
            let event = ReputationEvent {
                timestamp: env.ledger().timestamp(),
                event_type: REP_EVENT_PATTERN_PENALTY,
                change: penalty,
                new_score,
            };
            data.recent_events.push_back(event);
            if data.recent_events.len() > MAX_HISTORY_EVENTS {
                data.recent_events.pop_front();
            }

            Self::save_reputation_data(env, address, &data);
        }
    }

    /// Check and enforce removal threshold (< 100 reputation)
    /// Returns true if the role was auto-removed
    fn enforce_removal_threshold(env: &Env, address: &Address, role: &RoleType) -> bool {
        let data = Self::get_reputation_data_internal(env, address, role);

        if data.score >= REMOVAL_THRESHOLD {
            return false;
        }

        match role {
            RoleType::Governor => {
                // Auto-disable trust when governor reputation < 100
                let governor_trusts: Map<Address, Address> = env
                    .storage()
                    .persistent()
                    .get(&KEY_GOVERNOR_TRUSTS)
                    .unwrap_or(Map::new(env));

                if let Some(trust_id) = governor_trusts.get(address.clone()) {
                    let mut trusts: Map<Address, TrustData> = env
                        .storage()
                        .persistent()
                        .get(&KEY_TRUSTS)
                        .unwrap_or(Map::new(env));

                    if let Some(mut trust) = trusts.get(trust_id.clone()) {
                        trust.is_active = false;
                        trusts.set(trust_id, trust);
                        env.storage().persistent().set(&KEY_TRUSTS, &trusts);
                    }
                }
                true
            }
            RoleType::Verifier => {
                // Auto-unregister verifier with double slash (handled in unregister_verifier)
                // Just mark for removal - actual removal requires calling unregister_verifier
                false // Don't auto-remove, require explicit unregister for stake handling
            }
            RoleType::Oracle => {
                // Auto-unregister oracle with double slash (handled in unregister_oracle)
                false // Don't auto-remove, require explicit unregister for stake handling
            }
            RoleType::Worker => {
                // Worker with < 100 reputation cannot submit claims
                // This is enforced in submit_work_claim via is_on_probation check
                true
            }
            RoleType::Member => {
                // Members can't be removed, just have low reputation
                false
            }
        }
    }

    /// Apply reputation decay based on inactivity (internal)
    fn apply_reputation_decay_internal(env: &Env, data: &mut ReputationData) {
        let current_time = env.ledger().timestamp();
        let days_inactive = (current_time.saturating_sub(data.last_change)) / SECONDS_PER_DAY;

        if data.score > REP_NEUTRAL && days_inactive >= DECAY_START_DAYS {
            // High scores decay toward 500
            let decay_days = days_inactive.saturating_sub(DECAY_START_DAYS);
            let max_decay = data.score.saturating_sub(REP_NEUTRAL);
            let decay = min(decay_days as u32, max_decay);

            if decay > 0 {
                data.score = data.score.saturating_sub(decay);
                data.last_change = current_time;

                let event = ReputationEvent {
                    timestamp: current_time,
                    event_type: REP_EVENT_DECAY,
                    change: -(decay as i32),
                    new_score: data.score,
                };
                data.recent_events.push_back(event);
                if data.recent_events.len() > MAX_HISTORY_EVENTS {
                    data.recent_events.pop_front();
                }
            }
        } else if data.score < REP_NEUTRAL && days_inactive >= RECOVERY_START_DAYS {
            // Low scores recover toward 500 (slower)
            let recovery_days = days_inactive.saturating_sub(RECOVERY_START_DAYS);
            let max_recovery = REP_NEUTRAL.saturating_sub(data.score);
            let recovery = min(recovery_days as u32, max_recovery);

            if recovery > 0 {
                data.score = data.score.saturating_add(recovery);
                data.last_change = current_time;

                let event = ReputationEvent {
                    timestamp: current_time,
                    event_type: REP_EVENT_RECOVERY,
                    change: recovery as i32,
                    new_score: data.score,
                };
                data.recent_events.push_back(event);
                if data.recent_events.len() > MAX_HISTORY_EVENTS {
                    data.recent_events.pop_front();
                }
            }
        }
    }

    /// Save reputation data to storage
    fn save_reputation_data(env: &Env, address: &Address, data: &ReputationData) {
        let mut reputations: Map<Address, Map<u32, ReputationData>> = env
            .storage()
            .persistent()
            .get(&KEY_REPUTATIONS)
            .unwrap_or(Map::new(env));

        let role_key = Self::role_type_to_key(&data.role_type);

        let mut role_map = reputations.get(address.clone()).unwrap_or(Map::new(env));
        role_map.set(role_key, data.clone());
        reputations.set(address.clone(), role_map);

        env.storage()
            .persistent()
            .set(&KEY_REPUTATIONS, &reputations);
    }

    /// Return proposal stake to proposer
    fn return_proposal_stake(env: &Env, proposer: &Address, stake: u64) {
        let mut accounts: Map<Address, AccountData> =
            env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();

        if let Some(mut account) = accounts.get(proposer.clone()) {
            account.balance = account.balance.add(&U256::from_u128(env, stake as u128));
            account.last_activity = env.ledger().timestamp();
            accounts.set(proposer.clone(), account);
            env.storage().persistent().set(&KEY_ACCOUNTS, &accounts);
        }
    }

    /// Get reputation score for a role (public)
    pub fn get_reputation(env: Env, address: Address, role: RoleType) -> u32 {
        let mut data = Self::get_reputation_data_internal(&env, &address, &role);
        Self::apply_reputation_decay_internal(&env, &mut data);
        data.score
    }

    /// Get full reputation data for a role (public)
    pub fn get_reputation_data(env: Env, address: Address, role: RoleType) -> ReputationData {
        let mut data = Self::get_reputation_data_internal(&env, &address, &role);
        Self::apply_reputation_decay_internal(&env, &mut data);
        data
    }

    /// Check if address is in probation for a role (public)
    pub fn is_on_probation(env: Env, address: Address, role: RoleType) -> bool {
        let data = Self::get_reputation_data_internal(&env, &address, &role);

        // Check explicit probation
        if let Some(probation_end) = data.probation_until
            && env.ledger().timestamp() < probation_end
        {
            return true;
        }

        // Check score-based probation (< 200)
        data.score < REP_RESTRICTED
    }

    /// Check if verifier can verify in multiple trusts (high reputation)
    pub fn can_multi_trust_verify(env: Env, address: Address) -> bool {
        let score = Self::get_reputation(env, address, RoleType::Verifier);
        score >= REP_HIGH
    }

    /// Get all reputation scores for an address
    pub fn get_all_reputations(env: Env, address: Address) -> Map<u32, u32> {
        let reputations: Map<Address, Map<u32, ReputationData>> = env
            .storage()
            .persistent()
            .get(&KEY_REPUTATIONS)
            .unwrap_or(Map::new(&env));

        let mut result = Map::new(&env);

        if let Some(role_map) = reputations.get(address.clone()) {
            for (role_key, data) in role_map.iter() {
                result.set(role_key, data.score);
            }
        } else {
            // Return default neutral scores for all roles
            for role_key in 0..5u32 {
                result.set(role_key, REP_NEUTRAL);
            }
        }

        result
    }

    /// Set probation period for an address
    pub fn set_probation(env: Env, address: Address, role: RoleType, duration_days: u64) {
        let admin: Address = env.storage().instance().get(&KEY_ADMIN).unwrap();
        admin.require_auth();

        let mut data = Self::get_reputation_data_internal(&env, &address, &role);
        let probation_end = env.ledger().timestamp() + (duration_days * SECONDS_PER_DAY);
        data.probation_until = Some(probation_end);

        Self::save_reputation_data(&env, &address, &data);
    }

    // ============================================================================
    // MIGRATION (for contract upgrades)
    // ============================================================================

    /// Migrate data from an old contract after an upgrade.
    ///
    /// During Soroban contract upgrades, instance storage is cleared while
    /// persistent storage survives. This function restores instance storage
    /// from the old contract and validates that persistent data is intact.
    ///
    /// # Arguments
    /// * `admin` - The admin address (must match the source contract's admin)
    /// * `source_contract` - Address of the old contract to migrate from
    /// * `expected_protocol_version` - Expected version of the source contract
    ///
    /// # Returns
    /// * `MigrationResult` - Details about the migration including validation counts
    ///
    /// # Panics
    /// * If caller is not the admin
    /// * If migration was already completed
    /// * If source protocol version doesn't match expected
    pub fn migrate_data(
        env: Env,
        admin: Address,
        source_contract: Address,
        expected_protocol_version: u32,
    ) -> MigrationResult {
        admin.require_auth();

        // Check if migration already completed (idempotency)
        if env
            .storage()
            .instance()
            .has(&KEY_MIGRATION_STATUS)
        {
            panic!("Migration already completed");
        }

        // Create client to call source contract
        let source_client = KchngTokenClient::new(&env, &source_contract);

        // Read instance data from source contract
        let source_admin = source_client.get_admin();
        let source_version = source_client.get_protocol_version();
        let source_total_supply = source_client.get_total_supply_raw();
        let source_next_claim_id = source_client.get_next_claim_id();
        let source_next_proposal_id = source_client.get_next_proposal_id();

        // Verify admin matches caller
        if source_admin != admin {
            panic!("Source contract admin does not match caller");
        }

        // Verify protocol version matches expected
        if source_version != expected_protocol_version {
            panic!("Source protocol version mismatch");
        }

        // Store instance data (version is incremented by 1)
        env.storage().instance().set(&KEY_ADMIN, &source_admin);
        env.storage()
            .instance()
            .set(&KEY_PROTOCOL_VERSION, &U256::from_u32(&env, source_version + 1));
        env.storage()
            .instance()
            .set(&KEY_TOTAL_SUPPLY, &source_total_supply);
        env.storage()
            .instance()
            .set(&KEY_NEXT_CLAIM_ID, &U256::from_u128(&env, source_next_claim_id as u128));
        env.storage()
            .instance()
            .set(&KEY_NEXT_PROPOSAL_ID, &U256::from_u128(&env, source_next_proposal_id as u128));

        // Validate persistent storage (count entries, don't fail on errors)
        let mut errors = Vec::new(&env);

        // Count accounts
        let accounts_validated = if env.storage().persistent().has(&KEY_ACCOUNTS) {
            let accounts: Map<Address, AccountData> =
                env.storage().persistent().get(&KEY_ACCOUNTS).unwrap();
            accounts.len() as u32
        } else {
            errors.push_back(String::from_str(&env, "No accounts found"));
            0
        };

        // Count trusts
        let trusts_validated = if env.storage().persistent().has(&KEY_TRUSTS) {
            let trusts: Map<Address, TrustData> =
                env.storage().persistent().get(&KEY_TRUSTS).unwrap();
            trusts.len() as u32
        } else {
            0 // Trusts can be empty
        };

        // Count verifiers
        let verifiers_validated = if env.storage().persistent().has(&KEY_VERIFIERS) {
            let verifiers: Map<Address, VerifierData> =
                env.storage().persistent().get(&KEY_VERIFIERS).unwrap();
            verifiers.len() as u32
        } else {
            0
        };

        // Count oracles
        let oracles_validated = if env.storage().persistent().has(&KEY_ORACLES) {
            let oracles: Map<Address, OracleData> =
                env.storage().persistent().get(&KEY_ORACLES).unwrap();
            oracles.len() as u32
        } else {
            0
        };

        // Count work claims
        let work_claims_validated = if env.storage().persistent().has(&KEY_WORK_CLAIMS) {
            let claims: Map<u64, WorkClaim> =
                env.storage().persistent().get(&KEY_WORK_CLAIMS).unwrap();
            claims.len() as u32
        } else {
            0
        };

        // Count proposals
        let proposals_validated = if env.storage().persistent().has(&KEY_PROPOSALS) {
            let proposals: Map<u64, Proposal> =
                env.storage().persistent().get(&KEY_PROPOSALS).unwrap();
            proposals.len() as u32
        } else {
            0
        };

        // Count grace periods
        let grace_periods_validated = if env.storage().persistent().has(&KEY_GRACE_PERIODS) {
            let grace: Map<Address, GracePeriod> =
                env.storage().persistent().get(&KEY_GRACE_PERIODS).unwrap();
            grace.len() as u32
        } else {
            0
        };

        // Count reputations
        let reputations_validated = if env.storage().persistent().has(&KEY_REPUTATIONS) {
            let reps: Map<Address, Map<u32, ReputationData>> =
                env.storage().persistent().get(&KEY_REPUTATIONS).unwrap();
            reps.len() as u32
        } else {
            0
        };

        // Store migration status
        let current_time = env.ledger().timestamp();
        let migration_status = MigrationStatus {
            instance_migrated: true,
            admin_migrated: true,
            protocol_version_migrated: true,
            total_supply_migrated: true,
            counters_migrated: true,
            persistent_validated: true,
            migrated_at: current_time,
            source_contract: source_contract.clone(),
        };
        env.storage()
            .instance()
            .set(&KEY_MIGRATION_STATUS, &migration_status);

        // Emit migration completed event
        MigrationCompleted {
            admin: admin.clone(),
            source_contract: source_contract.clone(),
            old_version: source_version,
            new_version: source_version + 1,
            timestamp: current_time,
        }
        .publish(&env);

        MigrationResult {
            success: true,
            accounts_validated,
            trusts_validated,
            verifiers_validated,
            oracles_validated,
            work_claims_validated,
            proposals_validated,
            grace_periods_validated,
            reputations_validated,
            errors,
        }
    }
}

#[cfg(test)]
mod test;
