import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface ModelInfo {
  id?: number;
  model_id: string;
  name: string;
  description: string;
  provider: string;
  size: string;
  parameters: string;
  quantization?: string;
  format: string;
  status: string;
  file_size: number;
  is_active: boolean;
  is_downloaded: boolean;
  download_url: string;
  tags: string[];
}

export interface DownloadProgress {
  model_id: string;
  downloaded_bytes: number;
  total_bytes: number;
  percentage: number;
  speed_mbps: number;
  status: 'Starting' | 'Downloading' | 'Completed' | 'Failed' | 'Cancelled';
}

class ModelService {
  /**
   * List all available models
   */
  async listModels(): Promise<ModelInfo[]> {
    try {
      const models = await invoke<ModelInfo[]>('list_models');
      return models;
    } catch (error) {
      console.error('Failed to list models:', error);
      throw error;
    }
  }

  /**
   * Download a model
   */
  async downloadModel(modelId: string): Promise<string> {
    try {
      const result = await invoke<string>('download_model', {
        modelId,
      });
      return result;
    } catch (error) {
      console.error('Failed to download model:', error);
      throw error;
    }
  }

  /**
   * Delete a model
   */
  async deleteModel(modelId: string): Promise<string> {
    try {
      const result = await invoke<string>('delete_model', {
        modelId,
      });
      return result;
    } catch (error) {
      console.error('Failed to delete model:', error);
      throw error;
    }
  }

  /**
   * Set the active model
   */
  async setActiveModel(modelId: string): Promise<string> {
    try {
      const result = await invoke<string>('set_active_model', {
        modelId,
      });
      return result;
    } catch (error) {
      console.error('Failed to set active model:', error);
      throw error;
    }
  }

  /**
   * Get the currently active model
   */
  async getActiveModel(): Promise<ModelInfo | null> {
    try {
      const model = await invoke<ModelInfo | null>('get_active_model');
      return model;
    } catch (error) {
      console.error('Failed to get active model:', error);
      throw error;
    }
  }

  /**
   * Listen to download progress events
   */
  async onDownloadProgress(callback: (progress: DownloadProgress) => void) {
    const unlisten = await listen<DownloadProgress>('model-download-progress', (event) => {
      callback(event.payload);
    });
    return unlisten;
  }

  /**
   * Format file size for display
   */
  formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  /**
   * Get model size category label
   */
  getModelSizeLabel(size: string): string {
    const labels: Record<string, string> = {
      small: 'Small (1-3B)',
      medium: 'Medium (7-13B)',
      large: 'Large (30-70B)',
    };
    return labels[size] || size;
  }

  /**
   * Get model status badge color
   */
  getModelStatusColor(status: string): string {
    const colors: Record<string, string> = {
      available: 'gray',
      downloading: 'blue',
      downloaded: 'green',
      failed: 'red',
    };
    return colors[status] || 'gray';
  }
}

export const modelService = new ModelService();
