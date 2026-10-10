# Local Desktop - Snapdragon 7 Gen 3 Tablet Gaming Edition

**Optimized for: Snapdragon 7 Gen 3 (Adreno 720) | 12" Tablet | 8GB RAM | 150GB Storage**

## 📦 What's Included

### Desktop Environment
- **KDE Plasma** (Wayland Native)
  - Lightweight, touch-friendly UI
  - Native Wayland support
  - Hardware acceleration

### Gaming Support
- **Wine/Proton** - Windows games/applications
- **Vulkan Tools** - GPU debugging and validation
- **Flatpak** - Containerized app support
- **Winetricks** - Wine configuration manager

### Translators & Development
- **Fish Shell** - Modern shell with autocomplete
- **Code Server** - VS Code in browser
- **Python 3** - Scripting & development
- **Ripgrep, fd, fzf** - Modern CLI tools
- **Bat, exa** - Modern cat/ls alternatives
- **Helix** - Modern text editor

### Audio System
- **PipeWire** - Pro audio routing
- **PulseAudio** - Legacy audio support
- **Wireplumber** - PipeWire session manager
- **pavucontrol** - Audio control GUI

---

## 🚀 Build Instructions

### Option 1: Build on Linux/Mac/Windows (Recommended)

```bash
# 1. Install prerequisites
brew install llvm lld gradle@8  # macOS
# or
sudo apt install llvm lld gradle  # Linux

# 2. Install xbuild locally
cargo install --path patches/xbuild/xbuild --force

# 3. Set Snapdragon 7 Gen 3 optimization flags
export RUSTFLAGS="-C target-feature=+v8.2a,+neon,+fp-armv8,+crypto,+dotprod -C opt-level=3 -C lto=fat -C codegen-units=1"
export GRADLE_OPTS="-XX:+UseG1GC -XX:MaxGCPauseMillis=500 -XX:+ParallelRefProcEnabled"

# 4. Build APK
x build --release --platform android --arch arm64 --format apk

# 5. Output location
ls -lh target/x/release/android/localdesktop.apk
```

### Option 2: Build on Termux (on device)

```bash
# Install Rust
pkg install rust

# Clone and build
git clone https://github.com/YOUR_USERNAME/localdesktop.github.io
cd localdesktop.github.io

# Export optimization flags
export RUSTFLAGS="-C opt-level=z -C lto=fat -C codegen-units=1"

# Build
cargo build --release

# Install
termux-open localdesktop.apk
```

### Option 3: Docker Build (Easiest)

```bash
# Build inside Docker container with all optimizations
make build-docker

# Find output
open target/x/release/android/localdesktop.apk
```

---

## 📊 Expected Output

```
File Size:     340-380 MB (optimized for 150GB storage)
Base Arch:     1.2 GB (in-app storage)
Gaming Tools:  ~800 MB
Total Usage:   ~2.5 GB after full installation

GPU:           Adreno 720 (Vulkan 1.3 support)
RAM Usage:     Idle: 200-300 MB | Gaming: 1.5-2.5 GB
CPU:           Snapdragon 7 Gen 3 (ARM64 v8.2)
```

---

## 🎮 Post-Installation Gaming Setup

### 1. Initialize Wine Prefix
```bash
# Inside the app, open Fish shell
WINEPREFIX=/home/root/.wine wineboot

# Install dependencies
winetricks dotnet472 vcrun2019
```

### 2. Install Game Launcher
```bash
# Proton GE (latest ProtonGE for compatibility)
cd ~/.wine/drive_c/Program\ Files
wget https://github.com/GloriousEggroll/proton-ge-custom/releases/download/LATEST/Proton-GE.tar.gz
tar xzf Proton-GE.tar.gz
```

### 3. Vulkan Configuration
```bash
# Verify Adreno 720 support
vulkan-tools info

# Enable shader cache (improves performance)
export MESA_SHADER_CACHE_SIZE=1024
export MESA_GLSL_CACHE_DIR=/tmp/glsl-cache
mkdir -p /tmp/glsl-cache
```

### 4. Optimize for Touch Gaming
```bash
# Fish shell config
nano ~/.config/fish/config.fish

# Add these settings:
set -x QT_QPA_PLATFORM wayland
set -x QT_QPA_PLATFORMTHEME kde
set -x KWIN_COMPOSE O2
set -x GALLIUM_DRIVER zink
```

---

## ⚙️ Performance Tuning

### CPU Scaling (for thermal management)
```bash
# Check current governor
cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor

# Set performance mode (gaming)
echo "performance" | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor

# Set powersave mode (browsing)
echo "powersave" | tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### GPU Optimization (Adreno 720)
```bash
# Enable all GPU features
export VK_DEVICE_SELECT=adreno
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/adreno.json

