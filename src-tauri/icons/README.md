# Application Icons

This directory contains the application icons for BEAR LLM AI.

## Required Icon Files

For proper multi-platform support, the following icon files are needed:

### Windows
- `icon.ico` - Windows application icon (256x256, 128x128, 64x64, 48x48, 32x32, 16x16)

### macOS
- `icon.icns` - macOS application icon (contains multiple resolutions)

### Linux / Web
- `32x32.png` - 32x32 PNG
- `128x128.png` - 128x128 PNG
- `128x128@2x.png` - 256x256 PNG (HiDPI)
- `icon.png` - 512x512 PNG (base icon)

## Icon Specifications

**Format Requirements**:
- PNG files: True color with alpha transparency
- ICO file: Multi-resolution (16, 32, 48, 64, 128, 256)
- ICNS file: Multi-resolution for macOS

**Design Guidelines**:
- Simple, recognizable design
- Works well at small sizes (16x16)
- Sufficient contrast for both light and dark backgrounds
- Represents BEAR (Bridge for Expertise, Audit & Research)

## Generating Icons

You can use tools like:
- **png2icons**: Convert PNG to ICO/ICNS
- **@tauri-apps/tauricon**: Official Tauri icon generator
- **Online tools**: CloudConvert, ICOConvert

Example using Tauri icon generator:
```bash
npm install -g @tauri-apps/tauricon
tauricon icon.png
```

This will generate all required icon formats from a single 512x512 PNG source.
