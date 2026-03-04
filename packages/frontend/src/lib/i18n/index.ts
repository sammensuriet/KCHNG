/**
 * i18n helper module for KCHNG
 * Provides language detection, persistence, and formatting utilities
 */

import { writable, derived, get } from 'svelte/store';
import { browser } from '$app/environment';

// Supported languages
export const languages = ['en', 'es', 'ru', 'zh', 'de', 'ar'] as const;
export type Language = (typeof languages)[number];

// Language labels for display
export const languageLabels: Record<Language, string> = {
  en: 'English',
  es: 'Español',
  ru: 'Русский',
  zh: '中文',
  de: 'Deutsch',
  ar: 'العربية',
};

// Default language
export const defaultLanguage: Language = 'en';

// Storage key for language preference
const STORAGE_KEY = 'kchng-language';

// Message type
export type Messages = Record<string, unknown>;

// Cache for loaded messages
const messageCache: Partial<Record<Language, Messages>> = {};

// Current language store
export const currentLanguage = writable<Language>(defaultLanguage);

// Messages store
export const messages = writable<Messages>({});

/**
 * Get the stored language preference from localStorage
 */
export function getStoredLanguage(): Language | null {
  if (!browser) return null;

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored && languages.includes(stored as Language)) {
      return stored as Language;
    }
  } catch {
    // localStorage not available
  }
  return null;
}

/**
 * Store language preference in localStorage
 */
export function setStoredLanguage(language: Language): void {
  if (!browser) return;

  try {
    localStorage.setItem(STORAGE_KEY, language);
  } catch {
    // localStorage not available
  }
}

/**
 * Detect browser language preference
 * Returns the closest matching supported language
 */
export function detectBrowserLanguage(): Language {
  if (!browser) return defaultLanguage;

  const browserLang = navigator.language?.split('-')[0]?.toLowerCase();

  // Map browser language codes to our supported languages
  const langMap: Record<string, Language> = {
    en: 'en',
    es: 'es',
    ru: 'ru',
    zh: 'zh',
    de: 'de',
    ar: 'ar',
  };

  return langMap[browserLang || ''] || defaultLanguage;
}

/**
 * Load messages for a specific language
 */
async function loadMessages(lang: Language): Promise<Messages> {
  if (messageCache[lang]) {
    return messageCache[lang]!;
  }

  try {
    const response = await fetch(`/messages/${lang}.json`);
    if (!response.ok) {
      throw new Error(`Failed to load messages for ${lang}`);
    }
    const data = await response.json();
    messageCache[lang] = data;
    return data;
  } catch (error) {
    console.error(`Error loading messages for ${lang}:`, error);
    // Fallback to English
    if (lang !== 'en' && !messageCache['en']) {
      return loadMessages('en');
    }
    return messageCache['en'] || {};
  }
}

/**
 * Set the current language and load messages
 */
export async function setLanguage(lang: Language): Promise<void> {
  currentLanguage.set(lang);
  setStoredLanguage(lang);
  const msgs = await loadMessages(lang);
  messages.set(msgs);
  document.documentElement.lang = lang;
}

/**
 * Initialize i18n with stored or detected language
 */
export async function initI18n(): Promise<void> {
  const storedLang = getStoredLanguage();
  const initialLang = storedLang || detectBrowserLanguage();
  await setLanguage(initialLang);
}

/**
 * Initialize i18n from server-side data (for SSR hydration)
 */
export function initFromServer(lang: Language, msgs: Messages): void {
  // Populate cache
  messageCache[lang] = msgs;
  // Set current language and messages immediately
  currentLanguage.set(lang);
  messages.set(msgs);
  // Set document language
  if (browser) {
    document.documentElement.lang = lang;
  }
}

/**
 * Get a nested value from an object using dot notation
 */
function getNestedValue(obj: Record<string, unknown>, path: string): string | undefined {
  const keys = path.split('.');
  let current: unknown = obj;

  for (const key of keys) {
    if (current && typeof current === 'object' && key in current) {
      current = (current as Record<string, unknown>)[key];
    } else {
      return undefined;
    }
  }

  return typeof current === 'string' ? current : undefined;
}

/**
 * Translate a message key
 * @param key - Dot notation key (e.g., 'nav.about')
 * @param params - Optional parameters for interpolation
 */
export function t(key: string, params?: Record<string, string | number>): string {
  const currentMessages = get(messages);
  let value = getNestedValue(currentMessages, key);

  if (value === undefined) {
    // Try to get from English as fallback
    const englishMessages = messageCache['en'];
    if (englishMessages) {
      value = getNestedValue(englishMessages, key);
    }
  }

  if (value === undefined) {
    // Return the key as fallback
    return key;
  }

  // Interpolate parameters
  if (params) {
    return value.replace(/\{(\w+)\}/g, (_, paramKey) => {
      return String(params[paramKey] ?? `{${paramKey}}`);
    });
  }

  return value;
}

/**
 * Create a derived store for a specific translation key
 * Usage: const text = $t('common.loading')
 */
export function translated(key: string, params?: Record<string, string | number>) {
  return derived([messages, currentLanguage], () => t(key, params));
}

/**
 * Get date formatter for a specific language
 */
export function getDateFormatter(
  language: Language,
  options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'long', day: 'numeric' }
): Intl.DateTimeFormat {
  return new Intl.DateTimeFormat(language, options);
}

/**
 * Format a date according to the current language
 */
export function formatDate(
  date: Date | number,
  language?: Language,
  options?: Intl.DateTimeFormatOptions
): string {
  const lang = language || get(currentLanguage);
  const formatter = getDateFormatter(lang, options);
  const d = typeof date === 'number' ? new Date(date * 1000) : date;
  return formatter.format(d);
}

/**
 * Get number formatter for a specific language
 */
export function getNumberFormatter(
  language: Language,
  options: Intl.NumberFormatOptions = {}
): Intl.NumberFormat {
  return new Intl.NumberFormat(language, options);
}

/**
 * Format a number according to the current language
 */
export function formatNumber(
  value: number,
  language?: Language,
  options?: Intl.NumberFormatOptions
): string {
  const lang = language || get(currentLanguage);
  const formatter = getNumberFormatter(lang, options);
  return formatter.format(value);
}
