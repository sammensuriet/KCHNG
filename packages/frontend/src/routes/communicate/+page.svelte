<script lang="ts">
  import { onMount } from "svelte";
  import { wallet, truncatedAddress } from "$lib/stores/wallet";
  import { t } from "$lib/i18n";
  import {
    chatStore,
    sendMessage,
    truncateAddress,
    formatTime,
  } from "$lib/stores/chat";

  let messageInput = $state("");
  let messagesContainer: HTMLDivElement | undefined = $state();
  let isSending = $state(false);

  onMount(() => {
    chatStore.initialize();

    return () => {
      // Cleanup on unmount
    };
  });

  // Auto-scroll to bottom when new messages arrive
  $effect(() => {
    const messages = $chatStore;
    if (messagesContainer) {
      setTimeout(() => {
        messagesContainer.scrollTop = messagesContainer.scrollHeight;
      }, 10);
    }
  });

  function handleSend() {
    if (!$wallet.connected || !messageInput.trim() || isSending) return;

    isSending = true;
    const success = sendMessage(messageInput);
    if (success) {
      messageInput = "";
    }
    isSending = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
  }

  function isOwnMessage(sender: string): boolean {
    return $wallet.address === sender;
  }
</script>

<svelte:head>
  <title>{t('communicate.title')}</title>
  <meta name="description" content={t('communicate.description')} />
</svelte:head>

