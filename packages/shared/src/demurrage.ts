/**
 * Demurrage calculation utilities for KCHNG
 */

import type { AccountData, DemurrageResult } from "./types.js";

// Constants matching the smart contract
export const SECONDS_PER_DAY = 86_400;
export const DEMURRAGE_PERIOD_DAYS = 7;
export const DEMURRAGE_AMOUNT = 2n; // 2 KCHNG burned per 7 days of inactivity

/**
 * Calculate the number of complete demurrage periods that have passed
 * @param lastActivity - Unix timestamp of last activity
 * @param currentTimestamp - Current Unix timestamp
 * @returns Number of complete 7-day periods
 */
export function calculateInactivePeriods(
  lastActivity: number,
  currentTimestamp: number,
): number {
  if (currentTimestamp <= lastActivity) {
    return 0;
  }

  const inactiveSeconds = currentTimestamp - lastActivity;
  const inactiveDays = Math.floor(inactiveSeconds / SECONDS_PER_DAY);

  if (inactiveDays < DEMURRAGE_PERIOD_DAYS) {
    return 0;
  }

  return Math.floor(inactiveDays / DEMURRAGE_PERIOD_DAYS);
}

/**
 * Calculate the demurrage amount based on inactive periods
 * @param periods - Number of complete 7-day inactive periods
 * @returns Amount to burn
 */
export function calculateDemurrageAmount(periods: number): bigint {
  return BigInt(periods) * DEMURRAGE_AMOUNT;
}

/**
 * Calculate balance after applying demurrage
 * @param accountData - The account data from smart contract
 * @param currentTimestamp - Current Unix timestamp
 * @returns The demurrage calculation result
 */
export function calculateBalanceWithDemurrage(
  accountData: AccountData,
  currentTimestamp: number,
): DemurrageResult {
  const periods = calculateInactivePeriods(
    accountData.last_activity,
    currentTimestamp,
  );
  const demurrageAmount = calculateDemurrageAmount(periods);

  const finalBalance =
    accountData.balance > demurrageAmount
      ? accountData.balance - demurrageAmount
      : 0n;

  return {
    original_balance: accountData.balance,
    demurrage_amount: demurrageAmount,
    final_balance: finalBalance,
    inactive_periods: periods,
  };
}

/**
 * Format a demurrage result for display
 * @param result - The demurrage calculation result
 * @returns A human-readable string
 */
export function formatDemurrageResult(result: DemurrageResult): string {
  const periodsText =
    result.inactive_periods === 1 ? "period" : "periods";

  if (result.demurrage_amount === 0n) {
    return `No demurrage applied. Balance: ${result.final_balance} KCHNG`;
  }

  return `${result.demurrage_amount} KCHNG burned (${result.inactive_periods} ${periodsText}). New balance: ${result.final_balance} KCHNG`;
}
