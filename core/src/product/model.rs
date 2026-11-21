use crate::category::Category;
use crate::product_state::ProductState;
use std::time::SystemTime;
use uuid::Uuid;

#[derive(uniffi::Record, Debug, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub state: ProductState,
    pub one_time_purchase: bool,
    pub category: Option<Category>,
    pub last_purchased_timestamp: Option<SystemTime>,
}

impl Product {
    pub fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error> {
        Ok(Product {
            id: row.get("id")?,
            name: row.get("name")?,
            state: ProductState::from_str(&row.get::<_, String>("product_state")?),
            one_time_purchase: row.get("one_time_purchase")?,
            last_purchased_timestamp: row
                .get::<_, Option<String>>("last_purchased_at")?
                .and_then(|s| s.parse::<chrono::DateTime<chrono::Utc>>().ok())
                .map(Into::into),
            category: row.get::<_, Option<Uuid>>("cat_id")?.map(|id| Category {
                id,
                name: row.get("cat_name").unwrap_or_default(),
                color_hex: row.get("cat_color_hex").unwrap_or_default(),
            }),
        })
    }
}
