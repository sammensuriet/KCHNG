<script lang="ts">
  import { wallet, truncatedAddress, formattedBalance, type NetworkName } from "$lib/stores/wallet";
  import { get } from "svelte/store";
  import { page } from "$app/stores";
  import { browser } from "$app/environment";
  import DemurrageInfo from "./DemurrageInfo.svelte";
  import LanguageSwitcher from "./LanguageSwitcher.svelte";
  import { t, messages } from "$lib/i18n";

  let showMenu = $state(false);
  let showNetworkSelector = $state(false);
  let currentNetwork = $state(get(wallet).network);

  // Test wallet creation state
  let isCreatingTestWallet = $state(false);
  let showTestWalletBanner = $state(false);
  let testWalletPublicKey = $state("");
  let testWalletSecretKey = $state("");
  let showSecretKey = $state(false);
  let secretKeyCopied = $state(false);

  // Feedback form state
  let showFeedbackForm = $state(false);
  let feedbackFirstName = $state("");
  let feedbackEmail = $state("");
  let feedbackPhone = $state("");
  let feedbackCategory = $state("general");
  let feedbackMessage = $state("");
  let contactConsent = $state(false);
  let feedbackStatus = $state<"idle" | "submitting" | "success" | "error">("idle");
  let honeypot = $state("");

  // Subscribe to wallet changes to sync network state
  $effect(() => {
    currentNetwork = get(wallet).network;
  });

  // Reactive arrays that update when language changes
  // Access $messages to create reactive dependency
  // Testnet-only during early-stage development
  const networks = $derived<{ id: NetworkName; label: string }[]>([
    { id: "testnet", label: ($messages, t('header.testnet')) },
  ]);

  const feedbackCategories = $derived([
    { id: "general", label: ($messages, t('header.generalFeedback')) },
    { id: "work-claims", label: ($messages, t('header.workClaims')) },
    { id: "communities", label: ($messages, t('header.communities')) },
    { id: "wallet", label: ($messages, t('header.walletConnection')) },
    { id: "governance", label: ($messages, t('header.governance')) },
    { id: "bug", label: ($messages, t('header.bugReport')) },
    { id: "feature", label: ($messages, t('header.featureRequest')) },
    { id: "kchng-request", label: ($messages, t('header.kchngRequest')) },
  ]);

  async function switchNetwork(network: NetworkName) {
    showNetworkSelector = false;
    currentNetwork = network;
  }

  async function handleCreateTestWallet() {
    if (isCreatingTestWallet) return;

    isCreatingTestWallet = true;
    try {
      const walletData = await wallet.createTestWallet();
      console.log("Test wallet created:", walletData.publicKey);
      // Store wallet data and show warning banner instead of auto-copying
      testWalletPublicKey = walletData.publicKey;
      testWalletSecretKey = walletData.secretKey;
      showSecretKey = false;
      secretKeyCopied = false;
      showTestWalletBanner = true;
    } catch (error) {
      console.error("Failed to create test wallet:", error);
      alert("Failed to create test wallet. Please try again.");
    } finally {
      isCreatingTestWallet = false;
    }
  }

  async function copySecretKey() {
    await navigator.clipboard.writeText(testWalletSecretKey);
    secretKeyCopied = true;
    setTimeout(() => {
      secretKeyCopied = false;
    }, 2000);
  }

  function closeTestWalletBanner() {
    showTestWalletBanner = false;
    testWalletPublicKey = "";
    testWalletSecretKey = "";
    showSecretKey = false;
    secretKeyCopied = false;
  }

  async function submitFeedback(event: Event) {
    event.preventDefault();

    // Bot protection: check honeypot
    if (honeypot) {
      console.log("Bot detected, ignoring submission");
      showFeedbackForm = false;
      return;
    }

    if (!feedbackMessage.trim()) {
      feedbackStatus = "error";
      return;
    }

    feedbackStatus = "submitting";

    try {
      const params = new URLSearchParams();
      params.append("form-name", "feedback");
      params.append("firstName", feedbackFirstName);
      params.append("email", feedbackEmail);
      params.append("phone", feedbackPhone);
      params.append("category", feedbackCategory);
      params.append("message", feedbackMessage);
      params.append("page", browser ? window.location.pathname : "unknown");
      params.append("network", get(wallet).network || "unknown");
      params.append("walletAddress", get(wallet).address || "not connected");
      params.append("contactConsent", contactConsent ? "yes" : "no");
      params.append("bot-field", honeypot);

      const response = await fetch("/", {
        method: "POST",
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
        body: params.toString(),
      });

      if (response.ok) {
        feedbackStatus = "success";
        feedbackMessage = "";
        feedbackFirstName = "";
        feedbackEmail = "";
        feedbackPhone = "";
        feedbackCategory = "general";
        contactConsent = false;
        setTimeout(() => {
          showFeedbackForm = false;
          feedbackStatus = "idle";
        }, 2000);
      } else {
        throw new Error("Submission failed");
      }
    } catch (error) {
      feedbackStatus = "error";
      console.error("Feedback submission error:", error);
    }
  }

  function closeFeedbackForm() {
    showFeedbackForm = false;
    feedbackStatus = "idle";
    feedbackMessage = "";
    feedbackFirstName = "";
    feedbackEmail = "";
    feedbackPhone = "";
    feedbackCategory = "general";
    contactConsent = false;
    honeypot = "";
  }
