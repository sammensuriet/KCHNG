/**
 * Network configuration for KCHNG frontend
 */

import { getNetworkConfig } from "@kchng/shared";
import type { NetworkConfig } from "@kchng/shared";

export type Network = "testnet" | "mainnet" | "standalone";

/**
 * Get network configuration with contract ID
 * Uses the shared package's configuration which includes the deployed contract IDs
 */
export function getKchngNetworkConfig(network: Network): NetworkConfig & {
  contractId: string;
} {
  const baseConfig = getNetworkConfig(network);

  return {
    ...baseConfig,
    contractId: baseConfig.contractId,
  };
}

/**
 * Current active network (can be made configurable via UI later)
 */
export const CURRENT_NETWORK: Network = "mainnet";

export const networkConfig = getKchngNetworkConfig(CURRENT_NETWORK);
