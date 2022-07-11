use crate::types::{
    currency_code::ECurrencyCode, currency_value::CurrencyValue, parameters::Parameters,
    pdf_invoice_line::PdfInvoiceLine, tax_data::TaxData,
};
use std::collections::HashMap;

pub fn get_test_data() -> Parameters<'static> {
    let pdf_invoice_line_1 = PdfInvoiceLine {
        name: String::from("Advertising"),
        description: String::from("Advertising on Kijiji."),
        quantity: 35,
        unit_cost: CurrencyValue::new(20.00),
        total: CurrencyValue::new(700.00),
        taxes: vec![String::from("HST")],
    };

    let pdf_invoice_line_2 = PdfInvoiceLine {
        name: String::from("Promoted Listings"),
        description: String::from("Promoted Listing Advertisements on Zumper."),
        quantity: 2,
        unit_cost: CurrencyValue::new(100.00),
        total: CurrencyValue::new(200.00),
        taxes: vec![String::from("GST"), String::from("HST")],
    };

    let pdf_invoice_line_3 = PdfInvoiceLine {
        name: String::from("Call Tracking"),
        description: String::from("Call Tracking lines on multiple buildings."),
        quantity: 45,
        unit_cost: CurrencyValue::new(2.00),
        total: CurrencyValue::new(90.00),
        taxes: vec![String::from("GST")],
    };

    let tax_data_1 = TaxData {
        rate: 0.05,
        percent: 0.05,
        total: CurrencyValue::new(45.00),
    };

    let tax_data_2 = TaxData {
        rate: 0.08,
        percent: 0.08,
        total: CurrencyValue::new(23.20),
    };

    let mut taxes = HashMap::new();
    taxes.insert("HST", tax_data_1);
    taxes.insert("GST", tax_data_2);

    let address = "Florida Man".to_owned()
        + "<br>35 Coast Ridge Road"
        + "<br>Palm Coast, Florida"
        + "<br>42069"
        + "<br>United States";

    let parameters = Parameters {
        address,
        create_date: String::from("7/9/2022"),
        currency_code: ECurrencyCode::CAD.as_str(),
        due_date: String::from("7/31/2022"),
        invoice_number: String::from("14355"),
        reference_number: Some(String::from("28")),
        // subtotal: CurrencyValue { _value: 420.69 },
        subtotal: CurrencyValue::new(420.69),
        lines: vec![pdf_invoice_line_1, pdf_invoice_line_2, pdf_invoice_line_3],
        taxes,
        total: CurrencyValue::new(488.89),
        paid_amount: CurrencyValue::new(231.35),
        due: CurrencyValue::new(257.54),
        notes: String::from("Our notes."),
        terms: String::from("Our terms."),
    };

    parameters
}
