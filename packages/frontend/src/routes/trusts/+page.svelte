<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";

  let trusts = $state<Array<{
    id: string;
    name: string;
    governor: string;
    annual_rate_bps: number;
    demurrage_period_days: number;
    member_count: number;
    is_active: boolean;
    created_at: number;
  }>>([]);

  let userTrustId = $state<string | null>(null);
  let loading = $state(true);
  let showCreateForm = $state(false);

  // New trust form
  let newTrustName = $state("");
  let newTrustRate = $state(1200); // 12%
  let newTrustPeriod = $state(30);

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Get all trusts
      const trustIds = await kchngClient.getAllTrusts();
      const trustData = await Promise.all(
        trustIds.map(async (id) => {
          const info = await kchngClient.getTrustInfo(id);
          return {
            id,
            name: info.name,
            governor: info.governor,
            annual_rate_bps: Number(info.annual_rate_bps),
            demurrage_period_days: Number(info.demurrage_period_days),
            member_count: Number(info.member_count),
            is_active: info.is_active,
            created_at: Number(info.created_at),
          };
        })
      );

      trusts = trustData;

      // Get user's trust membership
      if ($wallet.connected && $wallet.address) {
        const accountData = await kchngClient.getAccountData($wallet.address);
        userTrustId = accountData.trust_id;
      }

      loading = false;
    } catch (e) {
      console.error("Failed to load trusts:", e);
      loading = false;
    }
  }

  async function createTrust() {
    if (!$wallet.connected || !$wallet.address) {
      alert("Please connect your wallet first");
      return;
    }

    if (!newTrustName.trim()) {
      alert("Please enter a trust name");
      return;
    }

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // This would call the contract's register_trust function
      // For now, just show an alert
      alert(`Trust creation requires transaction signing.\n\nTrust: ${newTrustName}\nRate: ${newTrustRate / 100}%\nPeriod: ${newTrustPeriod} days`);

      showCreateForm = false;
      newTrustName = "";
    } catch (e) {
      alert("Failed to create trust: " + (e instanceof Error ? e.message : "Unknown error"));
    }
  }

  async function joinTrust(trustId: string) {
    if (!$wallet.connected) {
      alert("Please connect your wallet first");
      return;
    }

    try {
      alert(`Joining trust requires transaction signing.\n\nTrust ID: ${trustId.slice(0, 8)}...`);
    } catch (e) {
      alert("Failed to join trust: " + (e instanceof Error ? e.message : "Unknown error"));
    }
  }

  function rateToPercentage(bps: number): string {
    return (bps / 100).toFixed(1) + "%";
  }
</script>

