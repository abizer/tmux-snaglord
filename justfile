# Rust project checks

set positional-arguments
set shell := ["bash", "-euo", "pipefail", "-c"]

# List available commands
default:
    @just --list

# Run all checks
[parallel]
check: format clippy-fix build test clippy

# Format Rust files
format:
    @cargo fmt --all

# Run clippy and fail on any warnings
clippy:
    @cargo clippy --quiet -- -D clippy::all 2>&1 | { grep -v "^0 errors" || true; }

# Auto-fix clippy warnings
clippy-fix:
    @cargo clippy --fix --allow-dirty --quiet -- -W clippy::all 2>&1 | { grep -v "^0 errors" || true; }

# Build the project
build:
    cargo build --all

# Run tests
test:
    #!/usr/bin/env bash
    set -euo pipefail
    output=$(cargo test --quiet 2>&1) || { echo "$output"; exit 1; }
    echo "$output" | tail -1

# Run the application
run *ARGS:
    cargo run -- "$@"

# Release a new patch version
release-patch:
    @just _release patch

# Release a new minor version
release-minor:
    @just _release minor

# Release a new major version
release-major:
    @just _release major

# Internal release helper
_release bump:
    @cargo-release {{bump}}
