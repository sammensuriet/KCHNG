/**
 * Wallet connection and balance management
 * Uses @creit.tech/stellar-wallets-kit for multi-wallet support
 * Uses KchngClient to fetch real balance from deployed contract
 */

import { writable, derived, get } from "svelte/store";
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
}

const initialState: WalletState = {
  connected: false,
  address: null,
  balance: 0n,
  lastActivity: 0,
  walletName: null,
  error: null,
  network: "testnet",
};

let walletsKit: any = null;
let modalElement: any = null;

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

      // Dynamically import the wallet kit and modal
      const { StellarWalletsKit, allowAllModules, WalletNetwork, StellarWalletsModal } = await import("@creit.tech/stellar-wallets-kit");

      // Map network name to WalletNetwork enum
      let walletNetwork: string;
      switch (network) {
        case "mainnet":
          walletNetwork = WalletNetwork.PUBLIC;
          break;
        case "testnet":
          walletNetwork = WalletNetwork.TESTNET;
          break;
        case "futurenet":
          walletNetwork = WalletNetwork.FUTURENET;
          break;
        case "standalone":
          walletNetwork = WalletNetwork.STANDALONE;
          break;
        default:
          walletNetwork = WalletNetwork.TESTNET;
      }

      // Initialize the kit
      if (!walletsKit) {
        walletsKit = new StellarWalletsKit({
          modules: allowAllModules(),
          network: walletNetwork,
        });
      }

      // Create and show the modal for wallet selection
      console.log("[Wallet] Creating modal");
      const modal = new StellarWalletsModal({
        onWalletSelected: (wallet: any) => {
          console.log("[Wallet] Wallet selected:", wallet);
        },
        onConnected: (wallet: any) => {
          console.log("[Wallet] Connected to wallet:", wallet);
          // Get the address from the kit
          walletsKit.getAddress().then(({ address }: any) => {
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
          }).catch((err: any) => {
            console.error("[Wallet] Error getting address:", err);
            update((s) => ({
              ...s,
              error: err.message || "Failed to get wallet address",
            }));
          });
        },
        onDisconnected: () => {
          console.log("[Wallet] Disconnected");
          set(initialState);
        },
        onError: (err: any) => {
          console.error("[Wallet] Modal error:", err);
          update((s) => ({
            ...s,
            error: err.message || "Wallet connection failed",
          }));
        },
        network: walletNetwork,
        allowedWallets: await walletsKit.getSupportedWallets(),
      });

      // Show the modal
      console.log("[Wallet] Opening modal");
      modal.open();
      modalElement = modal;

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
    if (modalElement) {
      try {
        modalElement.close();
      } catch (e) {
        console.error("[Wallet] Error closing modal:", e);
      }
      modalElement = null;
    }
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
      update((s) => ({
        ...s,
        balance: accountData.balance,
        lastActivity: Number(accountData.last_activity),
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
      }));
    }
  }

  /**
   * Refresh balance (useful after transactions)
   */
  async function refreshBalance() {
    const state = get(store);
    if (state.connected && state.address) {
      await loadBalance(state.address, state.network);
    }
  }

  /**
   * Switch network
   */
  async function switchNetwork(network: NetworkName) {
    const state = get(store);
    if (state.connected) {
      // Disconnect and reconnect with new network
      walletsKit = null;
      modalElement = null;
      await connect(network);
    } else {
      update((s) => ({ ...s, network }));
    }
  }

  const store = {
    subscribe,
    connect,
    disconnect,
    loadBalance,
    refreshBalance,
    switchNetwork,
  };

  return store;
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
