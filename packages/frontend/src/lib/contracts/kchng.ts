/**
 * KCHNG Soroban contract client
 */

import { Contract, Address, SorobanRpc, xdr } from "@stellar/stellar-sdk";
import { networkConfig } from "$lib/config/networks";
import type { AccountData } from "./types";

// Type for the contract's balance() return value
interface BalanceResult {
  bigint: bigint;
}

/**
 * KCHNG contract client
 */
export class KchngClient {
  private contract: Contract;
  private contractId: string;
  private server: SorobanRpc.Server;

  constructor(contractId: string, rpcUrl: string) {
    this.contractId = contractId;
    this.contract = new Contract(contractId);
    this.server = new SorobanRpc.Server(rpcUrl, {
      allowHttp: networkConfig.networkPassphrase === "Test SDF Network ; September 2015",
    });
  }

  /**
   * Get the KCHNG balance for an account
   * @param accountId - Stellar address to query
   * @returns Balance in bigint
   */
  async getBalance(accountId: string): Promise<bigint> {
    try {
      const address = new Address(accountId);
      const result = await this.simulateContractCall(this.contractId, "balance", [
        address.toScVal(),
      ]);

      // Parse U256 from result
      if (result) {
        return this.u256FromScVal(result);
      }
      return 0n;
    } catch (error) {
      // If contract is not initialized, return 0 balance
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
   * @param accountId - Stellar address to query
   * @returns Account data
   */
  async getAccountData(accountId: string): Promise<AccountData> {
    try {
      const balance = await this.getBalance(accountId);

      // Note: The contract doesn't expose last_activity directly
      // We'd need to either:
      // 1. Add a get_account_data() method to the contract
      // 2. Infer it from demurrage calculations
      // For now, return current time as last activity
      return {
        last_activity: BigInt(Math.floor(Date.now() / 1000)),
        balance,
      };
    } catch (error) {
      console.error("Error fetching account data:", error);
      throw new Error("Failed to fetch account data from contract");
    }
  }

  /**
   * Create a transfer transaction
   * @param from - Sender address
   * @param to - Recipient address
   * @param amount - Amount to transfer
   * @returns Transaction XDR
   */
  async createTransfer(
    from: string,
    to: string,
    amount: bigint
  ): Promise<string> {
    try {
      const fromAddress = new Address(from);
      const toAddress = new Address(to);

      // Build the transaction
      // This would normally involve:
      // 1. Getting account info for source account
      // 2. Building a Soroban transaction
      // 3. Adding the contract invocation
      // 4. Returning the XDR for signing

      throw new Error("Transfer implementation requires full transaction setup");
    } catch (error) {
      console.error("Error creating transfer:", error);
      throw new Error("Failed to create transfer transaction");
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
      // If contract is not initialized, return 0
      const errorMsg = error instanceof Error ? error.message : String(error);
      if (errorMsg.includes("not initialized") || errorMsg.includes("HostError")) {
        console.warn("[KchngClient] Contract not yet initialized");
        return 0n;
      }
      console.error("Error fetching total supply:", error);
      throw new Error("Failed to fetch total supply");
    }
  }

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

      if (SorobanRpc.Api.isSimulationSuccess(simResult)) {
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
    // U256 in Soroban is represented as a 256-bit integer
    // The SDK may represent it differently depending on version
    if (val.u256()) {
      const u256 = val.u256()!;
      // Combine the high and low parts
      const hi = BigInt(u256.high().toString());
      const lo = BigInt(u256.low().toString());
      return (hi << 128n) | lo;
    }
    // Fallback for u32 values
    if (val.u32()) {
      return BigInt(val.u32()!);
    }
    // Fallback for i256 values (shouldn't happen for balance)
    if (val.i256()) {
      const i256 = val.i256()!;
      const hi = BigInt(i256.high().toString());
      const lo = BigInt(i256.low().toString());
      return (hi << 128n) | lo;
    }
    throw new Error("Cannot convert ScVal to bigint");
  }
}

/**
 * Create a KCHNG client with the current network's contract ID
 */
export function createKchngClient(): KchngClient {
  return new KchngClient(networkConfig.contractId, networkConfig.rpcUrl);
}
