use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PIIOperations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PIIOperations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    // Operation details
                    .col(
                        ColumnDef::new(PIIOperations::OperationType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PIIOperations::Language)
                            .string()
                            .not_null()
                            .default("en"),
                    )
                    // Text metadata
                    .col(
                        ColumnDef::new(PIIOperations::OriginalLength)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PIIOperations::AnonymizedLength).integer())
                    .col(
                        ColumnDef::new(PIIOperations::EntityCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    // Entity breakdown (JSON)
                    .col(ColumnDef::new(PIIOperations::EntityBreakdown).string())
                    // Settings used (JSON)
                    .col(ColumnDef::new(PIIOperations::SettingsJson).string())
                    // Performance metrics
                    .col(ColumnDef::new(PIIOperations::ProcessingTimeMs).integer())
                    // Compliance tracking
                    .col(ColumnDef::new(PIIOperations::UserId).string())
                    .col(ColumnDef::new(PIIOperations::CaseId).integer())
                    .col(ColumnDef::new(PIIOperations::SessionId).string())
                    // Timestamps
                    .col(
                        ColumnDef::new(PIIOperations::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on created_at for faster queries
        manager
            .create_index(
                Index::create()
                    .name("idx_pii_operations_created_at")
                    .table(PIIOperations::Table)
                    .col(PIIOperations::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create index on operation_type for filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_pii_operations_type")
                    .table(PIIOperations::Table)
                    .col(PIIOperations::OperationType)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PIIOperations::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PIIOperations {
    Table,
    Id,
    OperationType,
    Language,
    OriginalLength,
    AnonymizedLength,
    EntityCount,
    EntityBreakdown,
    SettingsJson,
    ProcessingTimeMs,
    UserId,
    CaseId,
    SessionId,
    CreatedAt,
}
