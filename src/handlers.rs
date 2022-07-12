use crate::{types::parameters::Parameters, PdfApp};
use actix_web::{web, HttpResponse, Responder};
use sailfish::TemplateOnce;

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Server running!")
}

pub async fn create_invoice_pdf(
    data: web::Json<Parameters>,
    pdf_application: web::Data<PdfApp>,
) -> impl Responder {
    println!("Parsed JSON.");

    let save_location = format!("/pdf_files/invoice-{}.pdf", data.invoice_number);

    println!("Rendering HTML template with data...");
    let html = data
        .into_inner()
        .render_once()
        .expect("Failed to render provided data!");
    println!("Rendered HTML template.");

    println!("Building PDF document...");
    let mut pdfout = pdf_application
        .instance
        .builder()
        .build_from_html(html)
        .expect("Failed to build PDF document!");
    println!("PDF document generated.");

    println!("Saving document...");
    match pdfout.save(save_location) {
        Ok(_file) => {
            println!("PDF file saved!");
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body("Invoice PDF created!")
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
