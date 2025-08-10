# Task Completion Guidelines

## When a Task is Completed

### Required Steps
1. **Run Tests**: Always run `cargo nextest run` to ensure all tests pass
2. **Check Compilation**: Run `cargo check` to verify no compilation errors
3. **Build Verification**: Run `cargo build` to ensure clean build
4. **Code Formatting**: Run `cargo fmt` if available to maintain consistent formatting

### Optional Steps (if tools are available)
1. **Linting**: Run `cargo clippy` for additional code quality checks
2. **Documentation**: Run `cargo doc` if public APIs were added or modified

### Testing Strategy
- **Primary Test Runner**: Use `cargo nextest run` (preferred)
- **Fallback**: Use `cargo test` if nextest is not available
- **Debug Failed Tests**: Use `RUST_BACKTRACE=1` for detailed error information
- **Specific Tests**: Use `cargo nextest run --lib` for library tests only

### Error Investigation
- If tests fail, examine error messages carefully
- Check for compilation errors first with `cargo check`
- Use enhanced error reporting for detailed diagnostics
- Verify template syntax and external package dependencies

### Before Committing
1. Ensure all tests pass
2. Verify clean compilation
3. Check that the server starts successfully with `cargo run`
4. Test HTTP endpoint manually if API changes were made

### Performance Considerations
- First compilation takes longer due to Typst dependencies
- Subsequent builds are faster due to incremental compilation
- Test compilation includes both lib and test targets