import React, { useState } from 'react';
import { modelService } from '../services/modelService';
import '../styles/AddCustomModel.css';

interface AddCustomModelProps {
  onClose: () => void;
  onSuccess: () => void;
}

const AddCustomModel: React.FC<AddCustomModelProps> = ({ onClose, onSuccess }) => {
  const [formData, setFormData] = useState({
    model_id: '',
    name: '',
    description: '',
    download_url: '',
    size: 'medium',
    parameters: '',
    quantization: '',
    format: 'gguf',
    file_size: '',
    checksum: '',
    tags: '',
  });

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      const fileSizeBytes = parseInt(formData.file_size) * 1_000_000_000; // Convert GB to bytes
      const tagsArray = formData.tags
        .split(',')
        .map((t) => t.trim())
        .filter((t) => t.length > 0);

      await modelService.addCustomModel({
        model_id: formData.model_id,
        name: formData.name,
        description: formData.description,
        download_url: formData.download_url,
        size: formData.size,
        parameters: formData.parameters,
        quantization: formData.quantization || undefined,
        format: formData.format,
        file_size: fileSizeBytes,
        checksum: formData.checksum || undefined,
        tags: tagsArray,
      });

      onSuccess();
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
  ) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Add Custom Model</h2>
          <button className="close-button" onClick={onClose}>
            &times;
          </button>
        </div>

        <form onSubmit={handleSubmit} className="custom-model-form">
          <div className="form-group">
            <label htmlFor="model_id">Model ID *</label>
            <input
              type="text"
              id="model_id"
              name="model_id"
              value={formData.model_id}
              onChange={handleChange}
              required
              placeholder="e.g., custom/my-model-7b"
            />
          </div>

          <div className="form-group">
            <label htmlFor="name">Name *</label>
            <input
              type="text"
              id="name"
              name="name"
              value={formData.name}
              onChange={handleChange}
              required
              placeholder="e.g., My Custom Model 7B"
            />
          </div>

          <div className="form-group">
            <label htmlFor="description">Description *</label>
            <textarea
              id="description"
              name="description"
              value={formData.description}
              onChange={handleChange}
              required
              placeholder="Describe the model and its use case..."
              rows={3}
            />
          </div>

          <div className="form-group">
            <label htmlFor="download_url">Download URL *</label>
            <input
              type="url"
              id="download_url"
              name="download_url"
              value={formData.download_url}
              onChange={handleChange}
              required
              placeholder="https://example.com/model.gguf"
            />
          </div>

          <div className="form-row">
            <div className="form-group">
              <label htmlFor="size">Size Category *</label>
              <select
                id="size"
                name="size"
                value={formData.size}
                onChange={handleChange}
                required
              >
                <option value="small">Small (1-3B)</option>
                <option value="medium">Medium (7-13B)</option>
                <option value="large">Large (30-70B)</option>
              </select>
            </div>

            <div className="form-group">
              <label htmlFor="parameters">Parameters *</label>
              <input
                type="text"
                id="parameters"
                name="parameters"
                value={formData.parameters}
                onChange={handleChange}
                required
                placeholder="e.g., 7B"
              />
            </div>
          </div>

          <div className="form-row">
            <div className="form-group">
              <label htmlFor="format">Format *</label>
              <select
                id="format"
                name="format"
                value={formData.format}
                onChange={handleChange}
                required
              >
                <option value="gguf">GGUF</option>
                <option value="safetensors">SafeTensors</option>
                <option value="bin">BIN</option>
              </select>
            </div>

            <div className="form-group">
              <label htmlFor="quantization">Quantization</label>
              <input
                type="text"
                id="quantization"
                name="quantization"
                value={formData.quantization}
                onChange={handleChange}
                placeholder="e.g., Q4_K_M"
              />
            </div>
          </div>

          <div className="form-row">
            <div className="form-group">
              <label htmlFor="file_size">File Size (GB) *</label>
              <input
                type="number"
                id="file_size"
                name="file_size"
                value={formData.file_size}
                onChange={handleChange}
                required
                step="0.01"
                min="0"
                placeholder="e.g., 4.5"
              />
            </div>

            <div className="form-group">
              <label htmlFor="checksum">SHA256 Checksum</label>
              <input
                type="text"
                id="checksum"
                name="checksum"
                value={formData.checksum}
                onChange={handleChange}
                placeholder="Optional for verification"
              />
            </div>
          </div>

          <div className="form-group">
            <label htmlFor="tags">Tags (comma-separated)</label>
            <input
              type="text"
              id="tags"
              name="tags"
              value={formData.tags}
              onChange={handleChange}
              placeholder="e.g., chat, instruction, legal"
            />
          </div>

          {error && <div className="error-message">{error}</div>}

          <div className="form-actions">
            <button type="button" onClick={onClose} className="btn btn-secondary">
              Cancel
            </button>
            <button type="submit" disabled={loading} className="btn btn-primary">
              {loading ? 'Adding...' : 'Add Model'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default AddCustomModel;