<div class="container">
  <h1>Community Trusts</h1>

  <div class="header-actions">
    <p class="subtitle">Federated community organizations with custom demurrage rates</p>
    {#if !showCreateForm}
      <button class="btn-create" onclick={() => showCreateForm = true}>
        + Create New Trust
      </button>
    {/if}
  </div>

  {#if showCreateForm}
    <div class="create-form">
      <h2>Create New Trust</h2>
      <div class="form-group">
        <label>Trust Name</label>
        <input type="text" bind:value={newTrustName} placeholder="e.g., Urban Elder Care Trust" />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>Annual Rate (%)</label>
          <input type="number" bind:value={newTrustRate} min="500" max="1500" step="100" />
          <small>Protocol limits: 5% - 15%</small>
        </div>
        <div class="form-group">
          <label>Demurrage Period (days)</label>
          <input type="number" bind:value={newTrustPeriod} min="1" max="365" />
        </div>
      </div>
      <div class="form-actions">
        <button onclick={createTrust}>Create Trust</button>
        <button class="btn-cancel" onclick={() => showCreateForm = false}>Cancel</button>
      </div>
    </div>
  {/if}

  {#if loading}
    <div class="loading">Loading trusts...</div>
  {:else if trusts.length === 0}
    <div class="empty-state">
      <div class="empty-icon">🏘️</div>
      <h3>No Trusts Yet</h3>
      <p>Be the first to create a community trust!</p>
    </div>
  {:else}
    <div class="trusts-grid">
      {#each trusts as trust (trust.id)}
        <div class="trust-card" class:in-trust={userTrustId === trust.id}>
          <div class="trust-header">
            <h3>{trust.name}</h3>
            {#if userTrustId === trust.id}
              <span class="member-badge">Member</span>
            {/if}
          </div>

          <div class="trust-stats">
            <div class="trust-stat">
              <span class="stat-label">Governor</span>
              <span class="stat-value stat-address">{trust.governor.slice(0, 8)}...</span>
            </div>
            <div class="trust-stat">
              <span class="stat-label">Demurrage Rate</span>
              <span class="stat-value rate-badge">{rateToPercentage(trust.annual_rate_bps)}</span>
            </div>
            <div class="trust-stat">
              <span class="stat-label">Period</span>
              <span class="stat-value">{trust.demurrage_period_days} days</span>
            </div>
            <div class="trust-stat">
              <span class="stat-label">Members</span>
              <span class="stat-value">{trust.member_count}</span>
            </div>
          </div>

          <div class="trust-actions">
            {#if userTrustId === trust.id}
              <button class="btn-view" disabled>✓ Joined</button>
            {:else}
              <button class="btn-join" onclick={() => joinTrust(trust.id)}>Join Trust</button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="info-box">
    <h3>About Trusts</h3>
    <p>Trusts are federated community organizations that set their own demurrage rates within protocol bounds (5-15% annually). Each trust is governed by a designated governor who manages membership and can propose rate changes.</p>
    <ul>
      <li><strong>Rate Range:</strong> 5% - 15% annual (protocol enforced)</li>
      <li><strong>Membership:</strong> Open to anyone, join via trust interface</li>
      <li><strong>Governance:</strong> Governor can propose rate changes via community vote</li>
      <li><strong>Cross-Trust:</strong> Exchange tokens between trusts at calculated rates</li>
    </ul>
  </div>

  <p class="value-footer">30 min verified work = 1000 KCHNG = 1 community meal</p>
</div>

<style>
  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--space-lg);
  }

  h1 {
    font-size: var(--font-size-3xl);
    margin-bottom: var(--space-sm);
  }

  .subtitle {
    color: var(--color-text-muted);
    margin: 0;
  }

  .header-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-lg);
  }

  .btn-create {
    padding: var(--space-sm) var(--space-lg);
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
  }

  .create-form {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    margin-bottom: var(--space-lg);
  }

  .create-form h2 {
    margin-top: 0;
  }

  .form-group {
    margin-bottom: var(--space-md);
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-md);
  }

  label {
    display: block;
    font-weight: 500;
    margin-bottom: var(--space-sm);
    color: var(--color-text);
  }

  input {
    width: 100%;
    padding: var(--space-sm);
    border: 1px solid #d1d5db;
    border-radius: var(--radius-sm);
    font-size: var(--font-size-base);
  }

  small {
    display: block;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    margin-top: var(--space-xs);
  }

  .form-actions {
    display: flex;
    gap: var(--space-md);
    margin-top: var(--space-lg);
  }

  .form-actions button {
    padding: var(--space-sm) var(--space-lg);
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
  }

  .form-actions button:first-child {
    background: var(--color-gradient);
    color: white;
  }

  .btn-cancel {
    background: var(--color-border-light) !important;
    color: var(--color-text) !important;
  }

  .loading, .empty-state {
    text-align: center;
    padding: var(--space-xl);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
  }

  .empty-icon {
    font-size: var(--font-size-4xl);
    margin-bottom: var(--space-md);
  }

  .trusts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: var(--space-lg);
    margin-bottom: var(--space-xl);
  }

  .trust-card {
    background: var(--color-bg);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    transition: all 0.2s;
  }

  .trust-card:hover {
    border-color: var(--color-primary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .trust-card.in-trust {
    border-color: var(--color-success);
    background: #f0fdf4;
  }

  .trust-header {
    display: flex;
    justify-content: space-between;
    align-items: start;
    margin-bottom: var(--space-md);
  }

  .trust-header h3 {
    margin: 0;
    font-size: var(--font-size-xl);
  }

  .member-badge {
    background: var(--color-success);
    color: white;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-full);
    font-size: var(--font-size-xs);
    font-weight: 500;
  }

  .trust-stats {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-md);
    margin-bottom: var(--space-lg);
  }

  .trust-stat {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .stat-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .stat-value {
    font-weight: 500;
  }

  .stat-address {
    font-family: monospace;
    font-size: var(--font-size-sm);
  }

  .rate-badge {
    background: #ede9fe;
    color: #7c3aed;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
  }

  .trust-actions button {
    width: 100%;
    padding: var(--space-sm);
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-join {
    background: var(--color-gradient);
    color: white;
  }

  .btn-join:hover {
    opacity: 0.9;
  }

  .btn-view {
    background: #d1fae5;
    color: #065f46;
    cursor: not-allowed;
  }

  .value-footer {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    margin-top: var(--space-lg);
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
  }

  @media (max-width: 640px) {
    .container {
      padding: var(--space-md);
    }

    .header-actions {
      flex-direction: column;
      gap: var(--space-md);
    }

    .trusts-grid {
      grid-template-columns: 1fr;
    }

    .form-row {
      grid-template-columns: 1fr;
    }
  }
</style>
