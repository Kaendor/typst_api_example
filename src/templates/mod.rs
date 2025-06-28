use tracing::instrument;
use typst::{diag::Warned, layout::PagedDocument};
use typst_pdf::PdfOptions;

use crate::TypstWrapperWorld;

pub mod german_invoice;

#[instrument]
pub fn template_to_pdf(content: String) -> Vec<u8> {
    // This function is a placeholder for the actual implementation.
    // It should return the path or content of the PDF generated from the template.
    let world = TypstWrapperWorld::new("./examples".to_owned(), content);

    let Warned {
        output,
        warnings: _warnings,
    } = typst::compile::<PagedDocument>(&world);

    let document = output.expect("Failed to compile template");

    let pdf_buf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("PDF Generation");

    pdf_buf
}

#[cfg(test)]
mod tests {
    #[test]
    fn pdf_generation_test() {
        let pdf_buf = super::template_to_pdf("Hello, Typst!".to_string());

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
