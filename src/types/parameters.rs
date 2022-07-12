use super::{pdf_invoice_line::PdfInvoiceLine, tax_data::TaxData};
use sailfish::TemplateOnce;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, TemplateOnce)]
#[template(path = "invoice-template.stpl")]
pub struct Parameters {
    pub address: String,
    pub create_date: String,
    pub currency_code: String,
    pub due_date: String,
    pub invoice_number: String,
    pub reference_number: Option<String>,
    pub subtotal: String,
    pub lines: Vec<PdfInvoiceLine>,
    pub taxes: HashMap<String, TaxData>,
    pub total: String,
    pub paid_amount: String,
    pub due: String,
    pub notes: String,
    pub terms: String,
}
