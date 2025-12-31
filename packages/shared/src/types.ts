/**
 * Shared types for KCHNG project
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

/**
 * Account data from the smart contract
 */
export interface AccountData {
  last_activity: Timestamp;
  balance: Amount;
}

/**
 * Demurrage calculation result
 */
export interface DemurrageResult {
  original_balance: Amount;
  demurrage_amount: Amount;
  final_balance: Amount;
  inactive_periods: number;
}

/**
 * App registration for additional demurrage
 */
export interface AppDemurrageEntry {
  app_id: AccountId;
  additional_rate: number;
}

/**
 * Transaction result
 */
export interface TransactionResult {
  hash: string;
  status: "success" | "pending" | "failed";
  error?: string;
}

/**
 * Network configuration
 */
export interface NetworkConfig {
  networkUrl: string;
  networkPassphrase: string;
  contractId: string;
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
