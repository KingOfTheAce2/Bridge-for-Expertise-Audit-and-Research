use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ner_models")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    // Model identification
    pub model_id: String,          // e.g., "dslim/bert-base-NER"
    pub name: String,               // Display name
    pub description: Option<String>,
    pub provider: String,           // "huggingface", "custom", etc.
    pub model_type: String,         // "bert", "roberta", "distilbert", etc.

    // NER-specific metadata
    pub entity_labels: String,      // JSON array of supported labels: ["PER", "ORG", "LOC", etc.]
    pub language: String,           // "en", "multilingual", etc.
    pub framework: String,          // "candle", "onnx", etc.

    // Model specifications
    pub size: String,               // "small" (< 100MB), "medium" (100-500MB), "large" (> 500MB)
    pub parameters: String,         // e.g., "110M", "355M"
    pub format: String,             // "safetensors", "onnx", etc.

    // Download information
    pub status: String,             // "available", "downloading", "downloaded", "failed", "deleted"
    pub model_url: Option<String>,  // URL to model weights
    pub config_url: Option<String>, // URL to config.json
    pub tokenizer_url: Option<String>, // URL to tokenizer files
    pub local_path: Option<String>, // Local directory path
    pub file_size: Option<i64>,     // Total size in bytes
    pub downloaded_size: Option<i64>, // Current download progress

    // Verification
    pub checksum: Option<String>,   // SHA256 checksum
    pub checksum_verified: bool,

    // Performance metrics
    pub avg_inference_time_ms: Option<i32>, // Average inference time
    pub accuracy: Option<f64>,      // F1 score or accuracy metric
    pub benchmark_dataset: Option<String>, // Dataset used for benchmarking

    // Metadata
    pub is_active: bool,            // Currently selected for NER inference
    pub is_favorite: bool,
    pub license: Option<String>,
    pub tags: Option<String>,       // JSON array of tags

    // Usage tracking
    pub download_started_at: Option<DateTime>,
    pub download_completed_at: Option<DateTime>,
    pub last_used_at: Option<DateTime>,
    pub use_count: i32,

    // Timestamps
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
