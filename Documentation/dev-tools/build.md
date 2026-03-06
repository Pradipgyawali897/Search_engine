# Pernox Build System Guide

The Pernox Kernel uses the standard Cargo build system for its micro-crate workspace.

## Compilation

Ensure you have the latest Rust toolchain installed (Edition 2024 recommended).

### Debug Mode
For standard development and testing:
```bash
cargo build
```

### Release Mode
For performance-critical testing and deployment:
```bash
cargo build --release
```

## Running Components

To run the main entry point:
```bash
cargo run -p app
```

To run individual crate tests:
```bash
cargo test -p crawler
cargo test -p indexer
```
