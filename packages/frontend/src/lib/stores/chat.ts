/**
 * Gun.js chat store for decentralized real-time messaging
 * Uses wallet address as user identity
 */

import { writable, derived, get } from "svelte/store";
import { browser } from "$app/environment";
import { wallet } from "./wallet";
import type { IGunInstance, IGunOnEvent } from "gun";

export interface ChatMessage {
  id: string;
  sender: string;
  text: string;
  timestamp: number;
}

interface GunMessage {
  sender: string;
  text: string;
  timestamp: number;
  _: any;
}

let gun: IGunInstance | null = null;
let messagesRef: any = null;

function createChatStore() {
  const { subscribe, set, update } = writable<ChatMessage[]>([]);
  let messageIds = new Set<string>();

  /**
   * Initialize Gun.js and subscribe to messages
   */
  function initialize() {
    if (!browser || gun) return;

    // Dynamically import Gun to avoid SSR issues
    import("gun").then((GunModule) => {
      const Gun = GunModule.default || GunModule;
      gun = Gun({
        peers: [
          "https://gun-manhattan.herokuapp.com/gun",
          "https://gun-us.herokuapp.com/gun",
        ],
      });

      // Subscribe to the kchng/chat channel
      messagesRef = gun.get("kchng").get("chat");

      // Listen for new messages
      messagesRef.map().on((data: GunMessage | undefined, id: string) => {
        if (!data || !data.sender || !data.text) return;

        // Skip duplicates
        if (messageIds.has(id)) return;
        messageIds.add(id);

        const message: ChatMessage = {
          id,
          sender: data.sender,
          text: data.text,
          timestamp: data.timestamp || Date.now(),
        };

        update((messages) => {
          // Check again for duplicates in the array
          if (messages.some((m) => m.id === id)) return messages;

          const newMessages = [...messages, message].sort(
            (a, b) => a.timestamp - b.timestamp
          );

          // Keep only last 200 messages to prevent memory issues
          return newMessages.slice(-200);
        });
      });
    });
  }

  /**
   * Send a message using the connected wallet address as identity
   */
  function sendMessage(text: string): boolean {
    if (!browser || !gun || !messagesRef) {
      console.error("[Chat] Gun not initialized");
      return false;
    }

    const walletState = get(wallet);
    if (!walletState.connected || !walletState.address) {
      console.error("[Chat] Wallet not connected");
      return false;
    }

    if (!text.trim()) {
      console.error("[Chat] Empty message");
      return false;
    }

    const message = {
      sender: walletState.address,
      text: text.trim(),
      timestamp: Date.now(),
    };

    // Generate unique ID
    const messageId = `${walletState.address}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

    // Save to Gun
    messagesRef.get(messageId).put(message);

    return true;
  }

  /**
   * Clear local messages (does not delete from Gun network)
   */
  function clearLocal() {
    messageIds.clear();
    set([]);
  }

  /**
   * Disconnect from Gun
   */
  function disconnect() {
    if (messagesRef) {
      messagesRef.off();
      messagesRef = null;
    }
    gun = null;
    clearLocal();
  }

  return {
    subscribe,
    initialize,
    sendMessage,
    clearLocal,
    disconnect,
  };
}

export const chatStore = createChatStore();
export const chatMessages = { subscribe: chatStore.subscribe };
export const sendMessage = chatStore.sendMessage;
export const clearLocalMessages = chatStore.clearLocal;
export const disconnectChat = chatStore.disconnect;

/**
 * Truncate address for display (e.g., "GABC...WXYZ")
 */
export function truncateAddress(address: string): string {
  if (!address || address.length < 12) return address;
  return `${address.slice(0, 4)}...${address.slice(-4)}`;
}

/**
 * Format timestamp for display
 */
export function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  // Less than 1 minute ago
  if (diff < 60000) {
    return "just now";
  }

  // Less than 1 hour ago
  if (diff < 3600000) {
    const mins = Math.floor(diff / 60000);
    return `${mins}m ago`;
  }

  // Less than 24 hours ago
  if (diff < 86400000) {
    const hours = Math.floor(diff / 3600000);
    return `${hours}h ago`;
  }

  // Otherwise show date
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}
