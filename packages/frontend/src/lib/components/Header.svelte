<script lang="ts">
  // TODO: Re-enable wallet integration after fixing stellar-sdk bundling
  // import { wallet, truncatedAddress, formattedBalance } from "$lib/stores/wallet";
  // import DemurrageInfo from "./DemurrageInfo.svelte";

  let showMenu = $state(false);
</script>

<header class="header">
  <div class="header-left">
    <a href="/" class="logo">KCHNG</a>
  </div>

  <div class="header-right">
    <!-- TODO: Wallet integration coming soon -->
    <button class="btn-connect" disabled>Wallet Coming Soon</button>
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

  .header-right {
    display: flex;
    align-items: center;
    position: relative;
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

  @media (max-width: 640px) {
    .header {
      padding: 1rem;
    }

    .logo {
      font-size: 1.25rem;
    }

    .wallet-dropdown {
      width: 260px;
      right: -0.5rem;
    }
  }
</style>
