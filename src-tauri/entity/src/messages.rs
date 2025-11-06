use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub conversation_id: i32,
    pub role: String,
    pub content: String,
    pub is_ai_generated: bool,
    pub was_edited: bool,

    // AI Act Compliance - Article 52
    pub content_source: String,  // "ai" | "human" | "ai-assisted"

    // Output Provenance - Article 52
    pub model_name: Option<String>,        // e.g., "mistral-7b-instruct-v0.2"
    pub model_version: Option<String>,     // e.g., "v0.2" or date
    pub generation_timestamp: Option<DateTime>,
    pub anonymization_applied: Option<String>, // "layer1-regex" | "layer2-ner" | "none"
    pub edit_count: i32,                   // Number of user edits
    pub metadata: Option<String>,          // JSON metadata

    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::conversations::Entity",
        from = "Column::ConversationId",
        to = "super::conversations::Column::Id",
        on_delete = "Cascade"
    )]
    Conversations,
}

impl Related<super::conversations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
