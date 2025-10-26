# Application Icons

This directory contains the application icons for **BEAR LLM AI**.

## Required Icon Files

All required icon files are already included for full multi-platform support.

### Windows
- `icon.ico` — Windows application icon (256x256, 128x128, 64x64, 48x48, 32x32)

### macOS
- `icon.icns` — macOS application icon (contains multiple resolutions)

### Linux / Web
- `32x32.png` — Small icon for taskbar or notifications  
- `128x128.png` — Standard application icon  
- `128x128@2x.png` — High-DPI (256x256) application icon  
- `icon.png` — Base icon (512x512 PNG)

---

## Icon Specifications

**Format Requirements**
- PNG files: True color with alpha transparency  
- ICO file: Multi-resolution (16, 32, 48, 64, 128, 256)  
- ICNS file: Multi-resolution for macOS  

**Design Guidelines**
- Simple, recognizable design  
- Works well at small sizes (16x16)  
- High contrast on both light and dark backgrounds  
- Represents **BEAR (Bridge for Expertise, Audit & Research)**  

---

## Optional: Regenerating Icons

If you need to regenerate icons from the main `icon.png` source:

```bash
npm install -g @tauri-apps/tauricon
tauricon icon.png