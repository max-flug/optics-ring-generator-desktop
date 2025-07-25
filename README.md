# Optics Ring Generator - Desktop Application

A Tauri-based desktop application for generating 3D-printable support rings for optical lenses and components.

## Features

- **Three Ring Types:**
  - **Convex (CX):** Curved inward surface for gentle lens support
  - **Concave (CC):** Curved outward surface for specific curvature requirements  
  - **Three-Point (3P):** Minimal contact design with precise contact points

- **Customizable Parameters:**
  - Outer diameter (10-500mm)
  - Inner diameter (5-495mm)
  - Automatic height calculation based on wall thickness

- **STL Output:** Ready-to-print STL files for 3D printing
- **Cross-Platform:** Works on Windows, macOS, and Linux

## Getting Started

### Prerequisites

- [Rust](https://rustlang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v16 or later)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Development

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd optics-ring-generator-desktop
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build for production:
   ```bash
   npm run tauri build
   ```

## 📦 Building Installers

### 🚀 GitHub Actions (Recommended)
**Build installers automatically in the cloud:**

1. **Automatic builds** on every push/PR:
   - Builds macOS (Intel + Apple Silicon) and Windows installers
   - No local setup required, works from any platform
   - Artifacts available for download after build completes

2. **Manual Windows build:**
   - Go to **Actions** tab → **Build Windows Installer**
   - Click **Run workflow**
   - Download the Windows installer from artifacts

3. **Release builds:**
   - Push a git tag: `git tag v1.0.0 && git push --tags`
   - Creates GitHub Release with all installers attached

### 🛠️ Local Builds
```bash
# Validate environment and build
./setup-build.sh

# Or build directly
./build-installers.sh         # Auto-detect platform
./build-installer-macos.sh    # macOS DMG
./build-installer-windows.bat # Windows NSIS (requires Windows)
```

### 📁 Output Files
- **macOS:** `src-tauri/target/*/release/bundle/dmg/*.dmg`
- **Windows:** `src-tauri/target/*/release/bundle/nsis/*.exe`
- **GitHub Actions:** Download from workflow artifacts

### ✅ What's Included
✅ All dependencies (WebView2, runtime libraries)  
✅ Professional installer with proper metadata  
✅ One-click installation for end users  
✅ Cross-platform builds via GitHub Actions  
✅ Automatic release creation  
✅ Proper uninstaller

📚 **Detailed Instructions:** See [INSTALLER.md](INSTALLER.md)

## Usage

1. Select a ring type from the dropdown menu
2. Enter the outer diameter of your lens mount or housing
3. Enter the inner diameter (lens diameter)
4. Click "Generate STL" to create the 3D model
5. The STL file will be saved in the current directory

## Ring Types Explained

### Convex (CX)
- Curved inward contact surface
- Provides gentle, distributed support
- Best for general-purpose optical elements
- Reduces stress concentration points

### Concave (CC) 
- Curved outward contact surface
- Complements specific lens curvatures
- Good for lenses with pronounced curves
- Maintains proper optical alignment

### Three-Point (3P)
- Three discrete contact points at 120° intervals
- Minimal contact area for delicate optics
- Reduces contamination risk
- Ideal for precision optical components

## Technical Details

- **Minimum Wall Thickness:** 1.0mm (recommended for 3D printing)
- **Resolution:** 64 segments for smooth curves
- **File Format:** Binary STL
- **Coordinate System:** Right-handed, Z-up

## Built With

- [Tauri](https://tauri.app/) - Desktop application framework
- [Rust](https://www.rust-lang.org/) - Backend geometry generation
- Vanilla HTML/CSS/JavaScript - Frontend interface
- [nalgebra](https://nalgebra.org/) - Linear algebra operations
- [stl_io](https://crates.io/crates/stl_io) - STL file generation

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
- [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
