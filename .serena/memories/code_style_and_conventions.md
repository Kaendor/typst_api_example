# Code Style and Conventions

## General Rust Conventions
- Uses Rust 2024 edition
- Standard Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- Error handling with Result types and custom error enums
- Structured logging with tracing crate

## Project-Specific Patterns

### Error Handling
- Custom `AppError` enum using thiserror derive macro
- Errors are converted to HTTP responses via `IntoResponse` trait
- Generic error messages for user-facing responses

### Async/Await
- Uses tokio runtime for async operations
- Functions marked with `#[tokio::main]` for entry point
- Async handlers for HTTP endpoints

### Logging
- Uses tracing ecosystem for structured logging
- `#[instrument]` attribute for function tracing
- Debug level logging configured in main

### HTTP API Design
- Single route pattern: `GET /` accepts JSON, returns PDF
- Proper HTTP headers for PDF responses:
  - `Content-Type: application/pdf`
  - `Content-Disposition: inline; filename="output.pdf"`
- JSON serialization with serde

### Template System
- Structured data models with conversion methods
- Templates embedded using `include_str!` macro
- External Typst packages via `@preview/package-name:version`

### Testing Patterns
- Tests in `#[cfg(test)]` modules within source files
- Unit tests verify basic functionality
- Integration tests check end-to-end compilation
- PDF validation by checking format markers (`%PDF-`, `%%EOF`)

### Documentation
- Function documentation with `///` comments
- Module-level documentation for major components
- Inline comments for complex logic