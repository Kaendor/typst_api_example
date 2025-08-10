# Suggested Commands

## Development Commands

### Build and Run
```bash
cargo run                    # Start the server on port 3000
cargo build                  # Compile the project
cargo build --release       # Compile with optimizations
cargo check                  # Check for compilation errors without building
```

### Testing
```bash
cargo nextest run           # Run all tests (preferred test runner)
cargo nextest run --lib     # Run only library tests
cargo nextest run --test <test_name>  # Run specific integration test
cargo test                  # Standard test runner (fallback)
RUST_BACKTRACE=1 cargo nextest run  # Run tests with backtrace
```

### Development Tools
```bash
cargo clean                 # Clean build artifacts
cargo doc --open           # Generate and open documentation
cargo fmt                  # Format code (if rustfmt is configured)
cargo clippy               # Run linter (if clippy is available)
```

## System Commands (Linux)
```bash
git status                  # Check git status
git add .                   # Stage changes
git commit -m "message"     # Commit changes
ls -la                      # List files with details
grep -r "pattern" src/      # Search for patterns in source
find . -name "*.rs"         # Find Rust files
```

## HTTP Testing
```bash
# Test the API endpoint
curl -X GET http://localhost:3000/

# Or use any HTTP client to send GET request to port 3000
```

## Package Management
```bash
cargo update               # Update dependencies
cargo tree                 # Show dependency tree
cargo audit               # Security audit (if installed)
```