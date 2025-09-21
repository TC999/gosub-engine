#!/bin/bash

# Script to build Windows executables using cross-compilation
# This script builds core functionality without GTK/Cairo dependencies

set -e

echo "🏗️  Building Windows executables for gosub-engine..."

# Install required target if not present
echo "📦 Adding Windows target..."
rustup target add x86_64-pc-windows-gnu

# Set cross-compilation environment
export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

# Build core binaries without GUI dependencies
echo "⚡ Building core binaries..."

# Build minimal CSS3 parser - this works!
echo "🎨 Building css3-parser-minimal..."
cargo build --release --target x86_64-pc-windows-gnu --package gosub_css3 --bin css3-parser-minimal

echo ""
echo "✅ Windows executables built successfully!"
echo "📍 Location: target/x86_64-pc-windows-gnu/release/"
echo ""

# List built executables with file sizes
echo "📊 Built executables:"
ls -lh target/x86_64-pc-windows-gnu/release/*.exe 2>/dev/null | while read line; do
    echo "   $line"
done

echo ""
echo "🎯 Usage examples:"
echo "   ./target/x86_64-pc-windows-gnu/release/css3-parser-minimal.exe styles.css"
echo "   ./target/x86_64-pc-windows-gnu/release/css3-parser-minimal.exe https://example.com/style.css"
echo ""
echo "🚀 Ready for Windows deployment!"