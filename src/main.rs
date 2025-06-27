use axum::{
    Json, Router,
    body::Body,
    http::{HeaderMap, header},
    response::IntoResponse,
    routing::get,
};
use typst::{
    Library, World,
    diag::{FileResult, Warned},
    foundations::{Bytes, Datetime},
    layout::PagedDocument,
    pdf,
    syntax::{FileId, Source},
    text::{Font, FontBook},
    utils::LazyHash,
};
use typst_pdf::PdfOptions;
use typst_pdf_api::TypstWrapperWorld;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(template_to_pdf));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

// #[axum::debug_handler]
async fn template_to_pdf(Json(payload): Json<CreatePDF>) -> impl IntoResponse {
    // This is where you would implement the logic to convert a template to PDF.
    // For now, we return a simple string.
    let content = "=Hello, World!";

    let world = TypstWrapperWorld::new("./examples".to_owned(), content.to_owned());

    let Warned {
        output,
        warnings: _warnings,
    } = typst::compile::<PagedDocument>(&world);

    let document = output.expect("Failed to compile template");

    let pdf_buf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("PDF Generation");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/pdf".parse().expect("pdf header parse"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "inline; filename=\"output.pdf\""
            .parse()
            .expect("filename header parse"),
    );

    (headers, pdf_buf)
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreatePDF {
    pub template_id: String,
    pub content: String,
    // pub data: Value,
}
