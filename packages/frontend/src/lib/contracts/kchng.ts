/**
 * KCHNG Soroban contract client
 * Time-Standard Economic Model
 */

import {
  Contract,
  Address,
  xdr,
  rpc,
  TransactionBuilder,
  Networks,
  BASE_FEE,
  Account,
} from "@stellar/stellar-sdk";
const { Server: SorobanRpcServer, Api } = rpc;

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
} from "@kchng/shared";
import {
  WorkType,
  GraceType,
  ProposalType,
  ProposalStatus,
  ClaimStatus,
} from "@kchng/shared";

// Type for the contract's balance() return value
interface BalanceResult {
  bigint: bigint;
}

// Callback type for signing transactions
type SignTransactionCallback = (xdr: string) => Promise<string>;

/**
 * KCHNG contract client - Complete time-standard economic model
 */
export class KchngClient {
  private contract: Contract;
  private contractId: string;
  private server: InstanceType<typeof SorobanRpcServer>;
  private networkPassphrase: string;
  private signTransactionCallback: SignTransactionCallback | null = null;

  constructor(contractId: string, rpcUrl: string, networkPassphrase: string) {
    this.contractId = contractId;
    this.contract = new Contract(contractId);
    this.server = new SorobanRpcServer(rpcUrl, {
      allowHttp: networkPassphrase === "Test SDF Network ; September 2015",
    });
    this.networkPassphrase = networkPassphrase;
  }

  /**
   * Set the callback for signing transactions
   * This must be set before calling any write methods
   */
  setSignTransactionCallback(callback: SignTransactionCallback) {
    this.signTransactionCallback = callback;
  }

  /**
   * Submit a contract call (write operation)
   * Builds, simulates, signs, and submits a transaction
   */
  private async submitContractCall(
    method: string,
    args: xdr.ScVal[],
    sourceAddress: string
  ): Promise<string> {
    if (!this.signTransactionCallback) {
      throw new Error("Transaction signing callback not set. Call setSignTransactionCallback first.");
    }

    try {
      // 1. Get the source account from the network
      const account = await this.server.getAccount(sourceAddress);

      // 2. Build the transaction with the contract call
      const transaction = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: this.networkPassphrase,
      })
        .addOperation(this.contract.call(method, ...args))
        .setTimeout(30)
        .build();

      // 3. Simulate the transaction to get the storage footprint
      const simulated = await this.server.simulateTransaction(transaction);

      if (Api.isSimulationError(simulated)) {
        throw new Error(`Transaction simulation failed: ${simulated.error}`);
      }

      // 4. Sign the transaction via the callback
      const signedXdr = await this.signTransactionCallback(
        transaction.toXDR()
      );

      // 5. Submit the signed transaction
      const signedTx = TransactionBuilder.fromXDR(signedXdr, this.networkPassphrase);
      const result = await this.server.sendTransaction(signedTx);

      if (result.status === "ERROR") {
        throw new Error(`Transaction submission failed: ${result.errorResult?.result()}`);
      }

