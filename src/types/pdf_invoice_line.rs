use serde::Deserialize;

#[derive(Deserialize)]
pub struct PdfInvoiceLine {
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub unit_cost: String,
    pub total: String,
    pub taxes: Vec<String>,
}
