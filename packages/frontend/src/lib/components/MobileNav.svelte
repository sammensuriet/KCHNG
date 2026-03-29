<script lang="ts">
  import { page } from "$app/stores";
  import { t } from "$lib/i18n";
  import { wallet } from "$lib/stores/wallet";

  // Navigation items with icons
  const navItems = [
    { href: "/dashboard", labelKey: "nav.dashboard", icon: "home" },
    { href: "/work", labelKey: "nav.work", icon: "hammer" },
    { href: "/communities", labelKey: "nav.communities", icon: "users" },
    { href: "/governance", labelKey: "nav.governance", icon: "vote" },
  ];

  function isActive(href: string): boolean {
    if (href === "/dashboard" && $page.url.pathname === "/") return true;
    return $page.url.pathname.startsWith(href);
  }
</script>

<nav class="mobile-nav" aria-label="Mobile navigation">
  {#each navItems as item}
    <a
      href={item.href}
      class="nav-item"
      class:active={isActive(item.href)}
      aria-current={isActive(item.href) ? "page" : undefined}
    >
      <span class="nav-icon" aria-hidden="true">
        {#if item.icon === "home"}
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
        {:else if item.icon === "hammer"}
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 12-8.373 8.373a1 1 0 1 1-3-3L12 9"/><path d="m18.6 12.6a2 2 0 1 0 3-3l-6.6-6.6a2 2 0 1 0-3 3l.5.5"/><path d="m14 10.5 2 2"/></svg>
        {:else if item.icon === "users"}
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>
        {:else if item.icon === "vote"}
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 12 2 2 4-4"/><path d="M5 7a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V7Z"/></svg>
        {/if}
      </span>
      <span class="nav-label">{t(item.labelKey)}</span>
    </a>
  {/each}

  <!-- Wallet/Profile Button -->
  {#if $wallet.connected}
    <a href="/dashboard" class="nav-item wallet-item" class:active={$page.url.pathname === "/dashboard"}>
      <span class="nav-icon wallet-icon" aria-hidden="true">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="14" x="2" y="5" rx="2"/><line x1="2" x2="22" y1="10" y2="10"/></svg>
      </span>
      <span class="nav-label">{t('header.balance')}</span>
    </a>
  {:else}
    <button
      class="nav-item wallet-item"
      onclick={() => wallet.connect()}
      aria-label={t('common.connectWallet')}
    >
      <span class="nav-icon" aria-hidden="true">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
      </span>
      <span class="nav-label">{t('common.connectWallet')}</span>
    </button>
  {/if}
</nav>

<style>
  .mobile-nav {
    display: none;
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    background: var(--color-bg);
    border-top: 1px solid var(--color-border);
    padding: 0.5rem 0.25rem;
    padding-bottom: max(0.5rem, env(safe-area-inset-bottom));
    z-index: 1000;
    box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
  }

  @media (max-width: 640px) {
    .mobile-nav {
      display: flex;
      justify-content: space-around;
      align-items: center;
    }
  }

  .nav-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 0.5rem 0.25rem;
    min-width: 56px;
    min-height: 56px;
    color: var(--color-text-muted);
    text-decoration: none;
    border-radius: var(--radius-md);
    transition: all 0.15s ease;
    -webkit-tap-highlight-color: rgba(102, 126, 234, 0.1);
    border: none;
    background: none;
    cursor: pointer;
    font-family: inherit;
    flex: 1;
  }

  .nav-item:active {
    transform: scale(0.95);
    background: var(--color-bg-subtle);
  }

  .nav-item.active {
    color: var(--color-primary);
  }

  .nav-item.active .nav-icon {
    color: var(--color-primary);
  }

  .nav-icon {
    width: 24px;
    height: 24px;
    margin-bottom: 0.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .nav-icon svg {
    width: 22px;
    height: 22px;
  }

  .nav-label {
    font-size: 0.65rem;
    font-weight: 500;
    text-align: center;
    line-height: 1.2;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 64px;
  }

  .wallet-item {
    color: var(--color-primary);
  }

  .wallet-item.active {
    background: var(--color-bg-subtle);
  }

  /* Add padding to page content to account for nav */
  :global(.page-wrapper) {
    padding-bottom: 0;
  }

  @media (max-width: 640px) {
    :global(.page-wrapper) {
      padding-bottom: calc(72px + env(safe-area-inset-bottom));
    }
  }
</style>
