import type { LayoutServerLoad } from './$types';
import { defaultLanguage, type Language, type Messages } from '$lib/i18n';
import enMessages from '../../static/messages/en.json';
import esMessages from '../../static/messages/es.json';
import ruMessages from '../../static/messages/ru.json';
import zhMessages from '../../static/messages/zh.json';
import deMessages from '../../static/messages/de.json';
import arMessages from '../../static/messages/ar.json';

const allMessages: Record<Language, Messages> = {
  en: enMessages,
  es: esMessages,
  ru: ruMessages,
  zh: zhMessages,
  de: deMessages,
  ar: arMessages,
};

export const load: LayoutServerLoad = async ({ cookies }) => {
  const lang = (cookies.get('kchng-language') as Language) || defaultLanguage;
  return {
    initialLanguage: lang,
    initialMessages: allMessages[lang],
  };
};
