/**
 * Wallet connection and balance management
 * Uses KchngClient to fetch real balance from deployed contract
 */

import { writable, derived } from "svelte/store";

// Dynamically import stellar-sdk only when needed (on client-side, after user interaction)
// This avoids CommonJS bundling issues with Vite
let createKchngClient: any = null;

export interface WalletState {
  connected: boolean;
  address: string | null;
  balance: bigint;
  lastActivity: number;
  walletName: string | null;
  error: string | null;
}

const initialState: WalletState = {
  connected: false,
  address: null,
  balance: 0n,
  lastActivity: 0,
  walletName: null,
  error: null,
};

function createWalletStore() {
  const { subscribe, set, update } = writable<WalletState>(initialState);

  /**
   * Check if Freighter is available
   */
  async function isFreighterAvailable(): Promise<boolean> {
    if (!window.freighter) {
      return false;
    }
    try {
      const isConnected = await window.freighter.isConnected();
      return isConnected;
    } catch {
      return false;
    }
  }

  /**
   * Connect to Freighter wallet
   */
  async function connect() {
    if (typeof window === "undefined") {
      update((s) => ({ ...s, error: "Wallet only available in browser" }));
      return;
    }

    try {
      // Check if Freighter is installed
      if (!window.freighter) {
        update((s) => ({
          ...s,
          error: "Freighter wallet not found. Please install the Freighter extension.",
        }));
        return;
      }

      // Request access and get address
      await window.freighter.requestAccess();

      const addressObj = await window.freighter.getPublicKey();
      const address = addressObj;

      update((s) => ({
        ...s,
        connected: true,
        address,
        walletName: "Freighter",
        error: null,
      }));

      // Load balance after connection
      await loadBalance(address);
    } catch (e: unknown) {
      update((s) => ({
        ...s,
        error: e instanceof Error ? e.message : "Failed to connect wallet",
      }));
    }
  }

  /**
   * Disconnect wallet
   */
  function disconnect() {
    set(initialState);
  }

  /**
   * Load KCHNG balance from deployed contract
   * Dynamically imports KchngClient to avoid bundling issues
   */
  async function loadBalance(address: string) {
    try {
      // Dynamically import the contract client
      if (!createKchngClient) {
        const { createKchngClient: client } = await import("$lib/contracts/kchng");
        createKchngClient = client;
      }

      const kchngClient = createKchngClient();
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
    const state = get();
    if (state.connected && state.address) {
      await loadBalance(state.address);
    }
  }

  const store = {
    subscribe,
    connect,
    disconnect,
    loadBalance,
    refreshBalance,
  };

  return store;
}

function get(): WalletState {
  let state: WalletState | undefined;
  const unsubscribe = createWalletStore().subscribe((s) => {
    state = s;
  });
  unsubscribe();
  return state!;
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

// Add Freighter types to window
declare global {
  interface Window {
    freighter?: {
      isConnected(): Promise<boolean>;
      requestAccess(): Promise<void>;
      getPublicKey(): Promise<string>;
      signTransaction(xdr: string, opts?: {
        network?: string;
        networkPassphrase?: string;
        account?: string;
      }): Promise<string>;
    };
  }
}
