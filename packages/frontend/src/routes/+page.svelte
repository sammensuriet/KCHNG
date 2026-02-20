<script lang="ts">
  import { calculateBalanceWithDemurrage } from "@kchng/shared";
  import { onMount } from "svelte";
  import { NETWORKS } from "@kchng/shared";

  let mainnetContractId = "";
  let isLive = false;

  onMount(() => {
    mainnetContractId = NETWORKS.mainnet.contractId;
    isLive = !!mainnetContractId;
  });
</script>

<div class="hero">
  <div class="live-badge">
    {#if isLive}
      <span class="live-indicator"></span>
      <span class="live-text">LIVE ON STELLAR MAINNET</span>
    {/if}
  </div>
  <h1>KCHNG</h1>
  <p class="tagline">A Community Currency with Demurrage</p>
  <p class="description">
    Built on Stellar - where 1000 KCHNG = 30 minutes work = 1 community meal
  </p>
  {#if isLive}
    <p class="contract-id">
      <small>Contract:</small><br>
      <code>{mainnetContractId.slice(0, 20)}...{mainnetContractId.slice(-4)}</code>
    </p>
  {/if}
</div>

<div class="why-section">
  <h2>Why KCHNG?</h2>
  <div class="benefits-grid">
    <div class="benefit">
      <div class="benefit-icon">🍽️</div>
      <h3>Real Value Anchor</h3>
      <p>1 KCHNG unit = 1 community meal. Your labor has tangible, stable value.</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🤝</div>
      <h3>Community Verified</h3>
      <p>Work is validated by your peers, not algorithms. Trust is built through relationships.</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🔄</div>
      <h3>Encourages Circulation</h3>
      <p>Demurrage keeps currency moving, strengthening the local economy for everyone.</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🌐</div>
      <h3>Transparent & Decentralized</h3>
      <p>Built on Stellar blockchain. No hidden fees, no central authority controlling supply.</p>
    </div>
  </div>
</div>

<div class="info">
  <h2>What is Demurrage?</h2>
  <p>
    Demurrage keeps your community currency active. With KCHNG,
    <strong>idle balances support community circulation</strong> — stay engaged to maintain your full balance.
  </p>
</div>

<div class="info">
  <h2>Labor-Backed Currency</h2>
  <p>
    KCHNG is a labor-backed currency where <strong>new tokens are minted when work is verified</strong>.
    Unlike traditional currencies that can be printed at will, KCHNG enters circulation through:
  </p>
  <ul>
    <li>Workers submit claims for hours worked</li>
    <li>Community verifiers approve valid work</li>
    <li>New KCHNG is minted directly to the worker's account</li>
  </ul>
  <p>
    <small>Economic model: 30 minutes of work = 1000 KCHNG = 1 community meal</small>
  </p>
</div>

<div class="info">
  <h2>Key Features</h2>
  <ul>
    <li><strong>Labor-backed</strong> - New tokens minted only through verified work</li>
    <li><strong>Native on-chain demurrage</strong> - Via Soroban smart contracts</li>
    <li><strong>Community verification</strong> - Peers validate work claims</li>
    <li><strong>Grace periods</strong> - Economic hardship protection</li>
    <li><strong>Transparent and trustless</strong> - All on Stellar blockchain</li>
    <li><strong>Multi-trust support</strong> - Different communities can set their own rules</li>
  </ul>
</div>

<div class="cta-section">
  <h2>Get Started</h2>
  <p>Connect your wallet to access the full KCHNG platform:</p>
  <div class="cta-grid">
    <a href="/dashboard" class="cta-card">
      <div class="cta-icon">📊</div>
      <div class="cta-title">Dashboard</div>
      <div class="cta-desc">View your balance and account details</div>
    </a>
    <a href="/trusts" class="cta-card">
      <div class="cta-icon">🏘️</div>
      <div class="cta-title">Trusts</div>
      <div class="cta-desc">Join or create community trusts</div>
    </a>
    <a href="/work" class="cta-card">
      <div class="cta-icon">🔨</div>
      <div class="cta-title">Work</div>
      <div class="cta-desc">Submit and verify work claims</div>
    </a>
    <a href="/governance" class="cta-card">
      <div class="cta-icon">🗳️</div>
      <div class="cta-title">Governance</div>
      <div class="cta-desc">Vote on proposals</div>
    </a>
  </div>
</div>

<style>
  .hero {
    text-align: center;
    padding: var(--space-xl) 0;
  }

  .live-badge {
    margin-bottom: var(--space-md);
  }

  .live-indicator {
    display: inline-block;
    width: 10px;
    height: 10px;
    background: var(--color-success);
    border-radius: 50%;
    animation: pulse 2s infinite;
    margin-right: var(--space-sm);
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 1;
      transform: scale(1);
    }
    50% {
      opacity: 0.7;
      transform: scale(1.2);
    }
  }

  .live-text {
    color: var(--color-success);
    font-weight: 600;
    font-size: var(--font-size-sm);
    letter-spacing: 0.05em;
  }

  .hero h1 {
    font-size: var(--font-size-4xl);
    font-weight: 800;
    margin: 0;
    background: var(--color-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .tagline {
    font-size: var(--font-size-2xl);
    color: var(--color-text-muted);
    margin: var(--space-sm) 0;
  }

  .description {
    font-size: var(--font-size-lg);
    color: var(--color-text-light);
  }

  .contract-id {
    margin-top: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    background: #f0fdf4;
    border: 1px solid var(--color-success);
    border-radius: var(--radius-md);
    display: inline-block;
  }

  .contract-id small {
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .contract-id code {
    color: var(--color-success);
    font-size: var(--font-size-sm);
    font-family: monospace;
    background: transparent;
  }

  .why-section {
    margin: var(--space-xl) 0;
    text-align: center;
  }

  .why-section h2 {
    color: var(--color-text);
    margin-bottom: var(--space-lg);
  }

  .benefits-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-lg);
  }

  .benefit {
    padding: var(--space-lg);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-lg);
    text-align: center;
  }

  .benefit-icon {
    font-size: var(--font-size-4xl);
    margin-bottom: var(--space-md);
  }

  .benefit h3 {
    font-size: var(--font-size-base);
    color: var(--color-text);
    margin: 0 0 var(--space-sm) 0;
  }

  .benefit p {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin: 0;
    line-height: 1.5;
  }

  .info {
    margin: var(--space-lg) 0;
    padding: var(--space-lg);
    border-radius: var(--radius-md);
    background: var(--color-bg-subtle);
  }

  .info h2 {
    margin-top: 0;
    color: var(--color-text);
  }

  .info ul {
    line-height: 1.8;
  }

  .cta-section {
    margin: var(--space-xl) 0;
    text-align: center;
  }

  .cta-section h2 {
    margin-bottom: var(--space-sm);
  }

  .cta-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-md);
    margin-top: var(--space-lg);
  }

  .cta-card {
    background: var(--color-bg);
    border: 2px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-xl) var(--space-md);
    text-decoration: none;
    color: inherit;
    transition: all 0.2s;
  }

  .cta-card:hover {
    border-color: var(--color-primary);
    transform: translateY(-4px);
    box-shadow: var(--shadow-lg);
  }

  .cta-icon {
    font-size: var(--font-size-4xl);
    margin-bottom: var(--space-md);
  }

  .cta-title {
    font-weight: 600;
    margin-bottom: var(--space-sm);
    color: var(--color-text);
  }

  .cta-desc {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  @media (max-width: 768px) {
    .benefits-grid,
    .cta-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 480px) {
    .benefits-grid,
    .cta-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
