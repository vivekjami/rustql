#!/bin/bash
set -e

echo "📊 Running RustQL benchmarks..."

# Install criterion if not present
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found. Please install Rust."
    exit 1
fi

# Run benchmarks
cargo bench

echo "✅ Benchmarks completed!"
