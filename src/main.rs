use sailfish::TemplateOnce;
use wkhtmltopdf::PdfApplication;

mod test_data;
mod types;

fn main() {
    println!("Beginning HTML generation...");
    let save_location = "/pdf_files/invoice-42069.pdf";

    let ctx = test_data::get_test_data();
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
