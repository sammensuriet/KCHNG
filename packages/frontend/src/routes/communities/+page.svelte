<script lang="ts">
  import { onMount } from "svelte";
  import { t } from "$lib/i18n";
  import { wallet } from "$lib/stores/wallet";

  let communities = $state<Array<{
    id: string;
    name: string;
    governor: string;
    annual_rate_bps: number;
    demurrage_period_days: number;
    member_count: number;
    is_active: boolean;
    created_at: number;
  }>>([]);

  let userCommunityId = $state<string | null>(null);
  let loading = $state(true);
  let showCreateForm = $state(false);

  // New community form - defaults: 12% annual rate, 28 day period
  let newCommunityName = $state("");
  let newCommunityRatePercent = $state(12); // User enters percentage (5-15)
  let newCommunityPeriod = $state(28); // Default 28 days

  // Transaction state
  let txPending = $state(false);
  let txMessage = $state<{ type: "success" | "error" | "info"; text: string } | null>(null);

  // Oracle state
  let isOracle = $state(false);
  let oracleLoading = $state(false);
  let showOracleModal = $state(false);

  // Governor management state
  let isGovernor = $state(false);
  let governorCommunityId = $state<string | null>(null);
  let currentSuccessor = $state<string | null>(null);
  let showSuccessorModal = $state(false);
  let showStepDownModal = $state(false);
  let newSuccessorAddress = $state("");

  onMount(async () => {
    await loadData();
    await checkOracleStatus();
  });

  async function checkOracleStatus() {
    if (!$wallet.connected || !$wallet.address) {
      isOracle = false;
      return;
    }

    oracleLoading = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      const oracleData = await kchngClient.getOracle($wallet.address);
      isOracle = oracleData && oracleData.oracle_address === $wallet.address;
    } catch {
      // Not an oracle or error - that's fine
      isOracle = false;
    } finally {
      oracleLoading = false;
    }
  }

  async function registerAsOracle() {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: t('communities.errors.connectWallet') };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: t('communities.oracle.registering') };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const txHash = await kchngClient.registerOracle($wallet.address);

      txMessage = {
        type: "success",
        text: `You are now a Community Oracle! Transaction: ${txHash.slice(0, 8)}...`
      };

      isOracle = true;
      showOracleModal = false;

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : t('communities.oracle.failed')
      };
    } finally {
      txPending = false;
    }
  }

  async function designateSuccessor() {
    if (!$wallet.connected || !$wallet.address || !newSuccessorAddress.trim()) {
      txMessage = { type: "error", text: t('communities.errors.enterSuccessor') };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: t('communities.tx.namingSuccessor') };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const txHash = await kchngClient.designateSuccessor($wallet.address, newSuccessorAddress.trim());

      txMessage = {
        type: "success",
        text: `Successor named! Transaction: ${txHash.slice(0, 8)}...`
      };

      currentSuccessor = newSuccessorAddress.trim();
      newSuccessorAddress = "";
      showSuccessorModal = false;

      // Refresh data
      await loadData();

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : t('communities.errors.failedSuccessor')
      };
    } finally {
      txPending = false;
    }
  }

  async function stepDownAsGovernor() {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: t('communities.errors.connectWallet') };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: t('communities.tx.passingTorch') };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const { RoleType } = await import("@kchng/shared");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const txHash = await kchngClient.stepDown($wallet.address, RoleType.Governor);

      txMessage = {
        type: "success",
        text: `You've passed the torch! Transaction: ${txHash.slice(0, 8)}...`
      };

      isGovernor = false;
      governorCommunityId = null;
      currentSuccessor = null;
      showStepDownModal = false;

      // Refresh data
      await loadData();

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : t('communities.errors.failedStepDown')
      };
    } finally {
      txPending = false;
    }
  }

  async function loadData() {
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Get all communities
      const communityIds = await kchngClient.getAllCommunities();
      const communityData = await Promise.all(
        communityIds.map(async (id) => {
          const info = await kchngClient.getCommunityInfo(id);
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

      communities = communityData;

      // Get user's community membership
      if ($wallet.connected && $wallet.address) {
        const accountData = await kchngClient.getAccountData($wallet.address);
        userCommunityId = accountData.community_id;

        // Check if user is a governor of any community
        for (const community of communityData) {
          if (community.governor === $wallet.address) {
            isGovernor = true;
            governorCommunityId = community.id;
            // Get full community info to check successor
            const fullCommunityInfo = await kchngClient.getCommunityInfo(community.id);
            currentSuccessor = fullCommunityInfo.successor;
            break;
          }
        }
      }

      loading = false;
    } catch (e) {
      console.error("Failed to load communities:", e);
      loading = false;
    }
  }

  async function createCommunity() {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: t('communities.errors.connectWallet') };
      return;
    }

    if (!newCommunityName.trim()) {
      txMessage = { type: "error", text: t('communities.errors.enterName') };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: t('communities.tx.preparing') };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Set up the signing callback
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      // Convert percentage to basis points (e.g., 12% = 1200 bps)
      const annualRateBps = newCommunityRatePercent * 100;

      // Create the community
      const txHash = await kchngClient.registerCommunity(
        $wallet.address!,
        newCommunityName,
        annualRateBps,
        newCommunityPeriod
      );

      txMessage = {
        type: "success",
        text: `Community created! Transaction: ${txHash.slice(0, 8)}...`
      };

      showCreateForm = false;
      newCommunityName = "";
      newCommunityRatePercent = 12;
      newCommunityPeriod = 28;

      // Refresh data
      await loadData();
      await wallet.refreshCommunityStatus();

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : t('communities.errors.failedCreate')
      };
    } finally {
      txPending = false;
    }
  }

  async function joinCommunity(communityId: string) {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: t('communities.errors.connectWallet') };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: t('communities.tx.preparing') };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Set up the signing callback
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      // Join the community
      const txHash = await kchngClient.joinCommunity(communityId, $wallet.address);

      txMessage = {
        type: "success",
        text: `Joined community! Transaction: ${txHash.slice(0, 8)}...`
      };

      // Refresh data
      await loadData();
      await wallet.refreshCommunityStatus();

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : t('communities.errors.failedJoin')
      };
    } finally {
      txPending = false;
    }
  }

  function rateToPercentage(bps: number): string {
    return (bps / 100).toFixed(1) + "%";
  }

  // Calculate per-period rate for display
  function periodRatePercent(annualBps: number, periodDays: number): string {
    const periodRate = (annualBps * periodDays) / 365 / 100;
    return periodRate.toFixed(2) + "%";
  }
