<script lang="ts">
  import { onMount } from "svelte";
  import { wallet } from "$lib/stores/wallet";
  import { t } from "$lib/i18n";
  import FileUpload from "$lib/components/FileUpload.svelte";
  import type { VerifierData, WorkClaim } from "@kchng/shared";
  import { WorkType, ClaimStatus } from "@kchng/shared";

  let activeTab = $state<"submit" | "verify" | "my-claims">("submit");

  // Submit work form
  let workType = $state(0); // BasicCare
  let minutesWorked = $state(30);
  let evidenceCid = $state("");

  // Submission state
  let submitting = $state(false);
  let submitMessage = $state<{ type: "success" | "error" | "info"; text: string } | null>(null);

  // Work claims
  let workClaims = $state<WorkClaim[]>([]);

  // Verifier state
  let verifierData = $state<VerifierData | null>(null);
  let isVerifier = $state(false);
  let pendingClaims = $state<WorkClaim[]>([]);
  let verifierLoading = $state(false);
  let showRegisterModal = $state(false);
  let registerTxPending = $state(false);

  // Verification action state
  let verifyingClaimId = $state<number | null>(null);
  let verifyAction = $state<"approve" | "reject" | null>(null);

  let loading = $state(false);

  // Derived: can submit work only if connected AND in a community
  let canSubmitWork = $derived($wallet.connected && $wallet.isCommunityMember);

  const workTypes = $derived([
    { value: 0, label: t('work.workTypeStandard'), multiplier: 1.0, examples: t('work.workTypeStandardEx') },
    { value: 1, label: t('work.workTypeSkilled'), multiplier: 1.3, examples: t('work.workTypeSkilledEx') },
    { value: 2, label: t('work.workTypeKnowledge'), multiplier: 1.5, examples: t('work.workTypeKnowledgeEx') },
    { value: 3, label: t('work.workTypeCritical'), multiplier: 2.0, examples: t('work.workTypeCriticalEx') },
  ]);

  // Required stake for verifier (100K KCHNG ≈ 100 meals)
  const VERIFIER_STAKE = 100000n;

  onMount(async () => {
    await loadWorkClaims();
    await loadVerifierStatus();
  });

  async function loadWorkClaims() {
    if (!$wallet.connected || !$wallet.address) return;

    loading = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      loading = false;
    } catch (e) {
      console.error("Failed to load work claims:", e);
      loading = false;
    }
  }

  async function loadVerifierStatus() {
    if (!$wallet.connected || !$wallet.address) return;

    verifierLoading = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Check if user is a verifier
      try {
        verifierData = await kchngClient.getVerifier($wallet.address);
        isVerifier = verifierData !== null && verifierData.stake > 0n;

        // Load pending claims if verifier
        if (isVerifier) {
          const pendingIds = await kchngClient.getVerifierPendingClaims($wallet.address);
          pendingClaims = [];
          for (const id of pendingIds) {
            try {
              const claim = await kchngClient.getWorkClaim(id);
              pendingClaims.push(claim);
            } catch {
              // Skip claims that can't be loaded
            }
          }
        }
      } catch {
        // Not a verifier yet
        isVerifier = false;
        verifierData = null;
      }
    } catch (e) {
      console.error("Failed to load verifier status:", e);
    } finally {
      verifierLoading = false;
    }
  }

  async function registerAsVerifier() {
    if (!$wallet.connected || !$wallet.address || !$wallet.communityId) {
      submitMessage = { type: "error", text: t('work.mustJoinCommunity') };
      return;
    }

    registerTxPending = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      await kchngClient.registerVerifier($wallet.address, $wallet.communityId);

      // Reload verifier status
      await loadVerifierStatus();
      showRegisterModal = false;
    } catch (e) {
      console.error("Failed to register as verifier:", e);
      submitMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to register as verifier"
      };
    } finally {
      registerTxPending = false;
    }
  }

  async function verifyClaim(claimId: number, approve: boolean) {
    if (!$wallet.connected || !$wallet.address) return;

    verifyingClaimId = claimId;
    verifyAction = approve ? "approve" : "reject";

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      if (approve) {
        await kchngClient.approveWorkClaim($wallet.address, claimId);
      } else {
        await kchngClient.rejectWorkClaim($wallet.address, claimId);
      }

      // Reload pending claims
      await loadVerifierStatus();
    } catch (e) {
      console.error("Failed to verify claim:", e);
    } finally {
      verifyingClaimId = null;
      verifyAction = null;
    }
  }

  async function submitWorkClaim() {
    submitMessage = null;

    if (!$wallet.connected) {
      submitMessage = { type: "error", text: t('work.connectWalletFirst') };
      return;
    }

    if (!$wallet.isCommunityMember) {
      submitMessage = { type: "error", text: t('work.mustJoinCommunity') };
      return;
    }

    if (minutesWorked < 15) {
      submitMessage = { type: "error", text: t('work.minWorkTime') };
      return;
    }

    submitting = true;
    submitMessage = { type: "info", text: t('work.preparingTx') };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const txHash = await kchngClient.submitWorkClaim(
        $wallet.address!,
        workType,
        minutesWorked,
        evidenceCid || "0x0"
      );

      submitMessage = {
        type: "success",
        text: `${t('work.claimSubmitted')} ${t('work.pendingVerification')} ${calculateTokens().toFixed(2)} ${t('work.whenApproved')}`
      };

      minutesWorked = 30;
      evidenceCid = "";

    } catch (e) {
      submitMessage = {
        type: "error",
        text: e instanceof Error ? e.message : "Failed to submit work claim"
      };
    } finally {
      submitting = false;
    }
  }

  function calculateTokens(): number {
    const type = workTypes[workType];
    return (minutesWorked * type.multiplier * 1000) / 30;
  }

  function getWorkTypeName(type: number): string {
    return workTypes.find(t => t.value === type)?.label || "Unknown";
  }

  function getStatusName(status: number): string {
    const statuses = [t('work.statusPending'), t('work.statusApproved'), t('work.statusRejected'), t('work.statusExpired')];
    return statuses[status] || "Unknown";
  }

  function calculateClaimTokens(claim: WorkClaim): number {
    return (claim.minutes_worked * (claim.multiplier / 100) * 1000) / 30;
  }

  function formatAddress(address: string): string {
    if (!address) return "";
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  }
</script>

