<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";

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
      error = "Please connect your wallet";
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
  <h1>Dashboard</h1>

  {#if loading}
    <div class="loading">Loading account data...</div>
  {:else if error}
    <div class="error">
      {error}
      <button onclick={loadAccountData}>Retry</button>
    </div>
  {:else if accountData}
    <div class="dashboard-grid">
      <!-- Balance Card -->
      <div class="card balance-card">
        <h2>Balance</h2>
        <div class="balance-amount">{formatBalance(accountData.balance)} KCHNG</div>
        <div class="balance-subtext">
          1 KCHNG = 30 minutes community work = 1 community meal
        </div>
      </div>

      <!-- Activity Card -->
      <div class="card">
        <h2>Account Activity</h2>
        <div class="stat-row">
          <span class="stat-label">Last Activity:</span>
          <span class="stat-value">{formatDate(accountData.last_activity)}</span>
        </div>
        <div class="stat-row">
          <span class="stat-label">Days Inactive:</span>
          <span class="stat-value {daysSinceActivity(accountData.last_activity) >= 7 ? 'warning' : ''}">
            {daysSinceActivity(accountData.last_activity)} days
          </span>
        </div>
        {#if accountData.grace_period_end > 0}
          <div class="grace-period">
            <span class="grace-badge">Grace Period Active</span>
            <span>Until {formatDate(accountData.grace_period_end)}</span>
          </div>
        {/if}
      </div>

      <!-- Trust Card -->
      <div class="card">
        <h2>Trust Membership</h2>
        {#if accountData.trust_id}
          <div class="trust-info">
            <div class="stat-row">
              <span class="stat-label">Trust ID:</span>
              <span class="stat-value stat-address">{accountData.trust_id.slice(0, 8)}...</span>
            </div>
            <a href="/trusts" class="btn-view">View Trust Details</a>
          </div>
        {:else}
          <div class="no-trust">
            <p>You are not a member of any trust.</p>
            <a href="/trusts" class="btn-join">Browse Trusts</a>
          </div>
        {/if}
      </div>

      <!-- Contributions Card -->
      <div class="card">
        <h2>Contributions</h2>
        <div class="stat-row">
          <span class="stat-label">Hours Contributed:</span>
          <span class="stat-value">{accountData.contribution_hours}h</span>
        </div>
        <div class="stat-row">
          <span class="stat-label">Grace Periods Used:</span>
          <span class="stat-value">{accountData.grace_periods_used}/3 this year</span>
        </div>
        <a href="/work" class="btn-view">Submit Work</a>
      </div>
    </div>

    <!-- Quick Actions -->
    <div class="actions-section">
      <h2>Quick Actions</h2>
      <div class="actions-grid">
        <a href="/work" class="action-card">
          <div class="action-icon">🔨</div>
          <div class="action-title">Submit Work</div>
          <div class="action-desc">Log verified work hours</div>
        </a>

        <a href="/work/verify" class="action-card">
          <div class="action-icon">✓</div>
          <div class="action-title">Verify Work</div>
          <div class="action-desc">Review community work claims</div>
        </a>

        <a href="/trusts" class="action-card">
          <div class="action-icon">🏘️</div>
          <div class="action-title">Trusts</div>
          <div class="action-desc">Manage community trusts</div>
        </a>

        <a href="/governance" class="action-card">
          <div class="action-icon">🗳️</div>
          <div class="action-title">Governance</div>
          <div class="action-desc">Vote on proposals</div>
        </a>
      </div>
    </div>
  {:else}
    <div class="no-wallet">
      <p>Please connect your wallet to view your dashboard.</p>
      <button onclick={() => wallet.connect($wallet.network)}>Connect Wallet</button>
    </div>
  {/if}
</div>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    font-size: 2rem;
    margin-bottom: 2rem;
  }

  h2 {
    font-size: 1.25rem;
    margin-bottom: 1rem;
    color: #374151;
  }

  .loading, .error, .no-wallet {
    text-align: center;
    padding: 3rem;
    background: #f9fafb;
    border-radius: 8px;
  }

  .error {
    color: #991b1b;
    background: #fee2e2;
  }

  .dashboard-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
  }

  .card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 12px;
    padding: 1.5rem;
  }

  .balance-card {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .balance-card h2 {
    color: white;
  }

  .balance-amount {
    font-size: 2.5rem;
    font-weight: 700;
    margin: 0.5rem 0;
  }

  .balance-subtext {
    font-size: 0.875rem;
    opacity: 0.9;
  }

  .stat-row {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid #f3f4f6;
  }

  .stat-label {
    color: #6b7280;
    font-size: 0.875rem;
  }

  .stat-value {
    font-weight: 500;
  }

  .stat-address {
    font-family: monospace;
  }

  .warning {
    color: #dc2626;
  }

  .grace-period {
    margin-top: 1rem;
    padding: 0.75rem;
    background: #d1fae5;
    border-radius: 6px;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
  }

  .grace-badge {
    background: #10b981;
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-weight: 500;
  }

  .trust-info, .no-trust {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .no-trust p {
    color: #6b7280;
    margin: 0;
  }

  .btn-view, .btn-join {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    text-align: center;
    font-weight: 500;
    text-decoration: none;
    transition: all 0.2s;
  }

  .btn-view {
    background: #f3f4f6;
    color: #374151;
  }

  .btn-view:hover {
    background: #e5e7eb;
  }

  .btn-join {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  .actions-section {
    margin-top: 3rem;
  }

  .actions-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-top: 1rem;
  }

  .action-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1.5rem;
    text-decoration: none;
    color: inherit;
    transition: all 0.2s;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .action-card:hover {
    border-color: #667eea;
    transform: translateY(-2px);
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
  }

  .action-icon {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
  }

  .action-title {
    font-weight: 600;
    margin-bottom: 0.25rem;
    color: #374151;
  }

  .action-desc {
    font-size: 0.875rem;
    color: #6b7280;
  }

  button {
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    margin-top: 1rem;
  }

  @media (max-width: 640px) {
    .container {
      padding: 1rem;
    }

    .dashboard-grid {
      grid-template-columns: 1fr;
    }

    .balance-amount {
      font-size: 2rem;
    }
  }
</style>
