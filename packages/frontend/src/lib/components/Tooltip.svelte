<script lang="ts">
  import { t } from '$lib/i18n';

  interface Props {
    /** i18n key used to resolve tooltip text via t('tooltip.${term}') */
    term: string;
  }

  let { term }: Props = $props();

  let visible = $state(false);

  const tooltipText = $derived(t(`tooltip.${term}`));

  /** If the i18n key is missing, t() returns the raw key — show nothing. */
  const hasTooltip = $derived(tooltipText !== `tooltip.${term}`);

  function show() {
    visible = true;
  }

  function hide() {
    visible = false;
  }

  function toggle() {
    visible = !visible;
  }
</script>

{#if hasTooltip}
  <span
    class="tooltip-trigger"
    tabindex="0"
    role="button"
    aria-label={tooltipText}
    onmouseenter={show}
    onmouseleave={hide}
    onfocus={show}
    onblur={hide}
    onclick={toggle}
  >
    <sup class="tooltip-icon" aria-hidden="true">?</sup>

    {#if visible}
      <span class="tooltip-content" role="tooltip">
        {tooltipText}
      </span>
    {/if}
  </span>
{/if}

<style>
  .tooltip-trigger {
    position: relative;
    display: inline-flex;
    align-items: baseline;
    cursor: help;
    outline: none;
  }

  .tooltip-trigger:focus-visible {
    outline: 2px solid var(--color-primary);
    outline-offset: 2px;
    border-radius: var(--radius-sm);
  }

  .tooltip-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1em;
    height: 1em;
    font-size: 0.7em;
    font-weight: 700;
    line-height: 1;
    color: var(--color-bg, #fff);
    background: var(--color-text-muted, #6b7280);
    border-radius: var(--radius-full, 9999px);
    text-decoration: none;
    vertical-align: super;
    margin-left: 0.1em;
    padding: 0.15em;
    box-sizing: content-box;
    user-select: none;
  }

  .tooltip-trigger:hover .tooltip-icon,
  .tooltip-trigger:focus .tooltip-icon {
    background: var(--color-primary, #667eea);
  }

  .tooltip-content {
    position: absolute;
    bottom: calc(100% + 0.5rem);
    left: 50%;
    transform: translateX(-50%);
    z-index: 50;
    min-width: 12rem;
    max-width: 18rem;
    padding: var(--space-sm, 0.5rem) var(--space-md, 1rem);
    background: var(--color-tooltip-bg, #1f2937);
    color: var(--color-bg, #fff);
    font-size: var(--font-size-xs, 0.75rem);
    font-weight: 400;
    line-height: 1.5;
    border-radius: var(--radius-md, 8px);
    box-shadow: var(--shadow-lg, 0 10px 15px -3px rgb(0 0 0 / 0.1));
    pointer-events: none;
    white-space: normal;
  }

  .tooltip-content::after {
    content: '';
    position: absolute;
    top: 100%;
    left: 50%;
    transform: translateX(-50%);
    border: 0.375rem solid transparent;
    border-top-color: var(--color-tooltip-bg, #1f2937);
  }

  /* Mobile: ensure tooltip stays within viewport */
  @media (max-width: 640px) {
    .tooltip-content {
      left: 0;
      transform: none;
      min-width: 10rem;
    }

    .tooltip-content::after {
      left: 1rem;
      transform: none;
    }
  }
</style>
