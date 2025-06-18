#!/bin/bash
set -e

echo "🧪 Running RustQL test suite..."

# Format check
echo "Checking code formatting..."
cargo fmt --check

# Linting
echo "Running clippy..."
cargo clippy -- -D warnings

# Unit tests
echo "Running unit tests..."
cargo test

# Integration tests
echo "Running integration tests..."
cargo test --test integration

echo "✅ All tests passed!"
