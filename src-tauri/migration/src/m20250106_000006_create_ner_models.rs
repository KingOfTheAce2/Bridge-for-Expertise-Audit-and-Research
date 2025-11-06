use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(NerModels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(NerModels::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Model identification
                    .col(ColumnDef::new(NerModels::ModelId).string().not_null().unique_key())
                    .col(ColumnDef::new(NerModels::Name).string().not_null())
                    .col(ColumnDef::new(NerModels::Description).string())
                    .col(ColumnDef::new(NerModels::Provider).string().not_null())
                    .col(ColumnDef::new(NerModels::ModelType).string().not_null())
                    // NER-specific metadata
                    .col(ColumnDef::new(NerModels::EntityLabels).string().not_null())
                    .col(ColumnDef::new(NerModels::Language).string().not_null())
                    .col(ColumnDef::new(NerModels::Framework).string().not_null())
                    // Model specifications
                    .col(ColumnDef::new(NerModels::Size).string().not_null())
                    .col(ColumnDef::new(NerModels::Parameters).string().not_null())
                    .col(ColumnDef::new(NerModels::Format).string().not_null())
                    // Download information
                    .col(ColumnDef::new(NerModels::Status).string().not_null())
                    .col(ColumnDef::new(NerModels::ModelUrl).string())
                    .col(ColumnDef::new(NerModels::ConfigUrl).string())
                    .col(ColumnDef::new(NerModels::TokenizerUrl).string())
                    .col(ColumnDef::new(NerModels::LocalPath).string())
                    .col(ColumnDef::new(NerModels::FileSize).big_integer())
                    .col(ColumnDef::new(NerModels::DownloadedSize).big_integer())
                    // Verification
                    .col(ColumnDef::new(NerModels::Checksum).string())
                    .col(ColumnDef::new(NerModels::ChecksumVerified).boolean().not_null().default(false))
                    // Performance metrics
                    .col(ColumnDef::new(NerModels::AvgInferenceTimeMs).integer())
                    .col(ColumnDef::new(NerModels::Accuracy).double())
                    .col(ColumnDef::new(NerModels::BenchmarkDataset).string())
                    // Metadata
                    .col(ColumnDef::new(NerModels::IsActive).boolean().not_null().default(false))
                    .col(ColumnDef::new(NerModels::IsFavorite).boolean().not_null().default(false))
                    .col(ColumnDef::new(NerModels::License).string())
                    .col(ColumnDef::new(NerModels::Tags).string())
                    // Usage tracking
                    .col(ColumnDef::new(NerModels::DownloadStartedAt).timestamp())
                    .col(ColumnDef::new(NerModels::DownloadCompletedAt).timestamp())
                    .col(ColumnDef::new(NerModels::LastUsedAt).timestamp())
                    .col(ColumnDef::new(NerModels::UseCount).integer().not_null().default(0))
                    // Timestamps
                    .col(ColumnDef::new(NerModels::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(NerModels::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create indexes
        manager
            .create_index(
                Index::create()
                    .name("idx_ner_models_status")
                    .table(NerModels::Table)
                    .col(NerModels::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ner_models_is_active")
                    .table(NerModels::Table)
                    .col(NerModels::IsActive)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ner_models_language")
                    .table(NerModels::Table)
                    .col(NerModels::Language)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(NerModels::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NerModels {
    Table,
    Id,
    ModelId,
    Name,
    Description,
    Provider,
    ModelType,
    EntityLabels,
    Language,
    Framework,
    Size,
    Parameters,
    Format,
    Status,
    ModelUrl,
    ConfigUrl,
    TokenizerUrl,
    LocalPath,
    FileSize,
    DownloadedSize,
    Checksum,
    ChecksumVerified,
    AvgInferenceTimeMs,
    Accuracy,
    BenchmarkDataset,
    IsActive,
    IsFavorite,
    License,
    Tags,
    DownloadStartedAt,
    DownloadCompletedAt,
    LastUsedAt,
    UseCount,
    CreatedAt,
    UpdatedAt,
}
