/**
 * Wallet connection and balance management
 * Uses @creit.tech/stellar-wallets-kit for multi-wallet support
 * Uses KchngClient to fetch real balance from deployed contract
 */

import { writable, derived, get, type Readable } from "svelte/store";
import { browser } from "$app/environment";
import { getNetworkConfig } from "@kchng/shared";

export type NetworkName = "testnet" | "mainnet" | "futurenet" | "standalone";

export interface WalletState {
  connected: boolean;
  address: string | null;
  balance: bigint;
  lastActivity: number;
  walletName: string | null;
  error: string | null;
  network: NetworkName;
  trustId: string | null;
  isTrustMember: boolean;
}

const initialState: WalletState = {
  connected: false,
  address: null,
  balance: 0n,
  lastActivity: 0,
  walletName: null,
  error: null,
  network: "testnet",
  trustId: null,
  isTrustMember: false,
};

let walletsKit: any = null;

function createWalletStore() {
  const { subscribe, set, update } = writable<WalletState>(initialState);

  /**
   * Initialize Stellar Wallets Kit and connect
   */
  async function connect(network: NetworkName = "testnet") {
    if (typeof window === "undefined") {
      update((s) => ({ ...s, error: "Wallet only available in browser" }));
      return;
    }

    try {
      console.log("[Wallet] Starting connection for network:", network);

      // Dynamically import the wallet kit
      const { StellarWalletsKit, allowAllModules, WalletNetwork } = await import("@creit.tech/stellar-wallets-kit");

      // Map network name to WalletNetwork enum value
      const walletNetworkMap = {
        mainnet: WalletNetwork.PUBLIC,
        testnet: WalletNetwork.TESTNET,
        futurenet: WalletNetwork.FUTURENET,
        standalone: WalletNetwork.STANDALONE,
      };
      const walletNetwork = walletNetworkMap[network] ?? WalletNetwork.TESTNET;

      // Initialize the kit if not already created
      if (!walletsKit) {
        walletsKit = new StellarWalletsKit({
          modules: allowAllModules(),
          network: walletNetwork,
        });
      }

      // Open the modal for wallet selection using the kit's openModal method
      console.log("[Wallet] Opening modal");
      await walletsKit.openModal({
        onWalletSelected: async (wallet: { id: string; name: string }) => {
          console.log("[Wallet] Wallet selected:", wallet);

          // Set the selected wallet
          await walletsKit.setWallet(wallet.id);

          // Get the address
          try {
            const { address } = await walletsKit.getAddress();
            console.log("[Wallet] Got address:", address);
            update((s) => ({
              ...s,
              connected: true,
              address,
              network,
              walletName: wallet.name || "Wallet",
              error: null,
            }));
            loadBalance(address, network);
          } catch (err: any) {
            console.error("[Wallet] Error getting address:", err);
            update((s) => ({
              ...s,
              error: err.message || "Failed to get wallet address",
            }));
          }
        },
        onClosed: (err?: Error) => {
          if (err) {
            console.error("[Wallet] Modal closed with error:", err);
            update((s) => ({
              ...s,
              error: err.message || "Wallet selection cancelled",
            }));
          }
        },
      });

    } catch (e: unknown) {
      console.error("[Wallet] Connection error:", e);
      const errorMsg = e instanceof Error ? e.message : "Failed to connect wallet";
      console.error("[Wallet] Error message:", errorMsg);
      update((s) => ({
        ...s,
        error: errorMsg,
      }));
    }
  }

  /**
   * Disconnect wallet
   */
  function disconnect() {
    if (walletsKit) {
      walletsKit.disconnect();
      walletsKit = null;
    }
    set(initialState);
  }

  /**
   * Load KCHNG balance from deployed contract
   * Dynamically imports KchngClient to avoid bundling issues
   */
  async function loadBalance(address: string, network: NetworkName) {
    try {
      // Dynamically import the contract client
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient(network);
      const accountData = await kchngClient.getAccountData(address);

      // Extract trust membership status
      const trustId = accountData.trust_id;
      const isTrustMember = trustId !== null;

      update((s) => ({
        ...s,
        balance: accountData.balance,
        lastActivity: Number(accountData.last_activity),
        trustId,
        isTrustMember,
        error: null,
      }));
    } catch (e) {
      // Set error but don't disconnect - user can retry
      const errorMsg = e instanceof Error ? e.message : "Failed to load balance";
      console.error("[Wallet] Balance load error:", errorMsg);
      update((s) => ({
        ...s,
        error: errorMsg,
        balance: 0n,
        trustId: null,
        isTrustMember: false,
      }));
    }
  }

  /**
   * Sign a transaction using the connected wallet
   */
  async function signTransaction(xdr: string): Promise<string> {
    if (!walletsKit) throw new Error("Wallet not connected");
    const state = get({ subscribe });
    if (!state.connected) throw new Error("Wallet not connected");

    const networkConfig = getNetworkConfig(state.network);
    const { signedTxXdr } = await walletsKit.signTransaction(xdr, {
      address: state.address!,
      networkPassphrase: networkConfig.networkPassphrase,
    });
    return signedTxXdr;
  }

  /**
   * Refresh trust status after joining/leaving a trust
   */
  async function refreshTrustStatus() {
    const state = get({ subscribe });
    if (state.connected && state.address) {
      await loadBalance(state.address, state.network);
    }
  }

  /**
   * Refresh balance (useful after transactions)
   */
  async function refreshBalance() {
    const state = get({ subscribe });
    if (state.connected && state.address) {
      await loadBalance(state.address, state.network);
    }
  }

  /**
   * Switch network
   */
  async function switchNetwork(network: NetworkName) {
    const state = get({ subscribe });
    if (state.connected) {
      // Disconnect and reconnect with new network
      walletsKit = null;
      await connect(network);
    } else {
      update((s) => ({ ...s, network }));
    }
  }

  return {
    subscribe,
    connect,
    disconnect,
    loadBalance,
    refreshBalance,
    switchNetwork,
    signTransaction,
    refreshTrustStatus,
  };
}

export const wallet = createWalletStore();

export const truncatedAddress = derived(
  wallet,
  ($wallet) => {
    if (!$wallet.address) return "";
    return `${$wallet.address.slice(0, 4)}...${$wallet.address.slice(-4)}`;
  }
);

export const formattedBalance = derived(
  wallet,
  ($wallet) => {
    return $wallet.balance.toString();
  }
);
