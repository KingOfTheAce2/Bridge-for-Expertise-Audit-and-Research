import { invoke } from '@tauri-apps/api/core';

export const settingsService = {
  async getSetting(key: string): Promise<string | null> {
    try {
      const result = await invoke<string | null>('get_setting', { key });
      return result;
    } catch (e) {
      console.error('getSetting failed:', e);
      return null;
    }
  },

  async setSetting(key: string, value: string): Promise<void> {
    try {
      await invoke('set_setting', { key, value });
    } catch (e) {
      console.error('setSetting failed:', e);
    }
  },

  async getAppVersion(): Promise<string> {
    try {
      const version = await invoke<string>('get_app_version');
      return version;
    } catch (e) {
      console.error('getAppVersion failed:', e);
      return 'unknown';
    }
  },
};
