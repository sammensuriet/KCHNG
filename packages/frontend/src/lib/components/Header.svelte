<script lang="ts">
  import { wallet, truncatedAddress, formattedBalance, type NetworkName } from "$lib/stores/wallet";
  import { get } from "svelte/store";
  import DemurrageInfo from "./DemurrageInfo.svelte";

  let showMenu = $state(false);
  let showNetworkSelector = $state(false);
  let currentNetwork = $state(get(wallet).network);

  // Subscribe to wallet changes to sync network state
  $effect(() => {
    currentNetwork = get(wallet).network;
  });

  const networks: { id: NetworkName; label: string }[] = [
    { id: "testnet", label: "Testnet" },
    { id: "mainnet", label: "Mainnet" },
  ];

  async function switchNetwork(network: NetworkName) {
    showNetworkSelector = false;
    currentNetwork = network;
  }
</script>

<header class="header">
  <div class="header-left">
    <a href="/" class="logo">KCHNG</a>
    <div class="network-badge">
      <span class="network-dot"></span>
      {currentNetwork.toUpperCase()}
    </div>
  </div>

  <div class="header-right">
    {#if $wallet.error}
      <div class="error-message">
        {$wallet.error}
        <button class="error-dismiss" onclick={() => wallet.disconnect()}>×</button>
      </div>
    {/if}

    {#if !$wallet.connected}
      <div class="network-selector">
        <button
          class="btn-network"
          onclick={() => (showNetworkSelector = !showNetworkSelector)}
        >
          {currentNetwork.toUpperCase()} ▼
        </button>
        {#if showNetworkSelector}
          <div class="network-dropdown">
            {#each networks as network}
              <button
                class="network-option"
                class:active={currentNetwork === network.id}
                onclick={() => switchNetwork(network.id)}
              >
                {network.label}
              </button>
            {/each}
          </div>
        {/if}
      </div>
      <button class="btn-connect" onclick={() => wallet.connect(currentNetwork)}>
        Connect Wallet
      </button>
    {:else}
      <div class="wallet-info">
        <button class="btn-wallet" onclick={() => (showMenu = !showMenu)}>
          <span class="wallet-address">{truncatedAddress}</span>
          <span class="wallet-balance">{$formattedBalance} KCHNG</span>
        </button>

        {#if showMenu}
          <div class="wallet-dropdown">
            <div class="dropdown-section">
              <div class="dropdown-label">Connected as</div>
              <div class="dropdown-value">{$wallet.walletName}</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">Network</div>
              <div class="dropdown-value">{$wallet.network.toUpperCase()}</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">Address</div>
              <div class="dropdown-value dropdown-address">{$wallet.address}</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">Balance</div>
              <div class="dropdown-value">{$formattedBalance} KCHNG</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">Demurrage</div>
              <DemurrageInfo compact={true} />
            </div>

            <hr class="dropdown-divider" />

            <button
              class="btn-disconnect"
              onclick={() => wallet.disconnect()}
            >
              Disconnect
            </button>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</header>

<style>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: white;
    border-bottom: 1px solid #e5e7eb;
    position: sticky;
    top: 0;
    z-index: 100;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .logo {
    font-size: 1.5rem;
    font-weight: 700;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-decoration: none;
  }

  .network-badge {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    background: #f3f4f6;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
    color: #6b7280;
  }

  .network-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #10b981;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    position: relative;
  }

  .network-selector {
    position: relative;
  }

  .btn-network {
    padding: 0.5rem 1rem;
    background: white;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
  }

  .btn-network:hover {
    background: #f9fafb;
    border-color: #d1d5db;
  }

  .network-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    padding: 0.25rem;
    z-index: 101;
    min-width: 120px;
  }

  .network-option {
    width: 100%;
    padding: 0.5rem 0.75rem;
    background: none;
    border: none;
    border-radius: 4px;
    text-align: left;
    cursor: pointer;
    font-size: 0.875rem;
    color: #374151;
    transition: background 0.15s;
  }

  .network-option:hover {
    background: #f3f4f6;
  }

  .network-option.active {
    background: #ede9fe;
    color: #7c3aed;
    font-weight: 500;
  }

  .btn-connect {
    padding: 0.5rem 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-connect:hover {
    opacity: 0.9;
  }

  .wallet-info {
    position: relative;
  }

  .btn-wallet {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    padding: 0.5rem 1rem;
    background: #f3f4f6;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-wallet:hover {
    background: #e5e7eb;
  }

  .wallet-address {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
  }

  .wallet-balance {
    font-size: 0.75rem;
    color: #6b7280;
  }

  .wallet-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    width: 280px;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    padding: 1rem;
    z-index: 101;
  }

  .dropdown-section {
    margin-bottom: 0.75rem;
  }

  .dropdown-label {
    font-size: 0.75rem;
    color: #6b7280;
    margin-bottom: 0.25rem;
  }

  .dropdown-value {
    font-size: 0.875rem;
    color: #111827;
    font-weight: 500;
  }

  .dropdown-address {
    word-break: break-all;
    font-family: monospace;
    font-size: 0.75rem;
  }

  .dropdown-divider {
    border: none;
    border-top: 1px solid #e5e7eb;
    margin: 0.75rem 0;
  }

  .btn-disconnect {
    width: 100%;
    padding: 0.5rem;
    background: #fee2e2;
    color: #991b1b;
    border: none;
    border-radius: 4px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-disconnect:hover {
    background: #fecaca;
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 6px;
    font-size: 0.875rem;
  }

  .error-dismiss {
    background: none;
    border: none;
    color: #991b1b;
    font-size: 1.25rem;
    line-height: 1;
    cursor: pointer;
    padding: 0;
    width: 1.25rem;
    height: 1.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .error-dismiss:hover {
    background: #fecaca;
  }

  @media (max-width: 640px) {
    .header {
      padding: 1rem;
    }

    .logo {
      font-size: 1.25rem;
    }

    .network-badge {
      display: none;
    }

    .wallet-dropdown {
      width: 260px;
      right: -0.5rem;
    }
  }
</style>
