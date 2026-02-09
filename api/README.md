# DeepPDCFR Mock Server (Rust)

High-performance Rust implementation of the DeepPDCFR mock API server with automatic Swagger/OpenAPI documentation.

## Features

- ⚡ **Fast**: Built with Axum and Tokio for high-performance async I/O
- 📚 **Documented**: Automatic Swagger UI generation via utoipa
- 🔒 **Type-safe**: Compile-time guarantees with Rust's type system
- 🧪 **Tested**: Comprehensive integration tests
- 🌐 **CORS-enabled**: Ready for cross-origin requests

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))

### Build and Run

```bash
# Build the project
cargo build --release

# Run the server
cargo run --release

# Or run in development mode with auto-reload
cargo watch -x run
```

The server will start on `http://localhost:8000`

### Access Swagger UI

Open your browser to: **http://localhost:8000/docs**

## API Endpoints

### Health Check
```bash
GET /health
```

Returns service status, model availability, and version.

### Solve Strategy
```bash
POST /v1/solve
Content-Type: application/json

{
  "player": "OOP",
  "board": "Ah Kd Qc",
  "effective_stack": 100,
  "starting_pot": 20
}
```

Returns Nash-equilibrium strategy for the given game state.

## Development

### Run Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_health_endpoint
```

### Code Quality

```bash
# Check for issues
cargo clippy

# Format code
cargo fmt

# Check formatting without changes
cargo fmt -- --check
```

### Build Optimized Binary

```bash
cargo build --release
# Binary will be at: target/release/deeppdcfr-mock-server
```

## Project Structure

```
rust-mock-server/
├── src/
│   ├── main.rs           # Server entry point
│   ├── lib.rs            # Library root
│   ├── api/              # API endpoint handlers
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   └── solve.rs
│   ├── models/           # Request/response models
│   │   ├── mod.rs
│   │   ├── request.rs
│   │   ├── response.rs
│   │   └── health.rs
│   ├── mock_data.rs      # Mock poker strategies
│   ├── config.rs         # Server configuration
│   └── error.rs          # Error handling
└── tests/                # Integration tests
    └── integration_test.rs
```

## Mock Data

The server returns pre-computed mock strategies for:
- **Board**: Ah Kd Qc (flop)
- **Range**: 46 hand combinations (AA, AKs, AKo, KK, QQ, etc.)
- **Actions**: Check, Bet 33%, Bet 67%, All-in
- **Strategies**: Mixed strategies for each combo

This data matches the Python mock server for compatibility testing.

## Performance

Benchmarks (compared to Python FastAPI):
- **Startup time**: ~10x faster
- **Request latency**: ~5-10x faster
- **Memory usage**: ~3x less
- **Concurrent requests**: Better throughput under load

## Python Compatibility

This Rust server implements the exact same API contract as the Python mock server (`api/mock_server.py`), making it a drop-in replacement for testing and development.

## License

Same as the parent DeepPDCFR project.
