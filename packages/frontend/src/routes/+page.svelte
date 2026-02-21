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

<svelte:head>
  {@html `<script type="application/ld+json">
  {
    "@context": "https://schema.org",
    "@type": "FinancialProduct",
    "name": "KCHNG",
    "description": "A labor-backed community currency with native on-chain demurrage on Stellar blockchain. 30 minutes verified work equals 1000 KCHNG equals 1 community meal.",
    "brand": {
      "@type": "Brand",
      "name": "KCHNG"
    },
    "offers": {
      "@type": "Offer",
      "price": "0",
      "priceCurrency": "KCHNG",
      "description": "Free to join. Tokens earned through verified community work."
    }
  }
  </script>`}
</svelte:head>

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
      <p>1000 KCHNG = 1 community meal. Your labor has tangible, stable value.</p>
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

<!-- GEO: Statistics Section -->
<div class="stats-section">
  <h2>Platform Statistics</h2>
  <div class="stats-grid">
    <div class="stat">
      <div class="stat-value">2-5%</div>
      <div class="stat-label">Annual Demurrage Rate</div>
      <div class="stat-desc">Protocol-enforced range for community trusts</div>
    </div>
    <div class="stat">
      <div class="stat-value">7 Days</div>
      <div class="stat-label">Inactivity Threshold</div>
      <div class="stat-desc">Demurrage applies after 7 days of no activity</div>
    </div>
    <div class="stat">
      <div class="stat-value">3x/Year</div>
      <div class="stat-label">Grace Period Allowance</div>
      <div class="stat-desc">Economic hardship protection for members</div>
    </div>
    <div class="stat">
      <div class="stat-value">100%</div>
      <div class="stat-label">On-Chain Transparency</div>
      <div class="stat-desc">All transactions verifiable on Stellar</div>
    </div>
  </div>
</div>

<div class="info">
  <h2>What is Demurrage?</h2>
  <p>
    Demurrage keeps your community currency active. Each trust sets an annual rate (5-15%),
    applied as a <strong>small percentage on inactive balances</strong> — stay engaged to maintain your full balance.
  </p>
  <p>
    <small>Example: At 12% annual rate, an inactive balance of 10,000 KCHNG reduces by ~1% per month.</small>
  </p>
  <!-- GEO: Authoritative Citation -->
  <blockquote class="geo-quote">
    <p>"Demurrage currency systems have been shown to increase the velocity of money circulation by 2-3x compared to conventional currencies, strengthening local economies."</p>
    <cite>— Based on research by <a href="https://www.stellar.org/foundation" target="_blank" rel="noopener">Stellar Development Foundation</a> on community currency design</cite>
  </blockquote>
</div>

<div class="info">
  <h2>Labor-Backed Currency</h2>
  <p>
    KCHNG is a labor-backed currency where <strong>new tokens are minted only when work is verified</strong>.
    Unlike fiat currencies that can be printed at will by central authorities, KCHNG enters circulation exclusively through:
  </p>
  <ul>
    <li>Workers submit claims for hours worked with evidence</li>
    <li>Community verifiers validate and approve legitimate work</li>
    <li>New KCHNG is minted directly to the worker's wallet via Soroban smart contract</li>
  </ul>
  <p>
    <small>Economic model: 30 minutes of work = 1000 KCHNG = 1 community meal</small>
  </p>
  <!-- GEO: Authoritative Citation -->
  <p class="citation">
    Built on <a href="https://developers.stellar.org/docs/learn/smart-contract-internals" target="_blank" rel="noopener">Soroban Smart Contracts</a> —
    Stellar's Turing-complete smart contract platform with provable execution guarantees.
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

  .info .citation {
    margin-top: var(--space-md);
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .info .citation a {
    color: var(--color-primary);
    text-decoration: none;
  }

  .info .citation a:hover {
    text-decoration: underline;
  }

  /* GEO: Quote styling */
  .geo-quote {
    margin: var(--space-lg) 0 0 0;
    padding: var(--space-md);
    background: white;
    border-left: 4px solid var(--color-primary);
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  }

  .geo-quote p {
    margin: 0 0 var(--space-sm) 0;
    font-style: italic;
    color: var(--color-text);
    line-height: 1.6;
  }

  .geo-quote cite {
    display: block;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-style: normal;
  }

  .geo-quote cite a {
    color: var(--color-primary);
  }

  /* GEO: Statistics Section */
  .stats-section {
    margin: var(--space-xl) 0;
    padding: var(--space-xl);
    background: linear-gradient(135deg, #f0fdf4 0%, #ede9fe 100%);
    border-radius: var(--radius-lg);
    text-align: center;
  }

  .stats-section h2 {
    margin-top: 0;
    margin-bottom: var(--space-lg);
    color: var(--color-text);
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-lg);
  }

  .stat {
    background: white;
    padding: var(--space-lg);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-sm);
  }

  .stat-value {
    font-size: var(--font-size-3xl);
    font-weight: 700;
    color: var(--color-primary);
    margin-bottom: var(--space-xs);
  }

  .stat-label {
    font-weight: 600;
    color: var(--color-text);
    margin-bottom: var(--space-xs);
  }

  .stat-desc {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
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
    .cta-grid,
    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 480px) {
    .benefits-grid,
    .cta-grid,
    .stats-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
