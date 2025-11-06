pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_settings;
mod m20250101_000002_create_cases;
mod m20250101_000003_create_audit_log;
mod m20250106_000004_create_models;
mod m20250106_000005_create_pii_operations;
mod m20250106_000006_create_ner_models;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_settings::Migration),
            Box::new(m20250101_000002_create_cases::Migration),
            Box::new(m20250101_000003_create_audit_log::Migration),
            Box::new(m20250106_000004_create_models::Migration),
            Box::new(m20250106_000005_create_pii_operations::Migration),
            Box::new(m20250106_000006_create_ner_models::Migration),
        ]
    }
}
