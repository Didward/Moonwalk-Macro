# Releases

This folder contains pre-built executables for different platforms.

## Available Builds

### Windows (x86_64)
- `moonwalk-macros-windows-x64.exe` - Windows 64-bit executable
- Requires: Windows 10 or later
- Features: Full functionality with Windows API integration

### macOS (x86_64 & ARM64)
- `moonwalk-macros-macos-intel` - Intel-based Macs
- `moonwalk-macros-macos-apple` - Apple Silicon Macs (M1/M2/M3)
- Requires: macOS 10.15 or later
- Note: May require "Allow apps from unidentified developers"

### Linux (x86_64)
- `moonwalk-macros-linux-x64` - Linux 64-bit executable
- Requires: Modern Linux distribution with GUI support
- Dependencies: X11 or Wayland display server

## Installation

1. Download the appropriate executable for your platform
2. Make it executable (Linux/macOS): `chmod +x filename`
3. Run the application
4. On first run, configure your settings and hotkeys

## Security Notes

- Windows: May trigger Windows Defender - add exclusion if needed
- macOS: Right-click → "Open" to bypass Gatekeeper on first run
- Linux: Ensure the binary has execute permissions

## Build Information

- Version: 0.1.0
- Built with: Rust 2021 edition
- GUI Framework: egui with native backend
- Cross-compiled for maximum compatibility
