# Codebase Structure

## Directory Layout
```
.
├── src/
│   ├── main.rs                    # Application entry point, server setup
│   ├── lib.rs                     # Core library, TypstWrapperWorld implementation
│   ├── routes/
│   │   └── mod.rs                 # HTTP route handlers
│   └── templates/
│       ├── mod.rs                 # Template system, PDF generation logic
│       └── german_invoice.rs      # German invoice template implementation
├── templates/
│   └── german_invoice.typ         # Static Typst template file
├── Cargo.toml                     # Dependencies and project metadata
├── Cargo.lock                     # Dependency lock file
├── README.md                      # Basic project documentation
└── CLAUDE.md                      # Claude Code guidance

```

## Key Files and Their Roles

### `src/main.rs`
- Server entry point with tokio main function
- Axum router setup with single route
- Tracing subscriber configuration
- Server binding to 0.0.0.0:3000

### `src/lib.rs`
- `TypstWrapperWorld` struct: Core Typst integration
- Implements `typst::World` trait for compilation environment
- Handles file resolution, package downloads, font management
- Contains utility functions for HTTP and retry logic

### `src/routes/mod.rs`
- Single HTTP endpoint: `pdf_generation_controller`
- Accepts JSON payload, returns PDF response
- Error handling and proper HTTP headers
- Uses German invoice template as default

### `src/templates/mod.rs`
- `template_to_pdf` function: Main PDF generation pipeline
- `AppError` enum: Custom error types with thiserror
- HTTP response conversion for errors
- Basic PDF validation tests

### `src/templates/german_invoice.rs`
- `GermanTemplateData` struct with conversion logic
- Supporting structs: `Author`, `Client`, `InvoiceItem`, `BankAccount`, `Address`
- Template string generation with Typst syntax
- Comprehensive test suite with fake data

### `templates/german_invoice.typ`
- Static Typst template using `classy-german-invoice` package
- Example template with sample data
- Demonstrates Typst syntax and package imports