/**
 * Wallet connection and balance management
 * Uses @creit.tech/stellar-wallets-kit for multi-wallet support
 * Uses KchngClient to fetch real balance from deployed contract
 */

import { writable, derived, get, type Readable } from "svelte/store";
import { browser } from "$app/environment";
import { getNetworkConfig } from "@kchng/shared";

export type NetworkName = "testnet" | "futurenet" | "standalone";
// Note: "mainnet" is excluded during early-stage development

export interface WalletState {
  connected: boolean;
  address: string | null;
  balance: bigint;
  lastActivity: number;
  walletName: string | null;
  error: string | null;
  network: NetworkName;
  communityId: string | null;
  isCommunityMember: boolean;
  isTestWallet: boolean;
}

export interface TestWalletData {
  publicKey: string;
  secretKey: string;
}

const initialState: WalletState = {
  connected: false,
  address: null,
  balance: 0n,
  lastActivity: 0,
  walletName: null,
  error: null,
  network: "testnet",
  communityId: null,
  isCommunityMember: false,
  isTestWallet: false,
};

const TEST_WALLET_STORAGE_KEY = "kchng_test_wallet";

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
      let modalClosed = false;

      await walletsKit.openModal({
        onWalletSelected: async (wallet: { id: string; name: string }) => {
          console.log("[Wallet] Wallet selected:", wallet);
          modalClosed = true; // Modal closed normally

          try {
            // Set the selected wallet
            console.log("[Wallet] Setting wallet:", wallet.id);
            await walletsKit.setWallet(wallet.id);
            console.log("[Wallet] Wallet set, getting address...");

            // Get the address
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
            console.error("[Wallet] Error in wallet connection:", err);
            update((s) => ({
              ...s,
              error: err.message || "Failed to connect wallet",
            }));
          }
        },
        onClosed: (err?: Error) => {
          console.log("[Wallet] Modal closed, error:", err, "wasWalletSelected:", modalClosed);
          if (!modalClosed || err) {
            console.error("[Wallet] Modal closed unexpectedly");
            update((s) => ({
              ...s,
              error: err?.message || "Wallet connection cancelled",
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
   * Create a test wallet on testnet with 777 XLM funding from friendbot
   * Stores the wallet in localStorage for reuse
   */
  async function createTestWallet(): Promise<TestWalletData> {
    if (typeof window === "undefined") {
      throw new Error("Wallet only available in browser");
    }

    try {
      // Dynamically import Stellar SDK
      const { Keypair } = await import("@stellar/stellar-sdk");

      // Generate new keypair
      const keypair = Keypair.random();
      const publicKey = keypair.publicKey();
      const secretKey = keypair.secret();

      console.log("[Wallet] Created test wallet:", publicKey);

      // Fund via friendbot
      const friendbotUrl = `https://friendbot.stellar.org/?addr=${publicKey}`;
      const response = await fetch(friendbotUrl);

      if (!response.ok) {
        throw new Error("Failed to fund test wallet via friendbot");
      }

      console.log("[Wallet] Funded test wallet with XLM");

      // Store wallet data for later use
      const walletData: TestWalletData = { publicKey, secretKey };
      localStorage.setItem(TEST_WALLET_STORAGE_KEY, JSON.stringify(walletData));

      // Update state to connect this wallet
      update((s) => ({
        ...s,
        connected: true,
        address: publicKey,
        network: "testnet",
        walletName: "Test Wallet",
        error: null,
        isTestWallet: true,
      }));

      // Load balance from contract
      loadBalance(publicKey, "testnet");

      return walletData;
    } catch (e: unknown) {
      const errorMsg = e instanceof Error ? e.message : "Failed to create test wallet";
      console.error("[Wallet] Test wallet creation error:", errorMsg);
      update((s) => ({ ...s, error: errorMsg }));
      throw e;
    }
  }

  /**
   * Get stored test wallet data
   */
  function getStoredTestWallet(): TestWalletData | null {
    if (typeof window === "undefined") return null;
    try {
      const stored = localStorage.getItem(TEST_WALLET_STORAGE_KEY);
      if (stored) {
        return JSON.parse(stored);
      }
    } catch {
      // Ignore parse errors
    }
    return null;
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
      const communityId = accountData.trust_id;
      const isCommunityMember = communityId !== null;

      update((s) => ({
        ...s,
        balance: accountData.balance,
        lastActivity: Number(accountData.last_activity),
        communityId,
        isCommunityMember,
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
        communityId: null,
        isCommunityMember: false,
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
  async function refreshCommunityStatus() {
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
    createTestWallet,
    getStoredTestWallet,
    loadBalance,
    refreshBalance,
    switchNetwork,
    signTransaction,
    refreshCommunityStatus,
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
