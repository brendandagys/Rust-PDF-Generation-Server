use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credit {
    pub credit_number: String,
    pub description: Option<String>,
    pub subscription_id: Option<String>,
    pub subtotal: String,
    pub tax_amount: String,
    pub total: String,
}
