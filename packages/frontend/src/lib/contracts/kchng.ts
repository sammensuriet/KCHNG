/**
 * KCHNG Soroban contract client
 * Time-Standard Economic Model
 */

import pkg from "@stellar/stellar-sdk";
const { Contract, Address, xdr } = pkg;
const { Server: SorobanRpcServer, Api } = pkg.rpc;

import { getNetworkConfig } from "@kchng/shared";
import type { NetworkName } from "$lib/stores/wallet";
import type {
  AccountData,
  TrustData,
  VerifierData,
  WorkClaim,
  GracePeriod,
  OracleData,
  Proposal,
  WorkType,
  GraceType,
  ProposalType,
  ProposalStatus,
} from "@kchng/shared";

// Type for the contract's balance() return value
interface BalanceResult {
  bigint: bigint;
}

/**
 * KCHNG contract client - Complete time-standard economic model
 */
export class KchngClient {
  private contract: Contract;
  private contractId: string;
  private server: InstanceType<typeof SorobanRpcServer>;

  constructor(contractId: string, rpcUrl: string, networkPassphrase: string) {
    this.contractId = contractId;
    this.contract = new Contract(contractId);
    this.server = new SorobanRpcServer(rpcUrl, {
      allowHttp: networkPassphrase === "Test SDF Network ; September 2015",
    });
  }

  // ==========================================================================
  // CORE TOKEN FUNCTIONS
  // ==========================================================================

