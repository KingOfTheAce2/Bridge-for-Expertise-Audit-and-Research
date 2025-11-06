// Entity module exports
pub mod settings;
pub mod cases;
pub mod conversations;
pub mod messages;
pub mod audit_logs;
pub mod models;

// Re-export for convenience
pub use settings::Entity as Settings;
pub use cases::Entity as Cases;
pub use conversations::Entity as Conversations;
pub use messages::Entity as Messages;
pub use audit_logs::Entity as AuditLogs;
pub use models::Entity as Models;
