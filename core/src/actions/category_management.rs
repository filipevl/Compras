use crate::category::errors::CategoryError;
use crate::category::model::{Category, CategorySort, CategorySummary};
use crate::ComprasCore;
use uuid::Uuid;

#[uniffi::export]
impl ComprasCore {
    pub fn get_categories(&self) -> Result<Vec<Category>, CategoryError> {
        let conn = self.db.lock().map_err(|e| CategoryError::DatabaseError {
            message: format!("Failed to get database lock: {}", e),
        })?;

        let mut statement = conn
            .prepare("SELECT id, name, color_hex FROM categories ORDER BY name ASC")
            .map_err(|e| CategoryError::DatabaseError {
                message: e.to_string(),
            })?;

        let categories = statement
            .query_map([], Category::from_row)
            .map_err(|e| CategoryError::DatabaseError {
                message: e.to_string(),
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| CategoryError::DatabaseError {
                message: e.to_string(),
            })?;

        Ok(categories)
    }

    pub fn get_categories_summary(
        &self,
        sort: Option<CategorySort>,
        filter: Option<String>,
    ) -> Result<Vec<CategorySummary>, CategoryError> {
        let conn = self.db.lock().map_err(|e| CategoryError::DatabaseError {
            message: format!("Failed to get database lock: {}", e),
        })?;

        let sort_clause = match sort.unwrap_or(CategorySort::NameAsc) {
            CategorySort::NameAsc => "ORDER BY c.name ASC",
            CategorySort::NameDesc => "ORDER BY c.name DESC",
            CategorySort::ProductCountAsc => "ORDER BY product_count ASC, c.name ASC",
            CategorySort::ProductCountDesc => "ORDER BY product_count DESC, c.name ASC",
        };

        let filter_clause = if filter.is_some() {
            "WHERE c.name LIKE '%' || ?1 || '%'"
        } else {
            ""
        };

        let query = format!(
            "
            SELECT c.id, c.name, c.color_hex, COUNT(p.id) as product_count
            FROM categories c
            LEFT JOIN products p ON c.id = p.category_id
            {}
            GROUP BY c.id
            {}
            ",
            filter_clause, sort_clause
        );

        let mut statement = conn
            .prepare(&query)
            .map_err(|e| CategoryError::DatabaseError {
                message: format!("Failed to prepare query: {}", e),
            })?;

        let params: &[&dyn rusqlite::ToSql] = match &filter {
            Some(text) => &[text],
            None => &[],
        };

        let categories: Vec<CategorySummary> = statement
            .query_map(params, CategorySummary::from_row)
            .map_err(|e| CategoryError::DatabaseError {
                message: format!("Failed to execute query: {}", e),
            })?
            .collect::<Result<Vec<CategorySummary>, _>>()
            .map_err(|e| CategoryError::DatabaseError {
                message: format!("Failed to collect results: {}", e),
            })?;

        Ok(categories)
    }

    pub fn update_category(
        &self,
        id: Uuid,
        name: String,
        color_hex: String,
    ) -> Result<Category, CategoryError> {
        let conn = self.db.lock().map_err(|e| CategoryError::DatabaseError {
            message: format!("Failed to get database lock: {}", e),
        })?;

        conn.execute(
            "UPDATE categories SET name = ?1, color_hex = ?2 WHERE id = ?3",
            rusqlite::params![name, color_hex, id.to_string()],
        )
        .map_err(|e| CategoryError::DatabaseError {
            message: e.to_string(),
        })?;

        Ok(Category {
            id,
            name,
            color_hex,
        })
    }

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
