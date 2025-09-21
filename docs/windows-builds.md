# Building Windows Executables

This document explains how to build Windows executables from the gosub-engine project.

## Overview

The gosub-engine project supports cross-compilation to Windows from Linux using the MinGW-w64 toolchain. This allows you to build Windows `.exe` files without needing a Windows machine.

## Prerequisites

### Linux (Ubuntu/Debian)

Install the required cross-compilation tools:

```bash
# Install MinGW-w64 cross-compiler
sudo apt update
sudo apt install -y mingw-w64 gcc-mingw-w64-x86-64

# Add Windows target to Rust
rustup target add x86_64-pc-windows-gnu
```

## Available Windows Executables

Currently, the following Windows executables can be built:

### CSS3 Parser (Minimal)
- **Binary**: `css3-parser-minimal.exe`
- **Description**: A standalone CSS3 parser and tokenizer
- **Features**: 
  - Parse CSS files or URLs
  - Basic tokenization and structure analysis
  - No GUI dependencies
  - Small size (~2.6MB optimized)

## Building

### Method 1: Using the Build Script (Recommended)

Use the provided build script for an automated build:

```bash
# Make the script executable
chmod +x build-windows.sh

# Build Windows executables
./build-windows.sh
```

### Method 2: Using Make

```bash
make build-windows
```

### Method 3: Manual Build

```bash
# Add Windows target
rustup target add x86_64-pc-windows-gnu

# Build specific binary
cargo build --release --target x86_64-pc-windows-gnu --package gosub_css3 --bin css3-parser-minimal
```

## Output Location

Built executables are located in:
```
target/x86_64-pc-windows-gnu/release/
```

## Usage Examples

Once built, you can distribute the `.exe` files to Windows machines:

```cmd
# Parse a local CSS file
css3-parser-minimal.exe styles.css

# Parse CSS from a URL
css3-parser-minimal.exe https://example.com/style.css

# Enable debug output
css3-parser-minimal.exe -d styles.css
```

## Continuous Integration

The project includes automated Windows builds in GitHub Actions. Windows executables are built and uploaded as artifacts for each commit.

See the `windows` job in `.github/workflows/ci.yaml` for details.

## Technical Details

### Cross-Compilation Setup

The build uses the following configuration:

- **Target**: `x86_64-pc-windows-gnu`
- **Linker**: `x86_64-w64-mingw32-gcc`
- **Architecture**: 64-bit Windows

### Feature Flags

To avoid Linux-specific dependencies (GTK, Cairo, etc.), Windows builds use minimal feature sets:

```toml
# Disabled features for Windows
default-features = false
```

### Limitations

Currently, the following components are **not** available for Windows builds:

- GTK-based GUI applications
- Cairo rendering backend
- Font manager with system integration
- Full browser examples

These limitations are due to the cross-compilation complexity of system libraries. Future work may address these through:

1. Windows-native implementations
2. Alternative rendering backends
3. Conditional compilation improvements

## Troubleshooting

### Common Issues

**Error: `cannot find -lsqlite3`**
- **Cause**: Missing Windows SQLite library
- **Solution**: Build without database features or use embedded SQLite

**Error: `pkg-config has not been configured`**
- **Cause**: System libraries not available for cross-compilation
- **Solution**: Disable system library features for Windows builds

### Getting Help

If you encounter issues with Windows builds:

1. Check that all prerequisites are installed
2. Verify the target is added: `rustup target list --installed`
3. Try building individual packages: `cargo build --target x86_64-pc-windows-gnu --package gosub_css3`
4. Open an issue on GitHub with build logs

## Future Work

Planned improvements for Windows support:

- [ ] HTML5 parser executable
- [ ] Configuration utilities
- [ ] Windows-native rendering backend
- [ ] MSVC toolchain support
- [ ] Windows installer package
- [ ] Native Windows GUI applications

## Contributing

To contribute to Windows support:

1. Test builds on different Windows versions
2. Report compatibility issues
3. Add new Windows-compatible executables
4. Improve cross-compilation setup
5. Add Windows-specific features

See `CONTRIBUTING.md` for general contribution guidelines.