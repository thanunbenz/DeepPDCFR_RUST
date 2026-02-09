.PHONY: test test-api clean build run help

# Default target
.DEFAULT_GOAL := help

# Test the API only
test-api:
	@echo "Running API tests..."
	cargo test

# Alias for test-api
test: test-api

# Build the API
build:
	@echo "Building API..."
	cargo build

# Build release version
build-release:
	@echo "Building API in release mode..."
	cargo build --release

# Run the API server
run:
	@echo "Starting API server..."
	cargo run

# Run in release mode
run-release:
	@echo "Starting API server in release mode..."
	cargo run --release

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

# Run tests with output
test-verbose:
	@echo "Running API tests with verbose output..."
	cargo test -- --nocapture

# Run tests and show coverage (requires cargo-tarpaulin)
test-coverage:
	@echo "Running tests with coverage..."
	cargo tarpaulin --out Html

# Check code without building
check:
	@echo "Checking code..."
	cargo check

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy linter
lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Help target
help:
	@echo "DeepPDCFR API Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  test, test-api    - Run all API tests"
	@echo "  test-verbose      - Run tests with output"
	@echo "  test-coverage     - Run tests with coverage"
	@echo "  build             - Build the API"
	@echo "  build-release     - Build in release mode"
	@echo "  run               - Run the API server"
	@echo "  run-release       - Run in release mode"
	@echo "  check             - Check code without building"
	@echo "  fmt               - Format code"
	@echo "  lint              - Run clippy linter"
	@echo "  clean             - Clean build artifacts"
	@echo "  help              - Show this help message"
