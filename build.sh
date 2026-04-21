#!/bin/bash
set -e

echo "Building frontend with Dioxus..."
cd crates/frontend
trunk build --release
cd ../..

echo "Building backend with Axum..."
cargo build --release --workspace

echo "Build complete!"