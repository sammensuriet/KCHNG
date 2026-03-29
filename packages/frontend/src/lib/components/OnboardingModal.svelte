<script lang="ts">
  import { t, messages } from "$lib/i18n";
  import { browser } from "$app/environment";
  import { wallet } from "$lib/stores/wallet";
  import { goto } from "$app/navigation";

  const STORAGE_KEY = "kchng_onboarding_complete";
  const TOTAL_STEPS = 3;

  let currentStep = $state(0);
  let dismissed = $state(false);

  function hasCompletedOnboarding(): boolean {
    if (!browser) return true;
    return localStorage.getItem(STORAGE_KEY) === "true";
  }

  function markComplete() {
    if (browser) {
      localStorage.setItem(STORAGE_KEY, "true");
    }
    dismissed = true;
  }

  function shouldShow(): boolean {
    return $wallet.connected && !hasCompletedOnboarding() && !dismissed;
  }

  function next() {
    if (currentStep < TOTAL_STEPS - 1) {
      currentStep++;
    } else {
      markComplete();
    }
  }

  function back() {
    if (currentStep > 0) {
      currentStep--;
    }
  }

  function skip() {
    markComplete();
  }

  async function goToCommunities() {
    markComplete();
    await goto("/communities");
  }

  async function goToWork() {
    markComplete();
    await goto("/work");
  }
</script>

