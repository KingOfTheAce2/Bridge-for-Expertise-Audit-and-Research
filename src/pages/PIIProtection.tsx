import React, { useState, useEffect } from 'react';
import {
  piiService,
  AnonymizationResult,
  AnonymizationSettings,
  EntityStatistics,
} from '../services/piiService';
import '../styles/PIIProtection.css';

const PIIProtection: React.FC = () => {
  const [inputText, setInputText] = useState('');
  const [result, setResult] = useState<AnonymizationResult | null>(null);
  const [settings, setSettings] = useState<AnonymizationSettings | null>(null);
  const [statistics, setStatistics] = useState<EntityStatistics | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadDefaultSettings();
  }, []);

  const loadDefaultSettings = async () => {
    try {
      const defaultSettings = await piiService.getDefaultSettings();
      setSettings(defaultSettings);
    } catch (err) {
      console.error('Failed to load default settings:', err);
    }
  };

  const handleAnonymize = async () => {
    if (!inputText.trim()) {
      setError('Please enter some text to anonymize');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      const anonymizationResult = await piiService.anonymizeText(
        inputText,
        settings || undefined
      );
      setResult(anonymizationResult);

      // Load statistics
      const stats = await piiService.getStatistics();
      setStatistics(stats);
    } catch (err) {
      setError('Failed to anonymize text: ' + err);
    } finally {
      setLoading(false);
    }
  };

  const handleClearReplacements = async () => {
    try {
      await piiService.clearReplacements();
      setResult(null);
      setStatistics(null);
      alert('Replacement mappings cleared');
    } catch (err) {
      alert('Failed to clear replacements: ' + err);
    }
  };

  const handleCopyAnonymized = () => {
    if (result) {
      navigator.clipboard.writeText(result.anonymized_text);
      alert('Anonymized text copied to clipboard');
    }
  };

  const getSampleText = () => {
    return `John Smith filed a complaint under Article 6 GDPR on 2024-03-15.
Mr. Smith claimed that Acme Corporation violated his privacy rights by sharing his email address john.smith@example.com without consent.
The complaint was filed with Case Number 2024-PRIV-001 and seeks damages of $50,000.
Mr. Smith resides at 123 Main Street, New York, NY 10001 and can be reached at (555) 123-4567.`;
  };

  const loadSampleText = () => {
    setInputText(getSampleText());
  };

  return (
    <div className="pii-protection-container">
      <header className="pii-header">
        <div>
          <h1>PII Protection & Anonymization</h1>
          <p>Protect sensitive information with intelligent, context-aware anonymization</p>
        </div>
      </header>

      <div className="pii-content">
        <div className="input-section">
          <div className="section-header">
            <h2>Input Text</h2>
            <div className="button-group">
              <button className="btn btn-secondary btn-small" onClick={loadSampleText}>
                Load Sample
              </button>
              <button
                className="btn btn-secondary btn-small"
                onClick={() => setInputText('')}
              >
                Clear
              </button>
            </div>
          </div>
          <textarea
            className="text-input"
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            placeholder="Enter text containing sensitive information..."
            rows={12}
          />
          <button
            className="btn btn-primary btn-large"
            onClick={handleAnonymize}
            disabled={loading || !inputText.trim()}
          >
            {loading ? 'Anonymizing...' : 'Anonymize Text'}
          </button>
        </div>

        {result && (
          <div className="output-section">
            <div className="section-header">
              <h2>Anonymized Text</h2>
              <button className="btn btn-secondary btn-small" onClick={handleCopyAnonymized}>
                Copy
              </button>
            </div>
            <div className="text-output">
              {result.anonymized_text}
            </div>

            <div className="entities-section">
              <h3>Detected Entities ({result.entities.length})</h3>
              <div className="entities-grid">
                {result.entities.map((entity, index) => {
                  // Extract entity type
                  const entityTypeKey = Object.keys(entity.entity_type)[0];
                  const color = piiService.getEntityColor(entityTypeKey);

                  return (
                    <div key={index} className="entity-card">
                      <div
                        className="entity-badge"
                        style={{ backgroundColor: color }}
                      >
                        {piiService.formatEntityType(entityTypeKey)}
                      </div>
                      <div className="entity-details">
                        <div className="entity-original">"{entity.text}"</div>
                        {entity.replacement && (
                          <div className="entity-replacement">→ {entity.replacement}</div>
                        )}
                        <div className="entity-meta">
                          Confidence: {(entity.confidence * 100).toFixed(0)}%
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            {statistics && statistics.total_entities > 0 && (
              <div className="statistics-section">
                <h3>Statistics</h3>
                <div className="stats-grid">
                  <div className="stat-card">
                    <div className="stat-value">{statistics.total_entities}</div>
                    <div className="stat-label">Total Entities</div>
                  </div>
                  {statistics.entity_counts.map(([entityType, count]) => (
                    <div key={entityType} className="stat-card">
                      <div className="stat-value">{count}</div>
                      <div className="stat-label">
                        {piiService.formatEntityType(entityType)}
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            <div className="actions-section">
              <button
                className="btn btn-secondary"
                onClick={handleClearReplacements}
              >
                Clear Replacement Mappings
              </button>
            </div>
          </div>
        )}

        {error && <div className="error-message">{error}</div>}
      </div>

      <div className="pii-info">
        <h3>How It Works</h3>
        <ul>
          <li>
            <strong>Pattern Recognition</strong>: Detects emails, phone numbers, dates, money amounts, and more
          </li>
          <li>
            <strong>Named Entity Recognition</strong>: Identifies persons, organizations, and locations
          </li>
          <li>
            <strong>Consistent Replacement</strong>: Same entities get the same replacement throughout the document
          </li>
          <li>
            <strong>Legal Reference Preservation</strong>: Legal citations and references are NOT anonymized
          </li>
          <li>
            <strong>Privacy-First</strong>: All processing happens locally - your data never leaves your machine
          </li>
        </ul>
      </div>
    </div>
  );
};

export default PIIProtection;
