use crate::{Error, Result};
use axum::http::{header, HeaderMap};
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/", get(pdf_handler))
}

async fn pdf_handler() -> Result<(HeaderMap, Vec<u8>)> {
    let mut pdf_bytes: Vec<u8> = Vec::new();

    let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None).unwrap();
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("test ttitle");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc.push(genpdf::elements::Text::new("First PDF Text"));

    doc.render(&mut pdf_bytes).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/pdf; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "attachment; filename=\"Cargo.pdf\"".parse().unwrap(),
    );

    Ok((headers, pdf_bytes))
}
