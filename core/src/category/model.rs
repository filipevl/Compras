use uuid::Uuid;

#[derive(uniffi::Record, Debug, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub color_hex: String,
}