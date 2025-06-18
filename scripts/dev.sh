#!/bin/bash
set -e

echo "🚀 Starting RustQL development server..."

# Install cargo-watch if not present
if ! command -v cargo-watch &> /dev/null; then
    echo "Installing cargo-watch for hot reloading..."
    cargo install cargo-watch
fi

# Start with hot reloading
RUST_LOG=debug cargo watch -x 'run'
