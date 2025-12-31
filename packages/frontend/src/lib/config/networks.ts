/**
 * Network configuration for KCHNG frontend
 */

import { getNetworkConfig } from "@kchng/shared";
import type { NetworkConfig } from "@kchng/shared";

export type Network = "testnet" | "mainnet" | "standalone";

/**
 * Get network configuration with contract ID
 */
export function getKchngNetworkConfig(network: Network): NetworkConfig & {
  contractId: string;
} {
  const baseConfig = getNetworkConfig(network);

  // TODO: Replace with actual deployed contract IDs
  const contractIds: Record<Network, string> = {
    testnet: "CDXXXXX.....................", // Placeholder - update after deployment
    mainnet: "CDXXXXX.....................", // Placeholder - update after deployment
    standalone: "", // Local testing
  };

  return {
    ...baseConfig,
    contractId: contractIds[network],
  };
}

/**
 * Current active network (can be made configurable via UI later)
 */
export const CURRENT_NETWORK: Network = "testnet";

export const networkConfig = getKchngNetworkConfig(CURRENT_NETWORK);
