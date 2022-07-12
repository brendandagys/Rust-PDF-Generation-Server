use serde::Deserialize;

#[derive(Deserialize)]
pub struct TaxData {
    pub rate: f32,
    pub percent: f32,
    pub total: String,
}
