import React, { useEffect, useState } from 'react';
import { modelService, ModelInfo, DownloadProgress } from '../services/modelService';
import AddCustomModel from '../components/AddCustomModel';
import '../styles/Models.css';

const Models: React.FC = () => {
  const [models, setModels] = useState<ModelInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [downloadProgress, setDownloadProgress] = useState<Record<string, DownloadProgress>>({});
  const [filterSize, setFilterSize] = useState<string>('all');
  const [activeModelId, setActiveModelId] = useState<string | null>(null);
  const [showAddCustomModel, setShowAddCustomModel] = useState(false);
  const [diskSpace, setDiskSpace] = useState<number | null>(null);

  useEffect(() => {
    loadModels();
    loadActiveModel();
    loadDiskSpace();

    // Listen to download progress
    const setupListener = async () => {
      await modelService.onDownloadProgress((progress) => {
        setDownloadProgress((prev) => ({
          ...prev,
          [progress.model_id]: progress,
        }));

        // Reload models when download completes
        if (progress.status === 'Completed' || progress.status === 'Failed' || progress.status === 'Cancelled') {
          setTimeout(() => {
            loadModels();
            loadDiskSpace();
          }, 1000);
        }
      });
    };

    setupListener();
  }, []);

  const loadModels = async () => {
    try {
      setLoading(true);
      const modelsList = await modelService.listModels();
      setModels(modelsList);
      setError(null);
    } catch (err) {
      setError('Failed to load models');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const loadActiveModel = async () => {
    try {
      const activeModel = await modelService.getActiveModel();
      if (activeModel) {
        setActiveModelId(activeModel.model_id);
      }
    } catch (err) {
      console.error('Failed to load active model:', err);
    }
  };

  const loadDiskSpace = async () => {
    try {
      const space = await modelService.checkDiskSpace();
      setDiskSpace(space);
    } catch (err) {
      console.error('Failed to load disk space:', err);
    }
  };

  const handleDownload = async (modelId: string) => {
    try {
      await modelService.downloadModel(modelId);
      // Progress will be tracked via event listener
    } catch (err) {
      alert('Failed to start download: ' + err);
    }
  };

  const handleDelete = async (modelId: string) => {
    if (!confirm('Are you sure you want to delete this model?')) {
      return;
    }

    try {
      await modelService.deleteModel(modelId);
      await loadModels();
    } catch (err) {
      alert('Failed to delete model: ' + err);
    }
  };

  const handleSetActive = async (modelId: string) => {
    try {
      await modelService.setActiveModel(modelId);
      setActiveModelId(modelId);
      await loadModels();
    } catch (err) {
      alert('Failed to set active model: ' + err);
    }
  };

  const handleCancelDownload = async () => {
    try {
      await modelService.cancelDownload();
    } catch (err) {
      console.error('Failed to cancel download:', err);
    }
  };

  const filteredModels = filterSize === 'all'
    ? models
    : models.filter((m) => m.size === filterSize);

  const getModelProgress = (modelId: string) => {
    return downloadProgress[modelId];
  };

  if (loading) {
    return (
      <div className="models-container">
        <div className="loading">Loading models...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="models-container">
        <div className="error">{error}</div>
        <button onClick={loadModels}>Retry</button>
      </div>
    );
  }

  return (
    <div className="models-container">
      <header className="models-header">
        <div className="header-content">
          <div>
            <h1>AI Models</h1>
            <p>Download and manage AI models for local inference</p>
          </div>
          <button
            className="btn btn-primary"
            onClick={() => setShowAddCustomModel(true)}
          >
            + Add Custom Model
          </button>
        </div>
      </header>

      <div className="models-filters">
        <div className="filter-section">
          <label>Filter by size:</label>
          <select value={filterSize} onChange={(e) => setFilterSize(e.target.value)}>
            <option value="all">All Sizes</option>
            <option value="small">Small (1-3B)</option>
            <option value="medium">Medium (7-13B)</option>
            <option value="large">Large (30-70B)</option>
          </select>
        </div>
        {diskSpace !== null && (
          <div className="disk-space-info">
            <span>Available Space: {modelService.formatFileSize(diskSpace)}</span>
          </div>
        )}
      </div>

      <div className="models-grid">
        {filteredModels.map((model) => {
          const progress = getModelProgress(model.model_id);
          const isDownloading = progress?.status === 'Downloading' || progress?.status === 'Starting';

          return (
            <div key={model.model_id} className={`model-card ${model.is_active ? 'active' : ''}`}>
              <div className="model-header">
                <h3>{model.name}</h3>
                <span className={`status-badge ${modelService.getModelStatusColor(model.status)}`}>
                  {model.status}
                </span>
              </div>

              <p className="model-description">{model.description}</p>

              <div className="model-details">
                <div className="detail-item">
                  <span className="label">Size:</span>
                  <span className="value">{modelService.getModelSizeLabel(model.size)}</span>
                </div>
                <div className="detail-item">
                  <span className="label">Parameters:</span>
                  <span className="value">{model.parameters}</span>
                </div>
                <div className="detail-item">
                  <span className="label">File Size:</span>
                  <span className="value">{modelService.formatFileSize(model.file_size)}</span>
                </div>
                {model.quantization && (
                  <div className="detail-item">
                    <span className="label">Quantization:</span>
                    <span className="value">{model.quantization}</span>
                  </div>
                )}
              </div>

              {model.tags && model.tags.length > 0 && (
                <div className="model-tags">
                  {model.tags.map((tag) => (
                    <span key={tag} className="tag">{tag}</span>
                  ))}
                </div>
              )}

              {isDownloading && progress && (
                <div className="download-progress">
                  <div className="progress-info">
                    <span>Downloading... {progress.percentage.toFixed(1)}%</span>
                    <span>{progress.speed_mbps.toFixed(2)} MB/s</span>
                  </div>
                  <div className="progress-bar">
                    <div
                      className="progress-fill"
                      style={{ width: `${progress.percentage}%` }}
                    />
                  </div>
                  <div className="progress-details">
                    <span>
                      {modelService.formatFileSize(progress.downloaded_bytes)} / {modelService.formatFileSize(progress.total_bytes)}
                    </span>
                    <button
                      className="btn-cancel-download"
                      onClick={handleCancelDownload}
                      title="Cancel download"
                    >
                      Cancel
                    </button>
                  </div>
                </div>
              )}

              <div className="model-actions">
                {!model.is_downloaded && !isDownloading && (
                  <button
                    className="btn btn-primary"
                    onClick={() => handleDownload(model.model_id)}
                  >
                    Download
                  </button>
                )}

                {model.is_downloaded && !model.is_active && (
                  <>
                    <button
                      className="btn btn-primary"
                      onClick={() => handleSetActive(model.model_id)}
                    >
                      Activate
                    </button>
                    <button
                      className="btn btn-danger"
                      onClick={() => handleDelete(model.model_id)}
                    >
                      Delete
                    </button>
                  </>
                )}

                {model.is_active && (
                  <div className="active-indicator">
                    ✓ Active Model
                  </div>
                )}
              </div>
            </div>
          );
        })}
      </div>

      {filteredModels.length === 0 && (
        <div className="no-models">
          No models found for the selected filter.
        </div>
      )}

      {showAddCustomModel && (
        <AddCustomModel
          onClose={() => setShowAddCustomModel(false)}
          onSuccess={() => {
            loadModels();
            alert('Custom model added successfully!');
          }}
        />
      )}
    </div>
  );
};

export default Models;
