<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";
  import { t } from "$lib/i18n";
  import { GraceType } from "@kchng/shared";

  let accountData = $state<{
    balance: bigint;
    last_activity: number;
    grace_period_end: number;
    trust_id: string | null;
    contribution_hours: number;
    grace_periods_used: number;
    last_grace_year: number;
  } | null>(null);

  let gracePeriodData = $state<{
    account: string;
    grace_type: GraceType;
    start_time: number;
    end_time: number;
    oracle_verified: boolean;
    extension_votes: number;
  } | null>(null);

  let isInCommunityProtection = $state(false);
  let loading = $state(true);
  let error = $state<string | null>(null);

  // Cross-Trust Exchange state
  let availableTrusts = $state<Array<{ id: string; name: string }>>([]);
  let selectedTrustId = $state<string | null>(null);
  let exchangeRate = $state<number | null>(null);
  let simulatedResult = $state<bigint | null>(null);
  let exchangeAmount = $state<string>("100000");
  let exchangeLoading = $state(false);
  let exchangeTxPending = $state(false);
  let exchangeMessage = $state<{ type: "success" | "error" | "info"; text: string } | null>(null);
  let exchangeRateState = $state<number | null>(null);

  onMount(async () => {
    await loadAccountData();
    await loadAvailableTrusts();
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

      // Check community protection status
      isInCommunityProtection = await kchngClient.isInGracePeriod($wallet.address);
      if (isInCommunityProtection) {
        try {
          gracePeriodData = await kchngClient.getGracePeriod($wallet.address);
        } catch {
          // Grace period data not available
          gracePeriodData = null;
        }
      }

      loading = false;
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to load account data";
      loading = false;
    }
  }

  async function loadAvailableTrusts() {
    if (!$wallet.connected || !$wallet.address || !$wallet.trustId) return;

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Get all trusts
      const trustIds = await kchngClient.getAllTrusts();
      const trusts = await Promise.all(
        trustIds
          .filter(id => id !== $wallet.trustId) // Exclude current trust
          .map(async (id) => {
            const info = await kchngClient.getTrustInfo(id);
            return { id, name: info.name };
          })
      );

      availableTrusts = trusts;

      if (trusts.length > 0) {
        selectedTrustId = trusts[0].id;
        await updateExchangePreview();
      }
    } catch (e) {
      console.error("Failed to load trusts:", e);
    }
  }

  async function updateExchangePreview() {
    if (!$wallet.trustId || !selectedTrustId || !exchangeAmount) {
      exchangeRate = null;
      simulatedResult = null;
      return;
    }

    exchangeLoading = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Get exchange rate
      exchangeRate = await kchngClient.calculateExchangeRate($wallet.trustId, selectedTrustId);

      // Simulate the swap
      const amount = BigInt(exchangeAmount);
      if (amount > 0n) {
        simulatedResult = await kchngClient.simulateCrossTrustSwap(
          $wallet.trustId,
          selectedTrustId,
          amount
        );
      }
    } catch (e) {
      console.error("Failed to calculate exchange:", e);
      exchangeRate = null;
      simulatedResult = null;
    } finally {
      exchangeLoading = false;
    }
  }

  async function executeExchange() {
    if (!$wallet.connected || !$wallet.address || !$wallet.trustId || !selectedTrustId) {
      exchangeMessage = { type: "error", text: "Please connect wallet and select a trust" };
      return;
    }

    exchangeTxPending = true;
    exchangeMessage = { type: "info", text: "Processing exchange..." };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const amount = BigInt(exchangeAmount);
      const txHash = await kchngClient.crossTrustSwap(
        $wallet.address,
        selectedTrustId,
        amount
      );

      exchangeMessage = {
        type: "success",
        text: `Exchange complete! Transaction: ${txHash.slice(0, 8)}...`
      };

      // Refresh account data
      await loadAccountData();

      // Clear message after delay
      setTimeout(() => exchangeMessage = null, 3000);

    } catch (e) {
      exchangeMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to execute exchange"
      };
    } finally {
      exchangeTxPending = false;
    }
  }

  $effect(() => {
    // Update preview when amount or trust changes
    if (selectedTrustId && exchangeAmount) {
      updateExchangePreview();
    }
  });

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

  function daysRemaining(endTime: number): number {
    const now = Math.floor(Date.now() / 1000);
    return Math.max(0, Math.floor((endTime - now) / 86400));
  }

  function getProtectionTypeName(type: GraceType): string {
    switch (type) {
      case GraceType.Emergency: return "Emergency";
      case GraceType.Illness: return "Illness";
      case GraceType.Community: return "Community Voted";
      default: return "Protection";
    }
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
      </div>

      <!-- Community Protection Card (if active) -->
      {#if isInCommunityProtection && gracePeriodData}
        <div class="card community-protection-card">
          <h2>🛡️ Community Protection</h2>
          <div class="protection-type">
            <span class="protection-label">Type:</span>
            <span class="protection-value">{getProtectionTypeName(gracePeriodData.grace_type)}</span>
          </div>
          <div class="protection-stats">
            <div class="protection-stat">
              <span class="stat-label">Days Remaining</span>
              <span class="stat-value protection-days">{daysRemaining(gracePeriodData.end_time)}</span>
            </div>
            <div class="protection-stat">
              <span class="stat-label">Oracle Verified</span>
              <span class="stat-value {gracePeriodData.oracle_verified ? 'verified' : 'pending'}">
                {gracePeriodData.oracle_verified ? '✓ Yes' : '⏳ Pending'}
              </span>
            </div>
          </div>
          {#if gracePeriodData.extension_votes > 0}
            <div class="extension-info">
              <span class="extension-label">Extension Votes:</span>
              <span class="extension-value">{gracePeriodData.extension_votes}</span>
            </div>
          {/if}
          <p class="protection-note">
            Your demurrage is temporarily paused. This protection ends on {formatDate(gracePeriodData.end_time)}.
          </p>
        </div>
      {/if}

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

    <!-- Cross-Trust Exchange (only for trust members) -->
    {#if accountData.trust_id && availableTrusts.length > 0}
      <div class="exchange-section">
        <h2>{t('dashboard.exchangeBetweenCommunities')}</h2>
        <p class="exchange-subtitle">{t('dashboard.exchangeSubtitle')}</p>

        <div class="exchange-card">
          <div class="exchange-from">
            <div class="exchange-label">{t('dashboard.fromCommunity')}</div>
            <div class="trust-badge current">Your Community</div>
            <div class="amount-input">
              <label>{t('dashboard.amountToSend')}</label>
              <input
                type="number"
                bind:value={exchangeAmount}
                placeholder="100000"
                oninput={() => updateExchangePreview()}
              />
              <span class="amount-unit">KCHNG</span>
            </div>
            <div class="meal-equiv">
              ≈ {Number(exchangeAmount || 0) / 1000} meals
            </div>
          </div>

          <div class="exchange-arrow">
            <span class="arrow-icon">→</span>
          </div>

          <div class="exchange-to">
            <div class="exchange-label">{t('dashboard.toCommunity')}</div>
            <select bind:value={selectedTrustId} onchange={() => updateExchangePreview()}>
              {#each availableTrusts as trust (trust.id)}
                <option value={trust.id}>{trust.name}</option>
              {/each}
            </select>

            {#if exchangeLoading}
              <div class="loading-preview">{t('dashboard.calculating')}</div>
            {:else if simulatedResult !== null}
              <div class="result-preview">
                <div class="result-amount">{Number(simulatedResult).toLocaleString()} KCHNG</div>
                <div class="result-meals">≈ {Number(simulatedResult) / 1000} meals</div>
                <div class="same-value-note">{t('dashboard.sameValueNote')}</div>
              </div>
            {:else}
              <div class="no-rate">{t('dashboard.selectTrustToPreview')}</div>
            {/if}
          </div>

          {#if exchangeRate !== null}
            <div class="rate-info">
              <span class="rate-label">{t('dashboard.exchangeRate')}:</span>
              <span class="rate-value">{(exchangeRate / 100).toFixed(2)}%</span>
              <span class="rate-explanation">{t('dashboard.rateExplanation')}</span>
            </div>
          {/if}

          {#if exchangeMessage}
            <div class="exchange-message {exchangeMessage.type}">
              {exchangeMessage.text}
            </div>
          {/if}

          <div class="exchange-actions">
            <button
              class="btn-exchange"
              onclick={executeExchange}
              disabled={exchangeTxPending || !simulatedResult || simulatedResult === 0n}
            >
              {#if exchangeTxPending}
                <span class="btn-spinner"></span>
                {t('dashboard.exchanging')}
              {:else}
                {t('dashboard.executeExchange')}
              {/if}
            </button>
          </div>

          <p class="exchange-note">
            {t('dashboard.exchangeNote')}
          </p>
        </div>
      </div>
    {/if}
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

  /* Community Protection Card */
  .community-protection-card {
    background: linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%);
    border: 2px solid #10b981;
  }

  .community-protection-card h2 {
    color: #065f46;
    margin-bottom: var(--space-md);
  }

  .protection-type {
    display: flex;
    gap: var(--space-sm);
    margin-bottom: var(--space-md);
  }

  .protection-label {
    color: #047857;
    font-weight: 500;
  }

  .protection-value {
    color: #065f46;
    font-weight: 600;
  }

  .protection-stats {
    display: flex;
    gap: var(--space-lg);
    margin-bottom: var(--space-md);
  }

  .protection-stat {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .protection-stat .stat-label {
    font-size: var(--font-size-xs);
    color: #047857;
  }

  .protection-stat .stat-value {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: #065f46;
  }

  .protection-days {
    font-size: var(--font-size-2xl) !important;
  }

  .stat-value.verified {
    color: #059669;
  }

  .stat-value.pending {
    color: #d97706;
  }

  .extension-info {
    background: rgba(255, 255, 255, 0.5);
    padding: var(--space-sm);
    border-radius: var(--radius-sm);
    margin-bottom: var(--space-md);
    font-size: var(--font-size-sm);
  }

  .extension-label {
    color: #047857;
  }

  .extension-value {
    color: #065f46;
    font-weight: 600;
  }

  .protection-note {
    font-size: var(--font-size-sm);
    color: #047857;
    font-style: italic;
    margin: 0;
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

  /* Cross-Trust Exchange Styles */
  .exchange-section {
    margin-top: var(--space-xl);
    padding: var(--space-lg);
    background: white;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .exchange-section h2 {
    margin: 0 0 var(--space-sm) 0;
    font-size: var(--font-size-xl);
  }

  .exchange-subtitle {
    color: var(--color-text-muted);
    margin: 0 0 var(--space-lg) 0;
  }

  .exchange-card {
    background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
    border: 2px solid #86efac;
    border-radius: var(--radius-md);
    padding: var(--space-lg);
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: var(--space-lg);
    align-items: start;
  }

  .exchange-from, .exchange-to {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .exchange-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .trust-badge.current {
    background: var(--color-gradient);
    color: white;
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-sm);
    font-weight: 500;
    text-align: center;
  }

  .amount-input {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .amount-input label {
    font-size: var(--font-size-sm);
    color: var(--color-text);
    font-weight: 500;
  }

  .amount-input input {
    padding: var(--space-sm);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    font-size: 1rem;
    width: 100%;
  }

  .amount-unit {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .meal-equiv {
    font-size: var(--font-size-sm);
    color: #047857;
    font-style: italic;
  }

  .exchange-arrow {
    display: flex;
    align-items: center;
    justify-content: center;
    padding-top: var(--space-xl);
  }

  .arrow-icon {
    font-size: 2rem;
    color: #10b981;
  }

  .exchange-to select {
    padding: var(--space-sm);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    font-size: 1rem;
    width: 100%;
  }

  .loading-preview, .no-rate {
    padding: var(--space-lg);
    text-align: center;
    color: var(--color-text-muted);
    font-style: italic;
  }

  .result-preview {
    background: white;
    border: 2px solid #10b981;
    border-radius: var(--radius-md);
    padding: var(--space-md);
    text-align: center;
  }

  .result-amount {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: #047857;
  }

  .result-meals {
    font-size: var(--font-size-sm);
    color: #047857;
    margin-top: var(--space-xs);
  }

  .same-value-note {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: var(--space-sm);
    font-style: italic;
  }

  .rate-info {
    grid-column: 1 / -1;
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    background: rgba(255, 255, 255, 0.8);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    flex-wrap: wrap;
  }

  .rate-label {
    color: var(--color-text-muted);
  }

  .rate-value {
    font-weight: 600;
    color: #047857;
  }

  .rate-explanation {
    color: var(--color-text-muted);
    font-style: italic;
  }

  .exchange-message {
    grid-column: 1 / -1;
    padding: var(--space-md);
    border-radius: var(--radius-sm);
    text-align: center;
  }

  .exchange-message.success {
    background: #d1fae5;
    color: #065f46;
  }

  .exchange-message.error {
    background: #fee2e2;
    color: #991b1b;
  }

  .exchange-message.info {
    background: #dbeafe;
    color: #1e40af;
  }

  .exchange-actions {
    grid-column: 1 / -1;
    display: flex;
    justify-content: center;
    margin-top: var(--space-md);
  }

  .btn-exchange {
    padding: var(--space-md) var(--space-xl);
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    transition: transform 0.2s, box-shadow 0.2s;
    width: auto;
  }

  .btn-exchange:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(16, 185, 129, 0.4);
  }

  .btn-exchange:disabled {
    opacity: 0.7;
    cursor: not-allowed;
    transform: none;
  }

  .btn-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .exchange-note {
    grid-column: 1 / -1;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    text-align: center;
    margin: var(--space-md) 0 0 0;
    font-style: italic;
  }

  @media (max-width: 768px) {
    .exchange-card {
      grid-template-columns: 1fr;
    }

    .exchange-arrow {
      padding: var(--space-md) 0;
      transform: rotate(90deg);
    }
  }
</style>
