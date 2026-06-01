use crate::category::errors::CategoryError;
use crate::ComprasCore;
use uuid::Uuid;

#[uniffi::export]
impl ComprasCore {

    /// Associated products are automatically detached (category set to null) due to foreign key constraints
    pub fn delete_category(&self, id: Uuid) -> Result<(), CategoryError> {
        let conn = self.db.lock().map_err(|e| CategoryError::DatabaseError {
            message: format!("Failed to get database lock: {}", e),
        })?;

        conn.execute(
            "DELETE FROM categories WHERE id = ?1",
            rusqlite::params![id.to_string()],
        )
        .map_err(|e| CategoryError::DatabaseError {
            message: format!("Failed to delete category: {}", e),
        })?;

        Ok(())
    }
}
