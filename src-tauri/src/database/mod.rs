use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Database connection manager shared across Tauri commands.
#[derive(Clone)]
pub struct DatabaseManager {
    connection: Arc<Mutex<Option<DatabaseConnection>>>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
        }
    }

    /// Initialize the SQLite database and run migrations.
    pub async fn initialize(&self, db_path: &str) -> Result<(), DbErr> {
        let db_url = format!("sqlite://{}?mode=rwc", db_path);
        let conn = Database::connect(&db_url).await?;

        // Run migrations automatically
        migration::Migrator::up(&conn, None).await?;

        *self.connection.lock().await = Some(conn);
        Ok(())
    }

    /// Retrieve the active database connection.
    pub async fn get_connection(&self) -> Option<DatabaseConnection> {
        self.connection.lock().await.clone()
    }
}

// Make the migration module available
pub mod migration {
    pub use ::migration::*;
}
