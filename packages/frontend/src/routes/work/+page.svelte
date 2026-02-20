<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";

  let activeTab = $state<"submit" | "verify" | "my-claims">("submit");

  // Submit work form
  let workType = $state(0); // BasicCare
  let minutesWorked = $state(30);
  let evidenceHash = $state("");

  // Work claims
  let workClaims = $state<Array<{
    claim_id: number;
    worker: string;
    work_type: number;
    minutes_worked: number;
    status: number;
    multiplier: number;
    submitted_at: number;
  }>>([]);

  let loading = $state(false);

  const workTypes = [
    { value: 0, label: "Basic Work", multiplier: 1.0, examples: "care, agriculture, cooking" },
    { value: 1, label: "Skilled Labor", multiplier: 1.3, examples: "crafting, manufacturing, construction" },
    { value: 2, label: "Training/Teaching", multiplier: 1.5, examples: "education, hospitality, tour guiding" },
    { value: 3, label: "Emergency Response", multiplier: 2.0, examples: "crisis relief, urgent care, disaster response" },
  ];

  onMount(async () => {
    await loadWorkClaims();
  });

  async function loadWorkClaims() {
    if (!$wallet.connected || !$wallet.address) return;

    loading = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // For demo, just show a message about fetching claims
      // In real implementation, would fetch from contract
      loading = false;
    } catch (e) {
      console.error("Failed to load work claims:", e);
      loading = false;
    }
  }

  async function submitWorkClaim() {
    if (!$wallet.connected) {
      alert("Please connect your wallet first");
      return;
    }

    if (minutesWorked < 15) {
      alert("Minimum work time is 15 minutes");
      return;
    }

    try {
      alert(`Work claim submission requires transaction signing.\n\nType: ${workTypes[workType].label}\nMinutes: ${minutesWorked}\nMultiplier: ${workTypes[workType].multiplier}x\n\nEvidence hash: ${evidenceHash || "None"}`);
    } catch (e) {
      alert("Failed to submit work claim: " + (e instanceof Error ? e.message : "Unknown error"));
    }
  }

  function calculateTokens(): number {
    const type = workTypes[workType];
    return (minutesWorked * type.multiplier * 1000) / 30; // 30 min = 1000 KCHNG = 1 community meal
  }

  function getWorkTypeName(type: number): string {
    return workTypes.find(t => t.value === type)?.label || "Unknown";
  }

  function getStatusName(status: number): string {
    const statuses = ["Pending", "Approved", "Rejected", "Expired"];
    return statuses[status] || "Unknown";
  }
</script>

