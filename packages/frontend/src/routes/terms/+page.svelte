<script lang="ts">
  import { t } from "$lib/i18n";

  const glossaryTerms = [
    "demurrage",
    "minting",
    "verifier",
    "governor",
    "oracle",
    "governance",
    "slashing",
    "stake",
    "gracePeriod",
    "workClaim",
    "community",
    "reputation",
    "probation",
    "basisPoint",
    "crossCommunityExchange",
    "tf2t"
  ] as const;
</script>

<svelte:head>
  <title>{t('terms.title')} - KCHNG</title>
  <meta name="description" content="Glossary of KCHNG terms and concepts" />
</svelte:head>

<div class="page-header">
  <h1>{t('terms.title')}</h1>
</div>

<section class="section">
  <dl class="glossary">
    {#each glossaryTerms as term}
      <dt><strong>{t(`terms.${term}`)}</strong></dt>
      <dd>{t(`terms.${term}Def`)}</dd>
    {/each}
  </dl>
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

  .section {
    max-width: 700px;
    margin: 0 auto var(--space-xl);
  }

  .glossary {
    margin: 0;
  }

  .glossary dt {
    margin-top: var(--space-lg);
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
    font-size: var(--font-size-lg);
    color: var(--color-text);
  }

  .glossary dt:first-child {
    margin-top: 0;
    padding-top: 0;
    border-top: none;
  }

  .glossary dd {
    margin: var(--space-xs) 0 0 0;
    padding: 0;
    line-height: 1.7;
    color: var(--color-text-muted);
    font-size: var(--font-size-base);
  }
</style>
