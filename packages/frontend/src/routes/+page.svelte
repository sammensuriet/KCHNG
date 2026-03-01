<script lang="ts">
  import { calculateBalanceWithDemurrage } from "@kchng/shared";
</script>

<svelte:head>
  {@html `<script type="application/ld+json">
  {
    "@context": "https://schema.org",
    "@type": "FinancialProduct",
    "name": "KCHNG",
    "description": "A community work exchange system where 30 minutes of work equals one shared meal. Built for eco-villages, cooperatives, and community spaces.",
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
  <h1>KCHNG</h1>
  <p class="tagline">Community Work Exchange</p>
  <p class="description">
    Turn community service into shared value
  </p>
  <p class="audience">
    For communities, cooperatives, and collectives who want to<br />
    track and reward the work that keeps them running.
  </p>
</div>

<div class="why-section">
  <h2>How It Works</h2>
  <div class="steps-grid">
    <div class="step">
      <div class="step-number">1</div>
      <h3>Work</h3>
      <p>Do community service — cooking, cleaning, maintenance, teaching, organizing.</p>
    </div>
    <div class="step">
      <div class="step-number">2</div>
      <h3>Verify</h3>
      <p>Your community validates the work. No algorithms, just trust.</p>
    </div>
    <div class="step">
      <div class="step-number">3</div>
      <h3>Earn</h3>
      <p>Receive KCHNG tokens. 30 minutes = 1 meal.</p>
    </div>
  </div>
</div>

<div class="value-section">
  <h2>Simple Value</h2>
  <div class="value-grid">
    <div class="value-card">
      <div class="value-icon">30</div>
      <div class="value-label">minutes work</div>
      <div class="value-equals">=</div>
      <div class="value-icon">1</div>
      <div class="value-label">meal</div>
    </div>
    <div class="value-card">
      <div class="value-icon">1,000</div>
      <div class="value-label">KCHNG</div>
      <div class="value-equals">=</div>
      <div class="value-icon">1</div>
      <div class="value-label">meal</div>
    </div>
  </div>
  <p class="value-note">Value tied to real work, not speculation. One KCHNG always equals the same thing.</p>
</div>

<div class="benefits-section">
  <h2>Why KCHNG?</h2>
  <div class="benefits-grid">
    <div class="benefit">
      <div class="benefit-icon">🍽️</div>
      <h3>Real Value</h3>
      <p>1000 KCHNG = 1 community meal. Simple, stable, understandable.</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🤝</div>
      <h3>Community Verified</h3>
      <p>Work is validated by people you know. Trust built through relationships.</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🔄</div>
      <h3>Stays Active</h3>
      <p>Currency circulates instead of sitting idle. Good for everyone.</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🏠</div>
      <h3>Made for Communities</h3>
      <p>Built for eco-villages, cooperatives, housing collectives, and community spaces.</p>
    </div>
  </div>
</div>

<div class="info">
  <h2>Why Do Unused Tokens Decrease?</h2>
  <p>
    KCHNG is designed to <strong>keep value moving</strong> through your community.
    When tokens sit unused for too long, they gradually decrease — encouraging people
    to spend, share, or trade rather than hoard.
  </p>
  <p>
    This isn't a penalty. It's how the system stays healthy. Active currency means
    more trades, more connections, and a stronger community economy.
  </p>
  <p>
    <small>Example: If you don't use your balance for a month, it might decrease by about 1%. Stay active and it stays full.</small>
  </p>
</div>

<div class="info">
  <h2>Who Is This For?</h2>
  <p>KCHNG works best for communities that:</p>
  <ul>
    <li>Have a concept of community service or shared work</li>
    <li>Want to track and reward contributions fairly</li>
    <li>Are small enough that members know each other</li>
    <li>Value cooperation over competition</li>
  </ul>
  <p>
    <a href="/about">Learn more about how KCHNG works →</a>
  </p>
</div>

<div class="cta-section">
  <h2>Get Started</h2>
  <p>Connect your wallet to access the full KCHNG platform:</p>
  <div class="cta-grid">
    <a href="/dashboard" class="cta-card">
      <div class="cta-icon">📊</div>
      <div class="cta-title">Dashboard</div>
      <div class="cta-desc">View your balance and activity</div>
    </a>
    <a href="/trusts" class="cta-card">
      <div class="cta-icon">🏘️</div>
      <div class="cta-title">Communities</div>
      <div class="cta-desc">Join or create a community</div>
    </a>
    <a href="/work" class="cta-card">
      <div class="cta-icon">🔨</div>
      <div class="cta-title">Work</div>
      <div class="cta-desc">Submit and verify work</div>
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
    color: var(--color-text);
    margin: var(--space-sm) 0;
    font-weight: 600;
  }

  .description {
    font-size: var(--font-size-lg);
    color: var(--color-text-muted);
    margin-bottom: var(--space-sm);
  }

  .audience {
    font-size: var(--font-size-base);
    color: var(--color-text-light);
    max-width: 500px;
    margin: 0 auto;
    line-height: 1.6;
  }

  /* Steps / How It Works */
  .why-section {
    margin: var(--space-xl) 0;
    text-align: center;
  }

  .why-section h2 {
    color: var(--color-text);
    margin-bottom: var(--space-lg);
  }

  .steps-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-lg);
    max-width: 800px;
    margin: 0 auto;
  }

  .step {
    padding: var(--space-lg);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-lg);
    text-align: center;
  }

  .step-number {
    width: 48px;
    height: 48px;
    background: var(--color-gradient);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: var(--font-size-xl);
    font-weight: 700;
    margin: 0 auto var(--space-md);
  }

  .step h3 {
    font-size: var(--font-size-lg);
    color: var(--color-text);
    margin: 0 0 var(--space-sm) 0;
  }

  .step p {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin: 0;
    line-height: 1.5;
  }

  /* Value Section */
  .value-section {
    margin: var(--space-xl) 0;
    padding: var(--space-xl);
    background: linear-gradient(135deg, #f0fdf4 0%, #ede9fe 100%);
    border-radius: var(--radius-lg);
    text-align: center;
  }

  .value-section h2 {
    margin-top: 0;
    margin-bottom: var(--space-lg);
    color: var(--color-text);
  }

  .value-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-lg);
    max-width: 700px;
    margin: 0 auto;
  }

  .value-card {
    background: white;
    padding: var(--space-lg);
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-sm);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-xs);
  }

  .value-icon {
    font-size: var(--font-size-3xl);
    font-weight: 700;
    color: var(--color-primary);
  }

  .value-label {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .value-equals {
    font-size: var(--font-size-xl);
    color: var(--color-text-light);
    margin: var(--space-xs) 0;
  }

  .value-note {
    margin-top: var(--space-lg);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    max-width: 400px;
    margin-left: auto;
    margin-right: auto;
  }

  /* Benefits */
  .benefits-section {
    margin: var(--space-xl) 0;
    text-align: center;
  }

  .benefits-section h2 {
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

  .info a {
    color: var(--color-primary);
    text-decoration: none;
    font-weight: 500;
  }

  .info a:hover {
    text-decoration: underline;
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

  @media (max-width: 1024px) {
    .value-grid {
      grid-template-columns: 1fr;
      max-width: 300px;
    }
  }

  @media (max-width: 768px) {
    .benefits-grid,
    .cta-grid {
      grid-template-columns: repeat(2, 1fr);
    }

    .steps-grid {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 480px) {
    .benefits-grid,
    .cta-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
