use uniffi::Error;

#[derive(Debug, thiserror::Error, Error)]
pub enum CategoryError {
    #[error("Database operation failed: {message}")]
    DatabaseError {
        message: String
    },
}