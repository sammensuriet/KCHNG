<script lang="ts">
  import { calculateBalanceWithDemurrage } from "@kchng/shared";
</script>

<div class="hero">
  <h1>KCHNG</h1>
  <p class="tagline">A Community Currency with Demurrage</p>
  <p class="description">
    Built on Stellar - where 1 KCHNG = 1 local community meal
  </p>
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
  <h2>Key Features</h2>
  <ul>
    <li>Native on-chain demurrage via Soroban smart contracts</li>
    <li>Transparent and trustless</li>
    <li>Extensible for app-specific demurrage rules</li>
    <li>Community-focused local currency</li>
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
