use super::{comment::Comment, credit::Credit};
use sailfish::TemplateOnce;
use serde::Deserialize;

#[derive(Deserialize, TemplateOnce)]
#[template(path = "credit-note-template.stpl")]
pub struct Parameters {
    pub address: String,
    pub credit_note_date: String,
    pub credit_note_number: String,
    pub credits: Vec<Credit>,
    pub currency_code: String,
    pub notes: String,
    pub subtotal: String,
    pub tax_amount: String,
    pub tax_name: Option<String>,
    pub tax_rate: Option<String>,
    pub total: String,
    pub comments: Vec<Comment>,
}