  /**
   * Get the KCHNG balance for an account
   */
  async getBalance(accountId: string): Promise<bigint> {
    try {
      const address = new Address(accountId);
      const result = await this.simulateContractCall(this.contractId, "balance", [
        address.toScVal(),
      ]);

      if (result) {
        return this.u256FromScVal(result);
      }
      return 0n;
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error);
      if (errorMsg.includes("not initialized") || errorMsg.includes("HostError")) {
        console.warn("[KchngClient] Contract not yet initialized");
        return 0n;
      }
      console.error("Error fetching balance:", error);
      throw new Error("Failed to fetch balance from contract");
    }
  }

  /**
   * Get account data including balance and last activity
   */
  async getAccountData(accountId: string): Promise<AccountData> {
    try {
      const address = new Address(accountId);
      const result = await this.simulateContractCall(this.contractId, "get_account", [
        address.toScVal(),
      ]);

      if (result) {
        return this.accountDataFromScVal(result);
      }

      // Return default account data if not found
      return {
        balance: 0n,
        last_activity: 0,
        grace_period_end: 0,
        trust_id: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        contribution_hours: 0,
        grace_periods_used: 0,
        last_grace_year: 0,
      };
    } catch (error) {
      console.error("Error fetching account data:", error);
      throw new Error("Failed to fetch account data from contract");
    }
  }

  /**
   * Get total supply
   */
  async getTotalSupply(): Promise<bigint> {
    try {
      const result = await this.simulateContractCall(this.contractId, "total_supply", []);
      if (result) {
        return this.u256FromScVal(result);
      }
      return 0n;
    } catch (error) {
      const errorMsg = error instanceof Error ? error.message : String(error);
      if (errorMsg.includes("not initialized") || errorMsg.includes("HostError")) {
        console.warn("[KchngClient] Contract not yet initialized");
        return 0n;
      }
      console.error("Error fetching total supply:", error);
      throw new Error("Failed to fetch total supply");
    }
  }

  // ==========================================================================
  // TRUST SYSTEM
  // ==========================================================================

  /**
   * Register a new trust (community organization)
   */
  async registerTrust(
    name: string,
    governor: string,
    annualRateBps: number,
    demurragePeriodDays: number
  ): Promise<void> {
    // This would create a transaction for the user to sign
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Join a trust
   */
  async joinTrust(trustId: string): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Get trust info
   */
  async getTrustInfo(trustId: string): Promise<TrustData> {
    try {
      const address = new Address(trustId);
      const result = await this.simulateContractCall(this.contractId, "get_trust_info", [
        address.toScVal(),
      ]);

      if (result) {
        return this.trustDataFromScVal(result);
      }
      throw new Error("Trust not found");
    } catch (error) {
      console.error("Error fetching trust info:", error);
      throw new Error("Failed to fetch trust info from contract");
    }
  }

  /**
   * Get all trusts
   */
  async getAllTrusts(): Promise<string[]> {
    try {
      const result = await this.simulateContractCall(this.contractId, "get_all_trusts", []);

      if (result) {
        return this.addressArrayFromScVal(result);
      }
      return [];
    } catch (error) {
      console.error("Error fetching all trusts:", error);
      throw new Error("Failed to fetch trusts from contract");
    }
  }

  // ==========================================================================
  // WORK VERIFICATION
  // ==========================================================================

  /**
   * Submit a work claim
   */
  async submitWorkClaim(
    workType: WorkType,
    minutesWorked: number,
    evidenceHash: string,
    gpsLat?: number,
    gpsLon?: number
  ): Promise<number> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Approve a work claim (verifier only)
   */
  async approveWorkClaim(claimId: number): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Reject a work claim (verifier only)
   */
  async rejectWorkClaim(claimId: number): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Register as a verifier
   */
  async registerVerifier(trustId: string): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Get work claim details
   */
  async getWorkClaim(claimId: number): Promise<WorkClaim> {
    try {
      const result = await this.simulateContractCall(this.contractId, "get_work_claim", [
        this.u64ToScVal(claimId),
      ]);

      if (result) {
        return this.workClaimFromScVal(result);
      }
      throw new Error("Work claim not found");
    } catch (error) {
      console.error("Error fetching work claim:", error);
      throw new Error("Failed to fetch work claim from contract");
    }
  }

  // ==========================================================================
  // GRACE PERIODS
  // ==========================================================================

  /**
   * Register as a grace period oracle
   */
  async registerOracle(): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Activate a grace period for an account
   */
  async activateGracePeriod(
    account: string,
    graceType: GraceType,
    durationDays: number
  ): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Check if an account is in a grace period
   */
  async isInGracePeriod(account: string): Promise<boolean> {
    try {
      const address = new Address(account);
      const result = await this.simulateContractCall(this.contractId, "is_in_grace_period", [
        address.toScVal(),
      ]);

      if (result) {
        return this.boolFromScVal(result);
      }
      return false;
    } catch (error) {
      console.error("Error checking grace period:", error);
      return false;
    }
  }

  /**
   * Get grace period details
   */
  async getGracePeriod(account: string): Promise<GracePeriod> {
    try {
      const address = new Address(account);
      const result = await this.simulateContractCall(this.contractId, "get_grace_period", [
        address.toScVal(),
      ]);

      if (result) {
        return this.gracePeriodFromScVal(result);
      }
      throw new Error("Grace period not found");
    } catch (error) {
      console.error("Error fetching grace period:", error);
      throw new Error("Failed to fetch grace period from contract");
    }
  }

  // ==========================================================================
  // CROSS-TRUST EXCHANGE
  // ==========================================================================

  /**
   * Calculate exchange rate between two trusts
   * Returns rate in basis points
   */
  async calculateExchangeRate(sourceTrust: string, destTrust: string): Promise<number> {
    try {
      const sourceAddress = new Address(sourceTrust);
      const destAddress = new Address(destTrust);

      const result = await this.simulateContractCall(this.contractId, "calculate_exchange_rate", [
        sourceAddress.toScVal(),
        destAddress.toScVal(),
      ]);

      if (result) {
        return this.u64FromScVal(result);
      }
      throw new Error("Failed to calculate exchange rate");
    } catch (error) {
      console.error("Error calculating exchange rate:", error);
      throw new Error("Failed to calculate exchange rate");
    }
  }

  /**
   * Execute a cross-trust swap
   */
  async crossTrustSwap(destTrust: string, amount: bigint): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Simulate a cross-trust swap to see the result
   */
  async simulateCrossTrustSwap(
    sourceTrust: string,
    destTrust: string,
    amount: bigint
  ): Promise<bigint> {
    try {
      const sourceAddress = new Address(sourceTrust);
      const destAddress = new Address(destTrust);

      const result = await this.simulateContractCall(
        this.contractId,
        "simulate_cross_trust_swap",
        [
          sourceAddress.toScVal(),
          destAddress.toScVal(),
          this.u256ToScVal(amount),
        ]
      );

      if (result) {
        return this.u256FromScVal(result);
      }
      return 0n;
    } catch (error) {
      console.error("Error simulating cross-trust swap:", error);
      throw new Error("Failed to simulate cross-trust swap");
    }
  }

  // ==========================================================================
  // GOVERNANCE
  // ==========================================================================

  /**
   * Create a governance proposal
   */
  async createProposal(
    proposalType: ProposalType,
    title: string,
    description: string,
    trustId: string,
    newRateBps?: number
  ): Promise<number> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Vote on a proposal
   */
  async voteOnProposal(proposalId: number, support: boolean): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Process a proposal (transition states)
   */
  async processProposal(proposalId: number): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Implement an approved proposal
   */
  async implementProposal(proposalId: number): Promise<void> {
    throw new Error("Transaction creation not yet implemented");
  }

  /**
   * Get proposal details
   */
  async getProposal(proposalId: number): Promise<Proposal> {
    try {
      const result = await this.simulateContractCall(this.contractId, "get_proposal", [
        this.u64ToScVal(proposalId),
      ]);

      if (result) {
        return this.proposalFromScVal(result);
      }
      throw new Error("Proposal not found");
    } catch (error) {
      console.error("Error fetching proposal:", error);
      throw new Error("Failed to fetch proposal from contract");
    }
  }

  /**
   * Get all proposal IDs
   */
  async getAllProposals(): Promise<number[]> {
    try {
      const result = await this.simulateContractCall(this.contractId, "get_all_proposals", []);

      if (result) {
        return this.u64ArrayFromScVal(result);
      }
      return [];
    } catch (error) {
      console.error("Error fetching all proposals:", error);
      throw new Error("Failed to fetch proposals from contract");
    }
  }

  // ==========================================================================
  // HELPER METHODS
  // ==========================================================================

  /**
   * Simulate a contract call (read-only)
   */
  private async simulateContractCall(
    contractId: string,
    method: string,
    args: xdr.ScVal[]
  ): Promise<xdr.ScVal | null> {
    try {
      const transaction = this.prepareInvocation(contractId, method, args);
      const simResult = await this.server.simulateTransaction(transaction);

      if (Api.isSimulationSuccess(simResult)) {
        if (simResult.result?.retval) {
          return simResult.result.retval;
        }
      }
      return null;
    } catch (error) {
      console.error("Contract call simulation failed:", error);
      throw error;
    }
  }

  /**
   * Prepare a contract invocation transaction
   */
  private prepareInvocation(
    contractId: string,
    method: string,
    args: xdr.ScVal[]
  ): xdr.Transaction {
    const contract = new Contract(contractId);
    return contract.call(method, ...args);
  }

  /**
   * Convert U256 ScVal to bigint
   */
  private u256FromScVal(val: xdr.ScVal): bigint {
    if (val.u256()) {
      const u256 = val.u256()!;
      const hi = BigInt(u256.high().toString());
      const lo = BigInt(u256.low().toString());
      return (hi << 128n) | lo;
    }
    if (val.u32()) {
      return BigInt(val.u32()!);
    }
    if (val.i256()) {
      const i256 = val.i256()!;
      const hi = BigInt(i256.high().toString());
      const lo = BigInt(i256.low().toString());
      return (hi << 128n) | lo;
    }
    if (val.u64()) {
      return BigInt(val.u64()!.toString());
    }
    throw new Error("Cannot convert ScVal to bigint");
  }

  /**
   * Convert bigint to U256 ScVal
   */
  private u256ToScVal(val: bigint): xdr.ScVal {
    return xdr.ScVal.u256(
      xdr.Uint256.parse(
        val.toString(16).padStart(64, "0")
      )
    );
  }

  /**
   * Convert number to u64 ScVal
   */
  private u64ToScVal(val: number): xdr.ScVal {
    return xdr.ScVal.u64(new xdr.Uint64(val.toString()));
  }

  /**
   * Convert u64 ScVal to number
   */
  private u64FromScVal(val: xdr.ScVal): number {
    if (val.u64()) {
      return Number(val.u64()!.toString());
    }
    if (val.u32()) {
      return val.u32()!;
    }
    throw new Error("Cannot convert ScVal to number");
  }

  /**
   * Convert boolean ScVal to boolean
   */
  private boolFromScVal(val: xdr.ScVal): boolean {
    if (val.bool()) {
      return val.bool()!;
    }
    throw new Error("Cannot convert ScVal to boolean");
  }

  /**
   * Convert AccountData ScVal to AccountData
   */
  private accountDataFromScVal(val: xdr.ScVal): AccountData {
    // This is a simplified version - real implementation would parse the struct properly
    return {
      balance: 0n,
      last_activity: 0,
      grace_period_end: 0,
      trust_id: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      contribution_hours: 0,
      grace_periods_used: 0,
      last_grace_year: 0,
    };
  }

  /**
   * Convert TrustData ScVal to TrustData
   */
  private trustDataFromScVal(val: xdr.ScVal): TrustData {
    return {
      name: "",
      governor: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      annual_rate_bps: 1200,
      demurrage_period_days: 30,
      member_count: 0,
      is_active: false,
      created_at: 0,
    };
  }

  /**
   * Convert WorkClaim ScVal to WorkClaim
   */
  private workClaimFromScVal(val: xdr.ScVal): WorkClaim {
    return {
      claim_id: 0,
      worker: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      work_type: WorkType.Basic,
      minutes_worked: 0,
      evidence_hash: "",
      submitted_at: 0,
      verifiers_assigned: [],
      approvals_received: 0,
      rejections_received: 0,
      status: 0 as ClaimStatus.Pending,
      multiplier: 1000,
    };
  }

  /**
   * Convert GracePeriod ScVal to GracePeriod
   */
  private gracePeriodFromScVal(val: xdr.ScVal): GracePeriod {
    return {
      account: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      grace_type: 0 as GraceType.Emergency,
      start_time: 0,
      end_time: 0,
      oracle_verified: false,
      extension_votes: 0,
    };
  }

  /**
   * Convert Proposal ScVal to Proposal
   */
  private proposalFromScVal(val: xdr.ScVal): Proposal {
    return {
      proposal_id: 0,
      proposer: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      proposal_type: 0 as ProposalType.RateChange,
      title: "",
      description: "",
      trust_id: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      created_at: 0,
      review_end: 0,
      vote_end: 0,
      implementation_date: 0,
      status: 0 as ProposalStatus.Review,
      votes_for: 0,
      votes_against: 0,
      voters: [],
    };
  }

  /**
   * Convert address array ScVal to string array
   */
  private addressArrayFromScVal(val: xdr.ScVal): string[] {
    return [];
  }

  /**
   * Convert u64 array ScVal to number array
   */
  private u64ArrayFromScVal(val: xdr.ScVal): number[] {
    return [];
  }
}

/**
 * Create a KCHNG client with the specified network's contract ID
 */
export function createKchngClient(network: NetworkName = "testnet"): KchngClient {
  const config = getNetworkConfig(network);
  return new KchngClient(config.contractId, config.rpcUrl, config.networkPassphrase);
}
