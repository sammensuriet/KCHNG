<script lang="ts">
  import { wallet } from "$lib/stores/wallet";
  import {
    calculateInactivePeriods,
    calculateDemurrageAmount,
    SECONDS_PER_DAY,
    DEMURRAGE_PERIOD_DAYS,
    DEMURRAGE_AMOUNT,
  } from "@kchng/shared";

  interface Props {
    compact?: boolean;
  }

  let { compact = false }: Props = $props();

  // Calculate time until next burn
  const timeUntilNextBurn = $derived(() => {
    if (!$wallet.lastActivity) return null;

    const now = Math.floor(Date.now() / 1000);
    const elapsed = now - $wallet.lastActivity;

    if (elapsed < DEMURRAGE_PERIOD_DAYS * SECONDS_PER_DAY) {
      // Time remaining in current period
      const periodEnd = $wallet.lastActivity + DEMURRAGE_PERIOD_DAYS * SECONDS_PER_DAY;
      return periodEnd - now;
    }

    // Already in burn territory
    return 0;
  });

  // Calculate projected demurrage
  const projectedDemurrage = $derived(() => {
    if (!$wallet.lastActivity) return 0n;
    const now = Math.floor(Date.now() / 1000);
    const periods = calculateInactivePeriods($wallet.lastActivity, now);
    return calculateDemurrageAmount(periods);
  });

  // Format seconds into human readable time
  function formatTime(seconds: number): string {
    if (seconds <= 0) return "Now";

    const days = Math.floor(seconds / SECONDS_PER_DAY);
    const hours = Math.floor((seconds % SECONDS_PER_DAY) / 3600);

    if (days > 0) {
      return `${days} day${days > 1 ? "s" : ""}`;
    }
    if (hours > 0) {
      return `${hours} hour${hours > 1 ? "s" : ""}`;
    }
    return "< 1 hour";
  }

  // Calculate percentage of current period used
  const periodProgress = $derived(() => {
    if (!$wallet.lastActivity) return 0;
    const now = Math.floor(Date.now() / 1000);
    const elapsed = now - $wallet.lastActivity;
    const periodSeconds = DEMURRAGE_PERIOD_DAYS * SECONDS_PER_DAY;
    return Math.min((elapsed / periodSeconds) * 100, 100);
  });
</script>

{#if compact}
  <div class="demurrage-compact">
    {#if timeUntilNextBurn() && timeUntilNextBurn()! > 0}
      <span class="demurrage-safe">Burn in: {formatTime(timeUntilNextBurn()!)}</span>
    {:else}
      <span class="demurrage-warning">Burning now!</span>
    {/if}
  </div>
{:else}
  <div class="demurrage-info">
    <h3>Demurrage Status</h3>

    {#if timeUntilNextBurn() && timeUntilNextBurn()! > 0}
      <div class="demurrage-section">
        <div class="demurrage-label">Time until next burn</div>
        <div class="demurrage-value safe">{formatTime(timeUntilNextBurn()!)}</div>
      </div>

      <div class="demurrage-section">
        <div class="demurrage-label">Current period progress</div>
        <div class="progress-bar">
          <div class="progress-fill" style="width: {periodProgress()}%"></div>
        </div>
        <div class="progress-text">{Math.round(periodProgress())}% of 7 days</div>
      </div>

      <div class="demurrage-notice">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 1a7 7 0 100 14A7 7 0 008 1zM7 4h2v4H7V4zm0 5h2v2H7V9z"/>
        </svg>
        <span>
          Send any transaction to reset the timer and prevent demurrage
        </span>
      </div>
    {:else}
      <div class="demurrage-section">
        <div class="demurrage-label">Status</div>
        <div class="demurrage-value warning">Burning now!</div>
      </div>

      <div class="demurrage-section">
        <div class="demurrage-label">Projected burn</div>
        <div class="demurrage-value">
          {projectedDemurrage().toString()} KCHNG / 7 days
        </div>
      </div>

      <div class="demurrage-notice warning">
        <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 1a7 7 0 100 14A7 7 0 008 1zm1 11H7v-2h2v2zm0-4H7V4h2v4z"/>
        </svg>
        <span>
          Your KCHNG is being burned due to inactivity. Make a transaction now to stop it!
        </span>
      </div>
    {/if}
  </div>
{/if}

<style>
  .demurrage-compact {
    font-size: 0.75rem;
  }

  .demurrage-safe {
    color: #059669;
  }

  .demurrage-warning {
    color: #dc2626;
    font-weight: 500;
  }

  .demurrage-info {
    padding: 0.75rem 0;
  }

  .demurrage-info h3 {
    margin: 0 0 0.75rem 0;
    font-size: 0.875rem;
    font-weight: 600;
    color: #374151;
  }

  .demurrage-section {
    margin-bottom: 0.75rem;
  }

  .demurrage-label {
    font-size: 0.75rem;
    color: #6b7280;
    margin-bottom: 0.25rem;
  }

  .demurrage-value {
    font-size: 0.875rem;
    font-weight: 500;
    color: #111827;
  }

  .demurrage-value.safe {
    color: #059669;
  }

  .demurrage-value.warning {
    color: #dc2626;
  }

  .progress-bar {
    width: 100%;
    height: 6px;
    background: #e5e7eb;
    border-radius: 3px;
    overflow: hidden;
    margin-bottom: 0.25rem;
  }

  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
    transition: width 0.3s ease;
  }

  .progress-text {
    font-size: 0.75rem;
    color: #6b7280;
  }

  .demurrage-notice {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    padding: 0.5rem;
    background: #f0fdf4;
    border-radius: 4px;
    font-size: 0.75rem;
    color: #166534;
  }

  .demurrage-notice.warning {
    background: #fef2f2;
    color: #991b1b;
  }

  .demurrage-notice svg {
    flex-shrink: 0;
    margin-top: 0.125rem;
  }
</style>
