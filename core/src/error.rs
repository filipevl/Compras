#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum CoreError {
    #[error("Database error: {error_message}")]
    Database { error_message: String },
    #[error("Not found")]
    NotFound,
}

impl From<rusqlite::Error> for CoreError {
    fn from(error: rusqlite::Error) -> Self {
        CoreError::Database {
            error_message: error.to_string(),
        }
    }
}