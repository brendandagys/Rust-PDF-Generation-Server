use crate::{
    types::{credit_note, invoice},
    PdfApp,
};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use sailfish::TemplateOnce;

pub enum Parameters<T> {
    InvoiceParameters(T),
    CreditNoteParameters(T),
}

fn print_time(suffix: &str) -> DateTime<Utc> {
    let now = Utc::now();
    println!("{} {} {suffix} ", now.date(), now.time());
    now
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server running!")
}

pub async fn generate_pdf<T>(
    id: String,
    parameters: Parameters<T>,
    pdf_application: &PdfApp,
) -> impl Responder
where
    T: TemplateOnce,
{
    let document_type_kebab_case;

    let document_type = match parameters {
        Parameters::InvoiceParameters(_) => {
            document_type_kebab_case = "invoice";
            "INVOICE"
        }
        Parameters::CreditNoteParameters(_) => {
            document_type_kebab_case = "credit-note";
            "CREDIT NOTE"
        }
    };

    let data = match parameters {
        Parameters::InvoiceParameters(parameters) => parameters,
        Parameters::CreditNoteParameters(parameters) => parameters,
    };

    print_time(&format!("Received request for {document_type}: {id}"));
    print_time("Parsed JSON");

    let save_location = format!("/pdf_files/{document_type_kebab_case}-{id}.pdf");

    print_time("Rendering HTML template with data...");
    let html = match data.render_once() {
        Ok(html) => html,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to render provided data into template: {e}"))
        }
    };
    print_time("Rendered HTML template");

    print_time("Building PDF document...");
    let mut pdfout = match pdf_application.instance.builder().build_from_html(html) {
        Ok(pdfout) => pdfout,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to build PDF document: {e}"))
        }
    };
    print_time("PDF document generated");

    print_time("Saving document...");
    match pdfout.save(save_location) {
        Ok(_file) => {
            println!("PDF file saved!");

            let now = Utc::now();
            let response = format!("{} {}", now.date(), now.time());

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(response)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to save the generated PDF document: {e}")),
    }
}

pub async fn create_invoice_pdf(
    data: web::Json<invoice::parameters::Parameters>,
    pdf_application: web::Data<PdfApp>,
) -> impl Responder {
    generate_pdf(
        data.invoice_number.clone(),
        Parameters::InvoiceParameters::<invoice::parameters::Parameters>(data.into_inner()),
        pdf_application.get_ref(),
    )
    .await
}

pub async fn create_credit_note_pdf(
    data: web::Json<credit_note::parameters::Parameters>,
    pdf_application: web::Data<PdfApp>,
) -> impl Responder {
    generate_pdf(
        data.credit_note_number.clone(),
        Parameters::CreditNoteParameters::<credit_note::parameters::Parameters>(data.into_inner()),
        pdf_application.get_ref(),
    )
    .await
}
