/// NER (Named Entity Recognition) module for advanced PII detection
///
/// This module provides ML-based entity recognition using transformer models
/// through the Candle framework. It complements pattern-based detection with
/// context-aware entity extraction.

pub mod types;
pub mod model_loader;
pub mod tokenizer;
pub mod inference;
pub mod hybrid_detector;
pub mod registry;
pub mod downloader;

pub use types::*;
pub use model_loader::{NerModel, NerModelManager};
pub use tokenizer::NerTokenizer;
pub use inference::NerPipeline;
pub use hybrid_detector::{HybridDetector, DetectionMode};
pub use registry::NerModelRegistry;
pub use downloader::NerModelDownloader;
