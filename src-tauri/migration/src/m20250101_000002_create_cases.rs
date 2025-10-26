use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cases::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cases::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cases::Name).string().not_null())
                    .col(ColumnDef::new(Cases::ClientName).string().not_null())
                    .col(ColumnDef::new(Cases::CaseNumber).string().unique_key())
                    .col(ColumnDef::new(Cases::Description).text())
                    .col(
                        ColumnDef::new(Cases::Status)
                            .string()
                            .not_null()
                            .default("active"),
                    )
                    .col(ColumnDef::new(Cases::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Cases::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // Create conversations table linked to cases
        manager
            .create_table(
                Table::create()
                    .table(Conversations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Conversations::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Conversations::CaseId).integer().not_null())
                    .col(ColumnDef::new(Conversations::Title).string().not_null())
                    .col(ColumnDef::new(Conversations::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_conversation_case")
                            .from(Conversations::Table, Conversations::CaseId)
                            .to(Cases::Table, Cases::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create messages table
        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Messages::ConversationId).integer().not_null())
                    .col(ColumnDef::new(Messages::Role).string().not_null()) // "user" | "assistant"
                    .col(ColumnDef::new(Messages::Content).text().not_null())
                    .col(
                        ColumnDef::new(Messages::IsAiGenerated)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Messages::WasEdited)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Messages::CreatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_message_conversation")
                            .from(Messages::Table, Messages::ConversationId)
                            .to(Conversations::Table, Conversations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Messages::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Conversations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Cases::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Cases {
    Table,
    Id,
    Name,
    ClientName,
    CaseNumber,
    Description,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Conversations {
    Table,
    Id,
    CaseId,
    Title,
    CreatedAt,
}

#[derive(Iden)]
enum Messages {
    Table,
    Id,
    ConversationId,
    Role,
    Content,
    IsAiGenerated,
    WasEdited,
    CreatedAt,
}
