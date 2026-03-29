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
    rpcUrl: "https://mainnet.soroban.rpc.stellar.org",
    networkPassphrase: "Public Global Stellar Network ; September 2015",
    contractId: "CCGG5P7HU4TQNYOW6DK37JIDPZCAQ5ECBDENHEOULG72BZCD4BR7MKX6", // Deployed 2026-03-29 (v6 with verifier ecosystem)
  },
  testnet: {
    networkUrl: "https://horizon-testnet.stellar.org",
    rpcUrl: "https://soroban-testnet.stellar.org",
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CCIBWKAZYESALQMHAZOEH7FOMDMPPFLD74UGZMFEHIXUSFJWB2BDCGVQ", // Deployed 2026-03-29 (v6 with verifier ecosystem)
  },
  futurenet: {
    networkUrl: "https://horizon-futurenet.stellar.org",
    rpcUrl: "https://futurenet.soroban.rpc.stellar.org",
    networkPassphrase: "Test SDF Future Network ; October 2022",
    contractId: "", // To be filled after deployment
  },
  standalone: {
    networkUrl: "http://localhost:8000",
    rpcUrl: "http://localhost:8000/soroban/rpc",
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
