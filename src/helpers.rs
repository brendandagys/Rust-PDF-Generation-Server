use std::error::Error;

use chrono::{DateTime, Utc};
use sailfish::{RenderError, TemplateOnce};
use wkhtmltopdf::PdfOutput;

use crate::PdfApp;

pub fn print_time(suffix: &str) -> DateTime<Utc> {
    let now = Utc::now();
    println!("{} {} {suffix} ", now.date(), now.time());
    now
}

pub fn populate_html<T: TemplateOnce>(data: T) -> Result<String, RenderError> {
    match data.render_once() {
        Ok(html) => Ok(html),
        Err(e) => Err(e),
    }
}

pub fn build_pdf<'a>(
    pdf_application: &'a PdfApp,
    html: &'a str,
) -> Result<PdfOutput<'a>, Box<dyn Error>> {
    match pdf_application.instance.builder().build_from_html(html) {
        Ok(pdfout) => Ok(pdfout),
        Err(e) => Err(Box::new(e)),
    }
}