</script>

<header class="header">
  <div class="header-left">
    <a href="/" class="logo">KCHNG</a>
  </div>

  <nav class="header-nav">
    <a href="/about" class="nav-link" class:active={$page.url.pathname === "/about"}>{t('nav.about')}</a>
    <a href="/dashboard" class="nav-link" class:active={$page.url.pathname === "/dashboard"}>{t('nav.dashboard')}</a>
    <a href="/work" class="nav-link" class:active={$page.url.pathname.startsWith("/work")}>{t('nav.work')}</a>
    <a href="/communities" class="nav-link" class:active={$page.url.pathname === "/communities"}>{t('nav.communities')}</a>
    <a href="/governance" class="nav-link" class:active={$page.url.pathname === "/governance"}>{t('nav.governance')}</a>
    <a href="/communicate" class="nav-link" class:active={$page.url.pathname === "/communicate"}>{t('nav.chat')}</a>
  </nav>

  <div class="header-right">
    <!-- Feedback Button -->
    <LanguageSwitcher />

    <button
      class="btn-feedback"
      onclick={() => (showFeedbackForm = !showFeedbackForm)}
      title={t('header.submitFeedback')}
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
      </svg>
      <span class="feedback-text">{t('header.feedback')}</span>
    </button>

    {#if $wallet.error}
      <div class="error-message">
        {$wallet.error}
        <button class="error-dismiss" onclick={() => wallet.disconnect()}>×</button>
      </div>
    {/if}

    {#if !$wallet.connected}
      <div class="network-badge">TESTNET</div>
      {#if currentNetwork === "testnet"}
        <button
          class="btn-test-wallet"
          onclick={handleCreateTestWallet}
          disabled={isCreatingTestWallet}
          title={t('header.createTestWallet')}
        >
          {#if isCreatingTestWallet}
            <span class="btn-spinner-small"></span>
            {t('header.creatingWallet')}
          {:else}
            {t('header.createTestWallet')}
          {/if}
        </button>
      {/if}
      <button class="btn-connect" onclick={() => wallet.connect(currentNetwork)}>
        {t('common.connectWallet')}
      </button>
    {:else}
      <div class="wallet-info">
        <button class="btn-wallet" onclick={() => (showMenu = !showMenu)}>
          <span class="wallet-address">{$truncatedAddress}</span>
          <span class="wallet-balance">{$formattedBalance} KCHNG</span>
        </button>

        {#if showMenu}
          <div class="wallet-dropdown">
            <div class="dropdown-section">
              <div class="dropdown-label">{t('header.connectedAs')}</div>
              <div class="dropdown-value">{$wallet.walletName}</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">{t('header.network')}</div>
              <div class="dropdown-value">{$wallet.network.toUpperCase()}</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">{t('header.address')}</div>
              <div class="dropdown-value dropdown-address">{$wallet.address}</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">{t('header.balance')}</div>
              <div class="dropdown-value">{$formattedBalance} KCHNG</div>
            </div>

            <div class="dropdown-section">
              <div class="dropdown-label">{t('header.demurrage')}</div>
              <DemurrageInfo compact={true} />
            </div>

            <hr class="dropdown-divider" />

            <button
              class="btn-disconnect"
              onclick={() => wallet.disconnect()}
            >
              {t('common.disconnect')}
            </button>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</header>

<!-- Feedback Modal -->
{#if showFeedbackForm}
  <div class="feedback-overlay" onclick={closeFeedbackForm}>
    <div class="feedback-modal" onclick={(e) => e.stopPropagation()}>
      <div class="feedback-header">
        <h3>{t('header.submitFeedback')}</h3>
        <button class="btn-close" onclick={closeFeedbackForm} title={t('header.close')}>×</button>
      </div>

      <form name="feedback" method="POST" data-netlify="true" netlify-honeypot="bot-field" onsubmit={submitFeedback}>
        <input type="hidden" name="form-name" value="feedback" />
        <!-- Honeypot field for bot protection -->
        <input
          type="text"
          name="bot-field"
          bind:value={honeypot}
          class="honeypot"
          tabindex="-1"
          autocomplete="off"
        />

        <div class="form-row">
          <div class="form-group">
            <label for="feedback-firstName">{t('header.firstName')}</label>
            <input
              type="text"
              id="feedback-firstName"
              name="firstName"
              bind:value={feedbackFirstName}
              placeholder={t('header.yourName')}
              disabled={feedbackStatus === "submitting"}
            />
          </div>
          <div class="form-group">
            <label for="feedback-category">{t('header.category')}</label>
            <select id="feedback-category" bind:value={feedbackCategory} disabled={feedbackStatus === "submitting"}>
              {#each feedbackCategories as cat}
                <option value={cat.id}>{cat.label}</option>
              {/each}
            </select>
          </div>
        </div>

        <div class="form-row">
          <div class="form-group">
            <label for="feedback-email">{t('header.email')}</label>
            <input
              type="email"
              id="feedback-email"
              name="email"
              bind:value={feedbackEmail}
              placeholder={t('header.emailPlaceholder')}
              disabled={feedbackStatus === "submitting"}
            />
          </div>
          <div class="form-group">
            <label for="feedback-phone">{t('header.phone')}</label>
            <input
              type="tel"
              id="feedback-phone"
              name="phone"
              bind:value={feedbackPhone}
              placeholder={t('header.phonePlaceholder')}
              disabled={feedbackStatus === "submitting"}
            />
          </div>
        </div>

        <div class="form-group">
          <label for="feedback-message">{t('header.yourFeedback')}</label>
          <textarea
            id="feedback-message"
            name="message"
            bind:value={feedbackMessage}
            placeholder={t('header.feedbackPlaceholder')}
            rows="4"
            required
            disabled={feedbackStatus === "submitting"}
          ></textarea>
        </div>

        <div class="form-consent">
          <label class="checkbox-label">
            <input
              type="checkbox"
              name="contactConsent"
              bind:checked={contactConsent}
              disabled={feedbackStatus === "submitting"}
            />
            <span>{t('header.contactConsent')}</span>
          </label>
        </div>

        <div class="form-meta">
          <small>{t('header.page')}: {$page.url.pathname}</small>
          <small>{t('header.network')}: {$wallet.network}</small>
          <small>{t('header.wallet')}: {$wallet.address || t('header.notConnected')}</small>
        </div>

        {#if feedbackStatus === "success"}
          <div class="feedback-success">
            <span class="success-icon">✓</span>
            {t('header.feedbackSuccess')}
          </div>
        {:else if feedbackStatus === "error"}
          <div class="feedback-error">
            <span class="error-icon">⚠</span>
            {t('header.feedbackError')}
          </div>
        {/if}

        <div class="form-actions">
          <button type="button" class="btn-cancel" onclick={closeFeedbackForm} disabled={feedbackStatus === "submitting"}>
            {t('common.cancel')}
          </button>
          {#if feedbackStatus === "submitting"}
            <button type="submit" class="btn-submit" disabled>
              <span class="btn-spinner"></span>
              {t('header.sending')}
            </button>
          {:else}
            <button type="submit" class="btn-submit" disabled={!feedbackMessage.trim()}>
              {t('header.sendFeedback')}
            </button>
          {/if}
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Test Wallet Secret Key Warning Banner -->
{#if showTestWalletBanner}
  <div class="secret-key-overlay" onclick={closeTestWalletBanner}>
    <div class="secret-key-banner" onclick={(e) => e.stopPropagation()}>
      <div class="secret-key-header">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 1.71-3L13.71 3.86a2 2 0 0 0 3.42 0z"></path>
          <line x1="12" y1="9" x2="12" y2="17"></line>
          <line x1="12" y1="17" x2="12.01" y2="17"></line>
        </svg>
        <h3>{t('header.testWalletInstructions.titleWarning')}</h3>
        <button class="btn-close-banner" onclick={closeTestWalletBanner} title={t('header.close')}>×</button>
      </div>

      <div class="secret-key-content">
        <p class="secret-key-intro">{t('header.testWalletInstructions.intro')}</p>

        <div class="secret-key-instructions">
          <p><strong>{t('header.testWalletInstructions.whatIsSecretKey')}</strong></p>
          <p>{t('header.testWalletInstructions.instructions')}</p>
          <ul>
            <li>{t('header.testWalletInstructions.instruction1')}</li>
            <li>{t('header.testWalletInstructions.instruction2')}</li>
            <li>{t('header.testWalletInstructions.instruction3')}</li>
            <li>{t('header.testWalletInstructions.instruction4')}</li>
          </ul>
        </div>

        <div class="secret-key-display">
          <div class="secret-key-label">
            <span>{t('header.yourSecretKey')}:</span>
            <button
              class="btn-toggle-visibility"
              onclick={() => (showSecretKey = !showSecretKey)}
              type="button"
            >
              {#if showSecretKey}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M17.94 17.94A10.07 10.07 0 0 1 7 3.34 3.34a2 2 0 0 1 2.83-2.83l10.07-10.07a2 2 0 0 1 2.83 0 2.83 0 0 1 0-2.83l-10.07-10.07a2 2 0 0 1 0-2.83 2.83 0 0 1 0 2.83z"></path>
                  <line x1="1" y1="1" x2="23" y2="23"></line>
                </svg>
                {t('header.hide')}
              {:else}
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M1 12s4-8 11-8 11 8 11 8 11-8z"></path>
                  <circle cx="12" cy="12" r="3"></circle>
                </svg>
                {t('header.show')}
              {/if}
            </button>
          </div>
          <div class="secret-key-value" class:hidden={!showSecretKey}>
            {#if showSecretKey}
              <code>{testWalletSecretKey}</code>
            {:else}
              <code>•••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••••</code>
            {/if}
          </div>
        </div>

        <div class="secret-key-actions">
          <button
            class="btn-copy-key"
            onclick={copySecretKey}
            disabled={secretKeyCopied}
          >
            {#if secretKeyCopied}
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="20 6 9 17 4 12"></polyline>
              </svg>
              <span>{t('header.copied')}</span>
            {:else}
              <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
              </svg>
              <span>{t('header.copySecretKey')}</span>
            {/if}
          </button>

          <button class="btn-done" onclick={closeTestWalletBanner}>
            {t('header.saveKey')}
          </button>
        </div>

        <div class="secret-key-footer">
          <p class="test-wallet-note">
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"></circle>
              <line x1="12" y1="16" x2="12" y2="12"></line>
              <line x1="12" y1="20" x2="12.01" y2="20"></line>
            </svg>
            {t('header.testWalletInstructions.testWalletNote')}
          </p>
        </div>
      </div>
    </div>
  </div>
{/if}

<style>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: var(--color-bg);
    border-bottom: 1px solid var(--color-border);
    position: sticky;
    top: 0;
    z-index: 100;
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .logo {
    font-size: 1.5rem;
    font-weight: 700;
    background: var(--color-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-decoration: none;
  }

  .header-nav {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .nav-link {
    padding: 0.5rem 0.75rem;
    color: var(--color-text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 6px;
    transition: all 0.2s;
  }

  .nav-link:hover {
    color: var(--color-text);
    background: var(--color-border-light);
  }

  .nav-link.active {
    color: var(--color-primary);
    background: var(--color-primary-light);
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    position: relative;
  }

  .network-badge {
    padding: 0.5rem 1rem;
    background: var(--color-bg-subtle);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    letter-spacing: 0.05em;
  }

  .btn-connect {
    padding: 0.5rem 1rem;
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-connect:hover {
    opacity: 0.9;
  }

  .btn-test-wallet {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 1rem;
    background: var(--color-success);
    color: white;
    border: none;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
    font-size: 0.875rem;
  }

  .btn-test-wallet:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn-test-wallet:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-spinner-small {
    width: 12px;
    height: 12px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .wallet-info {
    position: relative;
  }

  .btn-wallet {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    padding: 0.5rem 1rem;
    background: var(--color-border-light);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-wallet:hover {
    background: var(--color-border);
  }

  .wallet-address {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text);
  }

  .wallet-balance {
    font-size: 0.75rem;
    color: var(--color-text-muted);
  }

  .wallet-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    width: 280px;
    background: var(--color-bg);
    border: 1px solid var(--color-border);
    border-radius: 8px;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    padding: 1rem;
    z-index: 101;
  }

  .dropdown-section {
    margin-bottom: 0.75rem;
  }

  .dropdown-label {
    font-size: 0.75rem;
    color: var(--color-text-muted);
    margin-bottom: 0.25rem;
  }

  .dropdown-value {
    font-size: 0.875rem;
    color: var(--color-text-darker);
    font-weight: 500;
  }

  .dropdown-address {
    word-break: break-all;
    font-family: monospace;
    font-size: 0.75rem;
  }

  .dropdown-divider {
    border: none;
    border-top: 1px solid var(--color-border);
    margin: 0.75rem 0;
  }

  .btn-disconnect {
    width: 100%;
    padding: 0.5rem;
    background: var(--color-error-light);
    color: var(--color-error-text);
    border: none;
    border-radius: 4px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-disconnect:hover {
    background: var(--color-error-lighter);
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--color-error-light);
    color: var(--color-error-text);
    border-radius: 6px;
    font-size: 0.875rem;
  }

  .error-dismiss {
    background: none;
    border: none;
    color: var(--color-error-text);
    font-size: 1.25rem;
    line-height: 1;
    cursor: pointer;
    padding: 0;
    width: 1.25rem;
    height: 1.25rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .error-dismiss:hover {
    background: var(--color-error-lighter);
  }

  @media (max-width: 640px) {
    .header {
      padding: 0.75rem 1rem;
    }

    .logo {
      font-size: 1.25rem;
    }

    .header-nav {
      display: none;
    }

    /* Simplify header-right on mobile */
    .header-right {
      gap: 0.5rem;
    }

    /* Smaller network badge on mobile */
    .network-badge {
      padding: 0.375rem 0.75rem;
      font-size: 0.75rem;
    }

    /* Hide feedback button text and reduce size */
    .btn-feedback {
      padding: 0.5rem;
      border: none;
      background: transparent;
    }

    /* Ensure wallet button is tappable */
    .btn-wallet {
      min-height: 44px;
      padding: 0.625rem 1rem;
    }

    .wallet-dropdown {
      position: fixed;
      bottom: 0;
      left: 0;
      right: 0;
      top: auto;
      width: 100%;
      border-radius: 16px 16px 0 0;
      max-height: 80vh;
      overflow-y: auto;
      padding-bottom: max(1rem, env(safe-area-inset-bottom));
    }
  }

  /* Feedback Button */
  .btn-feedback {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.75rem;
    background: transparent;
    color: var(--color-text-muted);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-feedback:hover {
    background: var(--color-border-light);
    color: var(--color-text);
    border-color: var(--color-border-dark);
  }

  .feedback-text {
    display: none;
  }

  @media (min-width: 768px) {
    .feedback-text {
      display: inline;
    }
  }

  /* Feedback Modal */
  .feedback-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
    padding: 1rem;
  }

  .feedback-modal {
    background: white;
    border-radius: 12px;
    width: 100%;
    max-width: 420px;
    box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
  }

  .feedback-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.25rem;
    border-bottom: 1px solid var(--color-border);
  }

  .feedback-header h3 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-text-darker);
  }

  .btn-close {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: var(--color-text-muted);
    cursor: pointer;
    padding: 0;
    line-height: 1;
    width: 2rem;
    height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
  }

  .btn-close:hover {
    background: var(--color-border-light);
    color: var(--color-text-darker);
  }

  .feedback-modal form {
    padding: 1.25rem;
  }

  .honeypot {
    position: absolute;
    left: -9999px;
    width: 1px;
    height: 1px;
    opacity: 0;
    overflow: hidden;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text);
    margin-bottom: 0.375rem;
  }

  .form-group input,
  .form-group select,
  .form-group textarea {
    width: 100%;
    padding: 0.625rem 0.75rem;
    border: 1px solid var(--color-border-dark);
    border-radius: 6px;
    font-size: 0.875rem;
    color: var(--color-text-darker);
    background: white;
    transition: border-color 0.15s, box-shadow 0.15s;
  }

  .form-group input:focus,
  .form-group select:focus,
  .form-group textarea:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.15);
  }

  .form-group textarea {
    resize: vertical;
    min-height: 100px;
  }

  .form-consent {
    margin-bottom: 1rem;
  }

  .checkbox-label {
    display: flex;
    align-items: flex-start;
    gap: 0.5rem;
    font-size: 0.8125rem;
    color: var(--color-text-muted);
    cursor: pointer;
    line-height: 1.4;
  }

  .checkbox-label input[type="checkbox"] {
    width: auto;
    margin-top: 0.125rem;
    flex-shrink: 0;
    cursor: pointer;
  }

  .form-meta {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    margin-bottom: 1rem;
    padding: 0.5rem 0.75rem;
    background: var(--color-bg-subtle);
    border-radius: 6px;
  }

  .form-meta small {
    color: var(--color-text-muted);
    font-size: 0.75rem;
  }

  .form-meta small:nth-child(3) {
    font-family: monospace;
    font-size: 0.625rem;
    max-width: 120px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .feedback-success {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: var(--color-success-light);
    color: var(--color-success-text);
    border-radius: 6px;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .feedback-error {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: var(--color-error-light);
    color: var(--color-error-text);
    border-radius: 6px;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .success-icon,
  .error-icon {
    font-size: 1rem;
  }

  .form-actions {
    display: flex;
    gap: 0.75rem;
    justify-content: flex-end;
  }

  .btn-cancel {
    padding: 0.625rem 1rem;
    background: var(--color-border-light);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-cancel:hover {
    background: var(--color-border);
  }

  .btn-submit {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.625rem 1rem;
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .btn-submit:hover:not(:disabled) {
    opacity: 0.9;
  }

  .btn-submit:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-spinner {
    width: 14px;
    height: 14px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  @media (max-width: 480px) {
    .form-row {
      grid-template-columns: 1fr;
    }

    .feedback-modal {
      max-width: 100%;
      margin: 0.5rem;
    }
  }

  /* Test Wallet Secret Key Warning Banner Styles */
  .secret-key-overlay {
    position: fixed;
    inset: 0;
    background: rgba(127, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 300;
    padding: 1rem;
  }

  .secret-key-banner {
    background: white;
    border-radius: 12px;
    width: 100%;
    max-width: 520px;
    box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);
    overflow-y: auto;
    animation: fadeIn 0.2s;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: scale(0.95); }
    to { opacity: 1; transform: scale(1); }
  }

  .secret-key-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    background: linear-gradient(135deg, var(--color-error) 0%, var(--color-error) 100%);
    color: white;
    border-radius: 8px 8px 0;
  }

  .secret-key-header h3 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
  }

  .secret-key-header svg {
    flex-shrink: 0;
  }

  .btn-close-banner {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: white;
    opacity: 0.8;
    cursor: pointer;
    padding: 0;
    line-height: 1;
    transition: opacity 0.15s;
  }

  .btn-close-banner:hover {
    opacity: 1;
  }

  .secret-key-content {
    padding: 1.25rem;
  }

      .secret-key-intro {
        font-size: 0.9375rem;
        color: var(--color-text);
        line-height: 1.5;
        margin-bottom: 0.5rem;
      }

      .secret-key-instructions {
        background: var(--color-error-light);
        border-radius: 8px;
        padding: 1rem;
        margin-bottom: 1rem;
      }

      .secret-key-instructions p {
        margin: 0 0 0.75rem 0;
        font-size: 0.875rem;
        color: var(--color-text);
        line-height: 1.5;
      }

      .secret-key-instructions ul {
        margin: 0;
        padding-left: 1.25rem;
      }

      .secret-key-instructions li {
        margin-bottom: 0.375rem;
        padding-left: 1.5rem;
        position: relative;
        color: var(--color-error-text);
        font-size: 0.8125rem;
      }

      .secret-key-instructions li::before {
        content: "•••";
      }

      .secret-key-instructions li:last-child {
        margin-bottom: 0;
      }

      .secret-key-display {
        margin-bottom: 1rem;
      }

      .secret-key-label {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 0.5rem;
      }

      .secret-key-label span {
        font-size: 0.75rem;
        font-weight: 600;
        color: var(--color-error-text);
        text-transform: uppercase;
        letter-spacing: 0.05em;
      }

      .btn-toggle-visibility {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        background: none;
        border: none;
        font-size: 0.8125rem;
        color: var(--color-error-text);
        cursor: pointer;
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
        transition: background 0.15s;
      }

      .btn-toggle-visibility:hover {
        background: var(--color-error-light);
      }

      .secret-key-value {
        background: white;
        border: 1px solid var(--color-error-lighter);
        border-radius: 6px;
        padding: 0.75rem;
        overflow-x: auto;
      }

      .secret-key-value code {
        font-family: monospace;
        font-size: 0.875rem;
        color: var(--color-error-text);
        word-break: break-all;
        line-height: 1.5;
      }

      .secret-key-value.hidden code {
        color: var(--color-text-light);
        letter-spacing: 0.15em;
      }

      .secret-key-actions {
        display: flex;
        gap: 0.75rem;
        padding: 1rem;
      }

      .btn-copy-key {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.375rem;
        padding: 0.75rem 1rem;
        background: white;
        color: var(--color-error-text);
        border: 1px solid var(--color-error-lighter);
        border-radius: 6px;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
      }

      .btn-copy-key:hover:not(:disabled) {
        background: var(--color-error-light);
        border-color: var(--color-error-lighter);
      }

      .btn-copy-key:disabled {
        background: var(--color-success-light);
        color: var(--color-success-text);
        border-color: var(--color-success-lighter);
        cursor: default;
      }

      .btn-copy-key svg {
        flex-shrink: 0;
      }

      .btn-done {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.375rem;
        padding: 0.75rem 1rem;
        background: linear-gradient(135deg, var(--color-error) 0%, var(--color-error) 100%);
        color: white;
        border: none;
        border-radius: 6px;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.2s;
      }

      .btn-done:hover {
        opacity: 0.9;
      }

      .secret-key-footer {
        padding: 1rem;
        background: var(--color-bg-subtle);
        border-top: 1px solid var(--color-border);
        text-align: center;
      }

      .test-wallet-note {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.375rem;
        font-size: 0.75rem;
        color: var(--color-text-muted);
        line-height: 1.4;
      }

      .test-wallet-note svg {
        flex-shrink: 0;
      }

      @media (max-width: 480px) {
        .secret-key-banner {
          max-width: 100%;
          margin: 0.5rem;
          max-height: 90vh;
          overflow-y: auto;
        }

        .secret-key-actions {
          flex-direction: column;
        }
      }
</style>
