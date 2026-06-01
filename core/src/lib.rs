uniffi::setup_scaffolding!();

mod category;
mod shared;
mod product;
mod quantity;

use crate::shared::InitError;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use uniffi::deps::anyhow;
use uuid::Uuid;

// Map UUIDs to Strings in uniffi
uniffi::custom_type!(Uuid, String, {
    remote,
    try_lift: |val| Uuid::parse_str(&val).map_err(|e| anyhow::anyhow!(e)),
    lower: |obj| obj.to_string(),
});

#[derive(uniffi::Object)]
pub struct ComprasCore {
    pub(crate) db: Arc<Mutex<Connection>>,
}

#[uniffi::export]
impl ComprasCore {
    #[uniffi::constructor]
    pub fn init(db_path: String) -> Result<Arc<Self>, InitError> {
        let mut conn = Connection::open(&db_path).map_err(|e| InitError::DatabaseOpenFailed {
            message: e.to_string(),
        })?;

        // SQLite has foreign keys disabled by default, and the setting is on a connection level.
        conn.execute("PRAGMA foreign_keys = ON;", []).map_err(|e| {
            InitError::DatabaseOpenFailed {
                message: e.to_string(),
            }
        })?;

        Self::run_migrations(&mut conn).map_err(|e| InitError::MigrationFailed {
            message: e.to_string(),
        })?;

        Ok(Arc::new(Self {
            db: Arc::new(Mutex::new(conn)),
        }))
    }
}

impl ComprasCore {
    fn run_migrations(conn: &mut Connection) -> rusqlite::Result<()> {
        let current_migration_version: i32 =
            conn.query_row("PRAGMA user_version", [], |row| row.get(0))?;

        let migrations = [include_str!("../migrations/01_initial.sql")];

        for (index, sql) in migrations.iter().enumerate() {
            let migration_version = (index + 1) as i32;

            if current_migration_version < migration_version {
                let tx = conn.transaction()?;
                tx.execute_batch(sql)?;
                tx.pragma_update(None, "user_version", migration_version)?;
                tx.commit()?;
            }
        }

        Ok(())
    }
}
