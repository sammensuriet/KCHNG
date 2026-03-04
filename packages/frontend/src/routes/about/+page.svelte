<script lang="ts">
  import { t } from "$lib/i18n";
</script>

<svelte:head>
  <title>{t('about.title')} - KCHNG</title>
  <meta name="description" content={t('about.subtitle')} />
</svelte:head>

<div class="page-header">
  <h1>{t('about.title')}</h1>
  <p class="subtitle">{t('about.subtitle')}</p>
</div>

<section class="section">
  <h2>{t('about.whatIsTitle')}</h2>
  <p>
    {@html t('about.whatIsP1')}
  </p>
  <p>
    {@html t('about.whatIsP2')}
  </p>
</section>

<section class="section">
  <h2>{t('about.whoForTitle')}</h2>
  <p>{t('about.whoForIntro')}</p>
  <ul>
    <li>{@html t('about.whoForEco')}</li>
    <li>{@html t('about.whoForCoop')}</li>
    <li>{@html t('about.whoForSpaces')}</li>
    <li>{@html t('about.whoForHousing')}</li>
    <li>{@html t('about.whoForAny')}</li>
  </ul>
  <div class="highlight">
    <p>
      {@html t('about.keyRequirement')}
    </p>
  </div>
</section>

<section class="section">
  <h2>{t('about.howWorksTitle')}</h2>
  <div class="steps">
    <div class="step">
      <div class="step-num">1</div>
      <div class="step-content">
        <h3>{t('about.step1Title')}</h3>
        <p>{t('about.step1Desc')}</p>
      </div>
    </div>
    <div class="step">
      <div class="step-num">2</div>
      <div class="step-content">
        <h3>{t('about.step2Title')}</h3>
        <p>{t('about.step2Desc')}</p>
      </div>
    </div>
    <div class="step">
      <div class="step-num">3</div>
      <div class="step-content">
        <h3>{t('about.step3Title')}</h3>
        <p>{t('about.step3Desc')}</p>
      </div>
    </div>
  </div>
</section>

<section class="section">
  <h2>{t('about.whyTimeTitle')}</h2>
  <p>
    {@html t('about.whyTimeP1')}
  </p>
  <p>
    {t('about.whyTimeP2')}
  </p>
  <div class="work-types">
    <h4>{t('about.workTypesTitle')}</h4>
    <p>{t('about.workTypesIntro')}</p>
    <div class="multiplier-grid">
      <div class="multiplier">
        <span class="mult-value">1.0×</span>
        <span class="mult-label">{t('about.multStandard')}</span>
        <span class="mult-examples">{t('about.multStandardEx')}</span>
      </div>
      <div class="multiplier">
        <span class="mult-value">1.3×</span>
        <span class="mult-label">{t('about.multSkilled')}</span>
        <span class="mult-examples">{t('about.multSkilledEx')}</span>
      </div>
      <div class="multiplier">
        <span class="mult-value">1.5×</span>
        <span class="mult-label">{t('about.multTeaching')}</span>
        <span class="mult-examples">{t('about.multTeachingEx')}</span>
      </div>
      <div class="multiplier">
        <span class="mult-value">2.0×</span>
        <span class="mult-label">{t('about.multEmergency')}</span>
        <span class="mult-examples">{t('about.multEmergencyEx')}</span>
      </div>
    </div>
  </div>
</section>

<section class="section">
  <h2>{t('about.whyDecreaseTitle')}</h2>
  <p>
    {@html t('about.whyDecreaseP1')}
  </p>
  <p>
    {t('about.whyDecreaseP2')}
  </p>
  <ul>
    <li>{@html t('about.whyDecreaseEncourage')}</li>
    <li>{@html t('about.whyDecreaseHoarding')}</li>
    <li>{@html t('about.whyDecreaseMoving')}</li>
  </ul>
  <p>
    <small>{t('about.whyDecreaseExample')}</small>
  </p>
</section>

