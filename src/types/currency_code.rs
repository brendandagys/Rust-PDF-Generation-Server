pub enum ECurrencyCode {
    CAD,
    USD,
}

impl ECurrencyCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ECurrencyCode::CAD => "CAD",
            ECurrencyCode::USD => "USD",
        }
    }
}
