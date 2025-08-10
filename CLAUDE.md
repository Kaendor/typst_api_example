# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based HTTP API server that generates PDF documents from Typst templates. The main functionality is converting Typst markup (a modern typesetting language) into PDF format through a REST API.

## Architecture

### Core Components

- **`TypstWrapperWorld`** (`src/lib.rs:19-46`): Custom implementation of Typst's `World` trait that handles file resolution, package downloads, and font management. Acts as the environment interface for Typst compilation.

- **PDF Generation Pipeline** (`src/templates/mod.rs:29-43`): 
  1. Creates a `TypstWrapperWorld` instance with template content
  2. Compiles Typst markup using `typst::compile`
  3. Converts the document to PDF using `typst_pdf::pdf`

- **Template System** (`src/templates/`): Structured data models that convert Rust structs into Typst template parameters
  - `GermanTemplateData` converts structured invoice data into Typst syntax
  - Uses the `classy-german-invoice` Typst package for formatting

### HTTP API Structure

- **Single Route**: `GET /` accepts JSON payload and returns PDF
- **Error Handling**: Custom `AppError` enum with proper HTTP status mapping
- **Response Format**: Returns PDF with appropriate headers (`Content-Type: application/pdf`, `Content-Disposition: inline`)

## Development Commands

### Build and Run
```bash
cargo run                    # Start the server on port 3000
cargo build                  # Compile the project
cargo check                  # Check for compilation errors without building
```

### Testing
```bash
cargo nextest run           # Run all tests (preferred test runner)
cargo nextest run --lib     # Run only library tests
cargo nextest run --test <test_name>  # Run specific integration test
```

### Other Commands
```bash
cargo clean                 # Clean build artifacts
cargo doc --open           # Generate and open documentation
```

## Key Dependencies

- **Typst ecosystem**: `typst`, `typst-kit`, `typst-pdf` for document compilation
- **Web server**: `axum` for HTTP API with `tokio` runtime
- **Logging**: `tracing` with subscriber for structured logging
- **Error handling**: `thiserror` for custom error types

## Template Development

Templates are stored in `templates/` directory as `.typ` files and embedded using `include_str!` macro. The German invoice template uses the external `@preview/classy-german-invoice:0.3.1` package.

To add new templates:
1. Create `.typ` file in `templates/`
2. Add corresponding Rust module in `src/templates/`
3. Implement data structure with `into_typst_template()` method
4. Add conversion logic to transform Rust data into Typst parameters

## Testing Strategy

Tests are located in `#[cfg(test)]` modules within each source file:
- `src/templates/mod.rs:88-104`: PDF generation validation
- `src/templates/german_invoice.rs:255-261`: Template compilation testing

Tests verify PDF format correctness (starts with `%PDF-`, ends with `%%EOF`) and successful template compilation.