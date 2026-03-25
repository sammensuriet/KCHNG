<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";
  import { t } from "$lib/i18n";

  let accountData = $state<{
    balance: bigint;
    last_activity: number;
    grace_period_end: number;
    trust_id: string | null;
    contribution_hours: number;
    grace_periods_used: number;
    last_grace_year: number;
  } | null>(null);

  let loading = $state(true);
  let error = $state<string | null>(null);

  onMount(async () => {
    await loadAccountData();
  });

  async function loadAccountData() {
    if (!$wallet.connected || !$wallet.address) {
      error = t('dashboard.pleaseConnect');
      loading = false;
      return;
    }

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      accountData = await kchngClient.getAccountData($wallet.address);
      loading = false;
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to load account data";
      loading = false;
    }
  }

  function formatBalance(balance: bigint): string {
    return balance.toString();
  }

  function formatDate(timestamp: number): string {
    if (!timestamp) return "Never";
    return new Date(timestamp * 1000).toLocaleDateString();
  }

  function daysSinceActivity(lastActivity: number): number {
    if (!lastActivity) return 0;
    const now = Math.floor(Date.now() / 1000);
    return Math.floor((now - lastActivity) / 86400);
  }
</script>

<div class="container">
  <h1>{t('dashboard.title')}</h1>

  {#if loading}
    <div class="loading">{t('dashboard.loading')}</div>
  {:else if error}
    <div class="error">
      {error}
      <button onclick={loadAccountData}>{t('common.retry')}</button>
    </div>
  {:else if accountData}
    <div class="dashboard-grid">
      <!-- Balance Card -->
      <div class="card balance-card">
        <h2>{t('dashboard.balance')}</h2>
        <div class="balance-amount">{formatBalance(accountData.balance)} KCHNG</div>
        <div class="balance-subtext">
          {t('dashboard.balanceSubtext')}
        </div>
      </div>

      <!-- Activity Card -->
      <div class="card">
        <h2>{t('dashboard.accountActivity')}</h2>
        <div class="stat-row">
          <span class="stat-label">{t('dashboard.lastActivity')}</span>
          <span class="stat-value">{formatDate(accountData.last_activity)}</span>
        </div>
        <div class="stat-row">
          <span class="stat-label">{t('dashboard.circulationStatus')}</span>
          <span class="stat-value {daysSinceActivity(accountData.last_activity) >= 7 ? 'warning' : 'success'}">
            {#if daysSinceActivity(accountData.last_activity) < 7}
              {t('dashboard.active')}
            {:else}
              {daysSinceActivity(accountData.last_activity)} {t('dashboard.daysInactive')}
            {/if}
          </span>
        </div>
        {#if daysSinceActivity(accountData.last_activity) >= 7}
          <p class="circulation-hint">{t('dashboard.circulationHint')}</p>
        {/if}
        {#if accountData.grace_period_end > 0}
          <div class="grace-period">
            <span class="grace-badge">{t('dashboard.gracePeriodActive')}</span>
            <span>{t('dashboard.until')} {formatDate(accountData.grace_period_end)}</span>
          </div>
        {/if}
      </div>

      <!-- Trust Card -->
      <div class="card">
        <h2>{t('dashboard.trustMembership')}</h2>
        {#if accountData.trust_id}
          <div class="trust-info">
            <div class="stat-row">
              <span class="stat-label">{t('dashboard.trustId')}</span>
              <span class="stat-value stat-address">{accountData.trust_id.slice(0, 8)}...</span>
            </div>
            <a href="/trusts" class="btn-view">{t('dashboard.viewTrustDetails')}</a>
          </div>
        {:else}
          <div class="no-trust">
            <p>{t('dashboard.notTrustMember')}</p>
            <a href="/trusts" class="btn-join">{t('dashboard.browseTrusts')}</a>
          </div>
        {/if}
      </div>

      <!-- Contributions Card -->
      <div class="card">
        <h2>{t('dashboard.contributions')}</h2>
        <div class="stat-row">
          <span class="stat-label">{t('dashboard.hoursContributed')}</span>
          <span class="stat-value">{accountData.contribution_hours}h</span>
        </div>
        <div class="stat-row">
          <span class="stat-label">{t('dashboard.gracePeriodsUsed')}</span>
          <span class="stat-value">{accountData.grace_periods_used}/3 {t('dashboard.thisYear')}</span>
        </div>
        <a href="/work" class="btn-view">{t('dashboard.submitWork')}</a>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="actions-section">
      <h2>{t('dashboard.quickActions')}</h2>
      <div class="actions-grid">
        <a href="/work" class="action-card">
          <div class="action-icon">🔨</div>
          <div class="action-title">{t('dashboard.actionSubmitWork')}</div>
          <div class="action-desc">{t('dashboard.actionSubmitWorkDesc')}</div>
        </a>

        <a href="/work/verify" class="action-card">
          <div class="action-icon">✓</div>
          <div class="action-title">{t('dashboard.actionVerifyWork')}</div>
          <div class="action-desc">{t('dashboard.actionVerifyWorkDesc')}</div>
        </a>

        <a href="/trusts" class="action-card">
          <div class="action-icon">🏘️</div>
          <div class="action-title">{t('dashboard.actionTrusts')}</div>
          <div class="action-desc">{t('dashboard.actionTrustsDesc')}</div>
        </a>

        <a href="/governance" class="action-card">
          <div class="action-icon">🗳️</div>
          <div class="action-title">{t('dashboard.actionGovernance')}</div>
          <div class="action-desc">{t('dashboard.actionGovernanceDesc')}</div>
        </a>
      </div>
    </div>
  {:else}
    <div class="no-wallet">
      <p>{t('dashboard.pleaseConnectWallet')}</p>
      <button onclick={() => wallet.connect($wallet.network)}>{t('dashboard.connectWallet')}</button>
    </div>
  {/if}
</div>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--space-lg);
  }

  h1 {
    font-size: var(--font-size-3xl);
    margin-bottom: var(--space-lg);
  }

  h2 {
    font-size: var(--font-size-xl);
    margin-bottom: var(--space-md);
    color: var(--color-text);
  }

  .loading, .error, .no-wallet {
    text-align: center;
    padding: var(--space-xl);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
  }

  .error {
    color: var(--color-error);
    background: #fee2e2;
  }

  .no-wallet {
    composes: empty-state;
  }

  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: var(--space-lg);
    margin-bottom: var(--space-xl);
  }

  .card {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
  }

  .balance-card {
    background: var(--color-gradient);
    color: white;
  }

  .balance-card h2 {
    color: white;
  }

  .balance-amount {
    font-size: var(--font-size-4xl);
    font-weight: 700;
    margin: var(--space-sm) 0;
  }

  .balance-subtext {
    font-size: var(--font-size-sm);
    opacity: 0.9;
  }

  .stat-row {
    display: flex;
    justify-content: space-between;
    padding: var(--space-sm) 0;
    border-bottom: 1px solid var(--color-border-light);
  }

  .stat-label {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .stat-value {
    font-weight: 500;
  }

  .stat-address {
    font-family: monospace;
  }

  .warning {
    color: var(--color-error);
  }

  .success {
    color: var(--color-success);
  }

  .circulation-hint {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-top: var(--space-sm);
    font-style: italic;
  }

  .grace-period {
    margin-top: var(--space-md);
    padding: var(--space-sm);
    background: #d1fae5;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-size: var(--font-size-sm);
  }

  .grace-badge {
    background: var(--color-success);
    color: white;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-weight: 500;
  }

  .trust-info, .no-trust {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .no-trust p {
    color: var(--color-text-muted);
    margin: 0;
  }

  .btn-view, .btn-join {
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-sm);
    text-align: center;
    font-weight: 500;
    text-decoration: none;
    transition: all 0.2s;
  }

  .btn-view {
    background: var(--color-border-light);
    color: var(--color-text);
  }

  .btn-view:hover {
    background: var(--color-border);
  }

  .btn-join {
    background: var(--color-gradient);
    color: white;
  }

  .actions-section {
    margin-top: var(--space-xl);
  }

  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--space-md);
    margin-top: var(--space-md);
  }

  .action-card {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-lg);
    text-decoration: none;
    color: inherit;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .action-card:hover {
    border-color: var(--color-primary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .action-icon {
    font-size: var(--font-size-3xl);
    margin-bottom: var(--space-sm);
  }

  .action-title {
    font-weight: 600;
    margin-bottom: var(--space-xs);
    color: var(--color-text);
  }

  .action-desc {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  button {
    padding: var(--space-sm) var(--space-lg);
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    margin-top: var(--space-md);
  }

  @media (max-width: 640px) {
    .container {
      padding: var(--space-md);
      padding-bottom: calc(var(--space-xl) + 72px); /* Account for bottom nav */
    }

    .dashboard-grid {
      grid-template-columns: 1fr;
    }

    .balance-amount {
      font-size: var(--font-size-3xl);
    }

    /* Horizontal scrolling actions on mobile */
    .actions-grid {
      display: flex;
      overflow-x: auto;
      scroll-snap-type: x mandatory;
      gap: var(--space-md);
      padding: var(--space-xs) 0;
      margin: 0 calc(-1 * var(--space-md));
      padding: 0 var(--space-md);
      -webkit-overflow-scrolling: touch;
    }

    .actions-grid::-webkit-scrollbar {
      display: none;
    }

    .action-card {
      min-width: 160px;
      scroll-snap-align: start;
      flex-shrink: 0;
      min-height: 120px;
    }

    .action-card h3 {
      font-size: var(--font-size-sm);
    }

    .action-card p {
      font-size: var(--font-size-xs);
    }

    /* Larger touch targets for buttons */
    .btn-view, .btn-join {
      min-height: 44px;
      display: flex;
      align-items: center;
      justify-content: center;
    }
  }
</style>
