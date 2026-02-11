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

<div class="info">
  <h2>What is Demurrage?</h2>
  <p>
    Demurrage is a holding tax that encourages circulation. With KCHNG,
    <strong>2 tokens are burned for every 7 days of account inactivity</strong>.
    This keeps the currency moving in the community!
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
    padding: 3rem 0;
  }

  .live-badge {
    margin-bottom: 1rem;
  }

  .live-indicator {
    display: inline-block;
    width: 10px;
    height: 10px;
    background: #10b981;
    border-radius: 50%;
    animation: pulse 2s infinite;
    margin-right: 0.5rem;
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
    color: #10b981;
    font-weight: 600;
    font-size: 0.875rem;
    letter-spacing: 0.05em;
  }

  .hero h1 {
    font-size: 4rem;
    font-weight: 800;
    margin: 0;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .tagline {
    font-size: 1.5rem;
    color: #666;
    margin: 0.5rem 0;
  }

  .description {
    font-size: 1.1rem;
    color: #888;
  }

  .contract-id {
    margin-top: 1rem;
    padding: 0.75rem 1rem;
    background: #f0fdf4;
    border: 1px solid #10b981;
    border-radius: 8px;
    display: inline-block;
  }

  .contract-id small {
    color: #6b7280;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .contract-id code {
    color: #10b981;
    font-size: 0.875rem;
    font-family: monospace;
    background: transparent;
  }

  .info {
    margin: 2rem 0;
    padding: 1.5rem;
    border-radius: 8px;
    background: #f8f9fa;
  }

  .info h2 {
    margin-top: 0;
    color: #333;
  }

  .info ul {
    line-height: 1.8;
  }

  .cta-section {
    margin: 3rem 0;
    text-align: center;
  }

  .cta-section h2 {
    margin-bottom: 0.5rem;
  }

  .cta-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-top: 2rem;
  }

  .cta-card {
    background: white;
    border: 2px solid #e5e7eb;
    border-radius: 12px;
    padding: 2rem 1rem;
    text-decoration: none;
    color: inherit;
    transition: all 0.2s;
  }

  .cta-card:hover {
    border-color: #667eea;
    transform: translateY(-4px);
    box-shadow: 0 8px 16px -4px rgb(0 0 0 / 0.1);
  }

  .cta-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .cta-title {
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #374151;
  }

  .cta-desc {
    font-size: 0.875rem;
    color: #6b7280;
  }
</style>
