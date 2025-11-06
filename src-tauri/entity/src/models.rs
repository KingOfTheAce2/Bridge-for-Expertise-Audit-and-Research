use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "models")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    // Model identification
    pub model_id: String,          // e.g., "mistralai/Mistral-7B-Instruct-v0.2"
    pub name: String,               // Display name
    pub description: Option<String>,
    pub provider: String,           // "huggingface", "ollama", etc.

    // Model specifications
    pub size: String,               // "small" (1-3B), "medium" (7-13B), "large" (30-70B)
    pub parameters: String,         // e.g., "7B", "13B"
    pub quantization: Option<String>, // e.g., "Q4_K_M", "Q8_0", null for full precision
    pub format: String,             // "gguf", "safetensors", etc.

    // Download information
    pub status: String,             // "available", "downloading", "downloaded", "failed", "deleted"
    pub download_url: Option<String>,
    pub file_path: Option<String>,  // Local path to downloaded model
    pub file_size: Option<i64>,     // Size in bytes
    pub downloaded_size: Option<i64>, // Current download progress

    // Verification
    pub checksum: Option<String>,   // SHA256 checksum
    pub checksum_verified: bool,

    // Metadata
    pub is_active: bool,            // Currently selected for inference
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
