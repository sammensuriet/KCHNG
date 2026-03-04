<script lang="ts">
  import { t } from "$lib/i18n";
</script>

<svelte:head>
  <title>{t('faq.title')} - KCHNG</title>
  <meta name="description" content={t('faq.subtitle')} />
</svelte:head>

<div class="page-header">
  <h1>{t('faq.title')}</h1>
  <p class="subtitle">{t('faq.subtitle')}</p>
</div>

<div class="faq-list">
  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q1Q')}
    </summary>
    <div class="faq-answer">
      <p>{t('faq.q1Intro')}</p>
      <ul>
        <li>{t('faq.q1a1')}</li>
        <li>{t('faq.q1a2')}</li>
        <li>{t('faq.q1a3')}</li>
        <li>{t('faq.q1a4')}</li>
      </ul>
      <p>{t('faq.q1outro')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q2Q')}
    </summary>
    <div class="faq-answer">
      <p>{t('faq.q2Intro')}</p>
      <ol>
        <li>{@html t('faq.q2s1')}</li>
        <li>{@html t('faq.q2s2')}</li>
        <li>{@html t('faq.q2s3')}</li>
        <li>{@html t('faq.q2s4')}</li>
      </ol>
      <p>{@html t('faq.q2outro')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q3Q')}
    </summary>
    <div class="faq-answer">
      <p>{@html t('faq.q3p1')}</p>
      <p>{t('faq.q3p2')}</p>
      <p>{@html t('faq.q3Why')}</p>
      <ul>
        <li>{t('faq.q3a1')}</li>
        <li>{t('faq.q3a2')}</li>
        <li>{t('faq.q3a3')}</li>
        <li>{t('faq.q3a4')}</li>
      </ul>
      <p>{t('faq.q3history')}</p>
      <p>{@html t('faq.q3good')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q4Q')}
    </summary>
    <div class="faq-answer">
      <p>{@html t('faq.q4No')}</p>
      <ul>
        <li>{t('faq.q4a1')}</li>
        <li>{t('faq.q4a2')}</li>
        <li>{t('faq.q4a3')}</li>
      </ul>
      <p>{t('faq.q4outro')}</p>
      <p>{t('faq.q4banking')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q5Q')}
    </summary>
    <div class="faq-answer">
      <p>{@html t('faq.q5p1')}</p>
      <p>{@html t('faq.q5process')}</p>
      <ul>
        <li>{t('faq.q5a1')}</li>
        <li>{t('faq.q5a2')}</li>
        <li>{t('faq.q5a3')}</li>
        <li>{t('faq.q5a4')}</li>
      </ul>
      <p>{t('faq.q5outro')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q6Q')}
    </summary>
    <div class="faq-answer">
      <p>{t('faq.q6p1')}</p>
      <ul>
        <li>{@html t('faq.q6a1')}</li>
        <li>{@html t('faq.q6a2')}</li>
        <li>{@html t('faq.q6a3')}</li>
        <li>{@html t('faq.q6a4')}</li>
      </ul>
      <p>{t('faq.q6outro')}</p>
      <p>{@html t('faq.q6note')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q7Q')}
    </summary>
    <div class="faq-answer">
      <p>{@html t('faq.q7YesNo')}</p>
      <p>{t('faq.q7p1')}</p>
      <p>{t('faq.q7p2')}</p>
      <ul>
        <li>{@html t('faq.q7a1')}</li>
        <li>{@html t('faq.q7a2')}</li>
        <li>{@html t('faq.q7a3')}</li>
        <li>{@html t('faq.q7a4')}</li>
      </ul>
      <p>{t('faq.q7outro')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q8Q')}
    </summary>
    <div class="faq-answer">
      <p>{@html t('faq.q8s1t')}</p>
      <ul>
        <li>{t('faq.q8s1a1')}</li>
        <li>{t('faq.q8s1a2')}</li>
      </ul>
      <p>{@html t('faq.q8s2t')}</p>
      <ul>
        <li>{@html t('faq.q8s2a1')}</li>
        <li>{t('faq.q8s2a2')}</li>
      </ul>
      <p>{@html t('faq.q8s3t')}</p>
      <ul>
        <li>{t('faq.q8s3a1')}</li>
        <li>{t('faq.q8s3a2')}</li>
        <li>{t('faq.q8s3a3')}</li>
      </ul>
      <p>{@html t('faq.q8outro')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q9Q')}
    </summary>
    <div class="faq-answer">
      <p>{@html t('faq.q9p1')}</p>
      <p>{t('faq.q9p2')}</p>
      <ul>
        <li>{@html t('faq.q9a1')}</li>
        <li>{@html t('faq.q9a2')}</li>
      </ul>
      <p>{t('faq.q9p3')}</p>
      <p>{@html t('faq.q9best')}</p>
    </div>
  </details>

  <details class="faq-item">
    <summary class="faq-question">
      <span class="q-icon">Q</span>
      {t('faq.q10Q')}
    </summary>
    <div class="faq-answer">
      <p>{t('faq.q10p1')}</p>
      <p>{t('faq.q10p2')}</p>
      <p>{@html t('faq.q10outro')}</p>
    </div>
  </details>
</div>

<div class="more-questions">
  <h2>{t('faq.stillQuestions')}</h2>
  <p>
    <a href="/communicate" class="cta-button">{t('faq.chatWithUs')}</a>
  </p>
  <p class="cta-note">
    {@html t('faq.orAbout')}
  </p>
</div>

<style>
  .page-header {
    text-align: center;
    padding: var(--space-xl) 0;
    border-bottom: 1px solid var(--color-border);
    margin-bottom: var(--space-xl);
  }

  .page-header h1 {
    font-size: var(--font-size-3xl);
    font-weight: 800;
    margin: 0;
    background: var(--color-gradient);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
  }

  .subtitle {
    font-size: var(--font-size-lg);
    color: var(--color-text-muted);
    margin-top: var(--space-sm);
  }

  .faq-list {
    max-width: 800px;
    margin: 0 auto;
  }

  .faq-item {
    margin-bottom: var(--space-md);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .faq-question {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-lg);
    cursor: pointer;
    font-weight: 600;
    font-size: var(--font-size-lg);
    color: var(--color-text);
    list-style: none;
  }

  .faq-question::-webkit-details-marker {
    display: none;
  }

  .faq-question::after {
    content: '+';
    margin-left: auto;
    font-size: var(--font-size-xl);
    color: var(--color-text-muted);
    transition: transform 0.2s;
  }

  .faq-item[open] .faq-question::after {
    transform: rotate(45deg);
  }

  .q-icon {
    width: 32px;
    height: 32px;
    background: var(--color-gradient);
    color: white;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 700;
    flex-shrink: 0;
  }

  .faq-answer {
    padding: 0 var(--space-lg) var(--space-lg);
    padding-left: calc(var(--space-lg) + 32px + var(--space-md));
  }

  .faq-answer p {
    margin: 0 0 var(--space-md) 0;
    line-height: 1.7;
    color: var(--color-text);
  }

  .faq-answer ul,
  .faq-answer ol {
    margin: 0 0 var(--space-md) 0;
    padding-left: var(--space-lg);
    line-height: 1.8;
    color: var(--color-text);
  }

  .faq-answer li {
    margin-bottom: var(--space-sm);
  }

  .faq-answer a {
    color: var(--color-primary);
    text-decoration: none;
    font-weight: 500;
  }

  .faq-answer a:hover {
    text-decoration: underline;
  }

  .faq-answer strong {
    color: var(--color-text);
  }

  .more-questions {
    text-align: center;
    padding: var(--space-xl);
    margin-top: var(--space-xl);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-lg);
  }

  .more-questions h2 {
    margin: 0 0 var(--space-md) 0;
    color: var(--color-text);
  }

  .cta-button {
    display: inline-block;
    padding: var(--space-md) var(--space-xl);
    background: var(--color-gradient);
    color: white;
    text-decoration: none;
    border-radius: var(--radius-md);
    font-weight: 600;
    transition: opacity 0.2s;
  }

  .cta-button:hover {
    opacity: 0.9;
  }

  .cta-note {
    margin-top: var(--space-md);
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: 0;
  }

  .cta-note a {
    color: var(--color-primary);
  }

  @media (max-width: 640px) {
    .faq-question {
      font-size: var(--font-size-base);
    }

    .faq-answer {
      padding-left: var(--space-lg);
    }
  }
</style>
