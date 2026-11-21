#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum InitError {
    #[error("Failed to open database: {error_message}")]
    DatabaseOpenFailed { error_message: String },
    #[error("Database migration failed: {error_message}")]
    MigrationFailed { error_message: String },
}