{#if shouldShow()}
  <div class="onboarding-overlay" onclick={skip}>
    <div class="onboarding-modal" onclick={(e) => e.stopPropagation()}>
      <!-- Progress Bar -->
      <div class="progress-bar">
        {#each Array(TOTAL_STEPS) as _, i}
          <div class="progress-segment" class:active={i <= currentStep} class:current={i === currentStep}></div>
        {/each}
      </div>

      <!-- Step Indicator -->
      <div class="step-indicator">
        {t('onboarding.step')
          .replace('{current}', String(currentStep + 1))
          .replace('{total}', String(TOTAL_STEPS))}
      </div>

      <!-- Step Content -->
      {#if currentStep === 0}
        <div class="step-content">
          <div class="step-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"></circle>
              <path d="M8 14s1.5 2 4 2 4-2 4-2"></path>
              <line x1="9" y1="9" x2="9.01" y2="9"></line>
              <line x1="15" y1="9" x2="15.01" y2="9"></line>
            </svg>
          </div>
          <h2>{t('onboarding.steps.welcome.title')}</h2>
          <p class="step-description">{t('onboarding.steps.welcome.description')}</p>

          <div class="feature-cards">
            <div class="feature-card">
              <div class="feature-icon earn">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M12 2L2 7l10 5 10-5-10-5z"></path>
                  <path d="M2 17l10 5 10-5"></path>
                  <path d="M2 12l10 5 10-5"></path>
                </svg>
              </div>
              <div class="feature-text">
                <strong>{t('onboarding.steps.welcome.feature1Title')}</strong>
                <span>{t('onboarding.steps.welcome.feature1Desc')}</span>
              </div>
            </div>
            <div class="feature-card">
              <div class="feature-icon spend">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
                  <line x1="1" y1="1" x2="23" y2="23"></line>
                </svg>
              </div>
              <div class="feature-text">
                <strong>{t('onboarding.steps.welcome.feature2Title')}</strong>
                <span>{t('onboarding.steps.welcome.feature2Desc')}</span>
              </div>
            </div>
            <div class="feature-card">
              <div class="feature-icon circulate">
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="23 4 23 10 17 10"></polyline>
                  <polyline points="1 20 1 14 7 14"></polyline>
                  <path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"></path>
                </svg>
              </div>
              <div class="feature-text">
                <strong>{t('onboarding.steps.welcome.feature3Title')}</strong>
                <span>{t('onboarding.steps.welcome.feature3Desc')}</span>
              </div>
            </div>
          </div>
        </div>
      {:else if currentStep === 1}
        <div class="step-content">
          <div class="step-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"></path>
              <circle cx="9" cy="7" r="4"></circle>
              <path d="M23 21v-2a4 4 0 0 0-3-3.87"></path>
              <path d="M16 3.13a4 4 0 0 1 0 7.75"></path>
            </svg>
          </div>
          <h2>{t('onboarding.steps.community.title')}</h2>
          <p class="step-description">{t('onboarding.steps.community.description')}</p>

          <button class="btn-action" onclick={goToCommunities}>
            {t('onboarding.steps.community.action')}
          </button>
          <p class="action-hint">{t('onboarding.steps.community.actionDesc')}</p>
        </div>
      {:else if currentStep === 2}
        <div class="step-content">
          <div class="step-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
              <polyline points="14 2 14 8 20 8"></polyline>
              <line x1="16" y1="13" x2="8" y2="13"></line>
              <line x1="16" y1="17" x2="8" y2="17"></line>
              <polyline points="10 9 9 9 8 9"></polyline>
            </svg>
          </div>
          <h2>{t('onboarding.steps.work.title')}</h2>
          <p class="step-description">{t('onboarding.steps.work.description')}</p>

          <button class="btn-action" onclick={goToWork}>
            {t('onboarding.steps.work.action')}
          </button>
          <p class="action-hint">{t('onboarding.steps.work.actionDesc')}</p>
        </div>
      {/if}

      <!-- Navigation -->
      <div class="step-nav">
        <div class="nav-left">
          {#if currentStep > 0}
            <button class="btn-back" onclick={back}>{t('onboarding.back')}</button>
          {/if}
        </div>
        <div class="nav-right">
          {#if currentStep > 0}
            <button class="btn-skip" onclick={skip}>{t('onboarding.steps.community.skip')}</button>
          {/if}
          <button class="btn-next" onclick={next}>
            {#if currentStep === TOTAL_STEPS - 1}
              {t('onboarding.done')}
            {:else}
              {t('onboarding.next')}
            {/if}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .onboarding-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 300;
    padding: 1rem;
  }

  .onboarding-modal {
    background: var(--color-bg);
    border-radius: var(--radius-lg);
    width: 100%;
    max-width: 480px;
    box-shadow: var(--shadow-lg);
    overflow: hidden;
    animation: slideUp 0.3s ease;
  }

  @keyframes slideUp {
    from { opacity: 0; transform: translateY(1rem); }
    to { opacity: 1; transform: translateY(0); }
  }

  .progress-bar {
    display: flex;
    gap: 4px;
    padding: 1rem 1.5rem 0;
  }

  .progress-segment {
    flex: 1;
    height: 4px;
    border-radius: 2px;
    background: var(--color-border);
    transition: background 0.3s;
  }

  .progress-segment.active {
    background: var(--color-primary);
  }

  .progress-segment.current {
    background: var(--color-gradient);
  }

  .step-indicator {
    text-align: center;
    padding: 0.5rem 1.5rem 0;
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .step-content {
    padding: 1rem 1.5rem 1.5rem;
    text-align: center;
  }

  .step-icon {
    margin-bottom: 1rem;
    color: var(--color-primary);
  }

  .step-content h2 {
    margin: 0 0 0.5rem;
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--color-text-darker);
  }

  .step-description {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    line-height: 1.5;
    margin: 0 0 1.25rem;
  }

  .feature-cards {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    text-align: left;
  }

  .feature-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem;
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
  }

  .feature-icon {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .feature-icon.earn {
    background: var(--color-success-light);
    color: var(--color-success-text);
  }

  .feature-icon.spend {
    background: var(--color-warning-light);
    color: var(--color-warning-text);
  }

  .feature-icon.circulate {
    background: var(--color-info-light);
    color: var(--color-info-text);
  }

  .feature-text {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .feature-text strong {
    font-size: var(--font-size-sm);
    color: var(--color-text-darker);
  }

  .feature-text span {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
  }

  .btn-action {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.625rem 1.25rem;
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-action:hover {
    opacity: 0.9;
  }

  .action-hint {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin: 0.5rem 0 0;
  }

  .step-nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-top: 1px solid var(--color-border);
  }

  .nav-left, .nav-right {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .btn-back {
    padding: 0.5rem 1rem;
    background: var(--color-bg-subtle);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-back:hover {
    background: var(--color-border-light);
  }

  .btn-skip {
    padding: 0.5rem 0.75rem;
    background: none;
    border: none;
    color: var(--color-text-muted);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: color 0.2s;
  }

  .btn-skip:hover {
    color: var(--color-text);
  }

  .btn-next {
    padding: 0.5rem 1.25rem;
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: var(--radius-md);
    font-size: var(--font-size-sm);
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-next:hover {
    opacity: 0.9;
  }

  @media (max-width: 480px) {
    .onboarding-modal {
      max-width: 100%;
      margin: 0.5rem;
    }
  }
</style>
