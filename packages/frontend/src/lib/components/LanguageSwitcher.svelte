<script lang="ts">
  import { currentLanguage, setLanguage, languageLabels, type Language } from '$lib/i18n';

  let isOpen = $state(false);

  const languageOptions: { code: Language; flag: string; label: string }[] = [
    { code: 'en', flag: '🇬🇧', label: languageLabels.en },
    { code: 'es', flag: '🇪🇸', label: languageLabels.es },
    { code: 'ru', flag: '🇷🇺', label: languageLabels.ru },
    { code: 'zh', flag: '🇨🇳', label: languageLabels.zh },
  ];

  function toggleDropdown() {
    isOpen = !isOpen;
  }

  async function selectLanguage(lang: Language) {
    await setLanguage(lang);
    isOpen = false;
  }

  function handleClickOutside(event: MouseEvent) {
    const target = event.target as HTMLElement;
    if (!target.closest('.language-switcher')) {
      isOpen = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="language-switcher">
  <button
    class="language-btn"
    onclick={toggleDropdown}
    aria-label="Select language"
    aria-expanded={isOpen}
    aria-haspopup="listbox"
  >
    <span class="current-flag">
      {languageOptions.find(l => l.code === $currentLanguage)?.flag || '🌐'}
    </span>
    <span class="current-lang">{$currentLanguage.toUpperCase()}</span>
    <span class="arrow" class:open={isOpen}>▼</span>
  </button>

  {#if isOpen}
    <ul class="language-dropdown" role="listbox" aria-label="Available languages">
      {#each languageOptions as option}
        <li>
          <button
            class="language-option"
            class:selected={$currentLanguage === option.code}
            onclick={() => selectLanguage(option.code)}
            role="option"
            aria-selected={$currentLanguage === option.code}
          >
            <span class="option-flag">{option.flag}</span>
            <span class="option-label">{option.label}</span>
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .language-switcher {
    position: relative;
  }

  .language-btn {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    padding: 0.5rem 0.75rem;
    background: transparent;
    color: #6b7280;
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .language-btn:hover {
    background: #f3f4f6;
    color: #374151;
    border-color: #d1d5db;
  }

  .current-flag {
    font-size: 1rem;
  }

  .current-lang {
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.025em;
  }

  .arrow {
    font-size: 0.625rem;
    transition: transform 0.2s;
  }

  .arrow.open {
    transform: rotate(180deg);
  }

  .language-dropdown {
    position: absolute;
    top: calc(100% + 0.5rem);
    right: 0;
    min-width: 140px;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -1px rgb(0 0 0 / 0.06);
    padding: 0.25rem;
    z-index: 50;
    list-style: none;
    margin: 0;
  }

  .language-option {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    padding: 0.5rem 0.75rem;
    background: none;
    border: none;
    border-radius: 6px;
    font-size: 0.875rem;
    color: #374151;
    cursor: pointer;
    transition: background 0.15s;
    text-align: left;
  }

  .language-option:hover {
    background: #f3f4f6;
  }

  .language-option.selected {
    background: #ede9fe;
    color: #7c3aed;
    font-weight: 500;
  }

  .option-flag {
    font-size: 1.125rem;
  }

  .option-label {
    flex: 1;
  }

  @media (max-width: 640px) {
    .language-btn {
      padding: 0.5rem;
    }

    .current-lang {
      display: none;
    }
  }
</style>
