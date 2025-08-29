use axum::{
    Json,
    extract::State,
    http::{HeaderMap, header},
    response::{IntoResponse, Result},
};
use tracing::{info, instrument};
use typst_pdf_api::{
    TypstWrapperWorld,
    templates::{AppError, german_invoice::GERMAN_INVOICE_TEMPLATE},
};

// #[axum::debug_handler]
#[instrument]
pub async fn pdf_generation_controller(
    State(world): State<TypstWrapperWorld>,
    Json(_payload): Json<CreatePDF>,
) -> Result<impl IntoResponse> {
    info!("Serving PDF");
    // This is where you would implement the logic to convert a template to PDF.
    // For now, we return a simple string.
    let german_template = GERMAN_INVOICE_TEMPLATE;

    let pdf_buf = typst_pdf_api::templates::template_to_pdf(german_template.to_string())?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/pdf"
            .parse()
            .map_err(|_| AppError::InternalServerError)?,
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        "inline; filename=\"output.pdf\""
            .parse()
            .map_err(|_| AppError::InternalServerError)?,
    );

    info!("PDF Served");
    Ok((headers, pdf_buf))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreatePDF {
    pub template_id: String,
    pub content: String,
    // pub data: Value,
}