<section class="section">
  <h2>{t('about.historyTitle')}</h2>
  <p>
    {t('about.historyIntro')}
  </p>

  <div class="history-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.historySystem')}</th>
          <th>{t('about.historyWhen')}</th>
          <th>{t('about.historyWhat')}</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>{@html t('about.historyWorgl')}</td>
          <td>1932-1933</td>
          <td>{t('about.historyWorglDesc')}</td>
        </tr>
        <tr>
          <td>{@html t('about.historyChiemgauer')}</td>
          <td>2003-present</td>
          <td>{t('about.historyChiemgauerDesc')}</td>
        </tr>
        <tr>
          <td>{@html t('about.historyKchng')}</td>
          <td>2026-</td>
          <td>{t('about.historyKchngDesc')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <blockquote>
    <p>
      {t('about.historyQuote')}
    </p>
    <cite>{t('about.historyCite')}</cite>
  </blockquote>
</section>

<section class="section">
  <h2>{t('about.techTitle')}</h2>
  <p>
    {@html t('about.techIntro')}
  </p>
  <ul>
    <li>{@html t('about.techTransparent')}</li>
    <li>{@html t('about.techNoCentral')}</li>
    <li>{@html t('about.techLowFees')}</li>
    <li>{@html t('about.techAccessible')}</li>
  </ul>
  <p>
    {t('about.techOutro')}
  </p>
</section>

<section class="section contract-info">
  <h2>{t('about.contractTitle')}</h2>
  <p class="version-note">{t('about.contractNote')}</p>
  <div class="contract-list">
    <div class="contract-item">
      <span class="network-label">{t('header.testnet')}</span>
      <a href="https://stellar.expert/explorer/testnet/contract/CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH" target="_blank" rel="noopener" class="contract-link">
        CDMSMELWB6ERPXOSD7L3DXXJIG5A6PMBT6R6VFV6FOENKYYN7QNQPBFH
      </a>
    </div>
    <div class="contract-item">
      <span class="network-label">{t('header.mainnet')}</span>
      <a href="https://stellar.expert/explorer/public/contract/CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK" target="_blank" rel="noopener" class="contract-link">
        CCPZSMXRKN3FM7WDIA3NZMJMZ6E577YDXFBUKACFQKTLBP7HZH63A5OK
      </a>
    </div>
  </div>
</section>

<section class="section cta-section">
  <h2>{t('about.questionsTitle')}</h2>
  <p>
    <a href="/communicate" class="cta-button">{t('about.chatWithUs')}</a>
  </p>
  <p class="cta-note">
    {@html t('about.orFaq')}
  </p>
</section>

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

  .section {
    margin: var(--space-xl) 0;
    max-width: 700px;
  }

  .section h2 {
    color: var(--color-text);
    font-size: var(--font-size-xl);
    margin-bottom: var(--space-md);
    padding-bottom: var(--space-sm);
    border-bottom: 2px solid var(--color-primary);
    display: inline-block;
  }

  .section p {
    line-height: 1.7;
    color: var(--color-text);
  }

  .section ul {
    line-height: 1.8;
    color: var(--color-text);
  }

  .section li {
    margin-bottom: var(--space-sm);
  }

  .highlight {
    background: var(--color-bg-subtle);
    padding: var(--space-md);
    border-radius: var(--radius-md);
    margin-top: var(--space-md);
    border-left: 4px solid var(--color-primary);
  }

  .highlight p {
    margin: 0;
  }

  /* Steps */
  .steps {
    display: flex;
    flex-direction: column;
    gap: var(--space-lg);
  }

  .step {
    display: flex;
    gap: var(--space-lg);
    align-items: flex-start;
  }

  .step-num {
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
    flex-shrink: 0;
  }

  .step-content h3 {
    margin: 0 0 var(--space-xs) 0;
    font-size: var(--font-size-lg);
    color: var(--color-text);
  }

  .step-content p {
    margin: 0;
    color: var(--color-text-muted);
  }

  /* Work Types */
  .work-types {
    margin-top: var(--space-lg);
    padding: var(--space-lg);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
  }

  .work-types h4 {
    margin: 0 0 var(--space-sm) 0;
    font-size: var(--font-size-base);
    color: var(--color-text);
  }

  .work-types p {
    margin: 0 0 var(--space-md) 0;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .multiplier-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: var(--space-md);
  }

  .multiplier {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: var(--space-sm);
    background: white;
    border-radius: var(--radius-sm);
  }

  .mult-value {
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--color-primary);
  }

  .mult-label {
    font-size: var(--font-size-sm);
    font-weight: 500;
    color: var(--color-text);
    margin-top: var(--space-xs);
  }

  .mult-examples {
    font-size: var(--font-size-xs);
    color: var(--color-text-muted);
    margin-top: 2px;
  }

  /* History Table */
  .history-table {
    overflow-x: auto;
    margin: var(--space-lg) 0;
  }

  .history-table table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
  }

  .history-table th,
  .history-table td {
    padding: var(--space-sm) var(--space-md);
    text-align: left;
    border-bottom: 1px solid var(--color-border);
  }

  .history-table th {
    font-weight: 600;
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .history-table td {
    color: var(--color-text);
    vertical-align: top;
  }

  blockquote {
    margin: var(--space-lg) 0;
    padding: var(--space-md);
    background: white;
    border-left: 4px solid var(--color-primary);
    border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  }

  blockquote p {
    margin: 0 0 var(--space-sm) 0;
    font-style: italic;
    color: var(--color-text);
    line-height: 1.6;
  }

  blockquote cite {
    display: block;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    font-style: normal;
  }

  /* CTA */
  .cta-section {
    text-align: center;
    padding: var(--space-xl);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-lg);
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
  }

  .cta-note a {
    color: var(--color-primary);
  }

  /* Contract Info */
  .contract-info {
    background: var(--color-bg-subtle);
    padding: var(--space-lg);
    border-radius: var(--radius-md);
    margin-top: var(--space-xl);
  }

  .contract-info h2 {
    font-size: var(--font-size-base);
    margin-bottom: var(--space-sm);
    border-bottom: none;
  }

  .version-note {
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
    margin-bottom: var(--space-md);
  }

  .contract-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .contract-item {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
    align-items: center;
    font-size: var(--font-size-xs);
  }

  .network-label {
    font-weight: 600;
    color: var(--color-text);
    min-width: 70px;
  }

  .contract-link {
    font-family: monospace;
    color: var(--color-text-muted);
    text-decoration: none;
    word-break: break-all;
    transition: color 0.2s;
  }

  .contract-link:hover {
    color: var(--color-primary);
  }

  @media (max-width: 768px) {
    .step {
      flex-direction: column;
      align-items: center;
      text-align: center;
    }

    .multiplier-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }
</style>
