/**
 * Shared types for KCHNG project
 * Time-Standard Economic Model
 */

/**
 * Stellar account address (StrKey encoded)
 */
export type AccountId = string;

/**
 * Token amount (using bigint for precision)
 */
export type Amount = bigint;

/**
 * Unix timestamp in seconds
 */
export type Timestamp = number;

// ============================================================================
// ENUMS
// ============================================================================

/**
 * Type of work for token issuance
 */
export enum WorkType {
  Basic = 0,         // Basic care/agriculture work (1.0x multiplier)
  Skilled = 1,       // Skilled care/heavy labor (1.3x multiplier)
  Training = 2,      // Training/teaching (1.5x multiplier)
  Emergency = 3,     // Emergency care (2.0x multiplier)
}

/**
 * Status of a work claim
 */
export enum ClaimStatus {
  Pending = 0,       // Waiting for verification
  Approved = 1,      // Approved and tokens minted
  Rejected = 2,      // Rejected by verifiers
  Expired = 3,       // Verification window expired,
}

/**
 * Type of grace period
 */
export enum GraceType {
  Emergency = 0,     // Emergency pause (14-90 days, oracle-activated)
  Illness = 1,       // Illness or injury (30+ days automatic)
  Community = 2,     // Community voted (30-180 days),
}

/**
 * Status of a governance proposal
 */
export enum ProposalStatus {
  Review = 0,        // In review period (7 days)
  Voting = 1,        // In voting period (3 days)
  Approved = 2,      // Approved, awaiting implementation
  Rejected = 3,      // Rejected by community
  Implemented = 4,   // Successfully implemented
  Expired = 5,       // Expired without passing,
}

/**
 * Type of proposal
 */
export enum ProposalType {
  RateChange = 0,           // Change trust demurrage rate
  TrustParameters = 1,      // Adjust trust parameters
  ProtocolUpgrade = 2,      // Protocol-level upgrade
  Emergency = 3,            // Emergency measure (crisis exception),
}

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/**
 * Account data including demurrage tracking
 */
export interface AccountData {
  balance: Amount;
  last_activity: Timestamp;
  grace_period_end: Timestamp;     // Timestamp when grace ends (0 if not in grace)
  trust_id: AccountId | null;      // Trust membership (null if none)
  contribution_hours: number;      // Total hours contributed
  grace_periods_used: number;      // Grace periods used this year
  last_grace_year: number;         // Year of last grace period
}

/**
 * Trust (community organization) data
 */
export interface TrustData {
  name: string;
  governor: AccountId;
  annual_rate_bps: number;         // Annual demurrage rate in basis points
  demurrage_period_days: number;
  member_count: number;
  is_active: boolean;
  created_at: Timestamp,
}

// ============================================================================
// ROLE-BASED REPUTATION TYPES
// ============================================================================

/**
 * High-level domain for aspect grouping
 *
 * Domains represent broad categories of human interaction and activity.
 * Each domain contains multiple aspects (activities), which in turn
 * contain multiple roles (positions within that activity).
 *
 * Example hierarchy:
 *   Domain: Transportation
 *     └── Aspect: Ride-sharing
 *         ├── Role: Driver
 *         └── Role: Passenger
 */
export enum AspectDomain {
  Hospitality = "hospitality",       // dining, hosting, events
  Transportation = "transportation", // ride-sharing, car rental, delivery
  Employment = "employment",         // work, management, freelance
  Verification = "verification",     // work verification, oracle services
  Community = "community",           // voting, governance, proposals
}

/**
 * Specific role within an aspect
 *
 * Roles represent positions or personas within a specific aspect/activity.
 * Examples: "driver", "passenger", "guest", "host", "employee", "employer"
 *
 * An individual can have different reputation scores for different roles,
 * reflecting real-world nuance where someone may excel in one role but
 * struggle in another.
 */
export type AspectRole = string;

/**
 * Aspect identifier within a domain
 *
 * An aspect represents a specific activity or context within a domain.
 * Examples: "ride_sharing", "dining", "freelance_work", "event_hosting"
 *
 * Combined with a role, creates a complete reputation key: "dining:guest"
 */
export type Aspect = string;

/**
 * Role-based reputation score (0-1000)
 * 500 = neutral (default for new roles)
 * 0 = lowest reputation
 * 1000 = highest reputation
 */
export type RoleScore = number;

/**
 * Aspect metadata (client-side managed)
 *
 * Defines an aspect (activity) within a domain, including which roles
 * are available for scoring. This metadata is stored client-side, while
 * the actual scores are stored on-chain.
 */
