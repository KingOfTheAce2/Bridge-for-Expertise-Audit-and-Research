use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Models::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Models::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Model identification
                    .col(ColumnDef::new(Models::ModelId).string().not_null().unique_key())
                    .col(ColumnDef::new(Models::Name).string().not_null())
                    .col(ColumnDef::new(Models::Description).string())
                    .col(ColumnDef::new(Models::Provider).string().not_null())
                    // Model specifications
                    .col(ColumnDef::new(Models::Size).string().not_null())
                    .col(ColumnDef::new(Models::Parameters).string().not_null())
                    .col(ColumnDef::new(Models::Quantization).string())
                    .col(ColumnDef::new(Models::Format).string().not_null())
                    // Download information
                    .col(
                        ColumnDef::new(Models::Status)
                            .string()
                            .not_null()
                            .default("available"),
                    )
                    .col(ColumnDef::new(Models::DownloadUrl).string())
                    .col(ColumnDef::new(Models::FilePath).string())
                    .col(ColumnDef::new(Models::FileSize).big_integer())
                    .col(ColumnDef::new(Models::DownloadedSize).big_integer())
                    // Verification
                    .col(ColumnDef::new(Models::Checksum).string())
                    .col(
                        ColumnDef::new(Models::ChecksumVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    // Metadata
                    .col(
                        ColumnDef::new(Models::IsActive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Models::IsFavorite)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Models::License).string())
                    .col(ColumnDef::new(Models::Tags).string())
                    // Usage tracking
                    .col(ColumnDef::new(Models::DownloadStartedAt).timestamp())
                    .col(ColumnDef::new(Models::DownloadCompletedAt).timestamp())
                    .col(ColumnDef::new(Models::LastUsedAt).timestamp())
                    .col(
                        ColumnDef::new(Models::UseCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    // Timestamps
                    .col(
                        ColumnDef::new(Models::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Models::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on status for faster queries
        manager
            .create_index(
                Index::create()
                    .name("idx_models_status")
                    .table(Models::Table)
                    .col(Models::Status)
                    .to_owned(),
            )
            .await?;

        // Create index on is_active for faster queries
        manager
            .create_index(
                Index::create()
                    .name("idx_models_is_active")
                    .table(Models::Table)
                    .col(Models::IsActive)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Models::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Models {
    Table,
    Id,
    // Model identification
    ModelId,
    Name,
    Description,
    Provider,
    // Model specifications
    Size,
    Parameters,
    Quantization,
    Format,
    // Download information
    Status,
    DownloadUrl,
    FilePath,
    FileSize,
    DownloadedSize,
    // Verification
    Checksum,
    ChecksumVerified,
    // Metadata
    IsActive,
    IsFavorite,
    License,
    Tags,
    // Usage tracking
    DownloadStartedAt,
    DownloadCompletedAt,
    LastUsedAt,
    UseCount,
    // Timestamps
    CreatedAt,
    UpdatedAt,
}
