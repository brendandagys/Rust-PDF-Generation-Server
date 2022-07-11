use super::currency_value::CurrencyValue;

pub struct PdfInvoiceLine {
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub unit_cost: CurrencyValue,
    pub total: CurrencyValue,
    pub taxes: Vec<String>,
}
