#[derive(uniffi::Error, Debug, thiserror::Error)]
pub enum InitError {
    #[error("Failed to open database: {message}")]
    DatabaseOpenFailed { message: String },
    #[error("Database migration failed: {message}")]
    MigrationFailed { message: String },
}