<div class="container">
  <h1>Work Verification</h1>
  <p class="subtitle">Earn KCHNG by contributing verified community work</p>

  <div class="tabs">
    <button
      class:active={activeTab === "submit"}
      onclick={() => activeTab = "submit"}
    >
      Submit Work
    </button>
    <button
      class:active={activeTab === "verify"}
      onclick={() => activeTab = "verify"}
    >
      Verify Work
    </button>
    <button
      class:active={activeTab === "my-claims"}
      onclick={() => activeTab = "my-claims"}
    >
      My Claims
    </button>
  </div>

  {#if activeTab === "submit"}
    <div class="tab-content">
      <div class="info-banner">
        <strong>How it works:</strong> Submit evidence of community work (basic care, skilled labor, teaching, or emergency response).
        Minimum 2 verifiers from your trust will review and approve your claim.
        Approved work earns KCHNG based on time and work type multiplier.
      </div>

      <div class="form-card">
        <h2>Submit Work Claim</h2>

        <div class="form-group">
          <label>Work Type</label>
          <select bind:value={workType}>
            {#each workTypes as type}
              <option value={type.value}>
                {type.label} ({type.multiplier}x) — {type.examples}
              </option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label>Minutes Worked (minimum: 15)</label>
          <input type="number" bind:value={minutesWorked} min="15" />
          <small>30 min verified work = 1000 KCHNG = 1 community meal</small>
        </div>

        <div class="form-group">
          <label>Evidence Hash (IPFS CID recommended)</label>
          <input type="text" bind:value={evidenceHash} placeholder="Qm..." />
          <small>Upload photos/documents to IPFS and enter the CID</small>
        </div>

        <div class="preview-box">
          <h3>Earned Tokens Preview</h3>
          <div class="token-amount">{calculateTokens().toFixed(2)} KCHNG</div>
          <div class="token-breakdown">
            {minutesWorked} minutes × {workTypes[workType].label} ({workTypes[workType].multiplier}x) ÷ 30
          </div>
        </div>

        <button onclick={submitWorkClaim}>Submit Work Claim</button>
      </div>

      <div class="work-types-info">
        <h3>Work Type Multipliers</h3>
        <div class="multiplier-grid">
          {#each workTypes as type}
            <div class="multiplier-card" class:selected={workType === type.value}>
              <div class="multiplier-value">{type.multiplier}x</div>
              <div class="multiplier-label">{type.label}</div>
              <div class="multiplier-examples">{type.examples}</div>
            </div>
          {/each}
        </div>
      </div>
    </div>

  {:else if activeTab === "verify"}
    <div class="tab-content">
      <div class="info-banner">
        <strong>Become a verifier:</strong> Stake 100,000 KCHNG to verify community work claims.
        Earn reputation for accurate verification. Required: minimum 2 verifiers per trust.
      </div>

      <div class="verifier-status">
        <h2>Verifier Status</h2>
        {#if !$wallet.connected}
          <p>Please connect your wallet to check verifier status.</p>
          <button onclick={() => wallet.connect($wallet.network)}>Connect Wallet</button>
        {:else}
          <div class="status-grid">
            <div class="status-card">
              <div class="status-label">Registered Verifier</div>
              <div class="status-value">No</div>
              <button class="btn-register">Register as Verifier</button>
            </div>
            <div class="status-card">
              <div class="status-label">Required Stake</div>
              <div class="status-value">100,000 KCHNG</div>
            </div>
            <div class="status-card">
              <div class="status-label">Your Stake</div>
              <div class="status-value">0 KCHNG</div>
            </div>
            <div class="status-card">
              <div class="status-label">Reputation</div>
              <div class="status-value">-</div>
            </div>
          </div>

          <div class="pending-claims">
            <h3>Pending Claims to Verify</h3>
            <div class="empty-state">No pending claims available for verification.</div>
          </div>
        {/if}
      </div>
    </div>

  {:else if activeTab === "my-claims"}
    <div class="tab-content">
      <div class="claims-list">
        <h2>My Work Claims</h2>

        {#if !$wallet.connected}
          <div class="empty-state">Please connect your wallet to view your claims.</div>
        {:else if loading}
          <div class="loading">Loading claims...</div>
        {:else if workClaims.length === 0}
          <div class="empty-state">
            <div class="empty-icon">📋</div>
            <h3>No Work Claims Yet</h3>
            <p>Submit your first work claim to start earning KCHNG!</p>
            <button onclick={() => activeTab = "submit"}>Submit Work</button>
          </div>
        {:else}
          <div class="claims-grid">
            {#each workClaims as claim (claim.claim_id)}
              <div class="claim-card">
                <div class="claim-header">
                  <span class="claim-id">Claim #{claim.claim_id}</span>
                  <span class="claim-status status-{claim.status}">
                    {getStatusName(claim.status)}
                  </span>
                </div>
                <div class="claim-details">
                  <div class="claim-detail">
                    <span class="detail-label">Type:</span>
                    <span>{getWorkTypeName(claim.work_type)}</span>
                  </div>
                  <div class="claim-detail">
                    <span class="detail-label">Minutes:</span>
                    <span>{claim.minutes_worked}</span>
                  </div>
                  <div class="claim-detail">
                    <span class="detail-label">Multiplier:</span>
                    <span>{claim.multiplier / 100}x</span>
                  </div>
                  <div class="claim-detail">
                    <span class="detail-label">Submitted:</span>
                    <span>{new Date(claim.submitted_at * 1000).toLocaleDateString()}</span>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .container {
    max-width: 1000px;
    margin: 0 auto;
    padding: var(--space-lg);
  }

  h1 {
    font-size: var(--font-size-3xl);
    margin-bottom: var(--space-sm);
  }

  .subtitle {
    color: var(--color-text-muted);
    margin: 0 0 var(--space-lg) 0;
  }

  .tabs {
    display: flex;
    gap: var(--space-sm);
    margin-bottom: var(--space-lg);
    border-bottom: 1px solid var(--color-border);
  }

  .tabs button {
    padding: var(--space-sm) var(--space-lg);
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    font-weight: 500;
    color: var(--color-text-muted);
    transition: all 0.2s;
  }

  .tabs button:hover {
    color: var(--color-text);
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
    background: #dbeafe;
    border: 1px solid #93c5fd;
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
    color: #1e40af;
  }

  .form-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 12px;
    padding: 2rem;
    margin-bottom: 2rem;
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

  input, select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 1rem;
  }

  small {
    display: block;
    color: #6b7280;
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }

  .preview-box {
    background: #f3f4f6;
    border-radius: 8px;
    padding: 1.5rem;
    margin: 1.5rem 0;
    text-align: center;
  }

  .preview-box h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: #6b7280;
  }

  .token-amount {
    font-size: 2.5rem;
    font-weight: 700;
    color: #667eea;
    margin-bottom: 0.5rem;
  }

  .token-breakdown {
    color: #6b7280;
    font-size: 0.875rem;
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

  .work-types-info h3 {
    margin-bottom: 1rem;
  }

  .multiplier-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }

  .multiplier-card {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    padding: 1.5rem;
    text-align: center;
    transition: all 0.2s;
  }

  .multiplier-card.selected {
    border-color: #667eea;
    background: #ede9fe;
  }

  .multiplier-value {
    font-size: 2rem;
    font-weight: 700;
    color: #667eea;
    margin-bottom: 0.5rem;
  }

  .multiplier-label {
    font-size: 0.875rem;
    color: #6b7280;
  }

  .multiplier-examples {
    font-size: 0.75rem;
    color: #9ca3af;
    margin-top: 0.5rem;
  }

  .verifier-status h2 {
    margin-bottom: 1.5rem;
  }

  .status-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .status-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1.5rem;
    text-align: center;
  }

  .status-label {
    font-size: 0.875rem;
    color: #6b7280;
    margin-bottom: 0.5rem;
  }

  .status-value {
    font-size: 1.5rem;
    font-weight: 600;
    color: #374151;
    margin-bottom: 1rem;
  }

  .btn-register {
    padding: 0.5rem 1rem;
    background: #f3f4f6;
    color: #374151;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .pending-claims h3 {
    margin-bottom: 1rem;
  }

  .empty-state, .loading {
    text-align: center;
    padding: 3rem;
    background: #f9fafb;
    border-radius: 8px;
    color: #6b7280;
  }

  .empty-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .claims-grid {
    display: grid;
    gap: 1rem;
  }

  .claim-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    padding: 1.5rem;
  }

  .claim-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .claim-id {
    font-weight: 600;
  }

  .claim-status {
    padding: 0.25rem 0.75rem;
    border-radius: 20px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .status-0 { background: #fef3c7; color: #92400e; }
  .status-1 { background: #d1fae5; color: #065f46; }
  .status-2 { background: #fee2e2; color: #991b1b; }
  .status-3 { background: #e5e7eb; color: #374151; }

  .claim-details {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
  }

  .claim-detail {
    font-size: 0.875rem;
  }

  .detail-label {
    color: #6b7280;
    margin-right: 0.5rem;
  }

  @media (max-width: 640px) {
    .container {
      padding: 1rem;
    }

    .tabs {
      flex-wrap: wrap;
    }

    .claim-details {
      grid-template-columns: 1fr;
    }
  }
</style>
