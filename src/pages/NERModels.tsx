import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import '../styles/NERModels.css';

interface NERModel {
  model_id: string;
  name: string;
  description: string;
  provider: string;
  model_type: string;
  language: string;
  size: string;
  parameters: string;
  file_size: number;
  accuracy: number | null;
  is_downloaded: boolean;
  is_active: boolean;
}

interface DownloadProgress {
  model_id: string;
  file_name: string;
  downloaded: number;
  total: number;
  progress: number;
  speed: number;
}

const NERModels: React.FC = () => {
  const [models, setModels] = useState<NERModel[]>([]);
  const [loading, setLoading] = useState(true);
  const [downloadingModel, setDownloadingModel] = useState<string | null>(null);
  const [downloadProgress, setDownloadProgress] = useState<DownloadProgress | null>(null);
  const [filterLanguage, setFilterLanguage] = useState<string>('all');
  const [filterSize, setFilterSize] = useState<string>('all');
  const [recommendations, setRecommendations] = useState<any>(null);

  useEffect(() => {
    loadModels();
    loadRecommendations();

    // Listen for download progress
    const unlisten = listen<DownloadProgress>('ner-download-progress', (event) => {
      setDownloadProgress(event.payload);
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const loadModels = async () => {
    try {
      setLoading(true);
      const result = await invoke<NERModel[]>('list_ner_models');
      setModels(result);
    } catch (error) {
      console.error('Failed to load NER models:', error);
      alert('Failed to load models: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const loadRecommendations = async () => {
    try {
      const result = await invoke<any>('get_ner_recommendations');
      setRecommendations(result);
    } catch (error) {
      console.error('Failed to load recommendations:', error);
    }
  };

  const downloadModel = async (model_id: string) => {
    try {
      setDownloadingModel(model_id);
      setDownloadProgress(null);

      await invoke('download_ner_model', {
        request: { model_id },
      });

      alert('Model downloaded successfully!');
      await loadModels();
    } catch (error) {
      console.error('Download failed:', error);
      alert('Download failed: ' + error);
    } finally {
      setDownloadingModel(null);
      setDownloadProgress(null);
    }
  };

  const deleteModel = async (model_id: string) => {
    if (!confirm(`Are you sure you want to delete ${model_id}?`)) {
      return;
    }

    try {
      await invoke('delete_ner_model', { modelId: model_id });
      alert('Model deleted successfully!');
      await loadModels();
    } catch (error) {
      console.error('Delete failed:', error);
      alert('Delete failed: ' + error);
    }
  };

  const loadModel = async (model_id: string) => {
    try {
      await invoke('load_ner_model', { modelId: model_id });
      alert('Model loaded successfully!');
      await loadModels();
    } catch (error) {
      console.error('Load failed:', error);
      alert('Load failed: ' + error);
    }
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  };

  const getSizeColor = (size: string): string => {
    switch (size) {
      case 'small':
        return '#48bb78'; // Green
      case 'medium':
        return '#ed8936'; // Orange
      case 'large':
        return '#f56565'; // Red
      default:
        return '#4299e1'; // Blue
    }
  };

  const getRecommendationBadge = (model_id: string): string | null => {
    if (!recommendations) return null;

    if (recommendations.recommended === model_id) return 'Recommended';
    if (recommendations.fastest === model_id) return 'Fastest';
    if (recommendations.most_accurate === model_id) return 'Most Accurate';
    if (recommendations.multilingual === model_id) return 'Multilingual';

    return null;
  };

  const filteredModels = models.filter((model) => {
    if (filterLanguage !== 'all' && model.language !== filterLanguage) {
      return false;
    }
    if (filterSize !== 'all' && model.size !== filterSize) {
      return false;
    }
    return true;
  });

  if (loading) {
    return (
      <div className="ner-models-page">
        <div className="loading">Loading NER models...</div>
      </div>
    );
  }

  return (
    <div className="ner-models-page">
      <div className="page-header">
        <h1>NER Models</h1>
        <p className="subtitle">
          Download and manage Named Entity Recognition models for enhanced PII detection
        </p>
      </div>

      {/* Filters */}
      <div className="filters">
        <div className="filter-group">
          <label>Language:</label>
          <select value={filterLanguage} onChange={(e) => setFilterLanguage(e.target.value)}>
            <option value="all">All Languages</option>
            <option value="en">English</option>
            <option value="multilingual">Multilingual</option>
          </select>
        </div>

        <div className="filter-group">
          <label>Size:</label>
          <select value={filterSize} onChange={(e) => setFilterSize(e.target.value)}>
            <option value="all">All Sizes</option>
            <option value="small">Small (&lt; 100MB)</option>
            <option value="medium">Medium (100-500MB)</option>
            <option value="large">Large (&gt; 500MB)</option>
          </select>
        </div>
      </div>

      {/* Models Grid */}
      <div className="models-grid">
        {filteredModels.map((model) => {
          const isDownloading = downloadingModel === model.model_id;
          const badge = getRecommendationBadge(model.model_id);

          return (
            <div key={model.model_id} className="model-card">
              {badge && <div className="recommendation-badge">{badge}</div>}

              <div className="model-header">
                <h3>{model.name}</h3>
                <span
                  className="size-badge"
                  style={{ backgroundColor: getSizeColor(model.size) }}
                >
                  {model.size}
                </span>
              </div>

              <p className="model-description">{model.description}</p>

              <div className="model-meta">
                <div className="meta-item">
                  <span className="meta-label">Provider:</span>
                  <span className="meta-value">{model.provider}</span>
                </div>
                <div className="meta-item">
                  <span className="meta-label">Type:</span>
                  <span className="meta-value">{model.model_type}</span>
                </div>
                <div className="meta-item">
                  <span className="meta-label">Language:</span>
                  <span className="meta-value">{model.language}</span>
                </div>
                <div className="meta-item">
                  <span className="meta-label">Parameters:</span>
                  <span className="meta-value">{model.parameters}</span>
                </div>
                <div className="meta-item">
                  <span className="meta-label">Size:</span>
                  <span className="meta-value">{formatFileSize(model.file_size)}</span>
                </div>
                {model.accuracy && (
                  <div className="meta-item">
                    <span className="meta-label">Accuracy (F1):</span>
                    <span className="meta-value">{(model.accuracy * 100).toFixed(1)}%</span>
                  </div>
                )}
              </div>

              {/* Download Progress */}
              {isDownloading && downloadProgress && downloadProgress.model_id === model.model_id && (
                <div className="download-progress">
                  <div className="progress-bar">
                    <div
                      className="progress-fill"
                      style={{ width: `${downloadProgress.progress}%` }}
                    />
                  </div>
                  <div className="progress-info">
                    <span>{downloadProgress.file_name}</span>
                    <span>
                      {formatFileSize(downloadProgress.downloaded)} /{' '}
                      {formatFileSize(downloadProgress.total)} ({downloadProgress.progress.toFixed(1)}%)
                    </span>
                    <span>{downloadProgress.speed.toFixed(2)} MB/s</span>
                  </div>
                </div>
              )}

              {/* Actions */}
              <div className="model-actions">
                {!model.is_downloaded ? (
                  <button
                    className="btn btn-primary"
                    onClick={() => downloadModel(model.model_id)}
                    disabled={isDownloading}
                  >
                    {isDownloading ? 'Downloading...' : 'Download'}
                  </button>
                ) : (
                  <>
                    <button
                      className="btn btn-success"
                      onClick={() => loadModel(model.model_id)}
                      disabled={model.is_active}
                    >
                      {model.is_active ? 'Active' : 'Load Model'}
                    </button>
                    <button
                      className="btn btn-danger"
                      onClick={() => deleteModel(model.model_id)}
                    >
                      Delete
                    </button>
                  </>
                )}
              </div>
            </div>
          );
        })}
      </div>

      {filteredModels.length === 0 && (
        <div className="no-models">
          <p>No models found matching the selected filters.</p>
        </div>
      )}
    </div>
  );
};

export default NERModels;
