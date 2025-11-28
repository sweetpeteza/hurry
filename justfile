# Hurry project commands

# List all available commands
default:
    @just --list

# Run all pre-commit checks (format, clippy, test)
check:
    @echo "Running all checks..."
    @just fmt
    @just clippy
    @just test
    @echo "✓ All checks passed!"

# Format code with rustfmt
fmt:
    @echo "Formatting code..."
    cargo fmt --all

# Check formatting without modifying files
fmt-check:
    @echo "Checking formatting..."
    cargo fmt --all -- --check

# Run clippy linter
clippy:
    @echo "Running clippy..."
    cargo clippy --all-features --all-targets -- -D warnings

# Run clippy with automatic fixes
clippy-fix:
    @echo "Running clippy with fixes..."
    cargo clippy --all-features --all-targets --fix --allow-dirty --allow-staged

# Run all tests
test:
    @echo "Running tests..."
    cargo test --all-features

# Run tests with output
test-verbose:
    @echo "Running tests (verbose)..."
    cargo test --all-features -- --nocapture

# Build the project
build:
    @echo "Building project..."
    cargo build --all-features

# Build release version
build-release:
    @echo "Building release..."
    cargo build --release --all-features

# Build documentation
docs:
    @echo "Building documentation..."
    cargo doc --no-deps --all-features --open

# Check documentation for warnings
docs-check:
    @echo "Checking documentation..."
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean

# Update dependencies
update:
    @echo "Updating dependencies..."
    cargo update
    @echo "Updated dependencies. Run 'just test' to verify."

# Run the full CI workflow locally
ci: fmt-check clippy test docs-check
    @echo "✓ All CI checks passed!"

# Run a specific example
example NAME:
    @echo "Running example: {{NAME}}"
    cargo run --example {{NAME}}

# Run all examples
examples:
    @echo "Running all examples..."
    @just example basic_pointers
    @just example cell_types
    @just example interior_mutability

# Install development dependencies
install-dev:
    @echo "Installing development tools..."
    cargo install cargo-edit
    cargo install cargo-outdated
    @echo "✓ Development tools installed"

# Check for outdated dependencies
outdated:
    @echo "Checking for outdated dependencies..."
    cargo outdated

# Prepare for release (all checks + clean build)
release-prep:
    @echo "Preparing for release..."
    @just clean
    @just ci
    @just build-release
    @echo "✓ Ready for release!"

# Quick development workflow (format and test)
dev:
    @just fmt
    @just test

# Watch for changes and run tests
watch:
    @echo "Watching for changes..."
    cargo watch -x test

# Display project info
info:
    @echo "Project: hurry"
    @echo "Version: $(grep '^version' Cargo.toml | head -1 | cut -d '=' -f 2 | tr -d ' \"')"
    @echo ""
    @echo "Common commands:"
    @echo "  just check      - Run all checks"
    @echo "  just dev        - Quick format + test"
    @echo "  just ci         - Full CI workflow"
    @echo "  just clippy-fix - Auto-fix clippy warnings"
