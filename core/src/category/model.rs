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