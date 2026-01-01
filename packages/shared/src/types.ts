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
  trust_id: AccountId;             // Trust membership (zero address if none)
  contribution_hours: number;      // Total hours contributed
  grace_periods_used: number;      // Grace periods used this year
  last_grace_year: number;         // Year of last grace period,
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

/**
 * Verifier data for work verification
 */
export interface VerifierData {
  trust_id: AccountId;
  stake: Amount;
  reputation_score: number;        // 0-1000
  verified_claims: number;
  rejected_claims: number;
  fraud_reports: number,
}

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
 * Oracle for grace period verification
 */
export interface OracleData {
  oracle_address: AccountId;
  stake: Amount;
  reputation_score: number;
  grace_periods_granted: number,
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
  trust_id: AccountId;             // Zero address for protocol-level
  new_rate_bps?: number;           // For rate change proposals
  created_at: Timestamp;
  review_end: Timestamp;
  vote_end: Timestamp;
  implementation_date: Timestamp;
  status: ProposalStatus;
  votes_for: number;
  votes_against: number;
  voters: AccountId[],
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
