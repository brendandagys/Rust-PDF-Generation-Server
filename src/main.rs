use actix_web::{guard, web, App, HttpServer};
use wkhtmltopdf::PdfApplication;

mod handlers;
mod types;

pub struct PdfApp {
    instance: PdfApplication,
}

#[actix_web::main]
async fn main() -> actix_web::Result<()> {
    let server = HttpServer::new(|| {
        let pdf_application = PdfApplication::new().expect("Failed to initialize `PDFApplication`");

        App::new()
            .app_data(web::Data::new(PdfApp {
                instance: pdf_application,
            }))
            .service(web::resource("/").guard(guard::Get()).to(handlers::health))
            .service(
                web::resource("/invoice")
                    .guard(guard::Post())
                    .to(handlers::create_invoice_pdf),
            )
    })
    .workers(1);

    let host = "0.0.0.0";
    let port = "8001";

    println!("Server running at http://{}:{}", host, port);

    server.bind(format!("{}:{}", host, port))?.run().await?;

    Ok(())
}
