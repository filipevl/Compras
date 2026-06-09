use uuid::Uuid;

#[derive(uniffi::Record, Debug, Clone)]
pub struct Category {
    pub id: Uuid,
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

#[derive(uniffi::Record, Debug, Clone)]
pub struct CategorySummary {
    pub id: Uuid,
    pub name: String,
    pub color_hex: String,
    pub product_count: u32,
}

impl CategorySummary {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let id = row.get("id")?;

        Ok(Self {
            id,
            name: row.get("name")?,
            color_hex: row.get("color_hex")?,
            product_count: row.get("product_count")?,
        })
    }
}

#[derive(uniffi::Enum)]
pub enum CategorySort {
    NameAsc,
    NameDesc,
    ProductCountAsc,
    ProductCountDesc,
}