<div class="container">
  <h1>{t('work.title')}</h1>
  <p class="subtitle">{t('work.subtitle')}</p>

  <div class="tabs">
    <button
      class:active={activeTab === "submit"}
      onclick={() => activeTab = "submit"}
    >
      {t('work.tabSubmit')}
    </button>
    <button
      class:active={activeTab === "verify"}
      onclick={() => activeTab = "verify"}
    >
      {t('work.tabVerify')}
    </button>
    <button
      class:active={activeTab === "my-claims"}
      onclick={() => activeTab = "my-claims"}
    >
      {t('work.tabMyClaims')}
    </button>
  </div>

  {#if activeTab === "submit"}
    <div class="tab-content">
      {#if $wallet.connected && !$wallet.isCommunityMember}
        <div class="warning-banner">
          <strong>{t('work.communityMembershipRequired')}</strong> {t('work.mustJoinCommunity')}
          <a href="/communities">{t('work.viewCommunities')}</a>
        </div>
      {/if}

      <div class="info-banner">
        <strong>{t('work.howItWorks')}</strong> {t('work.howItWorksDesc')}
      </div>

      <div class="form-card">
        <h2>{t('work.submitWorkClaim')}</h2>

        <div class="form-group">
          <label>{t('work.workType')}</label>
          <select bind:value={workType}>
            {#each workTypes as type}
              <option value={type.value}>
                {type.label} ({type.multiplier}x) — {type.examples}
              </option>
            {/each}
          </select>
        </div>

        <div class="form-group">
          <label>{t('work.minutesWorked')} ({t('work.minimum')} 15)</label>
          <input type="number" bind:value={minutesWorked} min="15" inputmode="numeric" pattern="[0-9]*" />
          <small>{t('work.minutesHint')}</small>
        </div>

        <div class="form-group">
          <label>{t('work.evidence')}</label>
          <FileUpload onUpload={(cid) => evidenceCid = cid} existingCid={evidenceCid} />
          <small>{t('work.evidenceHint')}</small>
        </div>

        <div class="preview-box">
          <h3>{t('work.earnedTokensPreview')}</h3>
          <div class="token-amount">{calculateTokens().toFixed(2)} KCHNG</div>
          <div class="token-breakdown">
            {minutesWorked} min × {workTypes[workType].label} ({workTypes[workType].multiplier}x) ÷ 30
          </div>
        </div>

        {#if submitMessage}
          <div class="message message-{submitMessage.type}">
            {#if submitMessage.type === "info"}
              <span class="spinner"></span>
            {:else if submitMessage.type === "success"}
              <span class="icon">✓</span>
            {:else}
              <span class="icon">⚠</span>
            {/if}
            {submitMessage.text}
          </div>
        {/if}

        <button
          class="submit-btn"
          class:submitting
          onclick={submitWorkClaim}
          disabled={submitting || !canSubmitWork}
        >
          {#if submitting}
            <span class="btn-spinner"></span>
            {t('work.submitting')}
          {:else}
            {t('work.submitClaim')}
          {/if}
        </button>
      </div>

      <div class="work-types-info">
        <h3>{t('work.workTypeMultipliers')}</h3>
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
      {#if !$wallet.connected}
        <div class="info-banner">
          <strong>{t('work.stepUp')}</strong> {t('work.stepUpDesc')}
        </div>
        <div class="verifier-status">
          <h2>{t('work.verifierStatus')}</h2>
          <p>{t('work.checkStatus')}</p>
          <button onclick={() => wallet.connect($wallet.network)}>{t('common.connectWallet')}</button>
        </div>
      {:else if verifierLoading}
        <div class="loading">{t('work.loading')}</div>
      {:else if !isVerifier}
        <!-- Step Up for Your Community section -->
        <div class="step-up-section">
          <div class="info-banner community-banner">
            <strong>🌱 {t('work.stepUp')}</strong>
            <p>{t('work.stepUpDescFull')}</p>
          </div>

          <div class="verifier-benefits">
            <h3>{t('work.whyVerifier')}</h3>
            <div class="benefits-grid">
              <div class="benefit-card">
                <div class="benefit-icon">🛡️</div>
                <div class="benefit-title">{t('work.protectCommunity')}</div>
                <div class="benefit-desc">{t('work.protectCommunityDesc')}</div>
              </div>
              <div class="benefit-card">
                <div class="benefit-icon">⭐</div>
                <div class="benefit-title">{t('work.buildReputation')}</div>
                <div class="benefit-desc">{t('work.buildReputationDesc')}</div>
              </div>
              <div class="benefit-card">
                <div class="benefit-icon">🤝</div>
                <div class="benefit-title">{t('work.strengthenCommunity')}</div>
                <div class="benefit-desc">{t('work.strengthenCommunityDesc')}</div>
              </div>
            </div>
          </div>

          <div class="stake-info">
            <h3>{t('work.commitmentRequired')}</h3>
            <div class="stake-card">
              <div class="stake-amount">{Number(VERIFIER_STAKE).toLocaleString()} KCHNG</div>
              <div class="stake-meals">≈ {Number(VERIFIER_STAKE / 1000n).toLocaleString()} meals</div>
              <div class="stake-note">{t('work.stakeNote')}</div>
            </div>
          </div>

          <button
            class="btn-step-up"
            onclick={() => showRegisterModal = true}
            disabled={!$wallet.isCommunityMember}
          >
            {#if !$wallet.isCommunityMember}
              {t('work.mustJoinCommunityFirst')}
            {:else}
              {t('work.stepUpButton')}
            {/if}
          </button>
        </div>

        <!-- Registration Modal -->
        {#if showRegisterModal}
          <div class="modal-overlay" onclick={() => showRegisterModal = false}>
            <div class="modal" onclick={(e) => e.stopPropagation()}>
              <h2>{t('work.stepUpTitle')}</h2>
              <p>{t('work.stepUpModalDesc')}</p>

              <div class="modal-info">
                <div class="modal-row">
                  <span>{t('work.stakeAmount')}</span>
                  <span class="modal-value">{Number(VERIFIER_STAKE).toLocaleString()} KCHNG</span>
                </div>
                <div class="modal-row">
                  <span>{t('work.mealEquivalent')}</span>
                  <span class="modal-value">{Number(VERIFIER_STAKE / 1000n).toLocaleString()} meals</span>
                </div>
                <div class="modal-row">
                  <span>{t('work.returnable')}</span>
                  <span class="modal-value">{t('work.yesFullReturn')}</span>
                </div>
              </div>

              <div class="modal-actions">
                <button
                  class="btn-confirm"
                  onclick={registerAsVerifier}
                  disabled={registerTxPending}
                >
                  {#if registerTxPending}
                    <span class="btn-spinner"></span>
                    {t('work.registering')}
                  {:else}
                    {t('work.confirmStepUp')}
                  {/if}
                </button>
                <button class="btn-cancel" onclick={() => showRegisterModal = false}>
                  {t('common.cancel')}
                </button>
              </div>
            </div>
          </div>
        {/if}
      {:else}
        <!-- Active Verifier Dashboard -->
        <div class="verifier-dashboard">
          <div class="verifier-header">
            <h2>✓ {t('work.activeVerifier')}</h2>
            <p class="verifier-subtitle">{t('work.reviewNeighborWork')}</p>
          </div>

          <div class="verifier-stats">
            <div class="stat-item">
              <span class="stat-label">{t('work.yourStake')}</span>
              <span class="stat-value">{verifierData?.stake ? Number(verifierData.stake).toLocaleString() : 0} KCHNG</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">{t('work.reputation')}</span>
              <span class="stat-value">{verifierData?.reputation_score ?? 500}/1000</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">{t('work.verified')}</span>
              <span class="stat-value">{verifierData?.verified_claims ?? 0}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">{t('work.rejected')}</span>
              <span class="stat-value">{verifierData?.rejected_claims ?? 0}</span>
            </div>
          </div>

          <div class="pending-claims-section">
            <h3>{t('work.pendingClaimsVerify')} ({pendingClaims.length})</h3>

            {#if pendingClaims.length === 0}
              <div class="empty-state">
                <div class="empty-icon">✓</div>
                <h3>{t('work.allCaughtUp')}</h3>
                <p>{t('work.noPendingClaimsVerifier')}</p>
              </div>
            {:else}
              <div class="claims-to-verify">
                {#each pendingClaims as claim (claim.claim_id)}
                  <div class="claim-verify-card">
                    <div class="claim-verify-header">
                      <span class="claimant">{t('work.from')} {formatAddress(claim.worker)}</span>
                      <span class="claim-time">{new Date(claim.submitted_at * 1000).toLocaleDateString()}</span>
                    </div>

                    <div class="claim-verify-details">
                      <div class="detail-row">
                        <span class="detail-label">{t('work.type')}</span>
                        <span>{getWorkTypeName(claim.work_type)}</span>
                      </div>
                      <div class="detail-row">
                        <span class="detail-label">{t('work.minutes')}</span>
                        <span>{claim.minutes_worked}</span>
                      </div>
                      <div class="detail-row">
                        <span class="detail-label">{t('work.tokens')}</span>
                        <span class="token-highlight">{calculateClaimTokens(claim).toFixed(2)} KCHNG</span>
                      </div>
                    </div>

                    {#if claim.evidence_hash}
                      <div class="evidence-section">
                        <span class="evidence-label">{t('work.evidenceProvided')}</span>
                        <span class="evidence-indicator">📎</span>
                      </div>
                    {/if}

                    <div class="verify-actions">
                      <button
                        class="btn-verify"
                        onclick={() => verifyClaim(claim.claim_id, true)}
                        disabled={verifyingClaimId === claim.claim_id}
                      >
                        {#if verifyingClaimId === claim.claim_id && verifyAction === "approve"}
                          <span class="btn-spinner"></span>
                        {/if}
                        ✓ {t('work.verify')}
                      </button>
                      <button
                        class="btn-cannot-verify"
                        onclick={() => verifyClaim(claim.claim_id, false)}
                        disabled={verifyingClaimId === claim.claim_id}
                      >
                        {#if verifyingClaimId === claim.claim_id && verifyAction === "reject"}
                          <span class="btn-spinner"></span>
                        {/if}
                        ✗ {t('work.cannotVerify')}
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>

  {:else if activeTab === "my-claims"}
    <div class="tab-content">
      <div class="claims-list">
        <h2>{t('work.myWorkClaims')}</h2>

        {#if !$wallet.connected}
          <div class="empty-state">{t('work.connectToView')}</div>
        {:else if loading}
          <div class="loading">{t('work.loadingClaims')}</div>
        {:else if workClaims.length === 0}
          <div class="empty-state">
            <div class="empty-icon">📋</div>
            <h3>{t('work.noClaimsYet')}</h3>
            <p>{t('work.noClaimsDesc')}</p>
            <button onclick={() => activeTab = "submit"}>{t('work.tabSubmit')}</button>
          </div>
        {:else}
          <div class="claims-grid">
            {#each workClaims as claim (claim.claim_id)}
              <div class="claim-card">
                <div class="claim-header">
                  <span class="claim-id">{t('work.claim')} #{claim.claim_id}</span>
                  <span class="claim-status status-{claim.status}">
                    {getStatusName(claim.status)}
                  </span>
                </div>
                <div class="claim-details">
                  <div class="claim-detail">
                    <span class="detail-label">{t('work.type')}</span>
                    <span>{getWorkTypeName(claim.work_type)}</span>
                  </div>
                  <div class="claim-detail">
                    <span class="detail-label">{t('work.minutes')}</span>
                    <span>{claim.minutes_worked}</span>
                  </div>
                  <div class="claim-detail">
                    <span class="detail-label">{t('work.multiplier')}</span>
                    <span>{claim.multiplier / 100}x</span>
                  </div>
                  <div class="claim-detail">
                    <span class="detail-label">{t('work.submitted')}</span>
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
    background: var(--color-info-light);
    border: 1px solid var(--color-info);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
    color: var(--color-info-text);
  }

  .warning-banner {
    background: var(--color-warning-light);
    border: 1px solid var(--color-warning);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-bottom: var(--space-lg);
    color: var(--color-warning-text);
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
    align-items: center;
  }

  .warning-banner a {
    color: var(--color-warning-text);
    text-decoration: underline;
    font-weight: 500;
    margin-left: var(--space-xs);
  }

  .warning-banner a:hover {
    color: var(--color-warning-text);
  }

  .form-card {
    background: white;
    border: 1px solid var(--color-border);
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
    color: var(--color-text);
  }

  input, select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--color-border-dark);
    border-radius: 6px;
    font-size: 1rem;
  }

  small {
    display: block;
    color: var(--color-text-muted);
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }

  .preview-box {
    background: var(--color-border-light);
    border-radius: 8px;
    padding: 1.5rem;
    margin: 1.5rem 0;
    text-align: center;
  }

  .preview-box h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
    color: var(--color-text-muted);
  }

  .token-amount {
    font-size: 2.5rem;
    font-weight: 700;
    color: var(--color-primary);
    margin-bottom: 0.5rem;
  }

  .token-breakdown {
    color: var(--color-text-muted);
    font-size: 0.875rem;
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

  .submit-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 1rem 1.5rem;
    font-size: 1rem;
    transition: opacity 0.2s;
  }

  .submit-btn:hover:not(:disabled) {
    opacity: 0.9;
  }

  .submit-btn:disabled {
    cursor: not-allowed;
    opacity: 0.7;
  }

  .submit-btn.submitting {
    background: linear-gradient(135deg, var(--color-text-light) 0%, var(--color-text-muted) 100%);
  }

  .btn-spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .message {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem;
    border-radius: 8px;
    margin: 1rem 0;
    font-size: 0.875rem;
  }

  .message-info {
    background: var(--color-info-light);
    color: var(--color-info-text);
    border: 1px solid var(--color-info);
  }

  .message-success {
    background: var(--color-success-light);
    color: var(--color-success-text);
    border: 1px solid var(--color-success);
  }

  .message-error {
    background: var(--color-error-light);
    color: var(--color-error-text);
    border: 1px solid var(--color-error);
  }

  .message .icon {
    font-size: 1.25rem;
    flex-shrink: 0;
  }

  .message .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid currentColor;
    border-top-color: transparent;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    flex-shrink: 0;
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
    border: 2px solid var(--color-border);
    border-radius: 8px;
    padding: 1.5rem;
    text-align: center;
    transition: all 0.2s;
  }

  .multiplier-card.selected {
    border-color: var(--color-primary);
    background: var(--color-primary-light);
  }

  .multiplier-value {
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-primary);
    margin-bottom: 0.5rem;
  }

  .multiplier-label {
    font-size: 0.875rem;
    color: var(--color-text-muted);
  }

  .multiplier-examples {
    font-size: 0.75rem;
    color: var(--color-text-light);
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
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 1.5rem;
    text-align: center;
  }

  .status-label {
    font-size: 0.875rem;
    color: var(--color-text-muted);
    margin-bottom: 0.5rem;
  }

  .status-value {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--color-text);
    margin-bottom: 1rem;
  }

  .btn-register {
    padding: 0.5rem 1rem;
    background: var(--color-border-light);
    color: var(--color-text);
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
    background: var(--color-bg-subtle);
    border-radius: 8px;
    color: var(--color-text-muted);
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
    border: 1px solid var(--color-border);
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

  .status-0 { background: var(--color-warning-light); color: var(--color-warning-text); }
  .status-1 { background: var(--color-success-light); color: var(--color-success-text); }
  .status-2 { background: var(--color-error-light); color: var(--color-error-text); }
  .status-3 { background: var(--color-border); color: var(--color-text); }

  .claim-details {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.75rem;
  }

  .claim-detail {
    font-size: 0.875rem;
  }

  .detail-label {
    color: var(--color-text-muted);
    margin-right: 0.5rem;
  }

  /* Step Up Section Styles */
  .step-up-section {
    max-width: 700px;
    margin: 0 auto;
  }

  .community-banner {
    background: linear-gradient(135deg, var(--color-success-light) 0%, var(--color-success-lighter) 100%);
    border: 1px solid var(--color-success);
    color: var(--color-success-text);
  }

  .community-banner p {
    margin: 0.5rem 0 0 0;
    font-size: 0.9rem;
  }

  .verifier-benefits {
    margin: 2rem 0;
  }

  .verifier-benefits h3 {
    margin-bottom: 1rem;
    color: var(--color-text);
  }

  .benefits-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
  }

  .benefit-card {
    background: white;
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 1.25rem;
    text-align: center;
  }

  .benefit-icon {
    font-size: 2rem;
    margin-bottom: 0.5rem;
  }

  .benefit-title {
    font-weight: 600;
    color: var(--color-text);
    margin-bottom: 0.25rem;
  }

  .benefit-desc {
    font-size: 0.8rem;
    color: var(--color-text-muted);
  }

  .stake-info {
    margin: 2rem 0;
  }

  .stake-info h3 {
    margin-bottom: 1rem;
    color: var(--color-text);
  }

  .stake-card {
    background: var(--color-gradient);
    border-radius: 12px;
    padding: 2rem;
    text-align: center;
    color: white;
  }

  .stake-amount {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.25rem;
  }

  .stake-meals {
    font-size: 1rem;
    opacity: 0.9;
    margin-bottom: 0.5rem;
  }

  .stake-note {
    font-size: 0.8rem;
    opacity: 0.8;
    font-style: italic;
  }

  .btn-step-up {
    display: block;
    width: 100%;
    max-width: 400px;
    margin: 2rem auto 0;
    padding: 1rem 2rem;
    font-size: 1.1rem;
    font-weight: 600;
  }

  .btn-step-up:disabled {
    background: var(--color-text-light);
    cursor: not-allowed;
  }

  /* Modal Styles */
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
    background: white;
    border-radius: 12px;
    padding: 2rem;
    max-width: 450px;
    width: 100%;
    max-height: 90vh;
    overflow-y: auto;
  }

  .modal h2 {
    margin: 0 0 1rem 0;
    color: var(--color-text);
  }

  .modal p {
    color: var(--color-text-muted);
    margin-bottom: 1.5rem;
  }

  .modal-info {
    background: var(--color-bg-subtle);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1.5rem;
  }

  .modal-row {
    display: flex;
    justify-content: space-between;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--color-border);
  }

  .modal-row:last-child {
    border-bottom: none;
  }

  .modal-value {
    font-weight: 600;
    color: var(--color-text);
  }

  .modal-actions {
    display: flex;
    gap: 1rem;
  }

  .modal-actions button {
    flex: 1;
  }

  .btn-confirm {
    background: linear-gradient(135deg, var(--color-success) 0%, var(--color-success) 100%);
  }

  .btn-cancel {
    background: var(--color-border-light);
    color: var(--color-text);
  }

  /* Verifier Dashboard Styles */
  .verifier-dashboard {
    max-width: 800px;
    margin: 0 auto;
  }

  .verifier-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .verifier-header h2 {
    color: var(--color-success);
    margin-bottom: 0.5rem;
  }

  .verifier-subtitle {
    color: var(--color-text-muted);
    font-size: 1rem;
  }

  .verifier-stats {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    justify-content: center;
    margin-bottom: 2rem;
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    background: white;
    border: 1px solid var(--color-border);
    border-radius: 8px;
    padding: 1rem 1.5rem;
    min-width: 120px;
  }

  .stat-item .stat-label {
    font-size: 0.75rem;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .stat-item .stat-value {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text);
    margin-top: 0.25rem;
  }

  .pending-claims-section h3 {
    margin-bottom: 1rem;
    color: var(--color-text);
  }

  .claims-to-verify {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .claim-verify-card {
    background: white;
    border: 2px solid var(--color-border);
    border-radius: 12px;
    padding: 1.5rem;
  }

  .claim-verify-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--color-border);
  }

  .claimant {
    font-weight: 500;
    font-family: monospace;
    font-size: 0.9rem;
    color: var(--color-text);
  }

  .claim-time {
    font-size: 0.8rem;
    color: var(--color-text-muted);
  }

  .claim-verify-details {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .detail-row {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .detail-row .detail-label {
    font-size: 0.75rem;
    margin-right: 0;
  }

  .detail-row span:last-child {
    font-weight: 500;
    color: var(--color-text);
  }

  .token-highlight {
    color: var(--color-primary);
    font-weight: 600 !important;
  }

  .evidence-section {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    background: var(--color-border-light);
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.85rem;
  }

  .evidence-label {
    color: var(--color-text-muted);
  }

  .evidence-indicator {
    font-size: 1rem;
  }

  .verify-actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn-verify, .btn-cannot-verify {
    flex: 1;
    padding: 0.75rem 1rem;
    font-size: 0.9rem;
    font-weight: 500;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: auto;
  }

  .btn-verify {
    background: linear-gradient(135deg, var(--color-success) 0%, var(--color-success) 100%);
  }

  .btn-verify:hover:not(:disabled) {
    background: linear-gradient(135deg, var(--color-success) 0%, var(--color-success-text) 100%);
  }

  .btn-cannot-verify {
    background: var(--color-border-light);
    color: var(--color-text-muted);
  }

  .btn-cannot-verify:hover:not(:disabled) {
    background: var(--color-border);
    color: var(--color-text);
  }

  .btn-verify:disabled, .btn-cannot-verify:disabled {
    opacity: 0.6;
    cursor: not-allowed;
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

    .benefits-grid {
      grid-template-columns: 1fr;
    }

    .claim-verify-details {
      grid-template-columns: 1fr;
    }

    .verify-actions {
      flex-direction: column;
    }

    .verifier-stats {
      gap: 0.5rem;
    }

    .stat-item {
      min-width: 80px;
      padding: 0.75rem;
    }

    .stat-item .stat-value {
      font-size: 1rem;
    }

    .modal {
      margin: 1rem;
      padding: 1.5rem;
    }

    .modal-actions {
      flex-direction: column;
    }
  }
</style>
