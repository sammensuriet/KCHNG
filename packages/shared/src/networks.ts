/**
 * Network configurations for Stellar
 */

import type { NetworkConfig } from "./types.js";

/**
 * Predefined network configurations
 */
export const NETWORKS: Record<string, NetworkConfig> = {
  mainnet: {
    networkUrl: "https://horizon.stellar.org",
    networkPassphrase: "Public Global Stellar Network ; September 2015",
    contractId: "", // To be filled after deployment
  },
  testnet: {
    networkUrl: "https://horizon-testnet.stellar.org",
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "", // To be filled after deployment
  },
  futurenet: {
    networkUrl: "https://horizon-futurenet.stellar.org",
    networkPassphrase: "Test SDF Future Network ; October 2022",
    contractId: "", // To be filled after deployment
  },
  standalone: {
    networkUrl: "http://localhost:8000",
    networkPassphrase: "Standalone Network ; February 2017",
    contractId: "", // Local testing
  },
};

/**
 * Get network configuration by name
 * @param network - Network name
 * @returns Network configuration
 */
export function getNetworkConfig(network: string): NetworkConfig {
  const config = NETWORKS[network];
  if (!config) {
    throw new Error(`Unknown network: ${network}`);
  }
  return config;
}
