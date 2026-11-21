use crate::category::model::{Category, CategorySort, CategorySummary};
use crate::error::CoreError;
use crate::ComprasCore;

#[uniffi::export]
impl ComprasCore {
    pub fn get_categories(&self) -> Result<Vec<Category>, CoreError> {
        let conn = self.db();

        let mut statement =
            conn.prepare("SELECT id, name, color_hex FROM categories ORDER BY name ASC")?;

        let categories = statement
            .query_map([], Category::from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn get_categories_summary(
        &self,
        sort: Option<CategorySort>,
        filter: Option<String>,
    ) -> Result<Vec<CategorySummary>, CoreError> {
        let conn = self.db();

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

        let mut statement = conn.prepare(&query)?;

        let params: &[&dyn rusqlite::ToSql] = match &filter {
            Some(text) => &[text],
            None => &[],
        };

        let categories: Vec<CategorySummary> = statement
            .query_map(params, CategorySummary::from_row)?
            .collect::<Result<Vec<CategorySummary>, _>>()?;

        Ok(categories)
    }

    pub fn update_category(
        &self,
        id: String,
        name: String,
        color_hex: String,
    ) -> Result<Category, CoreError> {
        let conn = self.db();

        let changed = conn.execute(
            "UPDATE categories SET name = ?1, color_hex = ?2 WHERE id = ?3",
            rusqlite::params![name, color_hex, id],
        )?;

        if changed == 0 {
            return Err(CoreError::NotFound);
        }

        Ok(Category {
            id,
            name,
            color_hex,
        })
    }

    /// Associated products are automatically detached (category set to null) due to foreign key constraints
    pub fn delete_category(&self, id: String) -> Result<(), CoreError> {
        let conn = self.db();

        let changed = conn.execute(
            "DELETE FROM categories WHERE id = ?1",
            rusqlite::params![id],
        )?;

        if changed == 0 {
            return Err(CoreError::NotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::CoreError;
    use crate::ComprasCore;
    use std::sync::Arc;

    /// A core on a fresh in-memory database, holding one category: "Bakery".
    fn core_with_bakery() -> Arc<ComprasCore> {
        let core = ComprasCore::init(":memory:".to_string()).unwrap();

        core.db()
            .execute(
                "INSERT INTO categories (id, name, color_hex) VALUES ('c1', 'Bakery', '#C08040')",
                [],
            )
            .unwrap();

        core
    }

    #[test]
    fn reads_text_ids() {
        let categories = core_with_bakery().get_categories().unwrap();

        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, "c1");
        assert_eq!(categories[0].name, "Bakery");
        assert_eq!(categories[0].color_hex, "#C08040");
    }

    #[test]
    fn update_changes_the_category() {
        let core = core_with_bakery();

        core.update_category("c1".into(), "Padaria".into(), "#A12345".into()).unwrap();
        let categories = core.get_categories().unwrap();

        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, "c1");
        assert_eq!(categories[0].name, "Padaria");
        assert_eq!(categories[0].color_hex, "#A12345");
    }

    #[test]
    fn delete_removes_the_category() {
        let core = core_with_bakery();
        core.delete_category("c1".into()).unwrap();

        assert!(core.get_categories().unwrap().is_empty());
    }

    #[test]
    fn unknown_ids_are_not_found() {
        let core = core_with_bakery();

        let update = core.update_category("nope".into(), "X".into(), "#000000".into());
        assert!(matches!(update, Err(CoreError::NotFound)));

        let delete = core.delete_category("nope".into());
        assert!(matches!(delete, Err(CoreError::NotFound)));
    }
}