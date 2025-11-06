use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pii_operations")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    // Operation details
    pub operation_type: String, // "anonymize", "detect", "batch_anonymize"
    pub language: String,        // "en", "nl", "de", etc.

    // Text metadata
    pub original_length: i32,
    pub anonymized_length: Option<i32>,
    pub entity_count: i32,

    // Entity breakdown (JSON)
    pub entity_breakdown: Option<String>, // JSON: {"PERSON": 3, "EMAIL": 2, ...}

    // Settings used (JSON)
    pub settings_json: Option<String>,

    // Performance metrics
    pub processing_time_ms: Option<i32>,

    // Compliance tracking
    pub user_id: Option<String>,
    pub case_id: Option<i32>,
    pub session_id: Option<String>,

    // Timestamps
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
