# Project Overview

## Purpose
This is a Rust-based HTTP API server that generates PDF documents from Typst templates. The project serves as a test server to demonstrate PDF generation capabilities using Typst (a modern typesetting language) through a REST API.

## Tech Stack
- **Language**: Rust (edition 2024)
- **Web Framework**: Axum 0.8.4 with HTTP/2 support and macros
- **Runtime**: Tokio async runtime with multi-threading
- **PDF Generation**: Typst ecosystem (typst, typst-kit, typst-pdf) version 0.13.1
- **Serialization**: Serde 1.0.219 with derive features
- **Logging**: Tracing ecosystem (tracing, tracing-subscriber, tracing-futures, tracing-log)
- **HTTP Client**: ureq 3.0.12 for downloading Typst packages
- **Error Handling**: thiserror 2.0.12
- **Archive Handling**: tar 0.4.44, zune-inflate 0.2.54
- **Date/Time**: time 0.3.41

## Architecture
- **Entry Point**: `src/main.rs` - Sets up Axum server on port 3000
- **Core Library**: `src/lib.rs` - Contains `TypstWrapperWorld` struct that implements Typst's World trait
- **Routes**: `src/routes/mod.rs` - HTTP endpoint handlers
- **Templates**: `src/templates/` - Template system with German invoice implementation
- **Static Templates**: `templates/` - Typst template files (.typ format)

## Key Components
- `TypstWrapperWorld`: Custom implementation of Typst's World trait for file resolution, package downloads, and font management
- PDF Generation Pipeline: Converts Typst markup to PDF via compilation and rendering
- Template System: Structured data models that convert Rust structs into Typst template parameters