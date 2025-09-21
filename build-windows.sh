#!/bin/bash

# Script to build Windows executables using cross-compilation
# This script builds core functionality without GTK/Cairo dependencies

set -e

echo "Building Windows executables for gosub-engine..."

# Install required target if not present
echo "Adding Windows target..."
rustup target add x86_64-pc-windows-gnu

# Set cross-compilation environment
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

# Build core binaries without GUI dependencies
echo "Building core binaries..."

# Build CSS3 parser - core functionality
echo "Building css3-parser..."
cargo build --release --target x86_64-pc-windows-gnu --bin css3-parser --no-default-features

# Build HTML5 parser test
echo "Building html5-parser-test..."
cargo build --release --target x86_64-pc-windows-gnu --bin html5-parser-test --no-default-features

# Build parser test
echo "Building parser-test..."
cargo build --release --target x86_64-pc-windows-gnu --bin parser-test --no-default-features

# Build config store
echo "Building config-store..."
cargo build --release --target x86_64-pc-windows-gnu --bin config-store --no-default-features

# Build gosub parser
echo "Building gosub-parser..."
cargo build --release --target x86_64-pc-windows-gnu --bin gosub-parser --no-default-features

# List built executables
echo ""
echo "Windows executables built successfully:"
echo "Location: target/x86_64-pc-windows-gnu/release/"
ls -la target/x86_64-pc-windows-gnu/release/*.exe 2>/dev/null || echo "No .exe files found, checking built binaries:"
ls -la target/x86_64-pc-windows-gnu/release/ | grep -E "(css3-parser|html5-parser-test|parser-test|config-store|gosub-parser)$"

echo ""
echo "Build completed successfully!"