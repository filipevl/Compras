#[derive(uniffi::Enum, Debug, Clone, Copy, PartialEq)]
pub enum ProductState {
    Archived,
    Listed,
    Unlisted,
}

impl ProductState {
    pub fn from_str(s: &str) -> Self {
        match s {
            "ARCHIVED" => ProductState::Archived,
            "LISTED" => ProductState::Listed,
            _ => ProductState::Unlisted,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ProductState::Archived => "ARCHIVED",
            ProductState::Listed => "LISTED",
            ProductState::Unlisted => "UNLISTED",
        }
    }
}
