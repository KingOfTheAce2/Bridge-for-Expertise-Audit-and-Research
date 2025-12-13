use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add AI Act Compliance fields to messages table
        // SQLite only supports one column per ALTER TABLE statement

        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_column(
                        ColumnDef::new(Messages::ContentSource)
                            .string()
                            .not_null()
                            .default("human"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_column(ColumnDef::new(Messages::ModelName).string())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_column(ColumnDef::new(Messages::ModelVersion).string())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_column(ColumnDef::new(Messages::GenerationTimestamp).timestamp())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_column(ColumnDef::new(Messages::AnonymizationApplied).string())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_column(
                        ColumnDef::new(Messages::EditCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_column(ColumnDef::new(Messages::Metadata).string())
                    .to_owned(),
            )
            .await?;

        // Add index on content_source for filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_messages_content_source")
                    .table(Messages::Table)
                    .col(Messages::ContentSource)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .drop_column(Messages::ContentSource)
                    .drop_column(Messages::ModelName)
                    .drop_column(Messages::ModelVersion)
                    .drop_column(Messages::GenerationTimestamp)
                    .drop_column(Messages::AnonymizationApplied)
                    .drop_column(Messages::EditCount)
                    .drop_column(Messages::Metadata)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Messages {
    Table,
    ContentSource,
    ModelName,
    ModelVersion,
    GenerationTimestamp,
    AnonymizationApplied,
    EditCount,
    Metadata,
}
