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

<div class="info">
  <h2>Coming Soon</h2>
  <p>
    The KCHNG platform is under development. Check back soon for wallet
    integration, merchant tools, and more!
  </p>
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
</style>
