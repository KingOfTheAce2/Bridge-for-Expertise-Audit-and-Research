// src/i18n/config.ts
import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

// Import locale files
import enUS from './locales/en-US.json';
import nlNL from './locales/nl-NL.json';
import frFR from './locales/fr-FR.json';
import deDE from './locales/de-DE.json';
import zhHansCN from './locales/zh-Hans-CN.json';
import zhHantHK from './locales/zh-Hant-HK.json';

i18n
  .use(initReactI18next)
  .init({
    resources: {
      'en-US': { translation: enUS },
      'nl-NL': { translation: nlNL },
      'fr-FR': { translation: frFR },
      'de-DE': { translation: deDE },
      'zh-Hans-CN': { translation: zhHansCN },
      'zh-Hant-HK': { translation: zhHantHK }
    },
    lng: 'en-US',
    fallbackLng: ['en-US', 'en-GB', 'nl-NL'],
    interpolation: { escapeValue: false },
  });

export default i18n;
