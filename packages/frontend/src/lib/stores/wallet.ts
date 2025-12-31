/**
 * Simple wallet connection - Freighter only for now
 * We'll add more wallets later once we figure out the SSR issue
 */

import { writable, derived, get } from "svelte/store";

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
   * Load KCHNG balance from contract
   * TODO: Implement contract call once deployed
   */
  async function loadBalance(address: string) {
    // TODO: Call contract to get balance
    update((s) => ({
      ...s,
      balance: 1000000000000n, // Mock balance
      lastActivity: Math.floor(Date.now() / 1000) - 86400, // 1 day ago
    }));
  }

  const store = {
    subscribe,
    connect,
    disconnect,
    loadBalance,
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
