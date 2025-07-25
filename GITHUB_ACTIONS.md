# 🚀 GitHub Actions Build Guide

This project uses GitHub Actions to automatically build installers for both macOS and Windows from any platform, eliminating the need for local cross-compilation setup.

## 🔄 Automatic Builds

### When Builds Trigger:
- ✅ **Every push** to `main` or `develop` branches
- ✅ **Every pull request** to `main`
- ✅ **Manual trigger** via GitHub Actions tab
- ✅ **Git tags** (creates releases)

### What Gets Built:
- 🍎 **macOS DMG** (Apple Silicon + Intel)
- 🪟 **Windows EXE** (NSIS installer)
- 🍎 **Universal macOS Binary** (both architectures in one file)

## 📥 How to Download Built Installers

### Method 1: From Workflow Artifacts
1. Go to your repository on GitHub
2. Click **Actions** tab
3. Click on the latest successful workflow run
4. Scroll down to **Artifacts** section
5. Download:
   - `macos-aarch64` - macOS Apple Silicon
   - `macos-x86_64` - macOS Intel
   - `macos-universal` - macOS Universal Binary
   - `windows-x64` - Windows installer

### Method 2: From Releases (Tags)
1. Push a version tag:
   ```bash
   git tag v1.0.0
   git push --tags
   ```
2. GitHub automatically creates a release with all installers
3. Download directly from the **Releases** page

## 🛠️ Manual Windows Build

If you need to build just the Windows installer:

1. Go to **Actions** tab
2. Click **Build Windows Installer**
3. Click **Run workflow** → **Run workflow**
4. Wait for completion (~5-10 minutes)
5. Download from artifacts

## 🔧 Workflow Configuration

### Files Created:
- `.github/workflows/build-installers.yml` - Main cross-platform build
- `.github/workflows/build-windows.yml` - Windows-only manual build

### Environment:
- **Node.js 20** with npm caching
- **Latest Rust toolchain** with target caching
- **Tauri CLI** with proper bundling
- **GitHub token** for release creation

## 📋 Build Matrix

| Platform | Target | Output | Architecture |
|----------|--------|--------|--------------|
| `macos-latest` | `aarch64-apple-darwin` | `.dmg` | Apple Silicon |
| `macos-latest` | `x86_64-apple-darwin` | `.dmg` | Intel Mac |
| `macos-latest` | `universal-apple-darwin` | `.dmg` | Universal |
| `windows-latest` | `x86_64-pc-windows-msvc` | `.exe` | Windows x64 |

## ⚡ Benefits of GitHub Actions

### ✅ Advantages:
- **No local setup required** - works from any platform
- **Consistent environment** - same build every time  
- **Automatic caching** - faster subsequent builds
- **Parallel builds** - all platforms simultaneously
- **Free for public repos** - 2000 minutes/month
- **Artifact storage** - 30-day retention
- **Release automation** - tag-triggered releases

### 🔍 Build Times:
- **Initial build**: ~10-15 minutes (compiling dependencies)
- **Subsequent builds**: ~5-8 minutes (with caching)
- **Windows-only**: ~5-7 minutes

## 🚨 Troubleshooting

### Build Failures:
1. **Check the Actions log** for detailed error messages
2. **Common issues:**
   - Dependency version conflicts
   - Tauri configuration errors
   - Missing targets or tools

### Artifact Download Issues:
1. **Artifacts expire after 30 days** - create releases for permanent storage
2. **Large files** may need to be split or compressed
3. **Authentication required** for private repositories

## 🔐 Security Considerations

### Secrets:
- `GITHUB_TOKEN` is automatically provided
- No additional secrets needed for basic builds
- For code signing, add certificates as repository secrets

### Code Signing (Optional):
```yaml
# Add to workflow for signed releases
- name: Code Sign (macOS)
  env:
    APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
    APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
    APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}

- name: Code Sign (Windows)  
  env:
    WINDOWS_CERTIFICATE: ${{ secrets.WINDOWS_CERTIFICATE }}
    WINDOWS_CERTIFICATE_PASSWORD: ${{ secrets.WINDOWS_CERTIFICATE_PASSWORD }}
```

## 📊 Monitoring Builds

### GitHub Actions Dashboard:
- View build status, logs, and artifacts
- Set up notifications for build failures
- Monitor build times and success rates

### Status Badges:
Add to README.md:
```markdown
[![Build Installers](https://github.com/username/repo/actions/workflows/build-installers.yml/badge.svg)](https://github.com/username/repo/actions/workflows/build-installers.yml)
```

## 🎯 Next Steps

1. **Push to GitHub** to trigger first build
2. **Test downloads** from different platforms
3. **Create a release** by pushing a version tag
4. **Set up notifications** for build status
5. **Consider code signing** for production releases

---

**Result**: Professional installers for both macOS and Windows, built automatically in the cloud! 🎉
