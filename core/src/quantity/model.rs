#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq)]
pub enum Quantity {
    Stocked,
    Need,
}