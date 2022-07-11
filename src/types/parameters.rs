use super::{currency_value::CurrencyValue, pdf_invoice_line::PdfInvoiceLine, tax_data::TaxData};
use sailfish::TemplateOnce;
use std::collections::HashMap;

#[derive(TemplateOnce)]
#[template(path = "invoice-template.stpl")]
pub struct Parameters<'a> {
    pub address: String,
    pub create_date: String,
    pub currency_code: &'a str,
    pub due_date: String,
    pub invoice_number: String,
    pub reference_number: Option<String>,
    pub subtotal: CurrencyValue,
    pub lines: Vec<PdfInvoiceLine>,
    pub taxes: HashMap<&'a str, TaxData>,
    pub total: CurrencyValue,
    pub paid_amount: CurrencyValue,
    pub due: CurrencyValue,
    pub notes: String,
    pub terms: String,
}
