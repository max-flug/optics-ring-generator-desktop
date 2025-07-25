#!/bin/bash

# Optics Ring Generator Professional - Installer Build Script
# This script builds installers for macOS and Windows

echo "🚀 Building Optics Ring Generator Professional Installers"
echo "=================================================="

# Check if we're on macOS or Windows/Linux
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "📱 Building macOS installer..."
    echo "Building DMG package for macOS (current architecture)..."
    npm run tauri build
    
    if [ $? -eq 0 ]; then
        echo "✅ macOS installer built successfully!"
        echo "📦 DMG file location: src-tauri/target/release/bundle/dmg/"
        echo ""
        echo "💡 For universal binary (Intel + Apple Silicon), use:"
        echo "   npm run tauri build -- --target universal-apple-darwin"
    else
        echo "❌ Build failed!"
        exit 1
    fi
    
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "🪟 Building Windows installer..."
    echo "Building NSIS installer for Windows..."
    npm run tauri build -- --target x86_64-pc-windows-msvc
    echo "✅ Windows installer built successfully!"
    echo "📦 Installer file location: src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/"
else
    echo "🐧 Building for current platform..."
    npm run tauri build
    echo "✅ Installer built successfully!"
    echo "📦 Check src-tauri/target/release/bundle/ for installer files"
fi

echo ""
echo "🎉 Build complete!"
echo "=================================================="
echo "📋 Installation Instructions:"
echo ""
echo "macOS:"
echo "  - Double-click the .dmg file"
echo "  - Drag 'Optics Ring Generator Professional' to Applications"
echo "  - Launch from Launchpad or Applications folder"
echo ""
echo "Windows:"
echo "  - Run the .exe installer"
echo "  - Follow the installation wizard"
echo "  - Launch from Start Menu or Desktop shortcut"
echo ""
echo "ℹ️  All dependencies (WebView2, etc.) are included automatically!"