</script>

<svelte:head>
  <title>{t('communities.title')}</title>
</svelte:head>

<div class="container">
  <h1>{t('communities.heading')}</h1>

  <div class="header-actions">
    <p class="subtitle">
      {t('communities.subtitle')}
    </p>
    {#if !showCreateForm}
      <button class="btn-create" onclick={() => showCreateForm = true}>
        {t('communities.createCommunity')}
      </button>
    {/if}
  </div>

  {#if showCreateForm}
    <div class="create-form">
      <h2>{t('communities.form.heading')}</h2>
      <div class="form-group">
        <label>{t('communities.form.name')}</label>
        <input type="text" bind:value={newCommunityName} placeholder={t('communities.form.namePlaceholder')} disabled={txPending} />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>{t('communities.form.annualRate')}</label>
          <input type="number" bind:value={newCommunityRatePercent} min="5" max="15" step="0.5" inputmode="decimal" disabled={txPending} />
          <small>Protocol limits: 5% - 15% annually (default: 12%)</small>
        </div>
        <div class="form-group">
          <label>{t('communities.form.period')}</label>
          <input type="number" bind:value={newCommunityPeriod} min="7" max="365" inputmode="numeric" pattern="[0-9]*" disabled={txPending} />
          <small>7 - 365 days (default: 28)</small>
        </div>
      </div>

      <div class="rate-preview">
        <span class="preview-label">{t('communities.form.periodDemurrage')}</span>
        <span class="preview-value">{periodRatePercent(newCommunityRatePercent * 100, newCommunityPeriod)} every {newCommunityPeriod} {t('communities.form.periodDays')}</span>
      </div>

      {#if txMessage}
        <div class="tx-message tx-message-{txMessage.type}">
          {#if txMessage.type === "info"}
            <span class="spinner"></span>
          {:else if txMessage.type === "success"}
            <span class="icon">✓</span>
          {:else}
            <span class="icon">⚠</span>
          {/if}
          {txMessage.text}
        </div>
      {/if}

      <div class="form-actions">
        <button onclick={createCommunity} disabled={txPending}>
          {#if txPending}
            <span class="btn-spinner"></span>
            {t('communities.form.creating')}
          {:else}
            {t('communities.form.create')}
          {/if}
        </button>
        <button class="btn-cancel" onclick={() => { showCreateForm = false; txMessage = null; }} disabled={txPending}>{t('common.cancel')}</button>
      </div>
    </div>
  {/if}

  {#if txMessage && !showCreateForm}
    <div class="tx-message tx-message-{txMessage.type}">
      {#if txMessage.type === "info"}
        <span class="spinner"></span>
      {:else if txMessage.type === "success"}
        <span class="icon">✓</span>
      {:else}
        <span class="icon">⚠</span>
      {/if}
      {txMessage.text}
      <button class="btn-dismiss" onclick={() => txMessage = null}>×</button>
    </div>
  {/if}

  {#if loading}
    <div class="loading">{t('communities.loading')}</div>
  {:else if communities.length === 0}
    <div class="empty-state">
      <div class="empty-icon">🏘️</div>
      <h3>{t('communities.empty.title')}</h3>
      <p>{t('communities.empty.description')}</p>
    </div>
  {:else}
    <div class="communities-grid">
      {#each communities as community (community.id)}
        <div class="community-card" class:in-community={userCommunityId === community.id}>
          <div class="community-header">
            <h3>{community.name}</h3>
            {#if userCommunityId === community.id}
              <span class="member-badge">Member</span>
            {/if}
          </div>

          <div class="community-stats">
            <div class="community-stat">
              <span class="stat-label">{t('communities.card.governor')}</span>
              <span class="stat-value stat-address">{community.governor.slice(0, 8)}...</span>
            </div>
            <div class="community-stat">
              <span class="stat-label">{t('communities.card.rate')}</span>
              <span class="stat-value rate-badge">{rateToPercentage(community.annual_rate_bps)}/year</span>
              <span class="stat-sub">({periodRatePercent(community.annual_rate_bps, community.demurrage_period_days)}/{community.demurrage_period_days}d)</span>
            </div>
            <div class="community-stat">
              <span class="stat-label">{t('communities.card.period')}</span>
              <span class="stat-value">{community.demurrage_period_days} {t('communities.form.periodDays')}</span>
            </div>
            <div class="community-stat">
              <span class="stat-label">{t('communities.card.members')}</span>
              <span class="stat-value">{community.member_count}</span>
            </div>
          </div>

          <div class="community-actions">
            {#if userCommunityId === community.id}
              <button class="btn-view" disabled>{t('communities.card.joined')}</button>
            {:else}
              <button
                class="btn-join"
                onclick={() => joinCommunity(community.id)}
                disabled={txPending}
              >
                {#if txPending}
                  <span class="btn-spinner"></span>
                  {t('communities.card.joining')}
                {:else}
                  {t('communities.card.join')}
                {/if}
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="info-box">
    <h3>{t('communities.about.title')}</h3>
    <p>{t('communities.about.description')}</p>
    <ul>
      <li><strong>{t('communities.about.rateRange')}</strong> {t('communities.about.rateRangeValue')}</li>
      <li><strong>{t('communities.about.membership')}</strong> {t('communities.about.membershipValue')}</li>
      <li><strong>{t('communities.about.governance')}</strong> {t('communities.about.governanceValue')}</li>
      <li><strong>{t('communities.about.crossCommunity')}</strong> {t('communities.about.crossCommunityValue')}</li>
    </ul>
  </div>

  <!-- Community Oracle Section -->
  <div class="oracle-section">
    <h2>{t('communities.oracle.heading')}</h2>
    <p class="oracle-description">
      {t('communities.oracle.description')}
    </p>

    <div class="oracle-card">
      <div class="oracle-info">
        <h4>{t('communities.oracle.responsibilities')}</h4>
        <ul>
          <li>{t('communities.oracle.responsibility1')}</li>
          <li>{t('communities.oracle.responsibility2')}</li>
          <li>{t('communities.oracle.responsibility3')}</li>
        </ul>
      </div>

      {#if oracleLoading}
        <div class="oracle-loading">{t('communities.loading')}</div>
      {:else if isOracle}
        <div class="oracle-status registered">
          <span class="status-icon">✓</span>
          <span class="status-text">{t('communities.card.governorOf')}</span>
        </div>
        <p class="oracle-hint">
          {t('communities.oracle.description')}
        </p>
      {:else if $wallet.connected}
        <div class="oracle-actions">
          <button
            class="btn-register-oracle"
            onclick={() => showOracleModal = true}
          >
            {t('communities.oracle.register')}
          </button>
          <p class="oracle-note">
            {t('communities.oracle.stakeValue')}
          </p>
        </div>
      {:else}
        <div class="connect-prompt">
          <p>{t('common.connectWallet')}</p>
        </div>
      {/if}

      {#if txMessage && !showOracleModal}
        <div class="tx-message {txMessage.type}">{txMessage.text}</div>
      {/if}
    </div>
  </div>

  <!-- Governor Management Section -->
  {#if isGovernor}
    <div class="governor-section">
      <h2>{t('communities.governor.heading')}</h2>
      <p class="governor-description">
        {t('communities.governor.description')}
      </p>

      <div class="governor-card">
        <div class="governor-status">
          <span class="status-icon governor-icon">👑</span>
          <span class="status-text">{t('communities.governor.description')}</span>
        </div>

        <div class="successor-info">
          <h4>{t('communities.governor.successor')}</h4>
          {#if currentSuccessor}
            <div class="successor-address">
              <span class="address-badge">{currentSuccessor.slice(0, 8)}...{currentSuccessor.slice(-6)}</span>
              <span class="successor-note">{t('communities.governor.successorDesc')}</span>
            </div>
          {:else}
            <div class="no-successor">
              <span class="warning-icon">⚠️</span>
              <span>{t('communities.governor.successor')}</span>
            </div>
            <p class="successor-hint">{t('communities.governor.successorDesc')}</p>
          {/if}
        </div>

        <div class="governor-actions">
          <button
            class="btn-name-successor"
            onclick={() => showSuccessorModal = true}
            disabled={txPending}
          >
            {currentSuccessor ? t('communities.governor.successor') : t('communities.governor.successor')}
          </button>

          {#if currentSuccessor}
            <button
              class="btn-step-down"
              onclick={() => showStepDownModal = true}
              disabled={txPending}
            >
              {t('communities.governor.stepDownConfirm')}
            </button>
          {/if}
        </div>

        {#if txMessage && !showSuccessorModal && !showStepDownModal}
          <div class="tx-message {txMessage.type}">{txMessage.text}</div>
        {/if}
      </div>
    </div>
  {/if}

  <p class="value-footer">{t('communities.valueFooter')}</p>
</div>

<!-- Oracle Registration Modal -->
{#if showOracleModal}
  <div class="modal-overlay" onclick={() => showOracleModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <button class="modal-close" onclick={() => showOracleModal = false}>&times;</button>
      <h2>{t('communities.oracle.register')}</h2>
      <p class="modal-subtitle">
        {t('communities.oracle.description')}
      </p>

      <div class="oracle-info-box">
        <h4>{t('communities.oracle.responsibilities')}</h4>
        <ul>
          <li>{t('communities.oracle.responsibility1')}</li>
          <li>{t('communities.oracle.responsibility2')}</li>
          <li>{t('communities.oracle.responsibility3')}</li>
        </ul>
      </div>

      <div class="oracle-stake-info">
        <strong>{t('communities.oracle.stake')}</strong> {t('communities.oracle.stakeValue')}
      </div>

      {#if txMessage}
        <div class="tx-message {txMessage.type}">{txMessage.text}</div>
      {/if}

      <div class="modal-actions">
        <button
          class="btn-cancel"
          onclick={() => showOracleModal = false}
          disabled={txPending}
        >
          {t('common.cancel')}
        </button>
        <button
          class="btn-submit"
          onclick={registerAsOracle}
          disabled={txPending}
        >
          {#if txPending}
            {t('communities.oracle.registering')}
          {:else}
            {t('communities.oracle.register')}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Successor Designation Modal -->
{#if showSuccessorModal}
  <div class="modal-overlay" onclick={() => showSuccessorModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <button class="modal-close" onclick={() => showSuccessorModal = false}>&times;</button>
      <h2>{t('communities.governor.successor')}</h2>
      <p class="modal-subtitle">
        {t('communities.governor.successorDesc')}
      </p>

      <div class="form-group">
        <label>{t('communities.card.governor')} Address</label>
        <input
          type="text"
          bind:value={newSuccessorAddress}
          placeholder="G... (Stellar address)"
          disabled={txPending}
        />
        <small>{t('communities.governor.successorNote')}</small>
      </div>

      {#if txMessage}
        <div class="tx-message {txMessage.type}">{txMessage.text}</div>
      {/if}

      <div class="modal-actions">
        <button
          class="btn-cancel"
          onclick={() => { showSuccessorModal = false; newSuccessorAddress = ""; txMessage = null; }}
          disabled={txPending}
        >
          {t('common.cancel')}
        </button>
        <button
          class="btn-submit"
          onclick={designateSuccessor}
          disabled={txPending || !newSuccessorAddress.trim()}
        >
          {#if txPending}
            {t('communities.governor.steppingDown')}
          {:else}
            {t('communities.governor.successor')}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Step Down Modal -->
{#if showStepDownModal}
  <div class="modal-overlay" onclick={() => showStepDownModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <button class="modal-close" onclick={() => showStepDownModal = false}>&times;</button>
      <h2>{t('communities.governor.stepDownConfirm')}</h2>
      <p class="modal-subtitle">
        {t('communities.governor.stepDownDesc')}
      </p>

      <div class="step-down-info">
        <div class="info-row">
          <span>{t('communities.governor.successor')}:</span>
          <span class="successor-badge">{currentSuccessor?.slice(0, 8)}...{currentSuccessor?.slice(-6)}</span>
        </div>
        <p class="step-down-note">
          {t('communities.governor.stepDownDesc')}
        </p>
      </div>

      {#if txMessage}
        <div class="tx-message {txMessage.type}">{txMessage.text}</div>
      {/if}

      <div class="modal-actions">
        <button
          class="btn-cancel"
          onclick={() => showStepDownModal = false}
          disabled={txPending}
        >
          {t('common.cancel')}
        </button>
        <button
          class="btn-step-down-confirm"
          onclick={stepDownAsGovernor}
          disabled={txPending}
        >
          {#if txPending}
            {t('communities.governor.steppingDown')}
          {:else}
            {t('communities.governor.stepDownConfirm')}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

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

  /* Tooltip styles */
  .term-with-tooltip {
    position: relative;
    cursor: help;
    border-bottom: 1px dotted var(--color-text-muted);
  }

  .term-with-tooltip-inline {
    position: relative;
    cursor: help;
  }

  .tooltip-trigger {
    font-size: 0.7em;
    color: var(--color-primary);
    margin-left: 1px;
    font-weight: bold;
  }

  .tooltip-trigger-sm {
    font-size: 0.65em;
    color: var(--color-text-muted);
    margin-left: 1px;
    font-weight: bold;
  }

  .tooltip-content {
    visibility: hidden;
    opacity: 0;
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    background: var(--color-tooltip-bg);
    color: white;
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    font-weight: normal;
    line-height: 1.4;
    white-space: normal;
    width: max-content;
    max-width: 280px;
    z-index: 100;
    transition: opacity 0.2s, visibility 0.2s;
    box-shadow: var(--shadow-lg);
    pointer-events: none;
  }

  .tooltip-content::after {
    content: '';
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border: 6px solid transparent;
    border-top-color: var(--color-tooltip-bg);
  }

  .term-with-tooltip:hover .tooltip-content,
  .term-with-tooltip-inline:hover .tooltip-content {
    visibility: visible;
    opacity: 1;
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
    border: 1px solid var(--color-border-dark);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-base);
  }

  small {
    display: block;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    margin-top: var(--space-xs);
  }

  .rate-preview {
    background: var(--color-bg-subtle);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-sm);
    margin-bottom: var(--space-md);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .preview-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .preview-value {
    font-weight: 500;
    color: var(--color-primary);
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

  .communities-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: var(--space-lg);
    margin-bottom: var(--space-xl);
  }

  .community-card {
    background: var(--color-bg);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-lg);
    transition: all 0.2s;
  }

  .community-card:hover {
    border-color: var(--color-primary);
    transform: translateY(-2px);
    box-shadow: var(--shadow-md);
  }

  .community-card.in-community {
    border-color: var(--color-success);
    background: var(--color-success-light);
  }

  .community-header {
    display: flex;
    justify-content: space-between;
    align-items: start;
    margin-bottom: var(--space-md);
  }

  .community-header h3 {
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

  .community-stats {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-md);
    margin-bottom: var(--space-lg);
  }

  .community-stat {
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

  .stat-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .stat-address {
    font-family: monospace;
    font-size: var(--font-size-sm);
  }

  .rate-badge {
    background: var(--color-primary-light);
    color: var(--color-primary-text);
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
  }

  .community-actions button {
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
    background: var(--color-success-light);
    color: var(--color-success-text);
    cursor: not-allowed;
  }

  .info-box {
    background: var(--color-bg-subtle);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-lg);
    margin-bottom: var(--space-lg);
  }

  .info-box h3 {
    margin-top: 0;
    margin-bottom: var(--space-sm);
  }

  .info-box p {
    color: var(--color-text-muted);
    margin-bottom: var(--space-md);
    line-height: 1.6;
  }

  .info-box ul {
    margin: 0;
    padding-left: var(--space-lg);
    color: var(--color-text-muted);
  }

  .info-box li {
    margin-bottom: var(--space-xs);
  }

  .value-footer {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    margin-top: var(--space-lg);
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
  }

  .tx-message {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-md);
    font-size: var(--font-size-sm);
  }

  .tx-message-info {
    background: var(--color-info-light);
    color: var(--color-info-text);
    border: 1px solid var(--color-info);
  }

  .tx-message-success {
    background: var(--color-success-light);
    color: var(--color-success-text);
    border: 1px solid var(--color-success);
  }

  .tx-message-error {
    background: var(--color-error-light);
    color: var(--color-error-text);
    border: 1px solid var(--color-error);
  }

  .tx-message .icon {
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .tx-message .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
  }

  .btn-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    display: inline-block;
    margin-right: var(--space-xs);
  }

  .btn-dismiss {
    margin-left: auto;
    background: none;
    border: none;
    font-size: 1.25rem;
    cursor: pointer;
    padding: 0;
    color: inherit;
    opacity: 0.7;
    width: auto;
  }

  .btn-dismiss:hover {
    opacity: 1;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  @media (max-width: 640px) {
    .container {
      padding: var(--space-md);
    }

    .header-actions {
      flex-direction: column;
      gap: var(--space-md);
    }

    .communities-grid {
      grid-template-columns: 1fr;
    }

    .form-row {
      grid-template-columns: 1fr;
    }

    .tooltip-content {
      max-width: 200px;
      font-size: 0.75rem;
    }

    .rate-preview {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-xs);
    }
  }

  /* Oracle Section Styles */
  .oracle-section {
    margin-top: var(--space-xl);
    padding: var(--space-lg);
    background: white;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .oracle-section h2 {
    margin: 0 0 var(--space-md) 0;
    font-size: var(--font-size-xl);
  }

  .oracle-description {
    color: var(--color-text-muted);
    margin-bottom: var(--space-lg);
    line-height: 1.6;
  }

  .oracle-card {
    background: var(--color-warning-light);
    border: 1px solid var(--color-warning);
    border-radius: var(--radius-md);
    padding: var(--space-lg);
  }

  .oracle-info h4 {
    margin: 0 0 var(--space-sm) 0;
    color: var(--color-warning-text);
  }

  .oracle-info ul {
    margin: 0;
    padding-left: var(--space-lg);
    color: var(--color-warning-text);
  }

  .oracle-info li {
    margin-bottom: var(--space-xs);
  }

  .oracle-status {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-md);
  }

  .oracle-status.registered {
    background: var(--color-success-light);
    color: var(--color-success-text);
  }

  .oracle-hint {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .oracle-hint a {
    color: var(--color-primary);
  }

  .oracle-actions {
    text-align: center;
  }

  .btn-register-oracle {
    padding: var(--space-md) var(--space-lg);
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    transition: transform 0.2s, box-shadow 0.2s;
    width: auto;
  }

  .btn-register-oracle:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  }

  .oracle-note {
    margin-top: var(--space-md);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .connect-prompt {
    text-align: center;
    padding: var(--space-lg);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
    color: var(--color-text-muted);
  }

  /* Modal styles */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    padding: var(--space-md);
  }

  .modal {
    background: white;
    border-radius: var(--radius-lg);
    padding: var(--space-xl);
    max-width: 500px;
    width: 100%;
    position: relative;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  }

  .modal-close {
    position: absolute;
    top: var(--space-md);
    right: var(--space-md);
    background: none;
    border: none;
    font-size: 1.5rem;
    cursor: pointer;
    color: var(--color-text-muted);
    width: auto;
    padding: 0;
  }

  .modal-close:hover {
    color: var(--color-text);
  }

  .modal h2 {
    margin: 0 0 var(--space-xs) 0;
    color: var(--color-text);
  }

  .modal-subtitle {
    color: var(--color-text-muted);
    margin-bottom: var(--space-lg);
  }

  .oracle-info-box {
    background: var(--color-info-light);
    border: 1px solid var(--color-info);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-md);
  }

  .oracle-info-box h4 {
    margin: 0 0 var(--space-sm) 0;
    color: var(--color-info-text);
  }

  .oracle-info-box ul {
    margin: 0;
    padding-left: var(--space-lg);
    color: var(--color-info-text);
  }

  .oracle-stake-info {
    background: var(--color-warning-light);
    border: 1px solid var(--color-warning);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
    text-align: center;
  }

  .stake-note {
    display: block;
    font-size: var(--font-size-xs);
    color: var(--color-warning-text);
    margin-top: var(--space-xs);
  }

  .modal-actions {
    display: flex;
    gap: var(--space-md);
    justify-content: flex-end;
    margin-top: var(--space-lg);
  }

  .btn-cancel {
    padding: var(--space-sm) var(--space-lg);
    background: var(--color-border-light);
    color: var(--color-text);
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    width: auto;
  }

  .btn-cancel:hover:not(:disabled) {
    background: var(--color-border);
  }

  .btn-submit {
    padding: var(--space-sm) var(--space-lg);
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    width: auto;
  }

  .btn-submit:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-submit:hover:not(:disabled) {
    opacity: 0.9;
  }

  /* Governor Section Styles */
  .governor-section {
    margin-top: var(--space-xl);
    padding: var(--space-lg);
    background: white;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
  }

  .governor-section h2 {
    margin: 0 0 var(--space-md) 0;
    font-size: var(--font-size-xl);
  }

  .governor-description {
    color: var(--color-text-muted);
    margin-bottom: var(--space-lg);
    line-height: 1.6;
  }

  .governor-card {
    background: linear-gradient(135deg, var(--color-warning-light) 0%, var(--color-warning-light) 100%);
    border: 2px solid var(--color-warning);
    border-radius: var(--radius-md);
    padding: var(--space-lg);
  }

  .governor-status {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-md);
    background: white;
    border-radius: var(--radius-md);
    margin-bottom: var(--space-lg);
  }

  .governor-icon {
    font-size: 1.5rem;
  }

  .successor-info {
    background: rgba(255, 255, 255, 0.7);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
  }

  .successor-info h4 {
    margin: 0 0 var(--space-sm) 0;
    color: var(--color-warning-text);
    font-size: var(--font-size-sm);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .successor-address {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
  }

  .address-badge {
    font-family: monospace;
    background: white;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
    color: var(--color-warning-text);
  }

  .successor-note {
    font-size: var(--font-size-xs);
    color: var(--color-warning-text);
  }

  .no-successor {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    color: var(--color-warning-text);
  }

  .warning-icon {
    font-size: 1.25rem;
  }

  .successor-hint {
    margin: var(--space-sm) 0 0 0;
    font-size: var(--font-size-sm);
    color: var(--color-warning-text);
    font-style: italic;
  }

  .governor-actions {
    display: flex;
    gap: var(--space-md);
    flex-wrap: wrap;
  }

  .btn-name-successor {
    padding: var(--space-sm) var(--space-lg);
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    width: auto;
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .btn-name-successor:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  }

  .btn-step-down {
    padding: var(--space-sm) var(--space-lg);
    background: white;
    color: var(--color-warning-text);
    border: 2px solid var(--color-warning);
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    width: auto;
    transition: all 0.2s;
  }

  .btn-step-down:hover:not(:disabled) {
    background: var(--color-warning-light);
    border-color: var(--color-warning);
  }

  .step-down-info {
    background: var(--color-warning-light);
    border: 1px solid var(--color-warning);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-sm) 0;
  }

  .successor-badge {
    font-family: monospace;
    background: white;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: var(--font-size-sm);
  }

  .step-down-note {
    margin: var(--space-md) 0 0 0;
    font-size: var(--font-size-sm);
    color: var(--color-warning-text);
  }

  .btn-step-down-confirm {
    padding: var(--space-sm) var(--space-lg);
    background: linear-gradient(135deg, var(--color-warning) 0%, var(--color-warning) 100%);
    color: white;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    width: auto;
  }

  .btn-step-down-confirm:hover:not(:disabled) {
    opacity: 0.9;
  }
</style>
