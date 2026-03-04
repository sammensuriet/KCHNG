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
  const networks = $derived<{ id: NetworkName; label: string }[]>([
    { id: "testnet", label: ($messages, t('header.testnet')) },
    { id: "mainnet", label: ($messages, t('header.mainnet')) },
  ]);

  const feedbackCategories = $derived([
    { id: "general", label: ($messages, t('header.generalFeedback')) },
    { id: "work-claims", label: ($messages, t('header.workClaims')) },
    { id: "trusts", label: ($messages, t('header.trusts')) },
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
      // Copy secret key to clipboard for user reference
      await navigator.clipboard.writeText(walletData.secretKey);
      alert("Test wallet created and funded with 777 XLM!\n\nSecret key copied to clipboard. Save it somewhere safe!");
    } catch (error) {
      console.error("Failed to create test wallet:", error);
      alert("Failed to create test wallet. Please try again.");
    } finally {
      isCreatingTestWallet = false;
    }
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
    <a href="/faq" class="nav-link" class:active={$page.url.pathname === "/faq"}>{t('nav.faq')}</a>
    <a href="/dashboard" class="nav-link" class:active={$page.url.pathname === "/dashboard"}>{t('nav.dashboard')}</a>
    <a href="/work" class="nav-link" class:active={$page.url.pathname.startsWith("/work")}>{t('nav.work')}</a>
    <a href="/trusts" class="nav-link" class:active={$page.url.pathname === "/trusts"}>{t('nav.communities')}</a>
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
      <div class="network-selector">
        <button
          class="btn-network"
          onclick={() => (showNetworkSelector = !showNetworkSelector)}
        >
          {currentNetwork.toUpperCase()} ▼
        </button>
        {#if showNetworkSelector}
          <div class="network-dropdown">
            {#each networks as network}
              <button
                class="network-option"
                class:active={currentNetwork === network.id}
                onclick={() => switchNetwork(network.id)}
              >
                {network.label}
              </button>
            {/each}
          </div>
        {/if}
      </div>
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

<style>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 2rem;
    background: white;
    border-bottom: 1px solid #e5e7eb;
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
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
    color: #6b7280;
    text-decoration: none;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 6px;
    transition: all 0.2s;
  }

  .nav-link:hover {
    color: #374151;
    background: #f3f4f6;
  }

  .nav-link.active {
    color: #667eea;
    background: #ede9fe;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    position: relative;
  }

  .network-selector {
    position: relative;
  }

  .btn-network {
    padding: 0.5rem 1rem;
    background: white;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 0.875rem;
  }

  .btn-network:hover {
    background: #f9fafb;
    border-color: #d1d5db;
  }

  .network-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    padding: 0.25rem;
    z-index: 101;
    min-width: 120px;
  }

  .network-option {
    width: 100%;
    padding: 0.5rem 0.75rem;
    background: none;
    border: none;
    border-radius: 4px;
    text-align: left;
    cursor: pointer;
    font-size: 0.875rem;
    color: #374151;
    transition: background 0.15s;
  }

  .network-option:hover {
    background: #f3f4f6;
  }

  .network-option.active {
    background: #ede9fe;
    color: #7c3aed;
    font-weight: 500;
  }

  .btn-connect {
    padding: 0.5rem 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
    background: #10b981;
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
    background: #f3f4f6;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-wallet:hover {
    background: #e5e7eb;
  }

  .wallet-address {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
  }

  .wallet-balance {
    font-size: 0.75rem;
    color: #6b7280;
  }

  .wallet-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    width: 280px;
    background: white;
    border: 1px solid #e5e7eb;
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
    color: #6b7280;
    margin-bottom: 0.25rem;
  }

  .dropdown-value {
    font-size: 0.875rem;
    color: #111827;
    font-weight: 500;
  }

  .dropdown-address {
    word-break: break-all;
    font-family: monospace;
    font-size: 0.75rem;
  }

  .dropdown-divider {
    border: none;
    border-top: 1px solid #e5e7eb;
    margin: 0.75rem 0;
  }

  .btn-disconnect {
    width: 100%;
    padding: 0.5rem;
    background: #fee2e2;
    color: #991b1b;
    border: none;
    border-radius: 4px;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s;
  }

  .btn-disconnect:hover {
    background: #fecaca;
  }

  .error-message {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    background: #fee2e2;
    color: #991b1b;
    border-radius: 6px;
    font-size: 0.875rem;
  }

  .error-dismiss {
    background: none;
    border: none;
    color: #991b1b;
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
    background: #fecaca;
  }

  @media (max-width: 640px) {
    .header {
      padding: 1rem;
    }

    .logo {
      font-size: 1.25rem;
    }

    .header-nav {
      display: none;
    }

    .wallet-dropdown {
      width: 260px;
      right: -0.5rem;
    }
  }

  /* Feedback Button */
  .btn-feedback {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.75rem;
    background: transparent;
    color: #6b7280;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-feedback:hover {
    background: #f3f4f6;
    color: #374151;
    border-color: #d1d5db;
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
    border-bottom: 1px solid #e5e7eb;
  }

  .feedback-header h3 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #111827;
  }

  .btn-close {
    background: none;
    border: none;
    font-size: 1.5rem;
    color: #6b7280;
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
    background: #f3f4f6;
    color: #111827;
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
    color: #374151;
    margin-bottom: 0.375rem;
  }

  .form-group input,
  .form-group select,
  .form-group textarea {
    width: 100%;
    padding: 0.625rem 0.75rem;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.875rem;
    color: #111827;
    background: white;
    transition: border-color 0.15s, box-shadow 0.15s;
  }

  .form-group input:focus,
  .form-group select:focus,
  .form-group textarea:focus {
    outline: none;
    border-color: #667eea;
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
    color: #6b7280;
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
    background: #f9fafb;
    border-radius: 6px;
  }

  .form-meta small {
    color: #6b7280;
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
    background: #d1fae5;
    color: #065f46;
    border-radius: 6px;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .feedback-error {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: #fee2e2;
    color: #991b1b;
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
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-cancel:hover {
    background: #e5e7eb;
  }

  .btn-submit {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.625rem 1rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
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
</style>
