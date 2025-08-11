#!/bin/bash
# Cross-platform build script for Moonwalk Macros

echo "🚀 Building Moonwalk Macros for multiple platforms..."
echo "=================================================="

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Create releases directory if it doesn't exist
mkdir -p releases

# Build for Windows (x86_64)
echo "🪟 Building for Windows x64..."
cargo build --release --target x86_64-pc-windows-msvc
if [ $? -eq 0 ]; then
    cp target/x86_64-pc-windows-msvc/release/moonwalk-macros.exe releases/moonwalk-macros-windows-x64.exe
    echo "✅ Windows x64 build completed"
else
    echo "❌ Windows x64 build failed"
fi

# Build for macOS Intel
echo "🍎 Building for macOS Intel..."
cargo build --release --target x86_64-apple-darwin
if [ $? -eq 0 ]; then
    cp target/x86_64-apple-darwin/release/moonwalk-macros releases/moonwalk-macros-macos-intel
    echo "✅ macOS Intel build completed"
else
    echo "❌ macOS Intel build failed"
fi

# Build for macOS Apple Silicon
echo "🍎 Building for macOS Apple Silicon..."
cargo build --release --target aarch64-apple-darwin
if [ $? -eq 0 ]; then
    cp target/aarch64-apple-darwin/release/moonwalk-macros releases/moonwalk-macros-macos-apple
    echo "✅ macOS Apple Silicon build completed"
else
    echo "❌ macOS Apple Silicon build failed"
fi

# Build for Linux
echo "🐧 Building for Linux x64..."
cargo build --release --target x86_64-unknown-linux-gnu
if [ $? -eq 0 ]; then
    cp target/x86_64-unknown-linux-gnu/release/moonwalk-macros releases/moonwalk-macros-linux-x64
    echo "✅ Linux x64 build completed"
else
    echo "❌ Linux x64 build failed"
fi

echo "=================================================="
echo "🎉 Build process completed!"
echo ""
echo "📦 Available releases:"
ls -la releases/
echo ""
echo "💡 Note: Cross-compilation may require additional setup for some targets"
echo "💡 macOS and Linux builds were cross-compiled from Windows and may need testing"
