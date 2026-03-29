<script lang="ts">
  import { onMount } from "svelte";
  import { wallet, type NetworkName } from "$lib/stores/wallet";
  import { t } from "$lib/i18n";
  import { GraceType, RoleType } from "@kchng/shared";
  import OnboardingModal from "$lib/components/OnboardingModal.svelte";
  import Tooltip from "$lib/components/Tooltip.svelte";

  let currentNetwork = $state<NetworkName>("testnet");
  let isCreatingTestWallet = $state(false);

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

  // Role detection state
  let rolesLoading = $state(false);

  // Governor role state
  let isGovernor = $state(false);
  let governorData = $state<{
    communityName: string;
    memberCount: number;
    annualRateBps: number;
    demurragePeriodDays: number;
    governorStake: bigint;
    collateralAtRisk: bigint;
    governorReputation: number;
    verifierFundBalance: bigint;
    communityId: string;
  } | null>(null);

  // Verifier role state
  let isVerifier = $state(false);
  let verifierRoleData = $state<{
    stake: bigint;
    reputationScore: number;
    verifiedClaims: number;
    rejectedClaims: number;
    pendingClaimIds: number[];
    trustId: string | null;
  } | null>(null);

  // Oracle role state
  let isOracle = $state(false);
  let oracleRoleData = $state<{
    stake: bigint;
    reputationScore: number;
    gracePeriodsGranted: number;
    grantsThisYear: number;
  } | null>(null);

  // Member role state
  let memberReputation = $state<number | null>(null);
  let workerReputation = $state<number | null>(null);

  // Cross-Community Exchange state
  let availableCommunities = $state<Array<{ id: string; name: string }>>([]);
  let selectedCommunityId = $state<string | null>(null);
  let exchangeRate = $state<number | null>(null);
  let simulatedResult = $state<bigint | null>(null);
  let exchangeAmount = $state<string>("100000");
  let exchangeLoading = $state(false);
  let exchangeTxPending = $state(false);
  let exchangeMessage = $state<{ type: "success" | "error" | "info"; text: string } | null>(null);
  let exchangeRateState = $state<number | null>(null);

  onMount(async () => {
    await loadAccountData();
    await loadAvailableCommunities();
  });

  /**
   * loadRoles() - Detect all on-chain roles for the connected account.
   * Uses Promise.allSettled() so each check can fail independently
   * without breaking the dashboard.
   *
   * Detects: governor, verifier, oracle, member
   * Loads: community details, verifier fund status, pending claims,
   *        election IDs, genesis pool status, and reputations.
   */
  async function loadRoles() {
    if (!$wallet.connected || !$wallet.address) return;

    rolesLoading = true;

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Run all role checks in parallel. Each settles independently.
      const results = await Promise.allSettled([
        // 1. Governor check - try to load community where user is governor
        (async () => {
          if (!$wallet.communityId) return;
          const communityInfo = await kchngClient.getCommunityInfo($wallet.communityId);
          if (communityInfo.governor === $wallet.address) {
            isGovernor = true;
            governorData = {
              communityName: communityInfo.name,
              memberCount: communityInfo.member_count,
              annualRateBps: communityInfo.annual_rate_bps,
              demurragePeriodDays: communityInfo.demurrage_period_days,
              governorStake: BigInt(500_000), // GOVERNOR_STAKE_AMOUNT from contract
              collateralAtRisk: BigInt(200_000), // GOVERNOR_COLLATERAL from contract
              governorReputation: 500,
              verifierFundBalance: 0n,
              communityId: $wallet.communityId,
            };

            // Load governor reputation and verifier fund in parallel
            const [repResult, fundResult] = await Promise.allSettled([
              kchngClient.getReputation($wallet.address, RoleType.Governor),
              (async () => {
                try {
                  const fund = await kchngClient.getVerifierFund($wallet.communityId!);
                  return fund.pool_balance;
                } catch {
                  return 0n;
                }
              })(),
            ]);

            if (repResult.status === "fulfilled") {
              governorData!.governorReputation = repResult.value;
            }
            if (fundResult.status === "fulfilled") {
              governorData!.verifierFundBalance = fundResult.value;
            }
          }
        })(),

        // 2. Verifier check - try to load verifier data
        (async () => {
          if (!$wallet.address) return;
          try {
            const vData = await kchngClient.getVerifier($wallet.address);
            isVerifier = true;
            verifierRoleData = {
              stake: vData.stake,
              reputationScore: vData.reputation_score,
              verifiedClaims: vData.verified_claims,
              rejectedClaims: vData.rejected_claims,
              pendingClaimIds: [],
              trustId: vData.trust_id,
            };

            // Load pending claims
            const claimsResult = await Promise.allSettled([
              kchngClient.getVerifierPendingClaims($wallet.address),
            ]);
            if (claimsResult[0].status === "fulfilled") {
              verifierRoleData!.pendingClaimIds = claimsResult[0].value;
            }
          } catch {
            // Not a verifier - this is expected for most accounts
          }
        })(),

        // 3. Oracle check - try to load oracle data
        (async () => {
          if (!$wallet.address) return;
          try {
            const oData = await kchngClient.getOracle($wallet.address);
            isOracle = true;
            oracleRoleData = {
              stake: oData.stake,
              reputationScore: oData.reputation_score,
              gracePeriodsGranted: oData.grace_periods_granted,
              grantsThisYear: oData.grants_this_year,
            };
          } catch {
            // Not an oracle - this is expected for most accounts
          }
        })(),

        // 4. Member & Worker reputations (only if in a community)
        (async () => {
          if (!$wallet.address) return;
          const [memberRepResult, workerRepResult] = await Promise.allSettled([
            kchngClient.getReputation($wallet.address, RoleType.Member),
            kchngClient.getReputation($wallet.address, RoleType.Worker),
          ]);
          if (memberRepResult.status === "fulfilled") {
            memberReputation = memberRepResult.value;
          }
          if (workerRepResult.status === "fulfilled") {
            workerReputation = workerRepResult.value;
          }
        })(),
      ]);

      // Log any failures for debugging (non-blocking)
      results.forEach((r, i) => {
        if (r.status === "rejected") {
          console.warn(`[loadRoles] Role check ${i} failed:`, (r as PromiseRejectedResult).reason);
        }
      });
    } catch (e) {
      console.warn("[loadRoles] Failed to detect roles:", e);
    } finally {
      rolesLoading = false;
    }
  }

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

      // After account data loads, detect roles in parallel (non-blocking)
      loadRoles();
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to load account data";
      loading = false;
    }
  }

  async function loadAvailableCommunities() {
    if (!$wallet.connected || !$wallet.address || !$wallet.communityId) return;

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Get all communities
      const communityIds = await kchngClient.getAllCommunities();
      const communities = await Promise.all(
        communityIds
          .filter(id => id !== $wallet.communityId) // Exclude current community
          .map(async (id) => {
            const info = await kchngClient.getCommunityInfo(id);
            return { id, name: info.name };
          })
      );

      availableCommunities = communities;

      if (communities.length > 0) {
        selectedCommunityId = communities[0].id;
        await updateExchangePreview();
      }
    } catch (e) {
      console.error("Failed to load communities:", e);
    }
  }

  async function updateExchangePreview() {
    if (!$wallet.communityId || !selectedCommunityId || !exchangeAmount) {
      exchangeRate = null;
      simulatedResult = null;
      return;
    }

    exchangeLoading = true;
    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);

      // Get exchange rate
      exchangeRate = await kchngClient.calculateExchangeRate($wallet.communityId, selectedCommunityId);

      // Simulate the swap
      const amount = BigInt(exchangeAmount);
      if (amount > 0n) {
        simulatedResult = await kchngClient.simulateCrossCommunitySwap(
          $wallet.communityId,
          selectedCommunityId,
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
    if (!$wallet.connected || !$wallet.address || !$wallet.communityId || !selectedCommunityId) {
      exchangeMessage = { type: "error", text: "Please connect wallet and select a community" };
      return;
    }

    exchangeTxPending = true;
    exchangeMessage = { type: "info", text: "Processing exchange..." };

    try {
      const { createKchngClient } = await import("$lib/contracts/kchng");
      const kchngClient = createKchngClient($wallet.network);
      kchngClient.setSignTransactionCallback(wallet.signTransaction);

      const amount = BigInt(exchangeAmount);
      const txHash = await kchngClient.crossCommunitySwap(
        $wallet.address,
        selectedCommunityId,
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
    // Update preview when amount or community changes
    if (selectedCommunityId && exchangeAmount) {
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

  function formatReputation(score: number): string {
    if (score >= 700) return `${score} (Excellent)`;
    if (score >= 500) return `${score} (Good)`;
    if (score >= 200) return `${score} (Probation)`;
    return `${score} (At Risk)`;
  }

  function reputationClass(score: number): string {
    if (score >= 700) return "rep-excellent";
    if (score >= 500) return "rep-good";
    if (score >= 200) return "rep-probation";
    return "rep-at-risk";
  }

  async function handleCreateTestWallet() {
    if (isCreatingTestWallet) return;
    isCreatingTestWallet = true;
    try {
      await wallet.createTestWallet();
      await loadAccountData();
    } catch (error) {
      console.error("Failed to create test wallet:", error);
    } finally {
      isCreatingTestWallet = false;
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
        <h2>{t('dashboard.balance')} <Tooltip term="balance" /></h2>
        <div class="balance-amount">{formatBalance(accountData.balance)} KCHNG</div>
        <div class="balance-subtext">
          {t('dashboard.balanceSubtext')}
        </div>
      </div>

      <!-- Activity Card -->
      <div class="card">
        <h2>{t('dashboard.accountActivity')} <Tooltip term="demurrage" /></h2>
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
          <h2>🛡️ Community Protection <Tooltip term="gracePeriod" /></h2>
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

      <!-- Community Card -->
      <div class="card">
        <h2>{t('dashboard.communityMembership')} <Tooltip term="community" /></h2>
        {#if accountData.trust_id}
          <div class="community-info">
            <div class="stat-row">
              <span class="stat-label">{t('dashboard.communityId')}</span>
              <span class="stat-value stat-address">{accountData.trust_id.slice(0, 8)}...</span>
            </div>
            <a href="/communities" class="btn-view">{t('dashboard.viewCommunityDetails')}</a>
          </div>
        {:else}
          <div class="no-community">
            <p>{t('dashboard.notCommunityMember')}</p>
            <a href="/communities" class="btn-join">{t('dashboard.browseCommunities')}</a>
          </div>
        {/if}
      </div>

      <!-- Contributions Card -->
      <div class="card">
        <h2>{t('dashboard.contributions')} <Tooltip term="contributions" /></h2>
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

    <!-- Role-Based Panels -->
    {#if rolesLoading}
      <div class="role-panels-loading">
        <span class="spinner-inline"></span>
        {t('roles.loading')}
      </div>
    {:else if isGovernor || isVerifier || isOracle}
      <div class="role-panels">
        <h2>{t('dashboard.rolePanels')}</h2>

        <!-- Governor Panel -->
        {#if isGovernor && governorData}
          <div class="card role-panel governor-panel">
            <div class="role-panel-header">
              <h2>{t('roles.governorPanel.title')} <Tooltip term="governorPanel" /></h2>
              <span class="role-badge badge-governor">Governor</span>
            </div>
            <p class="role-subtitle">{t('roles.governorPanel.subtitle')}</p>

            <div class="role-stats-grid">
              <div class="role-stat">
                <span class="stat-label">{t('roles.governorPanel.communityName')}</span>
                <span class="stat-value">{governorData.communityName}</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.governorPanel.memberCount')}</span>
                <span class="stat-value">{governorData.memberCount}</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.governorPanel.demurrageRate')}</span>
                <span class="stat-value">{(governorData.annualRateBps / 100).toFixed(1)}%</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.governorPanel.demurragePeriod')}</span>
                <span class="stat-value">{governorData.demurragePeriodDays} {t('roles.governorPanel.days')}</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.governorPanel.governorReputation')} <Tooltip term="governorStake" /></span>
                <span class="stat-value {reputationClass(governorData.governorReputation)}">
                  {formatReputation(governorData.governorReputation)}
                </span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.governorPanel.verifierFundBalance')} <Tooltip term="communityFund" /></span>
                <span class="stat-value">{formatBalance(governorData.verifierFundBalance)} KCHNG</span>
              </div>
            </div>

            <div class="role-actions">
              <a href="/communities" class="btn-role-action">{t('roles.governorPanel.designateSuccessor')}</a>
              <a href="/governance" class="btn-role-action btn-secondary">{t('roles.governorPanel.stepDown')}</a>
            </div>
          </div>
        {/if}

        <!-- Verifier Panel -->
        {#if isVerifier && verifierRoleData}
          <div class="card role-panel verifier-panel">
            <div class="role-panel-header">
              <h2>{t('roles.verifierPanel.title')} <Tooltip term="verifierPanel" /></h2>
              <span class="role-badge badge-verifier">Verifier</span>
            </div>
            <p class="role-subtitle">{t('roles.verifierPanel.subtitle')}</p>

            <div class="role-stats-grid">
              <div class="role-stat">
                <span class="stat-label">{t('roles.verifierPanel.stake')} <Tooltip term="verifierStake" /></span>
                <span class="stat-value">{formatBalance(verifierRoleData.stake)} KCHNG</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.verifierPanel.reputation')}</span>
                <span class="stat-value {reputationClass(verifierRoleData.reputationScore)}">
                  {formatReputation(verifierRoleData.reputationScore)}
                </span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.verifierPanel.verifiedClaims')}</span>
                <span class="stat-value success">{verifierRoleData.verifiedClaims}</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.verifierPanel.rejectedClaims')}</span>
                <span class="stat-value">{verifierRoleData.rejectedClaims}</span>
              </div>
            </div>

            <div class="pending-claims-section">
              <h3>
                {t('roles.verifierPanel.pendingReview')} <Tooltip term="verifierPendingClaims" />
                {#if verifierRoleData.pendingClaimIds.length > 0}
                  <span class="pending-count">{verifierRoleData.pendingClaimIds.length}</span>
                {/if}
              </h3>
              {#if verifierRoleData.pendingClaimIds.length > 0}
                <a href="/work/verify" class="btn-role-action">
                  {t('roles.verifierPanel.viewClaims')} ({verifierRoleData.pendingClaimIds.length})
                </a>
              {:else}
                <p class="no-pending">{t('roles.verifierPanel.noPending')}</p>
              {/if}
            </div>
          </div>
        {/if}

        <!-- Oracle Panel -->
        {#if isOracle && oracleRoleData}
          <div class="card role-panel oracle-panel">
            <div class="role-panel-header">
              <h2>{t('roles.oraclePanel.title')} <Tooltip term="oraclePanel" /></h2>
              <span class="role-badge badge-oracle">Oracle</span>
            </div>
            <p class="role-subtitle">{t('roles.oraclePanel.subtitle')}</p>

            <div class="role-stats-grid">
              <div class="role-stat">
                <span class="stat-label">{t('roles.oraclePanel.stake')} <Tooltip term="oracleStake" /></span>
                <span class="stat-value">{formatBalance(oracleRoleData.stake)} KCHNG</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.oraclePanel.reputation')}</span>
                <span class="stat-value {reputationClass(oracleRoleData.reputationScore)}">
                  {formatReputation(oracleRoleData.reputationScore)}
                </span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.oraclePanel.gracePeriodsGranted')}</span>
                <span class="stat-value">{oracleRoleData.gracePeriodsGranted}</span>
              </div>
              <div class="role-stat">
                <span class="stat-label">{t('roles.oraclePanel.grantsThisYear')}</span>
                <span class="stat-value">{oracleRoleData.grantsThisYear}</span>
              </div>
            </div>

            <div class="role-actions">
              <a href="/communities" class="btn-role-action">{t('roles.oraclePanel.activateGrace')}</a>
              <a href="/communities" class="btn-role-action btn-secondary">{t('roles.oraclePanel.viewMembers')}</a>
            </div>
          </div>
        {/if}
      </div>
    {/if}

    <!-- Member Panel (always shown for community members) -->
    {#if accountData.trust_id && memberReputation !== null}
      <div class="card role-panel member-panel">
        <div class="role-panel-header">
          <h2>{t('roles.memberPanel.title')} <Tooltip term="memberPanel" /></h2>
          <span class="role-badge badge-member">Member</span>
        </div>
        <p class="role-subtitle">{t('roles.memberPanel.subtitle')}</p>

        <div class="role-stats-grid">
          <div class="role-stat">
            <span class="stat-label">{t('roles.memberPanel.community')}</span>
            <span class="stat-value stat-address">{accountData.trust_id.slice(0, 12)}...</span>
          </div>
          <div class="role-stat">
            <span class="stat-label">{t('roles.memberPanel.contributionHours')}</span>
            <span class="stat-value">{accountData.contribution_hours}h</span>
          </div>
          <div class="role-stat">
            <span class="stat-label">{t('roles.memberPanel.gracePeriodsUsed')}</span>
            <span class="stat-value">{accountData.grace_periods_used}/3 {t('roles.memberPanel.thisYear')}</span>
          </div>
          <div class="role-stat">
            <span class="stat-label">{t('roles.memberPanel.memberReputation')}</span>
            <span class="stat-value {reputationClass(memberReputation)}">
              {formatReputation(memberReputation)}
            </span>
          </div>
          {#if workerReputation !== null}
            <div class="role-stat">
              <span class="stat-label">{t('roles.memberPanel.workerReputation')}</span>
              <span class="stat-value {reputationClass(workerReputation)}">
                {formatReputation(workerReputation)}
              </span>
            </div>
          {/if}
        </div>
      </div>
    {/if}

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

        <a href="/communities" class="action-card">
          <div class="action-icon">🏘️</div>
          <div class="action-title">{t('dashboard.actionCommunities')}</div>
          <div class="action-desc">{t('dashboard.actionCommunitiesDesc')}</div>
        </a>

        <a href="/governance" class="action-card">
          <div class="action-icon">🗳️</div>
          <div class="action-title">{t('dashboard.actionGovernance')}</div>
          <div class="action-desc">{t('dashboard.actionGovernanceDesc')}</div>
        </a>
      </div>
    </div>

    <!-- Cross-Community Exchange (only for community members) -->
    {#if accountData.trust_id && availableCommunities.length > 0}
      <div class="exchange-section">
        <h2>{t('dashboard.exchangeBetweenCommunities')}</h2>
        <p class="exchange-subtitle">{t('dashboard.exchangeSubtitle')}</p>

        <div class="exchange-card">
          <div class="exchange-from">
            <div class="exchange-label">{t('dashboard.fromCommunity')}</div>
            <div class="community-badge current">Your Community</div>
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
            <select bind:value={selectedCommunityId} onchange={() => updateExchangePreview()}>
              {#each availableCommunities as community (community.id)}
                <option value={community.id}>{community.name}</option>
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
              <div class="no-rate">{t('dashboard.selectCommunityToPreview')}</div>
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
      <div class="wallet-cta-buttons">
        <button
          class="btn-wallet-cta btn-wallet-secondary"
          onclick={handleCreateTestWallet}
          disabled={isCreatingTestWallet}
        >
          {#if isCreatingTestWallet}
            <span class="btn-spinner-inline"></span>
            {t('dashboard.creatingTestAccount')}
          {:else}
            {t('dashboard.createTestAccount')}
          {/if}
        </button>
        <button
          class="btn-wallet-cta btn-wallet-primary"
          onclick={() => wallet.connect(currentNetwork)}
        >
          {t('dashboard.connectWallet')}
        </button>
      </div>
      <p class="wallet-cta-hint">{t('dashboard.walletCtaHint')}</p>
    </div>
  {/if}
</div>

<OnboardingModal />

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
    background: var(--color-error-light);
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
    background: var(--color-success-light);
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
    background: linear-gradient(135deg, var(--color-success-light) 0%, var(--color-success-lighter) 100%);
    border: 2px solid var(--color-success);
  }

  .community-protection-card h2 {
    color: var(--color-success-text);
    margin-bottom: var(--space-md);
  }

  .protection-type {
    display: flex;
    gap: var(--space-sm);
    margin-bottom: var(--space-md);
  }

  .protection-label {
    color: var(--color-success-text);
    font-weight: 500;
  }

  .protection-value {
    color: var(--color-success-text);
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
    color: var(--color-success-text);
  }

  .protection-stat .stat-value {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--color-success-text);
  }

  .protection-days {
    font-size: var(--font-size-2xl) !important;
  }

  .stat-value.verified {
    color: var(--color-success);
  }

  .stat-value.pending {
    color: var(--color-warning);
  }

  .extension-info {
    background: rgba(255, 255, 255, 0.5);
    padding: var(--space-sm);
    border-radius: var(--radius-sm);
    margin-bottom: var(--space-md);
    font-size: var(--font-size-sm);
  }

  .extension-label {
    color: var(--color-success-text);
  }

  .extension-value {
    color: var(--color-success-text);
    font-weight: 600;
  }

  .protection-note {
    font-size: var(--font-size-sm);
    color: var(--color-success-text);
    font-style: italic;
    margin: 0;
  }

  .community-info, .no-community {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .no-community p {
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

  /* Wallet CTA buttons in no-wallet state */
  .wallet-cta-buttons {
    display: flex;
    justify-content: center;
    gap: var(--space-md);
    margin-top: var(--space-lg);
    flex-wrap: wrap;
  }

  .btn-wallet-cta {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-xs);
    padding: 0.75rem 1.5rem;
    border-radius: var(--radius-md);
    font-weight: 600;
    font-size: var(--font-size-base);
    cursor: pointer;
    transition: all 0.2s;
    border: none;
    margin-top: 0;
  }

  .btn-wallet-primary {
    background: var(--color-gradient);
    color: white;
    box-shadow: var(--shadow-md);
  }

  .btn-wallet-primary:hover {
    transform: translateY(-2px);
    box-shadow: var(--shadow-lg);
  }

  .btn-wallet-secondary {
    background: var(--color-bg);
    color: var(--color-text);
    border: 2px solid var(--color-border);
  }

  .btn-wallet-secondary:hover:not(:disabled) {
    border-color: var(--color-primary);
    color: var(--color-primary);
    transform: translateY(-2px);
  }

  .btn-wallet-secondary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-spinner-inline {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(0, 0, 0, 0.2);
    border-top-color: var(--color-text);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    display: inline-block;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .wallet-cta-hint {
    margin-top: var(--space-md);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-style: italic;
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

  /* Cross-Community Exchange Styles */
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
    background: linear-gradient(135deg, var(--color-success-light) 0%, var(--color-success-lighter) 100%);
    border: 2px solid var(--color-success);
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

  .community-badge.current {
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
    color: var(--color-success-text);
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
    color: var(--color-success);
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
    border: 2px solid var(--color-success);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    text-align: center;
  }

  .result-amount {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--color-success-text);
  }

  .result-meals {
    font-size: var(--font-size-sm);
    color: var(--color-success-text);
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
    color: var(--color-success-text);
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
    background: var(--color-success-light);
    color: var(--color-success-text);
  }

  .exchange-message.error {
    background: var(--color-error-light);
    color: var(--color-error-text);
  }

  .exchange-message.info {
    background: var(--color-info-light);
    color: var(--color-info-text);
  }

  .exchange-actions {
    grid-column: 1 / -1;
    display: flex;
    justify-content: center;
    margin-top: var(--space-md);
  }

  .btn-exchange {
    padding: var(--space-md) var(--space-xl);
    background: linear-gradient(135deg, var(--color-success) 0%, var(--color-success) 100%);
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

  /* Role Panels */
  .role-panels-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-sm);
    padding: var(--space-lg);
    color: var(--color-text-muted);
    font-style: italic;
  }

  .spinner-inline {
    width: 16px;
    height: 16px;
    border: 2px solid var(--color-border);
    border-top-color: var(--color-primary);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
    display: inline-block;
  }

  .role-panels {
    margin-top: var(--space-xl);
  }

  .role-panels h2 {
    font-size: var(--font-size-xl);
    margin-bottom: var(--space-md);
    color: var(--color-text);
  }

  .role-panel {
    margin-bottom: var(--space-lg);
    border-left: 4px solid var(--color-border);
  }

  .governor-panel {
    border-left-color: var(--color-primary, #667eea);
  }

  .verifier-panel {
    border-left-color: var(--color-success, #10b981);
  }

  .oracle-panel {
    border-left-color: var(--color-warning, #f59e0b);
  }

  .member-panel {
    border-left-color: var(--color-info, #3b82f6);
  }

  .role-panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: var(--space-sm);
    margin-bottom: var(--space-xs);
  }

  .role-panel-header h2 {
    margin-bottom: 0;
    font-size: var(--font-size-lg);
  }

  .role-subtitle {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    margin: 0 0 var(--space-md) 0;
  }

  .role-badge {
    display: inline-block;
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-full, 9999px);
    font-size: var(--font-size-xs);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .badge-governor {
    background: rgba(102, 126, 234, 0.15);
    color: var(--color-primary, #667eea);
  }

  .badge-verifier {
    background: rgba(16, 185, 129, 0.15);
    color: var(--color-success, #10b981);
  }

  .badge-oracle {
    background: rgba(245, 158, 11, 0.15);
    color: var(--color-warning, #f59e0b);
  }

  .badge-member {
    background: rgba(59, 130, 246, 0.15);
    color: var(--color-info, #3b82f6);
  }

  .role-stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: var(--space-md);
    margin-bottom: var(--space-md);
  }

  .role-stat {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    padding: var(--space-sm);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-sm);
  }

  .role-stat .stat-label {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    display: flex;
    align-items: center;
    gap: var(--space-xs);
  }

  .role-stat .stat-value {
    font-size: var(--font-size-lg);
    font-weight: 600;
  }

  /* Reputation color coding */
  .rep-excellent {
    color: var(--color-success);
  }

  .rep-good {
    color: var(--color-text);
  }

  .rep-probation {
    color: var(--color-warning);
  }

  .rep-at-risk {
    color: var(--color-error);
  }

  .role-actions {
    display: flex;
    gap: var(--space-sm);
    flex-wrap: wrap;
  }

  .btn-role-action {
    display: inline-block;
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-sm);
    font-weight: 500;
    font-size: var(--font-size-sm);
    text-decoration: none;
    cursor: pointer;
    border: 1px solid var(--color-primary, #667eea);
    color: var(--color-primary, #667eea);
    background: transparent;
    transition: all 0.2s;
    margin-top: 0;
  }

  .btn-role-action:hover {
    background: var(--color-primary, #667eea);
    color: white;
  }

  .btn-role-action.btn-secondary {
    border-color: var(--color-border);
    color: var(--color-text-muted);
  }

  .btn-role-action.btn-secondary:hover {
    background: var(--color-border-light);
    color: var(--color-text);
  }

  .pending-claims-section {
    margin-top: var(--space-md);
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border-light);
  }

  .pending-claims-section h3 {
    font-size: var(--font-size-base);
    margin-bottom: var(--space-sm);
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .pending-count {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.25rem;
    height: 1.25rem;
    padding: 0 0.35rem;
    background: var(--color-warning, #f59e0b);
    color: white;
    border-radius: var(--radius-full, 9999px);
    font-size: var(--font-size-xs);
    font-weight: 700;
  }

  .no-pending {
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    font-style: italic;
    margin: 0;
  }

  @media (max-width: 640px) {
    .role-stats-grid {
      grid-template-columns: 1fr 1fr;
    }

    .role-panel-header {
      flex-direction: column;
      align-items: flex-start;
    }
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
