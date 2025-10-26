use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuditLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuditLog::Action).string().not_null())
                    .col(ColumnDef::new(AuditLog::CaseId).integer())
                    .col(ColumnDef::new(AuditLog::EntityType).string())
                    .col(ColumnDef::new(AuditLog::EntityId).integer())
                    .col(ColumnDef::new(AuditLog::Details).json())
                    .col(
                        ColumnDef::new(AuditLog::Timestamp)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLog::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum AuditLog {
    Table,
    Id,
    Action,
    CaseId,
    EntityType,
    EntityId,
    Details,
    Timestamp,
}
