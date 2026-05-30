// src/models/category.rs

use crate::ComprasCore; // Import the core
use crate::errors::CoreError;
use rusqlite::params;
use uuid::Uuid;

// 1. The Model (What you already have)
#[derive(uniffi::Record, Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub color_hex: String,
}

// 2. The Operations (Moved from lib.rs)
#[uniffi::export]
impl ComprasCore {

    pub fn add_category(&self, name: String, color_hex: String) -> Result<Category, CoreError> {
        let id = Uuid::new_v4();
        let conn = self.db.lock().unwrap();

        conn.execute(
            "INSERT INTO categories (id, name, color_hex) VALUES (?1, ?2, ?3)",
            params![id.to_string(), name, color_hex],
        ).map_err(|e| CoreError::Database { message: e.to_string() })?;

        Ok(Category { id, name, color_hex })
    }

    pub fn get_categories(&self) -> Result<Vec<Category>, CoreError> {
        let conn = self.db.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, name, color_hex FROM categories ORDER BY name ASC")
            .map_err(|e| CoreError::Database { message: e.to_string() })?;

        let category_iter = stmt.query_map([], |row| {
            let id_str: String = row.get(0)?;
            Ok(Category {
                id: Uuid::parse_str(&id_str).unwrap_or_default(),
                name: row.get(1)?,
                color_hex: row.get(2)?,
            })
        }).map_err(|e| CoreError::Database { message: e.to_string() })?;

        let mut categories = Vec::new();
        for category in category_iter {
            if let Ok(c) = category { categories.push(c); }
        }

        Ok(categories)
    }
}