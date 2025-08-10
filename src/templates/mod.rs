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

/// Custom error type for the application
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to compile template: {0}")]
    CompilationError(String),
    #[error("PDF generation error: {0}")]
    PdfGenerationError(String),
    #[error("Internal server error")]
    InternalServerError,
}

/// Converts a Typst template string to a PDF byte buffer.
#[instrument]
pub fn template_to_pdf(content: String) -> Result<Vec<u8>, AppError> {
    tracing::debug!(
        "Template content to compile:
{}",
        content
    );

    let world = TypstWrapperWorld::with_source(content);

    let Warned {
        output,
        warnings: _warnings,
    } = typst::compile::<PagedDocument>(&world);

    let document = output.map_err(|errors| {
        let error_msg = errors
            .iter()
            .map(|e| format!("{:?}", e))
            .collect::<Vec<_>>()
            .join("; ");
        tracing::error!("Typst compilation errors: {}", error_msg);
        AppError::CompilationError(error_msg)
    })?;

    let pdf_buf = typst_pdf::pdf(&document, &PdfOptions::default()).map_err(|e| {
        let error_msg = format!("{:?}", e);
        tracing::error!("PDF generation error: {}", error_msg);
        AppError::PdfGenerationError(error_msg)
    })?;

    Ok(pdf_buf)
}

//FIXME: Add From AppError for ErrorResponse instead
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::CompilationError(error_details) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template compilation failed: {}", error_details),
            ),
            AppError::PdfGenerationError(error_details) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("PDF generation failed: {}", error_details),
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