export interface AspectMetadata {
  aspect: Aspect;                 // "ride_sharing"
  name: string;                  // "Ride-sharing"
  description: string;           // "Shared transportation services"
  domain: AspectDomain;          // AspectDomain.Transportation
  roles: AspectRole[];           // ["driver", "passenger"]
  created_by: AccountId;         // Trust leadership that created this aspect
  created_at: Timestamp;
  is_active: boolean;
}

/**
 * Role score update request
 *
 * Represents a single reputation update action for a specific role
 * within an aspect. The delta can be positive or negative.
 */
export interface RoleScoreUpdate {
  subject: AccountId;            // Account being scored
  aspect: Aspect;                // "dining"
  role: AspectRole;              // "guest"
  delta: number;                 // Change to apply (positive or negative)
  reason?: string;               // Optional justification for the score
  scored_by: AccountId;          // Account submitting this score
  timestamp: Timestamp;
}

/**
 * Compound key type for role-based scoring
 * Format: "aspect:role" → score
 * Example: "dining:guest" → 850
 */
export type RoleScoreKey = `${Aspect}:${AspectRole}`;

/**
 * Work claim for time-based token issuance
 */
export interface WorkClaim {
  claim_id: number;
  worker: AccountId;
  work_type: WorkType;
  minutes_worked: number;
  evidence_hash: string;           // Hash of evidence (IPFS, etc.)
  gps_lat?: number;
  gps_lon?: number;
  submitted_at: Timestamp;
  verifiers_assigned: AccountId[];
  approvals_received: number;
  rejections_received: number;
  status: ClaimStatus;
  multiplier: number,              // Multiplier based on work type
}

/**
 * Grace period data
 */
export interface GracePeriod {
  account: AccountId;
  grace_type: GraceType;
  start_time: Timestamp;
  end_time: Timestamp;
  oracle_verified: boolean;
  extension_votes: number,
}

/**
 * Verifier data for work verification
 */
export interface VerifierData {
  trust_id: AccountId | null;
  stake: Amount;
  reputation_score: number;        // 0-1000 (general trust, independent of roles)
  verified_claims: number;
  rejected_claims: number;
  fraud_reports: number;
  /**
   * Optional role-based scores (aspect:role → score)
   * Stored as Record for JSON serialization, maps to Map<Bytes, u32> in contract
   *
   * Examples:
   *   "dining:guest" → 850
   *   "dining:host" → 400
   *   "ride_sharing:driver" → 920
   *   "ride_sharing:passenger" → 610
   */
  role_scores?: Record<RoleScoreKey, RoleScore>;
}

/**
 * Oracle for grace period verification
 */
export interface OracleData {
  oracle_address: AccountId;
  stake: Amount;
  reputation_score: number;        // 0-1000 (general trust, independent of roles)
  grace_periods_granted: number;
  /**
   * Optional role-based scores (aspect:role → score)
   */
  role_scores?: Record<RoleScoreKey, RoleScore>;
}

/**
 * Governance proposal
 */
export interface Proposal {
  proposal_id: number;
  proposer: AccountId;
  proposal_type: ProposalType;
  title: string;
  description: string;
  trust_id: AccountId | null;       // Null for protocol-level
  new_rate_bps?: number;           // For rate change proposals
  created_at: Timestamp;
  review_end: Timestamp;
  vote_end: Timestamp;
  implementation_date: Timestamp;
  status: ProposalStatus;
  votes_for: number;
  votes_against: number;
  voters: AccountId[];
}

// ============================================================================
// LEGACY TYPES (for backward compatibility)
// ============================================================================

/**
 * Legacy account data structure (minimal)
 */
export interface LegacyAccountData {
  last_activity: Timestamp;
  balance: Amount,
}

/**
 * Demurrage calculation result
 */
export interface DemurrageResult {
  original_balance: Amount;
  demurrage_amount: Amount;
  final_balance: Amount;
  inactive_periods: number,
}

/**
 * App registration for additional demurrage (legacy)
 */
export interface AppDemurrageEntry {
  app_id: AccountId;
  additional_rate: number,
}

// ============================================================================
// TRANSACTION TYPES
// ============================================================================

/**
 * Transaction result
 */
export interface TransactionResult {
  hash: string;
  status: "success" | "pending" | "failed";
  error?: string,
}

/**
 * Network configuration
 */
export interface NetworkConfig {
  networkUrl: string;
  rpcUrl: string;
  networkPassphrase: string;
  contractId: string,
}

/**
 * Supported networks
 */
export enum Network {
  Mainnet = "mainnet",
  Testnet = "testnet",
  Futurenet = "futurenet",
  Standalone = "standalone",
}