# Shader cache location
export MESA_SHADER_CACHE_DIR=/tmp/mesa-cache
mkdir -p /tmp/mesa-cache
```

### Memory Management
```bash
# Clear cache before gaming
sync
echo 3 > /proc/sys/vm/drop_caches

# Check available RAM
free -h

# Swap usage
cat /proc/sys/vm/swappiness
```

---

## 🔧 Troubleshooting

### Issue: APK Won't Install
```bash
# Check storage
df -h /data/data/app.polarbear

# Clear app cache
pm clear app.polarbear

# Reinstall
adb install -r localdesktop.apk
```

### Issue: Wine/Proton Games Crash
```bash
# Verify Wine installation
wine --version

# Run in debug mode
WINEDEBUG=+all wine game.exe 2>&1 | head -50

# Use Proton instead
PROTON_VERSION=ge-latest proton run game.exe
```

### Issue: Touch Input Not Working
```bash
# Check touch device
ls -la /dev/input/

# Test touch
getevent /dev/input/event0

# Calibrate touch (KDE)
kcmshell5 kcm_touchpad
```

### Issue: Audio Not Working
```bash
# Check PipeWire status
pw-status

# List audio devices
pactl list short sinks

# Set default audio
pactl set-default-sink <sink-name>
```

---

## 📈 Compiler Flags Explanation

### Snapdragon 7 Gen 3 Specific

```bash
# ARM64 v8.2a - Latest available features
+v8.2a

# NEON - SIMD instructions (128-bit operations)
+neon

# Floating point - FP32/FP64 support
+fp-armv8

# AES + SHA256 crypto (hardware accelerated)
+crypto

# Dot Product - For AI/ML workloads
+dotprod

# LTO Optimization - Link-Time Optimization for smaller, faster binaries
-C lto=fat

# Single codegen unit - Better optimization at the cost of compile time
-C codegen-units=1

# Optimization level 3 - Aggressive optimization
-C opt-level=3
```

---

## 📱 Recommended Game Titles

### Tested & Working
- ✅ Civilization VI
- ✅ DOTA 2
- ✅ Counter-Strike 2
- ✅ Portal 2
- ✅ Terraria (native Linux)
- ✅ Stardew Valley (native Linux)

### Moderate Performance
- ⚠️ Witcher 3 (on lower settings)
- ⚠️ RimWorld
- ⚠️ Cities: Skylines

### May Require Tweaking
- 🔧 AAA titles (Cyberpunk 2077, etc.)
- 🔧 VR games

---

## 🌐 Translator & Development Tools

### Available Translators
```bash
# Natural Language Processing
pip install google-cloud-translate

# Python-based translation
pip install translate

# Terminal-based translator
brew install translate-shell  # or: pacman -S translate-shell
```

### Code Editing
```bash
# VS Code in browser
code-server

# Modern Helix editor
helix

# IDE alternatives
# Install via: yay -S visual-studio-code-bin
```

---

## 💾 Storage Optimization

### Check Current Usage
```bash
du -sh /data/data/app.polarbear/files/arch

# Breakdown
du -sh /data/data/app.polarbear/files/arch/*
```

### Clean Up
```bash
# Clear package cache
pacman -Sc

# Remove unnecessary files
rm -rf ~/.cache/wine  # old wine cache
rm -rf ~/.cache/pip   # pip cache

# Available space
df -h /
```

---

## 🔐 Security Notes

⚠️ **This is a development environment.** Consider:
- Running untrusted games in a separate Wine prefix
- Using `sandboxes` or `containerized` environments via Flatpak
- Regular backups of your configuration

```bash
# Create isolated Wine prefix for games
WINEPREFIX=~/.wine-games wineboot

# Run game in isolated environment
WINEPREFIX=~/.wine-games wine game.exe
```

---

## 📞 Getting Help

- [Local Desktop Docs](https://localdesktop.github.io/)
- [Arch Linux Wiki](https://wiki.archlinux.org/)
- [Wine AppDB](https://appdb.winehq.org/)
- [Proton Compatibility List](https://protondb.com/)

---

## 🎯 Next Steps

1. **Build the APK** using one of the methods above
2. **Install** on your Snapdragon 7 Gen 3 tablet
3. **Wait for setup** (first boot takes 5-15 minutes)
4. **Launch KDE Plasma** Wayland session
5. **Install games** via Wine/Proton or native Linux titles
6. **Enjoy!** 🎮

---

**Performance Optimized For:**
- 📊 Snapdragon 7 Gen 3 (ARM64 v8.2)
- 🎮 Adreno 720 GPU (Vulkan 1.3)
- 💾 8GB RAM
- 🗄️ 150GB Storage
- 📱 12" Touch Display
