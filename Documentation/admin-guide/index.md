# Admin Guide: Operating the Pernox Kernel

## Introduction

This guide describes how to configure, boot, and manage the Pernox kernel.

## Prerequisites

- Rust Toolchain (Edition 2024)
- Network connectivity (for Spyder discovery)

## Initialization

The Pernox kernel requires a seed list to begin its traversal.

```bash
# Prepare the seed file
echo "https://rust-lang.org" > seeds.txt

# Boot the engine
cargo run -p app
```

## Runtime Configuration

The engine reads `seeds.txt` and `index.json` during initialization. Ensure these files are located in the root of the workspace.

For alternate layouts, the application layer also accepts:

- `PERNOX_SEED_FILE` to point at a different seed list.
- `PERNOX_INDEX_PATH` to point at a different index output file.