      return result.hash;
    } catch (error) {
      console.error(`[KchngClient] Contract call ${method} failed:`, error);
      throw error;
    }
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
    sourceAddress: string,
    name: string,
    annualRateBps: number,
    demurragePeriodDays: number
  ): Promise<string> {
    const args = [
      xdr.ScVal.scvString(name),
      xdr.ScVal.scvU32(annualRateBps),
      xdr.ScVal.scvU64(new xdr.Uint64(demurragePeriodDays.toString())),
    ];

    return this.submitContractCall("register_trust", args, sourceAddress);
  }

  /**
   * Join a trust
   */
  async joinTrust(trustId: string, sourceAddress: string): Promise<string> {
    const trustAddress = new Address(trustId);
    const args = [trustAddress.toScVal()];

    return this.submitContractCall("join_trust", args, sourceAddress);
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
    sourceAddress: string,
    workType: WorkType,
    minutesWorked: number,
    evidenceHash: string,
    gpsLat?: number,
    gpsLon?: number
  ): Promise<string> {
    const args = [
      xdr.ScVal.scvU32(workType),
      xdr.ScVal.scvU64(new xdr.Uint64(minutesWorked.toString())),
      xdr.ScVal.scvBytes(Buffer.from(evidenceHash, "hex")),
      gpsLat !== undefined
        ? xdr.ScVal.scvI64(new xdr.Int64(gpsLat.toString()))
        : xdr.ScVal.scvVoid(),
      gpsLon !== undefined
        ? xdr.ScVal.scvI64(new xdr.Int64(gpsLon.toString()))
        : xdr.ScVal.scvVoid(),
    ];

    return this.submitContractCall("submit_work_claim", args, sourceAddress);
  }

  /**
   * Approve a work claim (verifier only)
   */
  async approveWorkClaim(verifier: string, claimId: number): Promise<string> {
    const args = [xdr.ScVal.scvU64(new xdr.Uint64(claimId.toString()))];
    return this.submitContractCall("approve_work_claim", args, verifier);
  }

  /**
   * Reject a work claim (verifier only)
   */
  async rejectWorkClaim(verifier: string, claimId: number): Promise<string> {
    const args = [xdr.ScVal.scvU64(new xdr.Uint64(claimId.toString()))];
    return this.submitContractCall("reject_work_claim", args, verifier);
  }

  /**
   * Register as a verifier
   */
  async registerVerifier(sourceAddress: string, trustId: string): Promise<string> {
    const trustAddress = new Address(trustId);
    const args = [trustAddress.toScVal()];
    return this.submitContractCall("register_verifier", args, sourceAddress);
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
  async registerOracle(sourceAddress: string): Promise<string> {
    const args: xdr.ScVal[] = [];
    return this.submitContractCall("register_oracle", args, sourceAddress);
  }

  /**
   * Activate a grace period for an account
   */
  async activateGracePeriod(
    sourceAddress: string,
    account: string,
    graceType: GraceType,
    durationDays: number
  ): Promise<string> {
    const accountAddress = new Address(account);
    const args = [
      accountAddress.toScVal(),
      xdr.ScVal.scvU32(graceType),
      xdr.ScVal.scvU64(new xdr.Uint64(durationDays.toString())),
    ];
    return this.submitContractCall("activate_grace_period", args, sourceAddress);
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
  async crossTrustSwap(sourceAddress: string, destTrust: string, amount: bigint): Promise<string> {
    const destAddress = new Address(destTrust);
    const args = [destAddress.toScVal(), this.u256ToScVal(amount)];
    return this.submitContractCall("cross_trust_swap", args, sourceAddress);
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
    sourceAddress: string,
    proposalType: ProposalType,
    title: string,
    description: string,
    trustId: string | null,
    newRateBps?: number
  ): Promise<string> {
    const args = [
      xdr.ScVal.scvU32(proposalType),
      xdr.ScVal.scvString(title),
      xdr.ScVal.scvString(description),
      trustId ? new Address(trustId).toScVal() : xdr.ScVal.scvVoid(),
      newRateBps !== undefined
        ? xdr.ScVal.scvU32(newRateBps)
        : xdr.ScVal.scvVoid(),
    ];
    return this.submitContractCall("create_proposal", args, sourceAddress);
  }

  /**
   * Vote on a proposal
   */
  async voteOnProposal(sourceAddress: string, proposalId: number, support: boolean): Promise<string> {
    const args = [
      xdr.ScVal.scvU64(new xdr.Uint64(proposalId.toString())),
      xdr.ScVal.scvBool(support),
    ];
    return this.submitContractCall("vote_on_proposal", args, sourceAddress);
  }

  /**
   * Process a proposal (transition states)
   */
  async processProposal(sourceAddress: string, proposalId: number): Promise<string> {
    const args = [xdr.ScVal.scvU64(new xdr.Uint64(proposalId.toString()))];
    return this.submitContractCall("process_proposal", args, sourceAddress);
  }

  /**
   * Implement an approved proposal
   */
  async implementProposal(sourceAddress: string, proposalId: number): Promise<string> {
    const args = [xdr.ScVal.scvU64(new xdr.Uint64(proposalId.toString()))];
    return this.submitContractCall("implement_proposal", args, sourceAddress);
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
   * Uses a dummy account for simulation since we don't need a real signer for read-only calls
   */
  private async simulateContractCall(
    contractId: string,
    method: string,
    args: xdr.ScVal[]
  ): Promise<xdr.ScVal | null> {
    try {
      const transaction = await this.prepareInvocation(contractId, method, args);
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
   * Builds a proper Transaction object for simulation
   */
  private async prepareInvocation(
    contractId: string,
    method: string,
    args: xdr.ScVal[]
  ) {
    const contract = new Contract(contractId);

    // Create a dummy source account for simulation using the Account class
    // The account ID doesn't need to exist for read-only simulations
    const dummyPublicKey = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
    const account = new Account(dummyPublicKey, "0");

    // Build the transaction with the contract call operation
    const transaction = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: this.networkPassphrase,
    })
      .addOperation(contract.call(method, ...args))
      .setTimeout(30)
      .build();

    return transaction;
  }

  /**
   * Convert U256 ScVal to bigint
   */
  private u256FromScVal(val: xdr.ScVal): bigint {
    const u256Parts = val.u256();
    if (u256Parts) {
      // UInt256Parts has hiHi, hiLo, loHi, loLo (each 64-bit)
      const hiHi = BigInt(u256Parts.hiHi().toString());
      const hiLo = BigInt(u256Parts.hiLo().toString());
      const loHi = BigInt(u256Parts.loHi().toString());
      const loLo = BigInt(u256Parts.loLo().toString());
      return (hiHi << 192n) | (hiLo << 128n) | (loHi << 64n) | loLo;
    }
    const u32Val = val.u32();
    if (u32Val !== undefined) {
      return BigInt(u32Val);
    }
    const i256Parts = val.i256();
    if (i256Parts) {
      const hiHi = BigInt(i256Parts.hiHi().toString());
      const hiLo = BigInt(i256Parts.hiLo().toString());
      const loHi = BigInt(i256Parts.loHi().toString());
      const loLo = BigInt(i256Parts.loLo().toString());
      let result = (hiHi << 192n) | (hiLo << 128n) | (loHi << 64n) | loLo;
      // Handle sign bit for i256
      if (result >= 2n ** 255n) {
        result -= 2n ** 256n;
      }
      return result;
    }
    const u64Val = val.u64();
    if (u64Val) {
      return BigInt(u64Val.toString());
    }
    throw new Error("Cannot convert ScVal to bigint");
  }

  /**
   * Convert bigint to U256 ScVal
   */
  private u256ToScVal(val: bigint): xdr.ScVal {
    const mask = 0xFFFFFFFFFFFFFFFFn;
    const parts = new xdr.UInt256Parts({
      hiHi: new xdr.Uint64(((val >> 192n) & mask).toString()),
      hiLo: new xdr.Uint64(((val >> 128n) & mask).toString()),
      loHi: new xdr.Uint64(((val >> 64n) & mask).toString()),
      loLo: new xdr.Uint64((val & mask).toString()),
    });
    return xdr.ScVal.scvU256(parts);
  }

  /**
   * Convert number to u64 ScVal
   */
  private u64ToScVal(val: number): xdr.ScVal {
    return xdr.ScVal.scvU64(new xdr.Uint64(val.toString()));
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
   * Convert u32 ScVal to number
   */
  private u32FromScVal(val: xdr.ScVal): number {
    if (val.u32()) {
      return val.u32()!;
    }
    if (val.u64()) {
      return Number(val.u64()!.toString());
    }
    throw new Error("Cannot convert ScVal to u32");
  }

  /**
   * Convert String ScVal to string
   */
  private stringFromScVal(val: xdr.ScVal): string {
    if (val.str()) {
      return val.str()!.toString();
    }
    throw new Error("Cannot convert ScVal to string");
  }

  /**
   * Convert Address ScVal to string
   */
  private addressFromScVal(val: xdr.ScVal): string {
    try {
      const address = Address.fromScVal(val);
      return address.toString();
    } catch {
      throw new Error("Cannot convert ScVal to address");
    }
  }

  /**
   * Convert Option<Address> ScVal to string | null
   * In Soroban, options are either the value or scvVoid for None
   */
  private optionAddressFromScVal(val: xdr.ScVal): string | null {
    // Check if it's void (None)
    if (val.switch().name === "scvVoid") {
      return null;
    }
    // Otherwise it's Some(address)
    try {
      const address = Address.fromScVal(val);
      return address.toString();
    } catch {
      return null;
    }
  }

  /**
   * Convert Vec<Address> ScVal to string array
   */
  private addressVecFromScVal(val: xdr.ScVal): string[] {
    const vec = val.vec();
    if (!vec) {
      return [];
    }
    return vec.map((item) => this.addressFromScVal(item));
  }

  /**
   * Convert Bytes ScVal to string (for evidence hash)
   */
  private bytesFromScVal(val: xdr.ScVal): string {
    const bytes = val.bytes();
    if (bytes) {
      return bytes.toString();
    }
    throw new Error("Cannot convert ScVal to bytes");
  }

  /**
   * Convert Option<i64> ScVal to number | undefined
   * In Soroban, options are either the value or scvVoid for None
   */
  private optionI64FromScVal(val: xdr.ScVal): number | undefined {
    // Check if it's void (None)
    if (val.switch().name === "scvVoid") {
      return undefined;
    }
    // Otherwise it's Some(i64)
    const i64Val = val.i64();
    if (i64Val !== undefined) {
      return Number(i64Val.toString());
    }
    return undefined;
  }

  /**
   * Get a struct field by index from an ScVal struct
   * Soroban structs are serialized as maps with numeric keys
   */
  private getStructField(val: xdr.ScVal, index: number): xdr.ScVal | null {
    const map = val.map();
    if (!map) {
      console.error("[KchngClient] Expected map for struct, got:", val.switch().name);
      return null;
    }

    // Find the field with the matching index key
    // Use bracket notation and unknown cast to avoid TypeScript conflict with .key() method
    for (const entry of map) {
      const key = (entry as unknown as { key: xdr.ScVal }).key;
      if (key && key.u32() === index) {
        return (entry as unknown as { val: xdr.ScVal }).val;
      }
    }

    console.warn(`[KchngClient] Struct field ${index} not found`);
    return null;
  }

  /**
   * Convert boolean ScVal to boolean
   */
  private boolFromScVal(val: xdr.ScVal): boolean {
    const boolVal = val.b();
    if (boolVal !== undefined) {
      return boolVal;
    }
    throw new Error("Cannot convert ScVal to boolean");
  }

  /**
   * Convert AccountData ScVal to AccountData
   * Fields: balance (U256), last_activity (u64), grace_period_end (u64),
   *         trust_id (Option<Address>), contribution_hours (u64),
   *         grace_periods_used (u32), last_grace_year (u32)
   */
  private accountDataFromScVal(val: xdr.ScVal): AccountData {
    try {
      return {
        balance: this.getStructField(val, 0) ? this.u256FromScVal(this.getStructField(val, 0)!) : 0n,
        last_activity: this.getStructField(val, 1) ? this.u64FromScVal(this.getStructField(val, 1)!) : 0,
        grace_period_end: this.getStructField(val, 2) ? this.u64FromScVal(this.getStructField(val, 2)!) : 0,
        trust_id: this.getStructField(val, 3) ? this.optionAddressFromScVal(this.getStructField(val, 3)!) : null,
        contribution_hours: this.getStructField(val, 4) ? this.u64FromScVal(this.getStructField(val, 4)!) : 0,
        grace_periods_used: this.getStructField(val, 5) ? this.u32FromScVal(this.getStructField(val, 5)!) : 0,
        last_grace_year: this.getStructField(val, 6) ? this.u32FromScVal(this.getStructField(val, 6)!) : 0,
      };
    } catch (error) {
      console.error("[KchngClient] Failed to parse AccountData:", error);
      return {
        balance: 0n,
        last_activity: 0,
        grace_period_end: 0,
        trust_id: null,
        contribution_hours: 0,
        grace_periods_used: 0,
        last_grace_year: 0,
      };
    }
  }

  /**
   * Convert TrustData ScVal to TrustData
   * Fields: name (String), governor (Address), annual_rate_bps (u32),
   *         demurrage_period_days (u64), member_count (u32), is_active (bool), created_at (u64)
   */
  private trustDataFromScVal(val: xdr.ScVal): TrustData {
    try {
      return {
        name: this.getStructField(val, 0) ? this.stringFromScVal(this.getStructField(val, 0)!) : "",
        governor: this.getStructField(val, 1) ? this.addressFromScVal(this.getStructField(val, 1)!) : "",
        annual_rate_bps: this.getStructField(val, 2) ? this.u32FromScVal(this.getStructField(val, 2)!) : 1200,
        demurrage_period_days: this.getStructField(val, 3) ? this.u64FromScVal(this.getStructField(val, 3)!) : 28,
        member_count: this.getStructField(val, 4) ? this.u32FromScVal(this.getStructField(val, 4)!) : 0,
        is_active: this.getStructField(val, 5) ? this.boolFromScVal(this.getStructField(val, 5)!) : false,
        created_at: this.getStructField(val, 6) ? this.u64FromScVal(this.getStructField(val, 6)!) : 0,
      };
    } catch (error) {
      console.error("[KchngClient] Failed to parse TrustData:", error);
      return {
        name: "",
        governor: "",
        annual_rate_bps: 1200,
        demurrage_period_days: 28,
        member_count: 0,
        is_active: false,
        created_at: 0,
      };
    }
  }

  /**
   * Convert WorkClaim ScVal to WorkClaim
   * Fields: claim_id (u64), worker (Address), work_type (WorkType enum),
   *         minutes_worked (u64), evidence_hash (Bytes), gps_lat (Option<i64>),
   *         gps_lon (Option<i64>), submitted_at (u64), verifiers_assigned (Vec<Address>),
   *         approvals_received (u32), rejections_received (u32), status (ClaimStatus enum),
   *         multiplier (u32)
   */
  private workClaimFromScVal(val: xdr.ScVal): WorkClaim {
    try {
      const workTypeField = this.getStructField(val, 2);
      let workType: WorkType = WorkType.Basic;
      if (workTypeField) {
        const enumVal = workTypeField.u32();
        if (enumVal !== undefined) {
          workType = enumVal as WorkType;
        }
      }

      const statusField = this.getStructField(val, 11);
      let status: ClaimStatus = ClaimStatus.Pending;
      if (statusField) {
        const enumVal = statusField.u32();
        if (enumVal !== undefined) {
          status = enumVal as ClaimStatus;
        }
      }

      return {
        claim_id: this.getStructField(val, 0) ? this.u64FromScVal(this.getStructField(val, 0)!) : 0,
        worker: this.getStructField(val, 1) ? this.addressFromScVal(this.getStructField(val, 1)!) : "",
        work_type: workType,
        minutes_worked: this.getStructField(val, 3) ? this.u64FromScVal(this.getStructField(val, 3)!) : 0,
        evidence_hash: this.getStructField(val, 4) ? this.bytesFromScVal(this.getStructField(val, 4)!) : "",
        gps_lat: this.getStructField(val, 5) ? this.optionI64FromScVal(this.getStructField(val, 5)!) : undefined,
        gps_lon: this.getStructField(val, 6) ? this.optionI64FromScVal(this.getStructField(val, 6)!) : undefined,
        submitted_at: this.getStructField(val, 7) ? this.u64FromScVal(this.getStructField(val, 7)!) : 0,
        verifiers_assigned: this.getStructField(val, 8) ? this.addressVecFromScVal(this.getStructField(val, 8)!) : [],
        approvals_received: this.getStructField(val, 9) ? this.u32FromScVal(this.getStructField(val, 9)!) : 0,
        rejections_received: this.getStructField(val, 10) ? this.u32FromScVal(this.getStructField(val, 10)!) : 0,
        status: status,
        multiplier: this.getStructField(val, 12) ? this.u32FromScVal(this.getStructField(val, 12)!) : 100,
      };
    } catch (error) {
      console.error("[KchngClient] Failed to parse WorkClaim:", error);
      return {
        claim_id: 0,
        worker: "",
        work_type: WorkType.Basic,
        minutes_worked: 0,
        evidence_hash: "",
        submitted_at: 0,
        verifiers_assigned: [],
        approvals_received: 0,
        rejections_received: 0,
        status: 0 as ClaimStatus.Pending,
        multiplier: 100,
      };
    }
  }

  /**
   * Convert GracePeriod ScVal to GracePeriod
   * Fields: account (Address), grace_type (GraceType enum), start_time (u64),
   *         end_time (u64), oracle_verified (bool), extension_votes (u32)
   */
  private gracePeriodFromScVal(val: xdr.ScVal): GracePeriod {
    try {
      const graceTypeField = this.getStructField(val, 1);
      let graceType: GraceType = GraceType.Emergency;
      if (graceTypeField) {
        const enumVal = graceTypeField.u32();
        if (enumVal !== undefined) {
          graceType = enumVal as GraceType;
        }
      }

      return {
        account: this.getStructField(val, 0) ? this.addressFromScVal(this.getStructField(val, 0)!) : "",
        grace_type: graceType,
        start_time: this.getStructField(val, 2) ? this.u64FromScVal(this.getStructField(val, 2)!) : 0,
        end_time: this.getStructField(val, 3) ? this.u64FromScVal(this.getStructField(val, 3)!) : 0,
        oracle_verified: this.getStructField(val, 4) ? this.boolFromScVal(this.getStructField(val, 4)!) : false,
        extension_votes: this.getStructField(val, 5) ? this.u32FromScVal(this.getStructField(val, 5)!) : 0,
      };
    } catch (error) {
      console.error("[KchngClient] Failed to parse GracePeriod:", error);
      return {
        account: "",
        grace_type: 0 as GraceType.Emergency,
        start_time: 0,
        end_time: 0,
        oracle_verified: false,
        extension_votes: 0,
      };
    }
  }

  /**
   * Convert Proposal ScVal to Proposal
   * Fields: proposal_id (u64), proposer (Address), proposal_type (ProposalType enum),
   *         title (String), description (String), trust_id (Option<Address>),
   *         new_rate_bps (Option<u32>), target_address (Option<Address>), stake (u64),
   *         created_at (u64), review_end (u64), vote_end (u64),
   *         implementation_date (u64), status (ProposalStatus enum), votes_for (u32),
   *         votes_against (u32), voters (Vec<Address>)
   */
  private proposalFromScVal(val: xdr.ScVal): Proposal {
    try {
      const proposalTypeField = this.getStructField(val, 2);
      let proposalType: ProposalType = ProposalType.RateChange;
      if (proposalTypeField) {
        const enumVal = proposalTypeField.u32();
        if (enumVal !== undefined) {
          proposalType = enumVal as ProposalType;
        }
      }

      const statusField = this.getStructField(val, 14);
      let status: ProposalStatus = ProposalStatus.Review;
      if (statusField) {
        const enumVal = statusField.u32();
        if (enumVal !== undefined) {
          status = enumVal as ProposalStatus;
        }
      }

      // new_rate_bps (field 6) - Option<u32>
      const newRateField = this.getStructField(val, 6);
      let newRateBps: number | undefined = undefined;
      if (newRateField && newRateField.switch().name !== "scvVoid") {
        const u32Val = newRateField.u32();
        if (u32Val !== undefined) {
          newRateBps = u32Val;
        }
      }

      return {
        proposal_id: this.getStructField(val, 0) ? this.u64FromScVal(this.getStructField(val, 0)!) : 0,
        proposer: this.getStructField(val, 1) ? this.addressFromScVal(this.getStructField(val, 1)!) : "",
        proposal_type: proposalType,
        title: this.getStructField(val, 3) ? this.stringFromScVal(this.getStructField(val, 3)!) : "",
        description: this.getStructField(val, 4) ? this.stringFromScVal(this.getStructField(val, 4)!) : "",
        trust_id: this.getStructField(val, 5) ? this.optionAddressFromScVal(this.getStructField(val, 5)!) : null,
        new_rate_bps: newRateBps,
        created_at: this.getStructField(val, 9) ? this.u64FromScVal(this.getStructField(val, 9)!) : 0,
        review_end: this.getStructField(val, 10) ? this.u64FromScVal(this.getStructField(val, 10)!) : 0,
        vote_end: this.getStructField(val, 11) ? this.u64FromScVal(this.getStructField(val, 11)!) : 0,
        implementation_date: this.getStructField(val, 12) ? this.u64FromScVal(this.getStructField(val, 12)!) : 0,
        status: status,
        votes_for: this.getStructField(val, 15) ? this.u32FromScVal(this.getStructField(val, 15)!) : 0,
        votes_against: this.getStructField(val, 16) ? this.u32FromScVal(this.getStructField(val, 16)!) : 0,
        voters: this.getStructField(val, 17) ? this.addressVecFromScVal(this.getStructField(val, 17)!) : [],
      };
    } catch (error) {
      console.error("[KchngClient] Failed to parse Proposal:", error);
      return {
        proposal_id: 0,
        proposer: "",
        proposal_type: 0 as ProposalType.RateChange,
        title: "",
        description: "",
        trust_id: null,
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
  }

  /**
   * Convert address array ScVal to string array
   */
  private addressArrayFromScVal(val: xdr.ScVal): string[] {
    const vec = val.vec();
    if (!vec) {
      return [];
    }
    return vec.map((item) => this.addressFromScVal(item));
  }

  /**
   * Convert u64 array ScVal to number array
   */
  private u64ArrayFromScVal(val: xdr.ScVal): number[] {
    const vec = val.vec();
    if (!vec) {
      return [];
    }
    return vec.map((item) => this.u64FromScVal(item));
  }
}

/**
 * Create a KCHNG client with the specified network's contract ID
 */
export function createKchngClient(network: NetworkName = "testnet"): KchngClient {
  const config = getNetworkConfig(network);
  return new KchngClient(config.contractId, config.rpcUrl, config.networkPassphrase);
}