<div class="communicate-page">
  <div class="page-header">
    <h1>{t('communicate.heading')}</h1>
    <p class="subtitle">{t('communicate.subtitle')}</p>
  </div>

  {#if !$wallet.connected}
    <div class="connect-prompt">
      <div class="prompt-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"></path>
        </svg>
      </div>
      <h2>{t('communicate.connectWallet.title')}</h2>
      <p>{t('communicate.connectWallet.description')}</p>
      <p class="wallet-info-text">{t('communicate.connectWallet.identity')}</p>
    </div>
  {:else}
    <div class="chat-container">
      <div class="messages-area" bind:this={messagesContainer}>
        {#if $chatStore.length === 0}
          <div class="empty-state">
            <p>{t('communicate.empty.noMessages')}</p>
          </div>
        {:else}
          {#each $chatStore as message (message.id)}
            <div class="message" class:own={isOwnMessage(message.sender)}>
              <div class="message-header">
                <span class="sender" title={message.sender}>
                  {isOwnMessage(message.sender) ? t('communicate.you') : truncateAddress(message.sender)}
                </span>
                <span class="time">{formatTime(message.timestamp)}</span>
              </div>
              <div class="message-text">{message.text}</div>
            </div>
          {/each}
        {/if}
      </div>

      <div class="input-area">
        <div class="input-wrapper">
          <textarea
            bind:value={messageInput}
            onkeydown={handleKeydown}
            placeholder={t('communicate.input.placeholder')}
            rows="1"
            disabled={isSending}
          ></textarea>
          <button
            class="send-button"
            onclick={handleSend}
            disabled={!messageInput.trim() || isSending}
            title={t('communicate.input.sendTitle')}
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="22" y1="2" x2="11" y2="13"></line>
              <polygon points="22 2 15 22 11 13 2 9 22 2"></polygon>
            </svg>
          </button>
        </div>
        <p class="input-hint">
          {t('communicate.hint.chattingAs')} <span class="address">{$truncatedAddress}</span>
          <span class="separator">·</span>
          {t('communicate.hint.messagesDecentralized')}
        </p>
      </div>
    </div>
  {/if}
</div>

<style>
  .communicate-page {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    min-height: calc(100vh - 80px);
    display: flex;
    flex-direction: column;
  }

  .page-header {
    margin-bottom: 1.5rem;
  }

  .page-header h1 {
    font-size: 1.75rem;
    font-weight: 600;
    color: var(--color-text-darker);
    margin: 0 0 0.5rem 0;
  }

  .subtitle {
    color: var(--color-text-muted);
    font-size: 0.875rem;
    margin: 0;
  }

  /* Connect Prompt */
  .connect-prompt {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 3rem;
    background: var(--color-bg-subtle);
    border-radius: 12px;
    border: 1px solid var(--color-border);
  }

  .prompt-icon {
    color: var(--color-text-light);
    margin-bottom: 1rem;
  }

  .connect-prompt h2 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text-darker);
    margin: 0 0 0.5rem 0;
  }

  .connect-prompt p {
    color: var(--color-text-muted);
    margin: 0;
    font-size: 0.875rem;
  }

  .wallet-info-text {
    margin-top: 0.5rem !important;
    font-size: 0.8125rem !important;
    color: var(--color-text-light) !important;
  }

  /* Chat Container */
  .chat-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    background: white;
    border: 1px solid var(--color-border);
    border-radius: 12px;
    overflow: hidden;
    min-height: 500px;
    max-height: calc(100vh - 200px);
  }

  /* Messages Area */
  .messages-area {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-text-light);
    font-size: 0.875rem;
  }

  .message {
    max-width: 80%;
    padding: 0.75rem 1rem;
    border-radius: 12px;
    background: var(--color-border-light);
  }

  .message.own {
    align-self: flex-end;
    background: var(--color-gradient);
    color: white;
  }

  .message.own .message-header {
    color: rgba(255, 255, 255, 0.8);
  }

  .message.own .time {
    color: rgba(255, 255, 255, 0.6);
  }

  .message-header {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.25rem;
    font-size: 0.75rem;
    color: var(--color-text-muted);
  }

  .sender {
    font-weight: 500;
  }

  .time {
    color: var(--color-text-light);
  }

  .message-text {
    font-size: 0.875rem;
    line-height: 1.4;
    word-wrap: break-word;
    white-space: pre-wrap;
  }

  /* Input Area */
  .input-area {
    padding: 1rem;
    border-top: 1px solid var(--color-border);
    background: var(--color-bg-subtle);
  }

  .input-wrapper {
    display: flex;
    gap: 0.5rem;
    align-items: flex-end;
  }

  .input-wrapper textarea {
    flex: 1;
    padding: 0.75rem 1rem;
    border: 1px solid var(--color-border);
    border-radius: 8px;
    font-size: 0.875rem;
    font-family: inherit;
    resize: none;
    min-height: 44px;
    max-height: 120px;
    line-height: 1.4;
  }

  .input-wrapper textarea:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  .input-wrapper textarea::placeholder {
    color: var(--color-text-light);
  }

  .send-button {
    width: 44px;
    height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-gradient);
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: opacity 0.2s, transform 0.1s;
    flex-shrink: 0;
  }

  .send-button:hover:not(:disabled) {
    opacity: 0.9;
    transform: scale(1.02);
  }

  .send-button:active:not(:disabled) {
    transform: scale(0.98);
  }

  .send-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .input-hint {
    margin: 0.5rem 0 0 0;
    font-size: 0.75rem;
    color: var(--color-text-light);
  }

  .input-hint .address {
    font-family: monospace;
    color: var(--color-text-muted);
  }

  .input-hint .separator {
    margin: 0 0.25rem;
  }

  /* Scrollbar */
  .messages-area::-webkit-scrollbar {
    width: 6px;
  }

  .messages-area::-webkit-scrollbar-track {
    background: transparent;
  }

  .messages-area::-webkit-scrollbar-thumb {
    background: var(--color-border-dark);
    border-radius: 3px;
  }

  .messages-area::-webkit-scrollbar-thumb:hover {
    background: var(--color-text-light);
  }

  /* Responsive */
  @media (max-width: 640px) {
    .communicate-page {
      padding: 1rem;
    }

    .chat-container {
      min-height: 400px;
      max-height: calc(100vh - 160px);
    }

    .message {
      max-width: 90%;
    }

    .page-header h1 {
      font-size: 1.5rem;
    }
  }
</style>
