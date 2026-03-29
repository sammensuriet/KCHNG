<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";
  import { t } from "$lib/i18n";
  import { ProposalStatus } from "@kchng/shared";

  let activeTab = $state<"proposals" | "create">("proposals");

  let proposals = $state<Array<{
    proposal_id: number;
    proposer: string;
    proposal_type: number;
    title: string;
    description: string;
    trust_id: string | null;
    new_rate_bps: number | null;
    created_at: number;
    review_end: number;
    vote_end: number;
    implementation_date: number;
    status: number;
    votes_for: number;
    votes_against: number;
    voters: string[];
  }>>([]);

  let loading = $state(false);

  // Create proposal form
  let proposalType = $state(0);
  let proposalTitle = $state("");
  let proposalDescription = $state("");
  let newRateBps = $state(1000);

  // Voting state
  let votingProposalId = $state<number | null>(null);
  let votingSupport = $state<boolean | null>(null);
  let txPending = $state(false);
  let txMessage = $state<{ type: "success" | "error" | "info"; text: string } | null>(null);

  const proposalTypes = [
    { value: 0, label: "governance.types.rateChange" },
    { value: 1, label: "governance.types.communityParameters" },
    { value: 2, label: "governance.types.protocolUpgrade" },
    { value: 3, label: "governance.types.emergency" },
  ];

  onMount(async () => {
    await loadProposals();
  });

  async function loadProposals() {
    loading = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Get all proposals
      const proposalIds = await kchngClient.getAllProposals();
      const proposalData = await Promise.all(
        proposalIds.map(async (id) => {
          const info = await kchngClient.getProposal(Number(id));
          return {
            proposal_id: Number(info.proposal_id),
            proposer: info.proposer,
            proposal_type: Number(info.proposal_type),
            title: info.title,
            description: info.description,
            trust_id: info.trust_id,
            new_rate_bps: info.new_rate_bps ? Number(info.new_rate_bps) : null,
            created_at: Number(info.created_at),
            review_end: Number(info.review_end),
            vote_end: Number(info.vote_end),
            implementation_date: Number(info.implementation_date),
            status: Number(info.status),
            votes_for: Number(info.votes_for),
            votes_against: Number(info.votes_against),
            voters: info.voters || [],
          };
        })
      );

      proposals = proposalData.sort((a, b) => b.created_at - a.created_at);
      loading = false;
    } catch (e) {
      console.error("Failed to load proposals:", e);
      loading = false;
    }
  }

  async function createProposal() {
    if (!$wallet.connected) {
      alert("Please connect your wallet first");
      return;
    }

    if (!proposalTitle.trim() || !proposalDescription.trim()) {
      alert("Please fill in all required fields");
      return;
    }

    try {
      alert(`Proposal creation requires transaction signing.\n\nType: ${proposalTypes[proposalType].label}\nTitle: ${proposalTitle}\n\nThis will initiate a 7-day review period followed by a 3-day voting period.`);
    } catch (e) {
      alert("Failed to create proposal: " + (e instanceof Error ? e.message : "Unknown error"));
    }
  }

  function openVotingModal(proposalId: number) {
    votingProposalId = proposalId;
    votingSupport = null;
    txMessage = null;
  }

  function closeVotingModal() {
    votingProposalId = null;
    votingSupport = null;
    txMessage = null;
  }

  async function castVote() {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: "Please connect your wallet first" };
      return;
    }

    if (votingSupport === null || votingProposalId === null) {
      txMessage = { type: "error", text: "Please select Support or Oppose" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Preparing your vote..." };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const txHash = await kchngClient.voteOnProposal(
        $wallet.address,
        votingProposalId,
        votingSupport
      );

      txMessage = {
        type: "success",
        text: `Vote cast successfully! Transaction: ${txHash.slice(0, 8)}...`
      };

      // Refresh proposals after successful vote
      await loadProposals();

      // Close modal after a short delay
      setTimeout(() => closeVotingModal(), 2000);

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to cast vote"
      };
    } finally {
      txPending = false;
    }
  }

  function hasVoted(proposal: { voters: string[] }): boolean {
    if (!$wallet.address) return false;
    return proposal.voters.includes($wallet.address);
  }

  function canVote(proposal: { status: number; voters: string[] }): boolean {
    // Can only vote on proposals in voting phase (status 1)
    if (proposal.status !== 1) return false;
    // Can't vote twice
    if (hasVoted(proposal)) return false;
    // Must be connected
    if (!$wallet.connected) return false;
    return true;
  }

  function canProcess(proposal: { status: number; review_end: number; vote_end: number }): boolean {
    // Can process review -> voting (status 0, review_end passed)
    if (proposal.status === 0) {
      const now = Math.floor(Date.now() / 1000);
      return now >= proposal.review_end;
    }
    // Can process voting -> approved/rejected (status 1, vote_end passed)
    if (proposal.status === 1) {
      const now = Math.floor(Date.now() / 1000);
      return now >= proposal.vote_end;
    }
    return false;
  }

  function canImplement(proposal: { status: number; implementation_date: number }): boolean {
    // Can implement approved proposals (status 2) after implementation date
    if (proposal.status !== 2) return false;
    const now = Math.floor(Date.now() / 1000);
    return now >= proposal.implementation_date;
  }

  async function processProposal(proposalId: number) {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: "Please connect your wallet first" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Making it official..." };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const txHash = await kchngClient.processProposal($wallet.address, proposalId);

      txMessage = {
        type: "success",
        text: `Proposal processed! Transaction: ${txHash.slice(0, 8)}...`
      };

      // Refresh proposals
      await loadProposals();

      // Clear message after delay
      setTimeout(() => txMessage = null, 3000);

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to process proposal"
      };
    } finally {
      txPending = false;
    }
  }

  async function implementProposal(proposalId: number) {
    if (!$wallet.connected || !$wallet.address) {
      txMessage = { type: "error", text: "Please connect your wallet first" };
      return;
    }

    txPending = true;
    txMessage = { type: "info", text: "Implementing proposal..." };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const txHash = await kchngClient.implementProposal($wallet.address, proposalId);

      txMessage = {
        type: "success",
        text: `Proposal implemented! Changes are now active. Transaction: ${txHash.slice(0, 8)}...`
      };

      // Refresh proposals
      await loadProposals();

      // Clear message after delay
      setTimeout(() => txMessage = null, 3000);

    } catch (e) {
      txMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to implement proposal"
      };
    } finally {
      txPending = false;
    }
  }

  function getProposalTypeName(type: number): string {
    const key = proposalTypes.find(p => p.value === type)?.label;
    return key ? t(key) : "Unknown";
  }

  function getStatusName(status: number): string {
    const statuses = [
      "governance.status.review",
      "governance.status.voting",
      "governance.status.approved",
      "governance.status.rejected",
      "governance.status.implemented",
      "governance.status.expired"
    ];
    return statuses[status] ? t(statuses[status]) : "Unknown";
  }

  function getStatusClass(status: number): string {
    const classes = ["review", "voting", "approved", "rejected", "implemented", "expired"];
    return classes[status] || "";
  }

  function formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleDateString();
  }

  function timeRemaining(endDate: number): string {
    const now = Math.floor(Date.now() / 1000);
    const remaining = endDate - now;
    if (remaining <= 0) return "Ended";
    const days = Math.floor(remaining / 86400);
    return `${days} day${days !== 1 ? 's' : ''}`;
  }
