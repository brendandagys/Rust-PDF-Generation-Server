use super::currency_value::CurrencyValue;

pub struct TaxData {
    pub rate: f32,
    pub percent: f32,
    pub total: CurrencyValue,
}
