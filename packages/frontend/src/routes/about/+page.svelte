<script lang="ts">
  import { t } from "$lib/i18n";
  import { NETWORKS } from "@kchng/shared";
  import { wallet } from "$lib/stores/wallet";
  import Tooltip from "$lib/components/Tooltip.svelte";

  let currentNetwork = $state("testnet");


  // Contract IDs loaded dynamically from shared config (single source of truth)
  const testnetContractId = NETWORKS.testnet.contractId;</script>

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
  <p>
    {@html t('about.whatIsP3')}
  </p>
  <p>
    {@html t('about.whatIsP4')}
  </p>
</section>

<section class="section">
  <h2>{t('about.comparisonTitle')}</h2>
  <p>{t('about.comparisonIntro')}</p>

  <h3>{t('about.comparisonVsTitle')}</h3>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.comparisonQuestion')}</th>
          <th>{t('about.comparisonFiat')}</th>
          <th>{t('about.comparisonCrypto')}</th>
          <th>{t('about.comparisonGold')}</th>
          <th>KCHNG</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>{@html t('about.comparisonQ1')}</td>
          <td>{t('about.comparisonFiatA1')}</td>
          <td>{t('about.comparisonCryptoA1')}</td>
          <td>{t('about.comparisonGoldA1')}</td>
          <td>{@html t('about.comparisonKchngA1')}</td>
        </tr>
        <tr>
          <td>{@html t('about.comparisonQ2')}</td>
          <td>{t('about.comparisonFiatA2')}</td>
          <td>{t('about.comparisonCryptoA2')}</td>
          <td>{t('about.comparisonGoldA2')}</td>
          <td>{@html t('about.comparisonKchngA2')}</td>
        </tr>
        <tr>
          <td>{@html t('about.comparisonQ3')}</td>
          <td>{t('about.comparisonFiatA3')}</td>
          <td>{t('about.comparisonCryptoA3')}</td>
          <td>{t('about.comparisonGoldA3')}</td>
          <td>{@html t('about.comparisonKchngA3')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <h3>{t('about.mintVsTransferTitle')}</h3>
  <p>{t('about.mintVsTransferIntro')}</p>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.mintVsTransferOp')}</th>
          <th>{t('about.mintVsTransferDebit')}</th>
          <th>{t('about.mintVsTransferCredit')}</th>
          <th>{t('about.mintVsTransferSupply')}</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>{t('about.mintVsTransferTransfer')}</strong></td>
          <td>{t('about.mintVsTransferTransferDebit')}</td>
          <td>{t('about.mintVsTransferTransferCredit')}</td>
          <td>{t('about.mintVsTransferTransferSupply')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.mintVsTransferMint')}</strong></td>
          <td>{@html t('about.mintVsTransferMintDebit')}</td>
          <td>{t('about.mintVsTransferMintCredit')}</td>
          <td>{t('about.mintVsTransferMintSupply')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <div class="highlight">
    <p>{@html t('about.mintVsTransferKey')}</p>
  </div>

  <h3>{t('about.barterVsKchngTitle')}</h3>
  <p>{t('about.barterVsKchngIntro')}</p>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.barterVsKchngAspect')}</th>
          <th>{t('about.barterVsKchngDirect')}</th>
          <th>KCHNG</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>{t('about.barterVsKchngQ1')}</strong></td>
          <td>{t('about.barterVsKchngDirectA1')}</td>
          <td>{t('about.barterVsKchngKchngA1')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.barterVsKchngQ2')}</strong></td>
          <td>{t('about.barterVsKchngDirectA2')}</td>
          <td>{t('about.barterVsKchngKchngA2')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.barterVsKchngQ3')}</strong></td>
          <td>{t('about.barterVsKchngDirectA3')}</td>
          <td>{t('about.barterVsKchngKchngA3')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.barterVsKchngQ4')}</strong></td>
          <td>{t('about.barterVsKchngDirectA4')}</td>
          <td>{t('about.barterVsKchngKchngA4')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.barterVsKchngQ5')}</strong></td>
          <td>{t('about.barterVsKchngDirectA5')}</td>
          <td>{@html t('about.barterVsKchngKchngA5')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <div class="highlight">
    <p>{@html t('about.barterVsKchngKey')}</p>
  </div>
</section>

<section class="section">
  <h2>{t('about.protocolTitle')}</h2>
  <p>{@html t('about.protocolIntro')}</p>

  <h3>{t('about.protocolMintingTitle')}</h3>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.protocolParam')}</th>
          <th>{t('about.protocolValue')}</th>
          <th>{t('about.protocolNote')}</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>{t('about.protocolMintRate')}</strong></td>
          <td>1,000 KCHNG</td>
          <td>{t('about.protocolMintRateNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolMinWork')}</strong></td>
          <td>15 min</td>
          <td>{t('about.protocolMinWorkNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolMultipliers')}</strong></td>
          <td>1.0× / 1.3× / 1.5× / 2.0×</td>
          <td>{@html t('about.protocolMultipliersNote')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <h3>{t('about.protocolVerificationTitle')}</h3>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.protocolParam')}</th>
          <th>{t('about.protocolValue')}</th>
          <th>{t('about.protocolNote')}</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>{t('about.protocolMinVerifiers')}</strong></td>
          <td>2</td>
          <td>{t('about.protocolMinVerifiersNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolApproval')}</strong></td>
          <td>{t('about.protocolApprovalValue')}</td>
          <td>{t('about.protocolApprovalNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolVerifierStake')}</strong></td>
          <td>100,000 KCHNG</td>
          <td>{@html t('about.protocolVerifierStakeNote')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <h3>{t('about.protocolRolesTitle')}</h3>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.protocolRole')}</th>
          <th>{t('about.protocolStake')}</th>
          <th>{t('about.protocolPowers')}</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>{t('about.protocolGovernor')}</strong></td>
          <td>{t('about.protocolNone')}</td>
          <td>{@html t('about.protocolGovernorPowers')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolVerifier')}</strong></td>
          <td>100,000 KCHNG</td>
          <td>{t('about.protocolVerifierPowers')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolOracle')}</strong></td>
          <td>5,000,000 KCHNG</td>
          <td>{@html t('about.protocolOraclePowers')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <h3>{t('about.protocolCommunityTitle')}</h3>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.protocolParam')}</th>
          <th>{t('about.protocolValue')}</th>
          <th>{t('about.protocolNote')}</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>{t('about.protocolMembership')}</strong></td>
          <td>{t('about.protocolMembershipValue')}</td>
          <td>{@html t('about.protocolMembershipNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolDemurrage')}</strong></td>
          <td>5–15% / year</td>
          <td>{t('about.protocolDemurrageNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolCrossTrust')}</strong></td>
          <td>{@html t('about.protocolCrossTrustValue')}</td>
          <td>{t('about.protocolCrossTrustNote')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <h3>{t('about.protocolReputationTitle')}</h3>
  <div class="comparison-table">
    <table>
      <thead>
        <tr>
          <th>{t('about.protocolParam')}</th>
          <th>{t('about.protocolValue')}</th>
          <th>{t('about.protocolNote')}</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>{t('about.protocolRange')}</strong></td>
          <td>0–1000</td>
          <td>{t('about.protocolRangeNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolRoles')}</strong></td>
          <td>5</td>
          <td>{t('about.protocolRolesNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolProbation')}</strong></td>
          <td>&lt; 200</td>
          <td>{@html t('about.protocolProbationNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolTf2t')}</strong></td>
          <td>{t('about.protocolTf2tValue')}</td>
          <td>{t('about.protocolTf2tNote')}</td>
        </tr>
        <tr>
          <td><strong>{t('about.protocolSlashing')}</strong></td>
          <td>10% / 25%</td>
          <td>{@html t('about.protocolSlashingNote')}</td>
        </tr>
      </tbody>
    </table>
  </div>

  <div class="highlight">
    <p>{@html t('about.protocolKey')}</p>
  </div>
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
      <a href="https://stellar.expert/explorer/testnet/contract/{testnetContractId}" target="_blank" rel="noopener" class="contract-link">
        {testnetContractId}
      </a>
    </div>
  </div>
</section>

<section class="section faq-section">
  <h2>{t('faq.title')}</h2>

  <div class="faq-list">
    <details class="faq-item">
      <summary class="faq-question">{t('faq.q1Q')}</summary>
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
      <summary class="faq-question">{t('faq.q2Q')}</summary>
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
      <summary class="faq-question">{t('faq.q3Q')}</summary>
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
      <summary class="faq-question">{t('faq.q5Q')}</summary>
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
      <summary class="faq-question">{t('faq.q6Q')}</summary>
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
      <summary class="faq-question">{t('faq.q7Q')}</summary>
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
      <summary class="faq-question">{t('faq.q8Q')}</summary>
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
      <summary class="faq-question">{t('faq.q9Q')}</summary>
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
      <summary class="faq-question">{t('faq.q10Q')}</summary>
      <div class="faq-answer">
        <p>{t('faq.q10p1')}</p>
        <p>{t('faq.q10p2')}</p>
        <p>{@html t('faq.q10outro')}</p>
      </div>
    </details>
  </div>
</section>

<section class="section wallet-cta-section">
  <h2>{t('about.walletCtaTitle')}</h2>
  <p>
    {@html t('about.walletCtaDesc')}
  </p>

    {#if $wallet.connected}
      <div class="wallet-connected-info">
        <span class="connected-badge">{t('dashboard.active')}</span>
        <span class="connected-address">{$wallet.address?.slice(0, 6)}...{$wallet.address?.slice(-4)}</span>
      </div>
      <div class="cta-actions">
        <a href="/dashboard" class="cta-button">{t('nav.dashboard')} →</a>
      </div>
    {:else}
      <div class="cta-actions">
        <button class="cta-button" onclick={() => wallet.connect(currentNetwork)}>
          {t('about.walletCtaButton')} <Tooltip term="wallet" />
        </button>
      </div>
      <p class="cta-note">
        {t('about.walletCtaOr')}
        <button class="link-button" onclick={() => wallet.createTestWallet()}>
          {t('about.walletCtaTest')} <Tooltip term="testnet" />
        </button>
      </p>
      <p class="cta-note">
        {t('about.walletCtaNote')} <Tooltip term="freighter" />
      </p>
    {/if}
</section>

<section class="section cta-section">
  <h2>{t('about.questionsTitle')}</h2>
  <p>
    <a href="/communicate" class="cta-button">{t('about.chatWithUs')}</a>
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

  /* Comparison Tables */
  .comparison-table {
    overflow-x: auto;
    margin: var(--space-lg) 0;
  }

  .comparison-table table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
  }

  .comparison-table th,
  .comparison-table td {
    padding: var(--space-sm) var(--space-md);
    text-align: left;
    border-bottom: 1px solid var(--color-border);
    vertical-align: top;
  }

  .comparison-table th {
    font-weight: 600;
    color: var(--color-text-muted);
    font-size: var(--font-size-xs);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    background: var(--color-bg-subtle);
  }

  .comparison-table td {
    color: var(--color-text);
  }

  .comparison-table td:first-child {
    font-weight: 500;
    color: var(--color-text);
  }

  section h3 {
    font-size: var(--font-size-lg);
    margin: var(--space-lg) 0 var(--space-md) 0;
    color: var(--color-text);
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

  /* FAQ Section */
  .faq-section {
    max-width: 800px;
  }

  .faq-list {
    margin-top: var(--space-lg);
  }

  .faq-item {
    margin-bottom: var(--space-sm);
    background: var(--color-bg-subtle);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .faq-question {
    display: flex;
    align-items: center;
    padding: var(--space-md) var(--space-lg);
    cursor: pointer;
    font-weight: 600;
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

  .faq-answer {
    padding: 0 var(--space-lg) var(--space-lg);
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

  /* Wallet CTA Section */
  .wallet-cta-section {
    text-align: center;
    padding: var(--space-xl);
    background: linear-gradient(135deg, #ede9fe 0%, #f0fdf4 100%);
    border-radius: var(--radius-lg);
    max-width: 700px;
  }

  .wallet-cta-section h2 {
    border-bottom: none;
  }

  .wallet-cta-section p {
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
  }

  .cta-actions {
    margin-top: var(--space-lg);
    display: flex;
    justify-content: center;
    gap: var(--space-md);
    flex-wrap: wrap;
    align-items: center;
  }

  .wallet-connected-info {
    display: inline-flex;
    align-items: center;
    gap: var(--space-sm);
    margin-top: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    background: white;
    border-radius: var(--radius-md);
    border: 1px solid var(--color-border);
  }

  .connected-badge {
    display: inline-block;
    padding: 2px 8px;
    background: #dcfce7;
    color: #166534;
    font-size: var(--font-size-xs);
    font-weight: 600;
    border-radius: var(--radius-sm);
    text-transform: uppercase;
  }

  .connected-address {
    font-family: monospace;
    font-size: var(--font-size-sm);
    color: var(--color-text-muted);
  }

  .link-button {
    background: none;
    border: none;
    color: var(--color-primary);
    cursor: pointer;
    font-size: var(--font-size-sm);
    font-weight: 500;
    padding: 0;
    display: inline;
    text-decoration: underline;
  }

  .link-button:hover {
    color: var(--color-primary-dark);
  }
</style>