</script>

<svelte:head>
  <title>{t('governance.title')}</title>
</svelte:head>

<div class="container">
  <h1>{t('governance.heading')}</h1>
  <p class="subtitle">{t('governance.subtitle')}</p>

  <div class="tabs">
    <button
      class:active={activeTab === "proposals"}
      onclick={() => activeTab = "proposals"}
    >
      {t('governance.tabProposals')}
    </button>
    <button
      class:active={activeTab === "create"}
      onclick={() => activeTab = "create"}
    >
      {t('governance.tabCreate')}
    </button>
  </div>

  {#if activeTab === "proposals"}
    <div class="tab-content">
      <div class="info-banner">
        <strong>{t('governance.process.heading')}</strong> {t('governance.process.description')}
      </div>

      {#if loading}
        <div class="loading">Loading proposals...</div>
      {:else if proposals.length === 0}
        <div class="empty-state">
          <div class="empty-icon">🗳️</div>
          <h3>{t('governance.empty.title')}</h3>
          <p>{t('governance.empty.description')}</p>
          <button onclick={() => activeTab = "create"}>{t('governance.empty.action')}</button>
        </div>
      {:else}
        <div class="proposals-list">
          {#each proposals as proposal (proposal.proposal_id)}
            <div class="proposal-card">
              <div class="proposal-header">
                <div class="proposal-title-row">
                  <h3>{proposal.title}</h3>
                  <span class="proposal-status status-{proposal.status}">
                    {getStatusName(proposal.status)}
                  </span>
                </div>
                <div class="proposal-meta">
                  <span class="proposal-type">{getProposalTypeName(proposal.proposal_type)}</span>
                  <span class="proposal-id">{t('governance.proposalId').replace('{id}', String(proposal.proposal_id))}</span>
                  <span class="proposal-date">{formatDate(proposal.created_at)}</span>
                </div>
              </div>

              <p class="proposal-description">{proposal.description}</p>

              {#if proposal.new_rate_bps}
                <div class="proposal-rate">
                  <strong>{t('governance.proposedRate')}</strong> {(proposal.new_rate_bps / 100).toFixed(1)}%
                </div>
              {/if}

              <div class="proposal-timeline">
                {#if proposal.status === 0}
                  <div class="timeline-item">
                    <span class="timeline-label">Review Period:</span>
                    <span class="timeline-value">{timeRemaining(proposal.review_end)} remaining</span>
                  </div>
                {:else if proposal.status === 1}
                  <div class="timeline-item">
                    <span class="timeline-label">Voting Period:</span>
                    <span class="timeline-value">{timeRemaining(proposal.vote_end)} remaining</span>
                  </div>
                {:else if proposal.status === 2}
                  <div class="timeline-item">
                    <span class="timeline-label">Implementation:</span>
                    <span class="timeline-value">{formatDate(proposal.implementation_date)}</span>
                  </div>
                {/if}
              </div>

              <div class="proposal-votes">
                <div class="vote-box vote-for">
                  <div class="vote-count">{proposal.votes_for}</div>
                  <div class="vote-label">For</div>
                </div>
                <div class="vote-box vote-against">
                  <div class="vote-count">{proposal.votes_against}</div>
                  <div class="vote-label">Against</div>
                </div>
                {#if proposal.status === 1}
                  {#if hasVoted(proposal)}
                    <span class="already-voted">{t('governance.alreadyVoted')}</span>
                  {:else if $wallet.connected}
                    <button
                      class="btn-vote"
                      onclick={() => openVotingModal(proposal.proposal_id)}
                    >
                      {t('governance.castVote')}
                    </button>
                  {:else}
                    <span class="connect-hint">{t('governance.connectWalletToVote')}</span>
                  {/if}
                {/if}
              </div>

              <!-- Proposal Actions -->
              {#if canProcess(proposal) || canImplement(proposal)}
                <div class="proposal-actions">
                  {#if canProcess(proposal)}
                    <button
                      class="btn-process"
                      onclick={() => processProposal(proposal.proposal_id)}
                      disabled={txPending}
                    >
                      {#if txPending}
                        <span class="btn-spinner"></span>
                      {/if}
                      {t('governance.makeOfficial')}
                    </button>
                    <p class="action-hint">
                      {#if proposal.status === 0}
                        {t('governance.processToVoting')}
                      {:else if proposal.status === 1}
                        {t('governance.processToTally')}
                      {/if}
                    </p>
                  {/if}

                  {#if canImplement(proposal)}
                    <div class="implementation-ready">
                      <div class="impact-preview">
                        <h4>Community Impact:</h4>
                        {#if proposal.new_rate_bps}
                          <p class="impact-text">
                            📊 Demurrage rate will change from current to <strong>{(proposal.new_rate_bps / 100).toFixed(1)}%</strong> annually.
                          </p>
                          <p class="impact-note">
                            This affects how inactive balances decay, promoting circulation in your community.
                          </p>
                        {:else}
                          <p class="impact-text">
                            📋 This proposal will enact changes to community parameters.
                          </p>
                        {/if}
                      </div>
                      <button
                        class="btn-implement"
                        onclick={() => implementProposal(proposal.proposal_id)}
                        disabled={txPending}
                      >
                        {#if txPending}
                          <span class="btn-spinner"></span>
                        {/if}
                        {t('governance.implementNow')}
                      </button>
                    </div>
                  {/if}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      {/if}
    </div>

  {:else if activeTab === "create"}
    <div class="tab-content">
      <div class="form-card">
        <h2>{t('governance.create.heading')}</h2>

        <div class="form-group">
          <label>{t('governance.create.proposalType')}</label>
          <select bind:value={proposalType}>
            {#each proposalTypes as type}
              <option value={type.value}>{t(type.label)}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label>{t('governance.create.title')}</label>
          <input type="text" bind:value={proposalTitle} placeholder="Brief description of your proposal" />
        </div>

        <div class="form-group">
          <label>{t('governance.create.description')}</label>
          <textarea
            bind:value={proposalDescription}
            placeholder="Detailed explanation of the proposal and its rationale"
            rows="5"
          ></textarea>
        </div>

        {#if proposalType === 0}
          <div class="form-group">
            <label>{t('governance.create.newRate')}</label>
            <input type="number" bind:value={newRateBps} min="500" max="1500" step="100" />
            <small>{t('governance.create.rateLimits')}</small>
          </div>
        {/if}

        <div class="info-box">
          <h4>{t('governance.create.timeline')}</h4>
          <ul>
            <li>{t('governance.create.reviewPeriod')}</li>
            <li>{t('governance.create.votingPeriod')}</li>
            <li>{t('governance.create.implementation')}</li>
          </ul>
        </div>

        <button onclick={createProposal}>{t('governance.create.submit')}</button>
      </div>
    </div>
  {/if}

  <p class="value-footer">{t('governance.valueFooter')}</p>
</div>

<!-- Voting Modal -->
{#if votingProposalId !== null}
  <div class="modal-overlay" onclick={closeVotingModal}>
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <button class="modal-close" onclick={closeVotingModal}>&times;</button>
      <h2>{t('governance.voteModal.title')}</h2>
      <p class="modal-subtitle">
        {#each proposals as p}
          {#if p.proposal_id === votingProposalId}
            <strong>{p.title}</strong>
          {/if}
        {/each}
      </p>

      <div class="vote-options">
        <button
          class="vote-option support"
          class:selected={votingSupport === true}
          onclick={() => votingSupport = true}
          disabled={txPending}
        >
          <span class="vote-icon">👍</span>
          <span class="vote-text">{t('governance.voteModal.support')}</span>
          <span class="vote-desc">{t('governance.voteModal.supportDesc')}</span>
        </button>
        <button
          class="vote-option oppose"
          class:selected={votingSupport === false}
          onclick={() => votingSupport = false}
          disabled={txPending}
        >
          <span class="vote-icon">👎</span>
          <span class="vote-text">{t('governance.voteModal.oppose')}</span>
          <span class="vote-desc">{t('governance.voteModal.opposeDesc')}</span>
        </button>
      </div>

      {#if txMessage}
        <div class="tx-message {txMessage.type}">{txMessage.text}</div>
      {/if}

      <div class="modal-actions">
        <button
          class="btn-cancel"
          onclick={closeVotingModal}
          disabled={txPending}
        >
          {t('governance.voteModal.cancel')}
        </button>
        <button
          class="btn-submit"
          onclick={castVote}
          disabled={votingSupport === null || txPending}
        >
          {#if txPending}
            {t('governance.voteModal.submitting')}
          {:else}
            {t('governance.voteModal.submit')}
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .container {
    max-width: 1000px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  .subtitle {
    color: var(--color-text-muted);
    margin: 0 0 2rem 0;
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 2rem;
    border-bottom: 1px solid var(--color-border);
  }

  .tabs button {
    padding: 0.75rem 1.5rem;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-weight: 500;
    color: var(--color-text-muted);
    transition: all 0.2s;
  }

  .tabs button.active {
    color: var(--color-primary);
    border-bottom-color: var(--color-primary);
  }

  .tab-content {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .info-banner {
    background: var(--color-info-light);
    border: 1px solid var(--color-info);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 2rem;
    color: var(--color-info-text);
  }

  .loading, .empty-state {
    text-align: center;
    padding: 3rem;
    background: var(--color-bg-subtle);
    border-radius: 8px;
  }

  .empty-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .proposals-list {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .proposal-card {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    padding: 1.5rem;
  }

  .proposal-header {
    margin-bottom: 1rem;
  }

  .proposal-title-row {
    display: flex;
    justify-content: space-between;
    align-items: start;
    margin-bottom: 0.5rem;
  }

  .proposal-title-row h3 {
    margin: 0;
    font-size: 1.25rem;
  }

  .proposal-status {
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 500;
    white-space: nowrap;
  }

  .status-review { background: var(--color-warning-light); color: var(--color-warning-text); }
  .status-voting { background: var(--color-info-light); color: var(--color-info-text); }
  .status-approved { background: var(--color-success-light); color: var(--color-success-text); }
  .status-rejected { background: var(--color-error-light); color: var(--color-error-text); }
  .status-implemented { background: var(--color-primary-light); color: var(--color-primary-text); }
  .status-expired { background: var(--color-border); color: var(--color-text); }

  .proposal-meta {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    font-size: 0.875rem;
  }

  .proposal-type {
    background: var(--color-primary-light);
    color: var(--color-primary-text);
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-weight: 500;
  }

  .proposal-id, .proposal-date {
    color: var(--color-text-muted);
  }

  .proposal-description {
    color: var(--color-text);
    line-height: 1.6;
    margin: 1rem 0;
  }

  .proposal-rate {
    background: var(--color-border-light);
    padding: 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .proposal-timeline {
    display: flex;
    gap: 1.5rem;
    margin-bottom: 1rem;
  }

  .timeline-item {
    font-size: 0.875rem;
  }

  .timeline-label {
    color: var(--color-text-muted);
  }

  .timeline-value {
    font-weight: 500;
  }

  .proposal-votes {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .vote-box {
    flex: 1;
    text-align: center;
    padding: 1rem;
    border-radius: 8px;
  }

  .vote-for {
    background: var(--color-success-light);
  }

  .vote-against {
    background: var(--color-error-light);
  }

  .vote-count {
    font-size: 1.5rem;
    font-weight: 700;
  }

  .vote-label {
    font-size: 0.875rem;
    color: var(--color-text-muted);
  }

  .btn-vote {
    padding: 0.5rem 1rem;
    background: var(--color-primary);
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-vote:hover {
    background: var(--color-primary-dark);
  }

  .already-voted {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.5rem 1rem;
    background: var(--color-success-light);
    color: var(--color-success-text);
    border-radius: 6px;
    font-weight: 500;
    font-size: 0.875rem;
  }

  .connect-hint {
    padding: 0.5rem 1rem;
    background: var(--color-border-light);
    color: var(--color-text-muted);
    border-radius: 6px;
    font-size: 0.875rem;
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
    padding: 1rem;
  }

  .modal {
    background: var(--color-bg);
    border-radius: 12px;
    padding: 2rem;
    max-width: 480px;
    width: 100%;
    position: relative;
    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  }

  .modal-close {
    position: absolute;
    top: 1rem;
    right: 1rem;
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
    margin: 0 0 0.5rem 0;
    color: var(--color-text-darker);
  }

  .modal-subtitle {
    color: var(--color-text-muted);
    margin-bottom: 1.5rem;
  }

  .vote-options {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .vote-option {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 1.25rem;
    border: 2px solid var(--color-border);
    border-radius: 12px;
    background: var(--color-bg);
    cursor: pointer;
    transition: all 0.2s;
    width: 100%;
  }

  .vote-option:hover:not(:disabled) {
    border-color: var(--color-border-dark);
  }

  .vote-option.support.selected {
    border-color: var(--color-success);
    background: var(--color-success-light);
  }

  .vote-option.oppose.selected {
    border-color: var(--color-error);
    background: var(--color-error-light);
  }

  .vote-option:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .vote-icon {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  .vote-text {
    font-weight: 600;
    font-size: 1.125rem;
    color: var(--color-text-darker);
    margin-bottom: 0.25rem;
  }

  .vote-desc {
    font-size: 0.75rem;
    color: var(--color-text-muted);
    text-align: center;
  }

  .tx-message {
    padding: 0.75rem 1rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .tx-message.success {
    background: var(--color-success-light);
    color: var(--color-success-text);
  }

  .tx-message.error {
    background: var(--color-error-light);
    color: var(--color-error-text);
  }

  .tx-message.info {
    background: var(--color-info-light);
    color: var(--color-info-text);
  }

  .modal-actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
  }

  .btn-cancel {
    padding: 0.75rem 1.5rem;
    background: var(--color-border-light);
    color: var(--color-text);
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    width: auto;
  }

  .btn-cancel:hover:not(:disabled) {
    background: var(--color-border);
  }

  .btn-submit {
    padding: 0.75rem 1.5rem;
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: 6px;
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

  .form-card {
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 12px;
    padding: 2rem;
  }

  .form-card h2 {
    margin-top: 0;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    font-weight: 500;
    margin-bottom: 0.5rem;
    color: var(--color-text);
  }

  input, select, textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--color-border-dark);
    border-radius: 6px;
    font-size: 1rem;
    font-family: inherit;
  }

  small {
    display: block;
    color: var(--color-text-muted);
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }

  .info-box {
    background: var(--color-warning-light);
    border: 1px solid var(--color-warning);
    border-radius: 8px;
    padding: 1rem;
    margin: 1.5rem 0;
  }

  .info-box h4 {
    margin: 0 0 0.5rem 0;
    color: var(--color-warning-text);
  }

  .value-footer {
    text-align: center;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    margin-top: var(--space-lg);
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
  }

  .info-box ul {
    margin: 0;
    padding-left: 1.5rem;
    color: var(--color-warning-text);
  }

  button {
    padding: 0.75rem 1.5rem;
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    width: 100%;
  }

  @media (max-width: 640px) {
    .container {
      padding: 1rem;
    }

    .proposal-title-row {
      flex-direction: column;
      gap: 0.5rem;
    }

    .proposal-votes {
      flex-wrap: wrap;
    }
  }

  .modal-subtitle {
    color: var(--color-text-muted);
    margin-bottom: 1.5rem;
  }
</style>
