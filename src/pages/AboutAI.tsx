import React from 'react';
import '../styles/AboutAI.css';

const AboutAI: React.FC = () => {
  return (
    <div className="about-ai-page">
      <div className="about-ai-container">
        {/* Header */}
        <div className="about-ai-header">
          <h1>About AI in BEAR LLM</h1>
          <p className="subtitle">
            Understanding how artificial intelligence is used in this application
          </p>
        </div>

        {/* Main Content */}
        <div className="about-ai-content">
          {/* What AI Does */}
          <section className="ai-section">
            <div className="section-icon">🤖</div>
            <h2>What AI Does in This App</h2>
            <p>
              This application uses <strong>local artificial intelligence</strong> to assist
              with legal document drafting, summarizing, and search. The AI acts as a
              smart assistant that helps you work faster and more efficiently.
            </p>
            <ul className="feature-list">
              <li>
                <strong>Document Drafting:</strong> Generate initial drafts of legal documents
                based on your requirements
              </li>
              <li>
                <strong>Summarization:</strong> Create concise summaries of long legal texts
              </li>
              <li>
                <strong>Search & Analysis:</strong> Find relevant information across your
                documents
              </li>
              <li>
                <strong>PII Detection:</strong> Automatically identify and protect personally
                identifiable information
              </li>
              <li>
                <strong>Translation:</strong> Translate legal documents while preserving
                terminology
              </li>
            </ul>
          </section>

          {/* Privacy First */}
          <section className="ai-section highlight-section">
            <div className="section-icon">🔒</div>
            <h2>Your Privacy is Protected</h2>
            <div className="privacy-guarantee">
              <p className="lead-text">
                <strong>100% Local Processing - No Data Ever Leaves Your Device</strong>
              </p>
              <ul className="privacy-list">
                <li>
                  All AI processing happens <strong>on your computer</strong>
                </li>
                <li>
                  <strong>No internet connection</strong> is required for AI features
                </li>
                <li>
                  <strong>No data is sent</strong> to external servers or cloud services
                </li>
                <li>
                  <strong>No tracking or analytics</strong> of your documents
                </li>
                <li>
                  You maintain <strong>full control</strong> over all your data
                </li>
              </ul>
            </div>
          </section>

          {/* How It Works */}
          <section className="ai-section">
            <div className="section-icon">⚙️</div>
            <h2>How Local AI Works</h2>
            <div className="how-it-works">
              <div className="step">
                <div className="step-number">1</div>
                <div className="step-content">
                  <h3>Download AI Models</h3>
                  <p>
                    AI models are downloaded once and stored on your computer. These models
                    are the "brains" that power the AI features.
                  </p>
                </div>
              </div>
              <div className="step">
                <div className="step-number">2</div>
                <div className="step-content">
                  <h3>Local Processing</h3>
                  <p>
                    When you use AI features, your computer processes everything locally
                    using these downloaded models. No internet required.
                  </p>
                </div>
              </div>
              <div className="step">
                <div className="step-number">3</div>
                <div className="step-content">
                  <h3>Your Control</h3>
                  <p>
                    You review and edit all AI-generated content. The AI assists, but you
                    maintain full control and responsibility.
                  </p>
                </div>
              </div>
            </div>
          </section>

          {/* AI Models */}
          <section className="ai-section">
            <div className="section-icon">📦</div>
            <h2>AI Models Available</h2>
            <p>
              You can choose from different AI models based on your needs. Each model
              represents a different balance of speed, accuracy, and resource usage.
            </p>
            <div className="models-grid">
              <div className="model-card">
                <h3>Small Models (1-3B parameters)</h3>
                <p>Fast processing, lower quality. Good for quick tasks.</p>
                <ul>
                  <li>TinyLlama 1.1B</li>
                  <li>Phi-2 2.7B</li>
                </ul>
              </div>
              <div className="model-card">
                <h3>Medium Models (7-13B parameters)</h3>
                <p>Balanced speed and quality. Recommended for most users.</p>
                <ul>
                  <li>Mistral 7B (Recommended)</li>
                  <li>Llama 2 7B/13B</li>
                </ul>
              </div>
              <div className="model-card">
                <h3>NER Models (Named Entity Recognition)</h3>
                <p>Specialized for detecting names, organizations, and locations.</p>
                <ul>
                  <li>BERT-base NER</li>
                  <li>RoBERTa NER (Most Accurate)</li>
                  <li>XLM-RoBERTa (Multilingual)</li>
                </ul>
              </div>
            </div>
          </section>

          {/* Limitations */}
          <section className="ai-section">
            <div className="section-icon">⚠️</div>
            <h2>Important Limitations</h2>
            <div className="limitations-box">
              <ul className="limitations-list">
                <li>
                  <strong>AI is an assistant, not a lawyer:</strong> All AI-generated
                  content must be reviewed and verified by qualified professionals.
                </li>
                <li>
                  <strong>Not always accurate:</strong> AI can make mistakes, produce
                  hallucinations, or misinterpret context.
                </li>
                <li>
                  <strong>No legal advice:</strong> This tool does not provide legal
                  advice and should not be used as a substitute for professional legal
                  counsel.
                </li>
                <li>
                  <strong>Your responsibility:</strong> You are responsible for all
                  content you create, edit, or submit, even if AI-assisted.
                </li>
                <li>
                  <strong>Training data cutoff:</strong> AI models are trained on
                  historical data and may not reflect the latest laws or regulations.
                </li>
              </ul>
            </div>
          </section>

          {/* Transparency */}
          <section className="ai-section">
            <div className="section-icon">🏷️</div>
            <h2>AI Content Labeling</h2>
            <p>
              In compliance with the <strong>EU AI Act (Article 52)</strong>, all
              AI-generated or AI-assisted content is clearly labeled:
            </p>
            <div className="badge-examples">
              <div className="badge-example">
                <div className="example-badge ai-badge-ai">
                  🤖 <strong>AI GENERATED</strong>
                </div>
                <p>Content created entirely by AI</p>
              </div>
              <div className="badge-example">
                <div className="example-badge ai-badge-assisted">
                  ✨ <strong>AI ASSISTED</strong>
                </div>
                <p>AI-generated content edited by humans</p>
              </div>
              <div className="badge-example">
                <div className="example-badge ai-badge-human">
                  👤 <strong>HUMAN CREATED</strong>
                </div>
                <p>Content created entirely by humans</p>
              </div>
            </div>
          </section>

          {/* Best Practices */}
          <section className="ai-section">
            <div className="section-icon">✅</div>
            <h2>Best Practices for Using AI</h2>
            <ol className="best-practices-list">
              <li>
                <strong>Always review:</strong> Carefully review all AI-generated content
                before using it.
              </li>
              <li>
                <strong>Fact-check:</strong> Verify facts, dates, and legal citations
                independently.
              </li>
              <li>
                <strong>Edit and refine:</strong> Use AI output as a starting point, then
                refine and personalize.
              </li>
              <li>
                <strong>Protect sensitive data:</strong> Use PII protection features when
                working with personal information.
              </li>
              <li>
                <strong>Keep control:</strong> Make final decisions yourself; don't blindly
                accept AI suggestions.
              </li>
              <li>
                <strong>Stay updated:</strong> Keep AI models updated to benefit from
                improvements.
              </li>
            </ol>
          </section>

          {/* Support */}
          <section className="ai-section">
            <div className="section-icon">💡</div>
            <h2>Need Help?</h2>
            <p>
              If you have questions about AI features or need assistance:
            </p>
            <ul>
              <li>Visit the <strong>Settings</strong> page to configure AI models</li>
              <li>Check the <strong>Models</strong> page to download or manage AI models</li>
              <li>Use the <strong>NER Models</strong> page for PII detection configuration</li>
              <li>
                Report issues on GitHub:{' '}
                <a
                  href="https://github.com/anthropics/claude-code/issues"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  https://github.com/anthropics/claude-code/issues
                </a>
              </li>
            </ul>
          </section>
        </div>

        {/* Footer */}
        <div className="about-ai-footer">
          <p className="compliance-notice">
            This application complies with the <strong>EU Artificial Intelligence Act</strong>{' '}
            (Regulation (EU) 2024/1689), particularly Article 52 requirements for AI
            transparency and disclosure.
          </p>
          <p className="last-updated">Last updated: January 2025</p>
        </div>
      </div>
    </div>
  );
};

export default AboutAI;
