use crate::{
    types::{credit_note, invoice},
    PdfApp,
};
use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use sailfish::TemplateOnce;

fn print_time(suffix: &str) -> DateTime<Utc> {
    let now = Utc::now();
    println!("{} {} {suffix} ", now.date(), now.time());
    now
}

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server running!")
}

pub async fn create_invoice_pdf(
    data: web::Json<invoice::parameters::Parameters>,
    pdf_application: web::Data<PdfApp>,
) -> impl Responder {
    print_time(&format!(
        "Received request for invoice {}",
        data.invoice_number
    ));
    print_time("Parsed JSON");

    let save_location = format!("/pdf_files/invoice-{}.pdf", data.invoice_number);

    print_time("Rendering HTML template with data...");
    let html = match data.into_inner().render_once() {
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

pub async fn create_credit_note_pdf(
    data: web::Json<credit_note::parameters::Parameters>,
    pdf_application: web::Data<PdfApp>,
) -> impl Responder {
    print_time(&format!(
        "Received request for credit note {}",
        data.credit_note_number
    ));

    let save_location = format!("/pdf_files/credit-note-{}.pdf", data.credit_note_number);

    print_time("Rendering HTML template with data...");
    let html = match data.into_inner().render_once() {
        Ok(html) => html,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to render provided data into template: {e}"))
        }
    };

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
