<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";

  // Term definitions for tooltips
  const TERM_DEFINITIONS: Record<string, string> = {
    trust: "A community organization that manages its own demurrage settings. Members share the same rate and period configuration.",
    demurrage: "A holding cost that gradually reduces inactive balances. Designed to encourage active participation and circulation of KCHNG.",
    "annual rate": "The percentage of balance that would be burned over one year if the account remained completely inactive.",
    "basis points": "1 basis point = 0.01%. So 1200 basis points = 12%. Used for precise rate calculations.",
    federated: "Independent organizations that share a common protocol. Trusts can exchange tokens with each other at calculated rates.",
    period: "How often demurrage is calculated and applied. Shorter periods = more frequent, smaller burns. Longer periods = less frequent, larger burns.",
    oracle: "A trusted community member who can activate community protection periods for members facing hardship."
  };

  // Reusable tooltip component inline
  function TermTooltip(term: string) {
    const def = TERM_DEFINITIONS[term.toLowerCase()];
    if (!def) return term;
    return `<span class="term-with-tooltip">${term}<sup class="tooltip-trigger">?</sup><span class="tooltip-content">${def}</span></span>`;
  }

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

  // New trust form - defaults: 12% annual rate, 28 day period
  let newTrustName = $state("");
  let newTrustRatePercent = $state(12); // User enters percentage (5-15)
  let newTrustPeriod = $state(28); // Default 28 days

  // Transaction state
  let txPending = $state(false);
  let txMessage = $state<{ type: "success" | "error" | "info"; text: string } | null>(null);

  // Oracle state
  let isOracle = $state(false);
  let oracleLoading = $state(false);
  let showOracleModal = $state(false);

  // Governor management state
  let isGovernor = $state(false);
  let governorTrustId = $state<string | null>(null);
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
      txMessage = { type: "error", text: "Please connect your wallet first" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Registering as Community Oracle..." };

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
        text: e instanceof Error ? e.message : "Failed to register as oracle"
      };
    } finally {
      txPending = false;
    }
  }

  async function designateSuccessor() {
    if (!$wallet.connected || !$wallet.address || !newSuccessorAddress.trim()) {
      txMessage = { type: "error", text: "Please enter a valid successor address" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Naming your successor..." };

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
        text: e instanceof Error ? e.message : "Failed to designate successor"
      };
    } finally {
      txPending = false;
    }
  }

  async function stepDownAsGovernor() {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: "Please connect your wallet first" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Passing the torch..." };

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
      governorTrustId = null;
      currentSuccessor = null;
      showStepDownModal = false;

      // Refresh data
      await loadData();

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to step down"
      };
    } finally {
      txPending = false;
    }
  }

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

        // Check if user is a governor of any trust
        for (const trust of trustData) {
          if (trust.governor === $wallet.address) {
            isGovernor = true;
            governorTrustId = trust.id;
            // Get full trust info to check successor
            const fullTrustInfo = await kchngClient.getTrustInfo(trust.id);
            currentSuccessor = fullTrustInfo.successor;
            break;
          }
        }
      }

      loading = false;
    } catch (e) {
      console.error("Failed to load trusts:", e);
      loading = false;
    }
  }

  async function createTrust() {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: "Please connect your wallet first" };
      return;
    }

    if (!newTrustName.trim()) {
      txMessage = { type: "error", text: "Please enter a trust name" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Preparing transaction..." };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Set up the signing callback
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      // Convert percentage to basis points (e.g., 12% = 1200 bps)
      const annualRateBps = newTrustRatePercent * 100;

      // Create the trust
      const txHash = await kchngClient.registerTrust(
        $wallet.address!,
        newTrustName,
        annualRateBps,
        newTrustPeriod
      );

      txMessage = {
        type: "success",
        text: `Trust created! Transaction: ${txHash.slice(0, 8)}...`
      };

      showCreateForm = false;
      newTrustName = "";
      newTrustRatePercent = 12;
      newTrustPeriod = 28;

      // Refresh data
      await loadData();
      await wallet.refreshTrustStatus();

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to create trust"
      };
    } finally {
      txPending = false;
    }
  }

  async function joinTrust(trustId: string) {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: "Please connect your wallet first" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Preparing transaction..." };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Set up the signing callback
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      // Join the trust
      const txHash = await kchngClient.joinTrust(trustId, $wallet.address);

      txMessage = {
        type: "success",
        text: `Joined trust! Transaction: ${txHash.slice(0, 8)}...`
      };

      // Refresh data
      await loadData();
      await wallet.refreshTrustStatus();

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to join trust"
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
  <title>Community Trusts | KCHNG</title>
</svelte:head>

<div class="container">
  <h1>Community Trusts</h1>

  <div class="header-actions">
    <p class="subtitle">
      <span class="term-with-tooltip">
        Federated<sup class="tooltip-trigger">?</sup>
        <span class="tooltip-content">{TERM_DEFINITIONS['federated']}</span>
      </span>
      community organizations with custom
      <span class="term-with-tooltip">
        demurrage<sup class="tooltip-trigger">?</sup>
        <span class="tooltip-content">{TERM_DEFINITIONS['demurrage']}</span>
      </span>
      rates
    </p>
    {#if !showCreateForm}
      <button class="btn-create" onclick={() => showCreateForm = true}>
        + Create New Trust
      </button>
    {/if}
  </div>

  {#if showCreateForm}
    <div class="create-form">
      <h2>Create New <span class="term-with-tooltip">
        Trust<sup class="tooltip-trigger">?</sup>
        <span class="tooltip-content">{TERM_DEFINITIONS['trust']}</span>
      </span></h2>
      <div class="form-group">
        <label>Trust Name</label>
        <input type="text" bind:value={newTrustName} placeholder="e.g., Urban Elder Care Trust" disabled={txPending} />
      </div>
      <div class="form-row">
        <div class="form-group">
          <label>
            <span class="term-with-tooltip">
              Annual Rate<sup class="tooltip-trigger">?</sup>
              <span class="tooltip-content">{TERM_DEFINITIONS['annual rate']}</span>
            </span>
            (%)
          </label>
          <input type="number" bind:value={newTrustRatePercent} min="5" max="15" step="0.5" inputmode="decimal" disabled={txPending} />
          <small>Protocol limits: 5% - 15% annually (default: 12%)</small>
        </div>
        <div class="form-group">
          <label>
            <span class="term-with-tooltip">
              Period<sup class="tooltip-trigger">?</sup>
              <span class="tooltip-content">{TERM_DEFINITIONS['period']}</span>
            </span>
            (days)
          </label>
          <input type="number" bind:value={newTrustPeriod} min="7" max="365" inputmode="numeric" pattern="[0-9]*" disabled={txPending} />
          <small>7 - 365 days (default: 28)</small>
        </div>
      </div>

      <div class="rate-preview">
        <span class="preview-label">Per-period demurrage:</span>
        <span class="preview-value">{periodRatePercent(newTrustRatePercent * 100, newTrustPeriod)} every {newTrustPeriod} days</span>
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
        <button onclick={createTrust} disabled={txPending}>
          {#if txPending}
            <span class="btn-spinner"></span>
            Creating...
          {:else}
            Create Trust
          {/if}
        </button>
        <button class="btn-cancel" onclick={() => { showCreateForm = false; txMessage = null; }} disabled={txPending}>Cancel</button>
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
              <span class="stat-label">
                <span class="term-with-tooltip-inline">
                  Demurrage<sup class="tooltip-trigger-sm">?</sup>
                  <span class="tooltip-content">{TERM_DEFINITIONS['demurrage']}</span>
                </span>
                Rate
              </span>
              <span class="stat-value rate-badge">{rateToPercentage(trust.annual_rate_bps)}/year</span>
              <span class="stat-sub">({periodRatePercent(trust.annual_rate_bps, trust.demurrage_period_days)}/{trust.demurrage_period_days}d)</span>
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
              <button
                class="btn-join"
                onclick={() => joinTrust(trust.id)}
                disabled={txPending}
              >
                {#if txPending}
                  <span class="btn-spinner"></span>
                  Joining...
                {:else}
                  Join Trust
                {/if}
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}

  <div class="info-box">
    <h3>About Trusts</h3>
    <p>
      <span class="term-with-tooltip">
        Trusts<sup class="tooltip-trigger">?</sup>
        <span class="tooltip-content">{TERM_DEFINITIONS['trust']}</span>
      </span>
      are
      <span class="term-with-tooltip">
        federated<sup class="tooltip-trigger">?</sup>
        <span class="tooltip-content">{TERM_DEFINITIONS['federated']}</span>
      </span>
      community organizations that set their own
      <span class="term-with-tooltip">
        demurrage<sup class="tooltip-trigger">?</sup>
        <span class="tooltip-content">{TERM_DEFINITIONS['demurrage']}</span>
      </span>
      rates within protocol bounds (5-15% annually). Each trust is governed by a designated governor who manages membership and can propose rate changes.
    </p>
    <ul>
      <li><strong>Rate Range:</strong> 5% - 15% annual (protocol enforced)</li>
      <li><strong>Membership:</strong> Open to anyone, join via trust interface</li>
      <li><strong>Governance:</strong> Governor can propose rate changes via community vote</li>
      <li><strong>Cross-Trust:</strong> Exchange tokens between trusts at calculated rates</li>
    </ul>
  </div>

  <!-- Community Oracle Section -->
  <div class="oracle-section">
    <h2>🛡️ Community Oracles</h2>
    <p class="oracle-description">
      Community Oracles help protect members in need by activating community protection periods during emergencies,
      illness, or other hardship situations.
    </p>

    <div class="oracle-card">
      <div class="oracle-info">
        <h4>Oracle Responsibilities:</h4>
        <ul>
          <li>Verify member hardship claims (emergency, illness, etc.)</li>
          <li>Activate community protection periods for verified members</li>
          <li>Help maintain trust in the system</li>
          <li>Contribute to community wellbeing and mutual aid</li>
        </ul>
      </div>

      {#if oracleLoading}
        <div class="oracle-loading">Checking oracle status...</div>
      {:else if isOracle}
        <div class="oracle-status registered">
          <span class="status-icon">✓</span>
          <span class="status-text">You are a registered Community Oracle</span>
        </div>
        <p class="oracle-hint">
          You can activate community protection for members from your <a href="/dashboard">dashboard</a>.
        </p>
      {:else if $wallet.connected}
        <div class="oracle-actions">
          <button
            class="btn-register-oracle"
            onclick={() => showOracleModal = true}
          >
            Become a Community Oracle
          </button>
          <p class="oracle-note">
            Requires a stake commitment to ensure oracle accountability.
          </p>
        </div>
      {:else}
        <div class="connect-prompt">
          <p>Connect your wallet to register as a Community Oracle</p>
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
      <h2>🏛️ Trust Governor</h2>
      <p class="governor-description">
        As the governor of your trust, you can name a successor and pass the torch when you're ready.
      </p>

      <div class="governor-card">
        <div class="governor-status">
          <span class="status-icon governor-icon">👑</span>
          <span class="status-text">You are the Governor of this trust</span>
        </div>

        <div class="successor-info">
          <h4>Named Successor:</h4>
          {#if currentSuccessor}
            <div class="successor-address">
              <span class="address-badge">{currentSuccessor.slice(0, 8)}...{currentSuccessor.slice(-6)}</span>
              <span class="successor-note">Ready to take over when you pass the torch</span>
            </div>
          {:else}
            <div class="no-successor">
              <span class="warning-icon">⚠️</span>
              <span>No successor named yet</span>
            </div>
            <p class="successor-hint">Name a trusted member to ensure smooth leadership transition.</p>
          {/if}
        </div>

        <div class="governor-actions">
          <button
            class="btn-name-successor"
            onclick={() => showSuccessorModal = true}
            disabled={txPending}
          >
            {currentSuccessor ? 'Change Successor' : 'Name Your Successor'}
          </button>

          {#if currentSuccessor}
            <button
              class="btn-step-down"
              onclick={() => showStepDownModal = true}
              disabled={txPending}
            >
              Pass the Torch
            </button>
          {/if}
        </div>

        {#if txMessage && !showSuccessorModal && !showStepDownModal}
          <div class="tx-message {txMessage.type}">{txMessage.text}</div>
        {/if}
      </div>
    </div>
  {/if}

  <p class="value-footer">Protocol: 30 min verified work → 1,000 KCHNG minted. Social peg: 1,000 KCHNG ≈ 1 meal.</p>
</div>

<!-- Oracle Registration Modal -->
{#if showOracleModal}
  <div class="modal-overlay" onclick={() => showOracleModal = false}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <button class="modal-close" onclick={() => showOracleModal = false}>&times;</button>
      <h2>Become a Community Oracle</h2>
      <p class="modal-subtitle">
        Help protect community members in need by activating grace periods during hardship.
      </p>

      <div class="oracle-info-box">
        <h4>What you'll do:</h4>
        <ul>
          <li>Verify member hardship claims (emergency, illness, etc.)</li>
          <li>Activate community protection periods</li>
          <li>Help maintain trust in the system</li>
        </ul>
      </div>

      <div class="oracle-stake-info">
        <strong>Stake Required:</strong> 50,000 KCHNG
        <span class="stake-note">(≈ 50 meals - returned when you step down)</span>
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
          Cancel
        </button>
        <button
          class="btn-submit"
          onclick={registerAsOracle}
          disabled={txPending}
        >
          {#if txPending}
            Registering...
          {:else}
            Step Up for Your Community
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
      <h2>Name Your Successor</h2>
      <p class="modal-subtitle">
        Choose a trusted member to take over as governor when you're ready to step down.
      </p>

      <div class="form-group">
        <label>Successor Address</label>
        <input
          type="text"
          bind:value={newSuccessorAddress}
          placeholder="G... (Stellar address)"
          disabled={txPending}
        />
        <small>The successor must be a member of your trust.</small>
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
          Cancel
        </button>
        <button
          class="btn-submit"
          onclick={designateSuccessor}
          disabled={txPending || !newSuccessorAddress.trim()}
        >
          {#if txPending}
            Naming...
          {:else}
            Name Successor
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
      <h2>Pass the Torch</h2>
      <p class="modal-subtitle">
        Transfer governorship to your named successor. This action cannot be undone.
      </p>

      <div class="step-down-info">
        <div class="info-row">
          <span>Current Successor:</span>
          <span class="successor-badge">{currentSuccessor?.slice(0, 8)}...{currentSuccessor?.slice(-6)}</span>
        </div>
        <p class="step-down-note">
          Once you pass the torch, your successor will become the new governor and you will become a regular member.
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
          Cancel
        </button>
        <button
          class="btn-step-down-confirm"
          onclick={stepDownAsGovernor}
          disabled={txPending}
        >
          {#if txPending}
            Passing...
          {:else}
            Pass the Torch
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
    background: #1f2937;
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
    border-top-color: #1f2937;
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

  .stat-sub {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
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
    background: #dbeafe;
    color: #1e40af;
    border: 1px solid #93c5fd;
  }

  .tx-message-success {
    background: #d1fae5;
    color: #065f46;
    border: 1px solid #6ee7b7;
  }

  .tx-message-error {
    background: #fee2e2;
    color: #991b1b;
    border: 1px solid #fca5a5;
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

    .trusts-grid {
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
    background: #fef3c7;
    border: 1px solid #fbbf24;
    border-radius: var(--radius-md);
    padding: var(--space-lg);
  }

  .oracle-info h4 {
    margin: 0 0 var(--space-sm) 0;
    color: #92400e;
  }

  .oracle-info ul {
    margin: 0;
    padding-left: var(--space-lg);
    color: #78350f;
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
    background: #d1fae5;
    color: #065f46;
  }

  .oracle-hint {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
  }

  .oracle-hint a {
    color: #667eea;
  }

  .oracle-actions {
    text-align: center;
  }

  .btn-register-oracle {
    padding: var(--space-md) var(--space-lg);
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
    background: #f9fafb;
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
    background: #dbeafe;
    border: 1px solid #93c5fd;
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-md);
  }

  .oracle-info-box h4 {
    margin: 0 0 var(--space-sm) 0;
    color: #1e40af;
  }

  .oracle-info-box ul {
    margin: 0;
    padding-left: var(--space-lg);
    color: #1e40af;
  }

  .oracle-stake-info {
    background: #fef3c7;
    border: 1px solid #fbbf24;
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
    text-align: center;
  }

  .stake-note {
    display: block;
    font-size: var(--font-size-xs);
    color: #92400e;
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
    background: #f3f4f6;
    color: #374151;
    border: none;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    width: auto;
  }

  .btn-cancel:hover:not(:disabled) {
    background: #e5e7eb;
  }

  .btn-submit {
    padding: var(--space-sm) var(--space-lg);
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
    background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
    border: 2px solid #fbbf24;
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
    color: #92400e;
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
    color: #78350f;
  }

  .successor-note {
    font-size: var(--font-size-xs);
    color: #92400e;
  }

  .no-successor {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    color: #92400e;
  }

  .warning-icon {
    font-size: 1.25rem;
  }

  .successor-hint {
    margin: var(--space-sm) 0 0 0;
    font-size: var(--font-size-sm);
    color: #78350f;
    font-style: italic;
  }

  .governor-actions {
    display: flex;
    gap: var(--space-md);
    flex-wrap: wrap;
  }

  .btn-name-successor {
    padding: var(--space-sm) var(--space-lg);
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
    color: #78350f;
    border: 2px solid #fbbf24;
    border-radius: var(--radius-sm);
    font-weight: 500;
    cursor: pointer;
    width: auto;
    transition: all 0.2s;
  }

  .btn-step-down:hover:not(:disabled) {
    background: #fef3c7;
    border-color: #f59e0b;
  }

  .step-down-info {
    background: #fef3c7;
    border: 1px solid #fbbf24;
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
    color: #78350f;
  }

  .btn-step-down-confirm {
    padding: var(--space-sm) var(--space-lg);
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
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
