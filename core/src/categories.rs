use crate::error::CoreError;
use crate::{now, ComprasCore};
use uuid::Uuid;

#[derive(uniffi::Record, Debug, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color_hex: String,
}

impl Category {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            color_hex: row.get("color_hex")?,
        })
    }
}

#[uniffi::export]
impl ComprasCore {
    /// The categories that aren't deleted, in store order.
    pub fn get_categories(&self) -> Result<Vec<Category>, CoreError> {
        let conn = self.db();

        let mut statement = conn.prepare(
            "SELECT id, name, color_hex
            FROM categories
            WHERE deleted_at IS NULL
            ORDER BY position",
        )?;

        let categories = statement
            .query_map([], Category::from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(categories)
    }

    pub fn add_category(&self, name: String, color_hex: String) -> Result<Category, CoreError> {
        let conn = self.db();
        let id = Uuid::new_v4().to_string();

        conn.execute(
            "INSERT INTO categories
            (id, name, color_hex, position, created_at, updated_at)
            VALUES
            (?1, ?2, ?3, (SELECT COALESCE(MAX(position), 0) + 1 FROM categories), ?4, ?4)",
            rusqlite::params![id, name, color_hex, now()],
        )?;

        Ok(Category {
            id,
            name,
            color_hex,
        })
    }

    pub fn update_category(
        &self,
        id: String,
        name: String,
        color_hex: String,
    ) -> Result<Category, CoreError> {
        let conn = self.db();

        let changed = conn.execute(
            "UPDATE categories
            SET name = ?1, color_hex = ?2, updated_at = ?3
            WHERE id = ?4 AND deleted_at IS NULL",
            rusqlite::params![name, color_hex, now(), id],
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

    /// Soft delete: the row stays marked with `deleted_at`.
    pub fn delete_category(&self, id: String) -> Result<(), CoreError> {
        let conn = self.db();

        let changed = conn.execute(
            "UPDATE categories
            SET deleted_at = ?1, updated_at = ?1
            WHERE id = ?2 AND deleted_at IS NULL",
            rusqlite::params![now(), id],
        )?;

        if changed == 0 {
            return Err(CoreError::NotFound);
        }

        Ok(())
    }

    pub fn reorder_categories(&self, ids: Vec<String>) -> Result<(), CoreError> {
        let mut conn = self.db();
        let tx = conn.transaction()?;
        let timestamp = now();

        for (index, id) in ids.iter().enumerate() {
            let changed = tx.execute(
                "UPDATE categories
                SET position = ?1, updated_at = ?2
                WHERE id = ?3 AND deleted_at IS NULL",
                rusqlite::params![index as i64 + 1, timestamp, id],
            )?;

            if changed == 0 {
                return Err(CoreError::NotFound); // `tx` is dropped without commit: rolled back
            }
        }

        tx.commit()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::error::CoreError;
    use crate::ComprasCore;
    use std::sync::Arc;
    use super::Category;

    /// A core on a fresh in-memory database, holding one category: "Bakery".
    fn core_with_bakery() -> (Arc<ComprasCore>, Category) {
        let core = ComprasCore::init(":memory:".to_string()).unwrap();
        let bakery = core.add_category("Bakery".into(), "#C08040".into()).unwrap();

        (core, bakery)
    }

    fn names(core: &ComprasCore) -> Vec<String> {
        core.get_categories()
            .unwrap()
            .into_iter()
            .map(|category| category.name)
            .collect()
    }

    #[test]
    fn add_returns_the_new_category() {
        let (core, bakery) = core_with_bakery();
        let categories = core.get_categories().unwrap();

        assert_eq!(bakery.id.len(), 36);
        assert_eq!(bakery.name, "Bakery");
        assert_eq!(bakery.color_hex, "#C08040");
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, bakery.id);
    }

    #[test]
    fn lists_in_the_order_they_were_added() {
        let (core, _) = core_with_bakery();
        core.add_category("Frozen".into(), "#C08041".into()).unwrap();
        core.add_category("Drinks".into(), "#C08042".into()).unwrap();

        let names = names(&core);

        assert_eq!(names, ["Bakery", "Frozen", "Drinks"]);
    }

    #[test]
    fn update_changes_the_category() {
        let (core, bakery) = core_with_bakery();

        core.update_category(bakery.id.clone(), "Padaria".into(), "#A12345".into()).unwrap();
        let categories = core.get_categories().unwrap();

        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].id, bakery.id);
        assert_eq!(categories[0].name, "Padaria");
        assert_eq!(categories[0].color_hex, "#A12345");
    }

    #[test]
    fn delete_hides_the_category_but_keeps_the_row() {
        let (core, bakery) = core_with_bakery();

        core.delete_category(bakery.id).unwrap();
        let rows: i64 = core
            .db()
            .query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))
            .unwrap();

        assert!(core.get_categories().unwrap().is_empty());
        assert_eq!(rows, 1, "a soft delete keeps the row");
    }

    #[test]
    fn deleted_and_unknown_ids_are_not_found() {
        let (core, bakery) = core_with_bakery();
        core.delete_category(bakery.id.clone()).unwrap();

        for id in [bakery.id, "nope".to_string()] {
            let update = core.update_category(id.clone(), "X".into(), "#012345".into());
            assert!(matches!(update, Err(CoreError::NotFound)));

            let delete = core.delete_category(id);
            assert!(matches!(delete, Err(CoreError::NotFound)));
        }

        let rows: i64 = core
            .db()
            .query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))
            .unwrap();

        assert!(core.get_categories().unwrap().is_empty());
        assert_eq!(rows, 1, "a soft delete keeps the row");
    }

    #[test]
    fn reorder_sets_the_new_store_order() {
        let (core, bakery) = core_with_bakery();
        let frozen = core.add_category("Frozen".into(), "#C08041".into()).unwrap();
        let drinks = core.add_category("Drinks".into(), "#C08042".into()).unwrap();

        core.reorder_categories(vec![drinks.id, bakery.id, frozen.id]).unwrap();

        let names = names(&core);

        assert_eq!(names, ["Drinks", "Bakery", "Frozen"]);
    }

    #[test]
    fn reorder_with_an_unknown_id_changes_nothing() {
        let (core, bakery) = core_with_bakery();
        let frozen = core.add_category("Frozen".into(), "#C08041".into()).unwrap();

        let reorder = core.reorder_categories(vec![frozen.id, "nope".into(), bakery.id]);
        let names = names(&core);

        assert!(matches!(reorder, Err(CoreError::NotFound)));
        assert_eq!(names, ["Bakery", "Frozen"], "rolled back");
    }
}
