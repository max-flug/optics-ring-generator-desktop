#!/bin/bash

# Optics Ring Generator Professional - Build Validation & Quick Start
# Run this script to validate your setup and build installers

echo "🔍 Validating Build Environment"
echo "================================"

# Check if we're in the right directory
if [ ! -f "src-tauri/tauri.conf.json" ]; then
    echo "❌ Error: Not in the correct project directory"
    echo "Please run this script from the project root directory"
    exit 1
fi

echo "✅ Project directory: OK"

# Check Node.js
if command -v node &> /dev/null; then
    NODE_VERSION=$(node --version)
    echo "✅ Node.js: $NODE_VERSION"
else
    echo "❌ Node.js not found. Please install Node.js 16+"
    exit 1
fi

# Check npm dependencies
if [ ! -d "node_modules" ]; then
    echo "📦 Installing npm dependencies..."
    npm install
fi
echo "✅ npm dependencies: OK"

# Check Rust
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo "✅ Rust: $RUST_VERSION"
    
    # Check and install required targets for macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "🔧 Checking Rust targets for macOS universal binary..."
        
        if ! rustup target list --installed | grep -q "x86_64-apple-darwin"; then
            echo "📦 Installing x86_64-apple-darwin target..."
            rustup target add x86_64-apple-darwin
        fi
        
        if ! rustup target list --installed | grep -q "aarch64-apple-darwin"; then
            echo "📦 Installing aarch64-apple-darwin target..."
            rustup target add aarch64-apple-darwin
        fi
        
        echo "✅ Universal binary targets: OK"
    fi
else
    echo "❌ Rust not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check Tauri CLI
if command -v tauri &> /dev/null; then
    TAURI_VERSION=$(tauri --version)
    echo "✅ Tauri CLI: $TAURI_VERSION"
else
    echo "📦 Installing Tauri CLI..."
    cargo install tauri-cli
fi

echo ""
echo "🎉 Environment validation complete!"
echo "=================================="
echo ""
echo "🚀 Ready to build installers!"
echo ""
echo "Choose your build option:"
echo ""
echo "1️⃣  Build for current platform:"
echo "   npm run build"
echo ""
echo "2️⃣  Build macOS installer (DMG):"
echo "   ./build-installer-macos.sh"
echo ""
echo "3️⃣  Build Windows installer (NSIS):"
echo "   ./build-installer-windows.bat"
echo ""
echo "4️⃣  Auto-detect platform and build:"
echo "   ./build-installers.sh"
echo ""
echo "📚 For detailed instructions, see: INSTALLER.md"
echo ""

# Ask user what they want to do
read -p "Would you like to build an installer now? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🚀 Starting build process..."
    ./build-installers.sh
else
    echo "👍 You can build later using the commands above!"
fi
