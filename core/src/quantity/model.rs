#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq)]
pub enum Quantity {
    Stocked,
    Need,
}

impl Quantity {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Need" => Quantity::Need,
            _ => Quantity::Stocked,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Quantity::Stocked => "Stocked",
            Quantity::Need => "Need",
        }
    }
}