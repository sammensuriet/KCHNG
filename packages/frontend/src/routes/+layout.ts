import type { LayoutLoad } from './$types';
import { defaultLanguage, type Language, type Messages, getStoredLanguage, detectBrowserLanguage } from '$lib/i18n';
import enMessages from '$lib/i18n/messages/en.json';
import esMessages from '$lib/i18n/messages/es.json';
import ruMessages from '$lib/i18n/messages/ru.json';
import zhMessages from '$lib/i18n/messages/zh.json';
import deMessages from '$lib/i18n/messages/de.json';
import arMessages from '$lib/i18n/messages/ar.json';

const allMessages: Record<Language, Messages> = {
  en: enMessages,
  es: esMessages,
  ru: ruMessages,
  zh: zhMessages,
  de: deMessages,
  ar: arMessages,
};

export const load: LayoutLoad = async () => {
  // Use localStorage (via getStoredLanguage) or browser detection
  // This runs client-side for SPA mode
  const storedLang = getStoredLanguage();
  const browserLang = detectBrowserLanguage();
  const lang = storedLang || browserLang;

  return {
    initialLanguage: lang,
    initialMessages: allMessages[lang],
  };
};

// Disable SSR for this layout - always render client-side
export const ssr = false;
export const prerender = true;
