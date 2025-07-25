#!/bin/bash

# Optics Ring Generator Professional - macOS Installer Build Script
# This script builds the macOS DMG installer

echo "🚀 Building Optics Ring Generator Professional - macOS Installer"
echo "================================================================="

echo "📱 Building macOS installer (DMG)..."
echo "Building for current Mac architecture..."

# Build for current architecture (more reliable)
npm run tauri build

BUILD_SUCCESS=$?

if [ $BUILD_SUCCESS -eq 0 ]; then
    echo "✅ macOS installer built successfully!"
    echo "📦 DMG location: src-tauri/target/release/bundle/dmg/"
    echo ""
    echo "🎉 Build complete!"
    echo "================================================================="
    echo "📋 Installation Instructions:"
    echo ""
    echo "  1. Navigate to: src-tauri/target/release/bundle/dmg/"
    echo "  2. Double-click the .dmg file"
    echo "  3. Drag 'Optics Ring Generator Professional' to Applications folder"
    echo "  4. Launch from Launchpad or Applications folder"
    echo ""
    echo "ℹ️  This binary is optimized for your current Mac architecture!"
    echo "ℹ️  All dependencies are included automatically!"
    echo ""
    echo "🔍 To create a universal binary (Intel + Apple Silicon):"
    echo "   Run: npm run tauri build -- --target universal-apple-darwin"
    echo "   (Requires both x86_64-apple-darwin and aarch64-apple-darwin targets)"
else
    echo "❌ Build failed! Please check the error messages above."
    echo ""
    echo "🔧 Troubleshooting tips:"
    echo "  - Ensure Xcode Command Line Tools are installed: xcode-select --install"
    echo "  - Check Rust installation: rustup --version"
    echo "  - Verify npm dependencies: npm install"
    echo "  - Try cleaning build cache: cargo clean"
    exit 1
fi
