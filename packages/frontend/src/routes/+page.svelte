<script lang="ts">
  import { calculateBalanceWithDemurrage } from "@kchng/shared";
  import { t, initI18n, currentLanguage, messages } from "$lib/i18n";
  import { onMount } from "svelte";

  // Reactive translation function
  $: translatedText = (key: string, params?: Record<string, string | number>) => t(key, params);

  onMount(() => {
    initI18n();
  });
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
  <h1>{translatedText('home.title')}</h1>
  <p class="tagline">{translatedText('home.tagline')}</p>
  <p class="description">
    {translatedText('home.description')}
  </p>
  <p class="audience">
    {translatedText('home.audience')}
  </p>
</div>

<div class="why-section">
  <h2>{translatedText('home.howItWorks')}</h2>
  <div class="steps-grid">
    <div class="step">
      <div class="step-number">1</div>
      <h3>{translatedText('home.step1Title')}</h3>
      <p>{translatedText('home.step1Desc')}</p>
    </div>
    <div class="step">
      <div class="step-number">2</div>
      <h3>{translatedText('home.step2Title')}</h3>
      <p>{translatedText('home.step2Desc')}</p>
    </div>
    <div class="step">
      <div class="step-number">3</div>
      <h3>{translatedText('home.step3Title')}</h3>
      <p>{translatedText('home.step3Desc')}</p>
    </div>
  </div>
</div>

<div class="value-section">
  <h2>{translatedText('home.simpleValue')}</h2>
  <div class="value-grid">
    <div class="value-card">
      <div class="value-icon">30</div>
      <div class="value-label">{translatedText('home.minutesWork')}</div>
      <div class="value-equals">=</div>
      <div class="value-icon">1</div>
      <div class="value-label">{translatedText('home.meal')}</div>
    </div>
    <div class="value-card">
      <div class="value-icon">1,000</div>
      <div class="value-label">KCHNG</div>
      <div class="value-equals">=</div>
      <div class="value-icon">1</div>
      <div class="value-label">{translatedText('home.meal')}</div>
    </div>
  </div>
  <p class="value-note">{translatedText('home.valueNote')}</p>
</div>

<div class="benefits-section">
  <h2>{translatedText('home.whyKchng')}</h2>
  <div class="benefits-grid">
    <div class="benefit">
      <div class="benefit-icon">🍽️</div>
      <h3>{translatedText('home.realValue')}</h3>
      <p>{translatedText('home.realValueDesc')}</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🤝</div>
      <h3>{translatedText('home.communityVerified')}</h3>
      <p>{translatedText('home.communityVerifiedDesc')}</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🔄</div>
      <h3>{translatedText('home.staysActive')}</h3>
      <p>{translatedText('home.staysActiveDesc')}</p>
    </div>
    <div class="benefit">
      <div class="benefit-icon">🏠</div>
      <h3>{translatedText('home.madeForCommunities')}</h3>
      <p>{translatedText('home.madeForCommunitiesDesc')}</p>
    </div>
  </div>
</div>

<div class="info">
  <h2>{translatedText('home.whyDemurrage')}</h2>
  <p>
    {translatedText('home.whyDemurrageDesc1')}
  </p>
  <p>
    {translatedText('home.whyDemurrageDesc2')}
  </p>
  <p>
    <small>{translatedText('home.whyDemurrageExample')}</small>
  </p>
</div>

<div class="info">
  <h2>{translatedText('home.whoIsThisFor')}</h2>
  <p>{translatedText('home.whoIsThisForDesc')}</p>
  <ul>
    <li>{translatedText('home.whoIsThisForItem1')}</li>
    <li>{translatedText('home.whoIsThisForItem2')}</li>
    <li>{translatedText('home.whoIsThisForItem3')}</li>
    <li>{translatedText('home.whoIsThisForItem4')}</li>
  </ul>
  <p>
    <a href="/about">{translatedText('home.learnMoreLink')} →</a>
  </p>
</div>

<div class="cta-section">
  <h2>{translatedText('home.getStarted')}</h2>
  <p>{translatedText('home.getStartedDesc')}</p>
  <div class="cta-grid">
    <a href="/dashboard" class="cta-card">
      <div class="cta-icon">📊</div>
      <div class="cta-title">{translatedText('home.ctaDashboard')}</div>
      <div class="cta-desc">{translatedText('home.ctaDashboardDesc')}</div>
    </a>
    <a href="/trusts" class="cta-card">
      <div class="cta-icon">🏘️</div>
      <div class="cta-title">{translatedText('home.ctaCommunities')}</div>
      <div class="cta-desc">{translatedText('home.ctaCommunitiesDesc')}</div>
    </a>
    <a href="/work" class="cta-card">
      <div class="cta-icon">🔨</div>
      <div class="cta-title">{translatedText('home.ctaWork')}</div>
      <div class="cta-desc">{translatedText('home.ctaWorkDesc')}</div>
    </a>
    <a href="/governance" class="cta-card">
      <div class="cta-icon">🗳️</div>
      <div class="cta-title">{translatedText('home.ctaGovernance')}</div>
      <div class="cta-desc">{translatedText('home.ctaGovernanceDesc')}</div>
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

    .value-section {
      padding: var(--space-lg);
      margin-left: -1rem;
      margin-right: -1rem;
      border-radius: 0;
    }

    .info {
      padding: var(--space-md);
      margin-left: -1rem;
      margin-right: -1rem;
      border-radius: 0;
    }
  }

  @media (max-width: 480px) {
    .benefits-grid,
    .cta-grid {
      grid-template-columns: 1fr;
    }

    .benefit {
      padding: var(--space-md);
    }
  }
</style>
