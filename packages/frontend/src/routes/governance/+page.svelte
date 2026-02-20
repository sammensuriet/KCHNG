<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";

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
  }>>([]);

  let loading = $state(false);

  // Create proposal form
  let proposalType = $state(0);
  let proposalTitle = $state("");
  let proposalDescription = $state("");
  let newRateBps = $state(1000);

  const proposalTypes = [
    { value: 0, label: "Rate Change" },
    { value: 1, label: "Trust Parameters" },
    { value: 2, label: "Protocol Upgrade" },
    { value: 3, label: "Emergency Measure" },
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

  function getProposalTypeName(type: number): string {
    return proposalTypes.find(t => t.value === type)?.label || "Unknown";
  }

  function getStatusName(status: number): string {
    const statuses = ["Review", "Voting", "Approved", "Rejected", "Implemented", "Expired"];
    return statuses[status] || "Unknown";
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

<div class="container">
  <h1>Governance</h1>
  <p class="subtitle">Community-driven proposal and voting system</p>

  <div class="tabs">
    <button
      class:active={activeTab === "proposals"}
      onclick={() => activeTab = "proposals"}
    >
      Proposals
    </button>
    <button
      class:active={activeTab === "create"}
      onclick={() => activeTab = "create"}
    >
      Create Proposal
    </button>
  </div>

  {#if activeTab === "proposals"}
    <div class="tab-content">
      <div class="info-banner">
        <strong>Governance Process:</strong> Proposals go through a 7-day review period,
        followed by a 3-day voting period. Approved proposals have a 30-day implementation notice.
        Rate changes require trust governor approval; protocol changes require admin approval.
      </div>

      {#if loading}
        <div class="loading">Loading proposals...</div>
      {:else if proposals.length === 0}
        <div class="empty-state">
          <div class="empty-icon">🗳️</div>
          <h3>No Proposals Yet</h3>
          <p>Be the first to create a governance proposal!</p>
          <button onclick={() => activeTab = "create"}>Create Proposal</button>
        </div>
      {:else}
        <div class="proposals-list">
          {#each proposals as proposal (proposal.proposal_id)}
            <div class="proposal-card">
              <div class="proposal-header">
                <div class="proposal-title-row">
                  <h3>{proposal.title}</h3>
                  <span class:proposal-status class:status-review={proposal.status === 0} class:status-voting={proposal.status === 1} class:status-approved={proposal.status === 2} class:status-rejected={proposal.status === 3} class:status-implemented={proposal.status === 4} class:status-expired={proposal.status === 5}>
                    {getStatusName(proposal.status)}
                  </span>
                </div>
                <div class="proposal-meta">
                  <span class="proposal-type">{getProposalTypeName(proposal.proposal_type)}</span>
                  <span class="proposal-id">#{proposal.proposal_id}</span>
                  <span class="proposal-date">{formatDate(proposal.created_at)}</span>
                </div>
              </div>

              <p class="proposal-description">{proposal.description}</p>

              {#if proposal.new_rate_bps}
                <div class="proposal-rate">
                  <strong>Proposed Rate:</strong> {(proposal.new_rate_bps / 100).toFixed(1)}%
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
                  <button class="btn-vote">Vote</button>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </div>

  {:else if activeTab === "create"}
    <div class="tab-content">
      <div class="form-card">
        <h2>Create Governance Proposal</h2>

        <div class="form-group">
          <label>Proposal Type</label>
          <select bind:value={proposalType}>
            {#each proposalTypes as type}
              <option value={type.value}>{type.label}</option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label>Title</label>
          <input type="text" bind:value={proposalTitle} placeholder="Brief description of your proposal" />
        </div>

        <div class="form-group">
          <label>Description</label>
          <textarea
            bind:value={proposalDescription}
            placeholder="Detailed explanation of the proposal and its rationale"
            rows="5"
          ></textarea>
        </div>

        {#if proposalType === 0}
          <div class="form-group">
            <label>New Annual Rate (%)</label>
            <input type="number" bind:value={newRateBps} min="500" max="1500" step="100" />
            <small>Protocol limits: 5% - 15% annually</small>
          </div>
        {/if}

        <div class="info-box">
          <h4>Proposal Timeline</h4>
          <ul>
            <li><strong>Review Period:</strong> 7 days for community discussion</li>
            <li><strong>Voting Period:</strong> 3 days for trust members to vote</li>
            <li><strong>Implementation:</strong> 30 days notice after approval</li>
          </ul>
        </div>

        <button onclick={createProposal}>Create Proposal</button>
      </div>
    </div>
  {/if}

  <p class="value-footer">30 min verified work = 1000 KCHNG = 1 community meal</p>
</div>

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
    color: #6b7280;
    margin: 0 0 2rem 0;
  }

  .tabs {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 2rem;
    border-bottom: 1px solid #e5e7eb;
  }

  .tabs button {
    padding: 0.75rem 1.5rem;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-weight: 500;
    color: #6b7280;
    transition: all 0.2s;
  }

  .tabs button.active {
    color: #667eea;
    border-bottom-color: #667eea;
  }

  .tab-content {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .info-banner {
    background: #dbeafe;
    border: 1px solid #93c5fd;
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 2rem;
    color: #1e40af;
  }

  .loading, .empty-state {
    text-align: center;
    padding: 3rem;
    background: #f9fafb;
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
    background: white;
    border: 1px solid #e5e7eb;
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

  .status-review { background: #fef3c7; color: #92400e; }
  .status-voting { background: #dbeafe; color: #1e40af; }
  .status-approved { background: #d1fae5; color: #065f46; }
  .status-rejected { background: #fee2e2; color: #991b1b; }
  .status-implemented { background: #ede9fe; color: #7c3aed; }
  .status-expired { background: #e5e7eb; color: #374151; }

  .proposal-meta {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
    font-size: 0.875rem;
  }

  .proposal-type {
    background: #ede9fe;
    color: #7c3aed;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-weight: 500;
  }

  .proposal-id, .proposal-date {
    color: #6b7280;
  }

  .proposal-description {
    color: #4b5563;
    line-height: 1.6;
    margin: 1rem 0;
  }

  .proposal-rate {
    background: #f3f4f6;
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
    color: #6b7280;
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
    background: #d1fae5;
  }

  .vote-against {
    background: #fee2e2;
  }

  .vote-count {
    font-size: 1.5rem;
    font-weight: 700;
  }

  .vote-label {
    font-size: 0.875rem;
    color: #6b7280;
  }

  .btn-vote {
    padding: 0.5rem 1rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
  }

  .form-card {
    background: white;
    border: 1px solid #e5e7eb;
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
    color: #374151;
  }

  input, select, textarea {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 1rem;
    font-family: inherit;
  }

  small {
    display: block;
    color: #6b7280;
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }

  .info-box {
    background: #fef3c7;
    border: 1px solid #fbbf24;
    border-radius: 8px;
    padding: 1rem;
    margin: 1.5rem 0;
  }

  .info-box h4 {
    margin: 0 0 0.5rem 0;
    color: #92400e;
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
    color: #78350f;
  }

  button {
    padding: 0.75rem 1.5rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
</style>
