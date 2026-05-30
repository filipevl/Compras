use std::time::SystemTime;
use uuid::Uuid;
use crate::category::Category;
use crate::quantity::Quantity;

#[derive(uniffi::Record, Debug, Clone)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub quantity: Quantity,
    pub category: Option<Category>,
    pub last_purchased_timestamp: Option<SystemTime>,
}