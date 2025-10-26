pub use sea_orm_migration::prelude::*;

mod m20250101_000001_create_settings;
mod m20250101_000002_create_cases;
mod m20250101_000003_create_audit_log;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_settings::Migration),
            Box::new(m20250101_000002_create_cases::Migration),
            Box::new(m20250101_000003_create_audit_log::Migration),
        ]
    }
}
