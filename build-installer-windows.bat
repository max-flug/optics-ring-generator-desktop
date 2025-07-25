@echo off
REM Optics Ring Generator Professional - Windows Installer Build Script
REM This script builds the Windows installer with all dependencies

echo 🚀 Building Optics Ring Generator Professional - Windows Installer
echo ==================================================================

echo 🪟 Building Windows installer (NSIS)...
echo Building installer with all dependencies included...

call npm run tauri build

if %ERRORLEVEL% == 0 (
    echo ✅ Windows installer built successfully!
    echo 📦 Installer location: src-tauri\target\release\bundle\nsis\
    echo.
    echo 🎉 Build complete!
    echo ==================================================================
    echo 📋 Installation Instructions:
    echo.
    echo   1. Navigate to: src-tauri\target\release\bundle\nsis\
    echo   2. Run the .exe installer file
    echo   3. Follow the installation wizard
    echo   4. Launch from Start Menu or Desktop shortcut
    echo.
    echo ℹ️  All dependencies ^(WebView2, Visual C++ Redistributables^) are included!
    echo.
    pause
) else (
    echo ❌ Build failed! Please check the error messages above.
    pause
    exit /b 1
)
