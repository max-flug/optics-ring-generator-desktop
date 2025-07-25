# 📦 Installer Build Guide
## Optics Ring Generator Professional

This guide explains how to create installers for both macOS and Windows platforms. All dependencies are automatically included.

## 🚀 Quick Start

### Option 1: Automated Build Scripts

**For macOS:**
```bash
./build-installer-macos.sh
```

**For Windows:**
```cmd
build-installer-windows.bat
```

**For Cross-Platform:**
```bash
./build-installers.sh
```

### Option 2: NPM Scripts

```bash
# Build installer for current platform
npm run build:installer

# Build specifically for macOS
npm run build:macos

# Build specifically for Windows  
npm run build:windows
```

### Option 3: Manual Tauri Build

```bash
# Build for current platform
npm run tauri build

# Build for specific targets
npm run tauri build -- --target x86_64-pc-windows-msvc
npm run tauri build -- --target universal-apple-darwin
```

## 📁 Output Locations

### macOS
- **Format:** DMG (Disk Image)
- **Location:** `src-tauri/target/universal-apple-darwin/release/bundle/dmg/`
- **File:** `Optics Ring Generator Professional_1.0.0_universal.dmg`

### Windows
- **Format:** NSIS Installer (.exe)
- **Location:** `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/`
- **File:** `Optics Ring Generator Professional_1.0.0_x64-setup.exe`

## 🔧 What's Included

### All Platforms
- ✅ Application binary
- ✅ WebView runtime (automatically downloaded/installed)
- ✅ All Rust dependencies (statically linked)
- ✅ Frontend assets (HTML, CSS, JS)
- ✅ Icons and metadata

### Windows Specific
- ✅ Microsoft WebView2 runtime (auto-download)
- ✅ Visual C++ Redistributables (if needed)
- ✅ Proper Windows registry entries
- ✅ Start Menu shortcuts
- ✅ Uninstaller

### macOS Specific
- ✅ Universal binary (Intel + Apple Silicon)
- ✅ macOS code signing preparation
- ✅ Proper app bundle structure
- ✅ Drag-to-Applications DMG layout

## 📋 Installation Instructions

### For End Users - Windows
1. Download the `.exe` installer
2. Run the installer (may require admin privileges)
3. Follow the installation wizard
4. Launch from Start Menu or Desktop shortcut

### For End Users - macOS
1. Download the `.dmg` file
2. Double-click to mount the disk image
3. Drag "Optics Ring Generator Professional" to Applications folder
4. Launch from Launchpad or Applications folder
5. If prompted about security, go to System Preferences → Security & Privacy → Allow

## 🛠 Build Requirements

### Development Machine Setup

**For macOS Builds:**
- macOS 10.15+ (Catalina or later)
- Xcode Command Line Tools
- Rust (latest stable)
- Node.js 16+

**For Windows Builds:**
- Windows 10/11
- Visual Studio Build Tools or Visual Studio Community
- Rust (latest stable)
- Node.js 16+

**Cross-Platform Dependencies:**
- Tauri CLI: `cargo install tauri-cli`
- Project dependencies: `npm install`

## 🔐 Code Signing (Optional)

### Windows Code Signing
```json
// In tauri.conf.json
"windows": {
  "certificateThumbprint": "YOUR_CERT_THUMBPRINT",
  "digestAlgorithm": "sha256",
  "timestampUrl": "http://timestamp.sectigo.com"
}
```

### macOS Code Signing
```json
// In tauri.conf.json
"macOS": {
  "signingIdentity": "Developer ID Application: Your Name (TEAMID)",
  "hardenedRuntime": true,
  "entitlements": "path/to/entitlements.plist"
}
```

## 🎯 Troubleshooting

### Common Issues

**Build Fails on Windows:**
- Ensure Visual Studio Build Tools are installed
- Check that Windows 10/11 SDK is available
- Verify Rust target: `rustup target add x86_64-pc-windows-msvc`

**Build Fails on macOS:**
- Install Xcode Command Line Tools: `xcode-select --install`
- For universal binary: `rustup target add aarch64-apple-darwin x86_64-apple-darwin`

**Large File Sizes:**
- This is normal - installers include all dependencies
- Windows: ~50-100MB (includes WebView2 bootstrapper)
- macOS: ~30-60MB (universal binary)

### Getting Help

If you encounter issues:
1. Check the Tauri documentation: https://tauri.app/
2. Verify all prerequisites are installed
3. Clear build cache: `cargo clean` and rebuild
4. Check the GitHub issues for this project

## 📊 Distribution

### Recommended Distribution Methods

**Enterprise/Professional:**
- Host installers on your own server
- Use package managers (Homebrew for macOS, Chocolatey for Windows)
- Corporate software distribution systems

**Public Distribution:**
- GitHub Releases (free)
- Microsoft Store (Windows)
- Mac App Store (macOS, requires Apple Developer account)

### File Naming Convention
- Windows: `OpticsRingGeneratorProfessional_1.0.0_x64-setup.exe`
- macOS: `OpticsRingGeneratorProfessional_1.0.0_universal.dmg`

---

🎉 **Your users will get a professional, one-click installation experience with all dependencies handled automatically!**
