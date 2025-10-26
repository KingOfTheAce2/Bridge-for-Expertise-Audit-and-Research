import React from 'react';
import { useTheme } from '../contexts/ThemeContext';
import { useTranslation } from 'react-i18next';

const ThemeToggle: React.FC = () => {
  const { theme, setTheme } = useTheme();
  const { t } = useTranslation();

  return (
    <div className="flex items-center gap-2">
      <label className="text-sm font-medium">{t('settings.theme')}</label>
      <select
        value={theme}
        onChange={(e) => setTheme(e.target.value as any)}
        className="border rounded px-2 py-1 dark:bg-gray-800 dark:border-gray-600"
      >
        <option value="light">{t('settings.lightMode')}</option>
        <option value="dark">{t('settings.darkMode')}</option>
        <option value="system">{t('settings.systemTheme')}</option>
      </select>
    </div>
  );
};

export default ThemeToggle;
