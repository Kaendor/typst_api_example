use axum::{
    extract::FromRequest,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use tracing::instrument;
use typst::{diag::Warned, layout::PagedDocument};
use typst_pdf::PdfOptions;

use crate::TypstWrapperWorld;

pub mod german_invoice;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to compile template")]
    CompilationError,
    #[error("PDF generation error")]
    PdfGenerationError,
    #[error("Internal server error")]
    InternalServerError,
}

#[instrument]
pub fn template_to_pdf(content: String) -> Result<Vec<u8>, AppError> {
    // This function is a placeholder for the actual implementation.
    // It should return the path or content of the PDF generated from the template.
    let world = TypstWrapperWorld::new("./examples".to_owned(), content);

    let Warned {
        output,
        warnings: _warnings,
    } = typst::compile::<PagedDocument>(&world);

    let document = output.map_err(|_| AppError::CompilationError)?;

    let pdf_buf = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|_| AppError::PdfGenerationError)?;

    Ok(pdf_buf)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::CompilationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Oopsie doopsie".to_owned(),
            ),
            AppError::PdfGenerationError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_owned(),
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occurred".to_owned(),
            ),
        };

        tracing::error!("PDF generation error: {}", message);

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pdf_generation_test() {
        let pdf_buf = super::template_to_pdf("Hello, Typst!".to_string()).expect("pdf gen");

        assert!(!pdf_buf.is_empty(), "PDF buffer should not be empty");
        assert!(
            pdf_buf.starts_with(b"%PDF-"),
            "PDF buffer should start with %PDF-"
        );
        assert!(
            pdf_buf.ends_with(b"%%EOF"),
            "PDF buffer should end with %%EOF"
        );
    }
}
