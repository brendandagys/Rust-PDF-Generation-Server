use sailfish::TemplateOnce;
use std::collections::HashMap;
use wkhtmltopdf::PdfApplication;

enum ECurrencyCode {
    CAD,
    USD,
}

impl ECurrencyCode {
    fn as_str(&self) -> &'static str {
        match self {
            ECurrencyCode::CAD => "CAD",
            ECurrencyCode::USD => "USD",
        }
    }
}

struct CurrencyValue {
    _value: f64,
}

impl CurrencyValue {
    pub fn new(value: f64) -> Self {
        Self { _value: value }
    }

    pub fn value(&self) -> &f64 {
        &self._value
    }

    pub fn formatted(&self) -> String {
        let mut prefix = String::from("$");
        prefix.push_str(&self._value.to_string());
        return prefix;
    }
}

struct PdfInvoiceLine {
    name: String,
    description: String,
    quantity: u32,
    unit_cost: CurrencyValue,
    total: CurrencyValue,
    taxes: Vec<String>,
}

struct TaxData {
    rate: f32,
    percent: f32,
    total: CurrencyValue,
}

#[derive(TemplateOnce)]
#[template(path = "invoice-template.stpl")]
struct Parameters<'a> {
    address: String,
    create_date: String,
    currency_code: &'a str,
    due_date: String,
    invoice_number: String,
    reference_number: Option<String>,
    subtotal: CurrencyValue,
    lines: Vec<PdfInvoiceLine>,
    taxes: HashMap<&'a str, TaxData>,
    total: CurrencyValue,
    paid_amount: CurrencyValue,
    due: CurrencyValue,
    notes: String,
    terms: String,
}

fn main() {
    println!("Beginning HTML generation...");

    let save_location = "/pdf_files/invoice-42069.pdf";

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

    let ctx = parameters;

    // println!("{}", ctx.render_once().unwrap());
    let html = ctx.render_once().unwrap();

    println!("Creating PDF document from HTML content...");

    let pdf_app = PdfApplication::new().expect("Failed to initialize PDF application!");

    let mut pdfout = pdf_app
        .builder()
        .build_from_html(&html)
        .expect("Failed to build PDF document!");

    println!("PDF document generated. Saving file...");

    pdfout
        .save(save_location)
        .expect("Failed to save PDF file!");

    println!("PDF file saved!")
}
