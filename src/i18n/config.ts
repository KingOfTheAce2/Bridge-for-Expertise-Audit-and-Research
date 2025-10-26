// src/i18n/config.ts
// Internationalisation configuration for BEAR LLM AI

import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

// Import all locale resources
import enGB from './locales/en-GB.json';
import nlNL from './locales/nl-NL.json';
import deDE from './locales/de-DE.json';
import frFR from './locales/fr-FR.json';
import zhHansCN from './locales/zh-Hans-CN.json';
import zhHantHK from './locales/zh-Hant-HK.json';
import ruRU from './locales/ru-RU.json';

i18n
  // Browser/OS language detection
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources: {
      'en-GB': { translation: enGB },
      'nl-NL': { translation: nlNL },
      'de-DE': { translation: deDE },
      'fr-FR': { translation: frFR },
      'zh-Hans-CN': { translation: zhHansCN },
      'zh-Hant-HK': { translation: zhHantHK },
      'ru-RU': { translation: ruRU }
    },

    // Main application language
    lng: 'en-GB',

    // Fallback chain (English preferred)
    fallbackLng: [
      'en-GB',    // British English — default
      'en-US',    // (future optional)
      'nl-NL',
      'de-DE',
      'fr-FR',
      'zh-Hans-CN'
    ],

    // Automatic detection settings
    detection: {
      order: ['localStorage', 'navigator', 'htmlTag', 'path', 'subdomain'],
      caches: ['localStorage'],
      lookupLocalStorage: 'bear-llm-language'
    },

    interpolation: {
      escapeValue: false // React already escapes
    },

    debug: false // set true during development to inspect language resolution
  });

// Manual language map for quick switching via LanguageSelector
export const supportedLanguages = [
  { code: 'en-GB', label: 'English (UK)' },
  { code: 'nl-NL', label: 'Nederlands' },
  { code: 'de-DE', label: 'Deutsch' },
  { code: 'fr-FR', label: 'Français' },
  { code: 'zh-Hans-CN', label: '简体中文' },
  { code: 'zh-Hant-HK', label: '繁體中文（香港）' },
  { code: 'ru-RU', label: 'Русский' }
];

export default i18n;
