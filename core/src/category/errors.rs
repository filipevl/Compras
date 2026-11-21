use uniffi::Error;

#[derive(Debug, thiserror::Error, Error)]
pub enum CategoryError {
    #[error("Database operation failed: {error_message}")]
    DatabaseError {
        error_message: String
    },
}