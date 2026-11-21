uniffi::setup_scaffolding!();

mod error;
mod categories;

use crate::error::CoreError;
use rusqlite::Connection;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(uniffi::Object)]
pub struct ComprasCore {
    connection: Mutex<Connection>,
}

#[uniffi::export]
impl ComprasCore {
    #[uniffi::constructor]
    pub fn init(db_path: String) -> Result<Arc<Self>, CoreError> {
        let mut conn = Connection::open(&db_path)?;

        // SQLite has foreign keys disabled by default, and the setting is on a connection level.
        conn.execute("PRAGMA foreign_keys = ON;", [])?;

        Self::run_migrations(&mut conn)?;

        Ok(Arc::new(Self {
            connection: Mutex::new(conn),
        }))
    }
}

impl ComprasCore {

    /// The database connection.
    /// If code panicked while holding it, the mutex is "poisoned" but the connection itself is fine,
    /// so carry on with it.
    fn db(&self) -> MutexGuard<'_, Connection> {
        self.connection
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

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

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("The system clock is set before 1970")
        .as_millis() as i64
}