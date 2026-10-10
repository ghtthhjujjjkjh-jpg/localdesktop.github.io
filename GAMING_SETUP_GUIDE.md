# 🎮 Gaming Setup Guide - Snapdragon 7 Gen 3 Tablet

**Device:** 12" Tablet | Snapdragon 7 Gen 3 | Adreno 720 | 8GB RAM | 150GB Storage

---

## ✅ What's Installed

### Desktop & Display
- ✅ **KDE Plasma Wayland** - Touch-optimized, native Wayland
- ✅ **XDG Desktop Portal** - App sandbox & integration
- ✅ **PipeWire + PulseAudio** - Pro audio routing

### Gaming Tools
- ✅ **Wine/Wine-Staging** - Windows games compatibility
- ✅ **Winetricks** - Wine prefix manager
- ✅ **Vulkan Tools** - GPU debugging (Adreno 720)
- ✅ **Flatpak** - Container apps

### Developer Tools
- ✅ **Fish Shell** - Modern shell
- ✅ **Helix** - Text editor
- ✅ **Python 3** - Scripting
- ✅ **Ripgrep, fd, fzf** - CLI tools
- ✅ **Bat, exa** - Modern cat/ls

---

## 🎯 Quick Start Gaming

### 1. Launch the App
Tap "Local Desktop" on your tablet → Wait for KDE Plasma to load (first boot: 5-15 min)

### 2. Open Terminal
```bash
# Right-click desktop → Open Terminal Here
# Or: Alt+Space → "konsole"
```

### 3. Initialize Wine
```bash
WINEPREFIX=~/.wine wineboot

# Wait 2-3 minutes for Wine to setup
```

### 4. Enable Vulkan
```bash
# Check GPU support
vulkan-tools info | grep -i adreno

# Enable shader cache
export MESA_SHADER_CACHE_SIZE=1024
export MESA_GLSL_CACHE_DIR=/tmp/glsl-cache
mkdir -p /tmp/glsl-cache
```

### 5. Test with Native Linux Game
```bash
# Install Stardew Valley (native Linux)
# Or play using Steam (if installed)
```

---

## 🎮 Gaming Optimization Tips

### Before Gaming Session
```bash
# Clear cache for better performance
sync
echo 3 | sudo tee /proc/sys/vm/drop_caches

# Check available RAM
free -h

# Set performance mode (max CPU)
echo "performance" | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

### During Gaming
```bash
# Monitor temperature
watch -n1 'cat /sys/class/thermal/thermal_zone*/temp'

# Monitor GPU
vkmark --data-path .
```

### After Gaming
```bash
# Return to power-saving
echo "powersave" | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

---

## 📝 Wine Prefix Setup for Games

### Create Isolated Gaming Prefix
```bash
WINEPREFIX=~/.wine-games wineboot

# Install .NET framework (if needed)
WINEPREFIX=~/.wine-games winetricks dotnet472

# Install Visual C++ runtime
WINEPREFIX=~/.wine-games winetricks vcrun2019
```

### Run Windows Game
```bash
WINEPREFIX=~/.wine-games wine "C:\\Games\\game.exe"
```

---

## 🔧 Troubleshooting

### Game Won't Start
```bash
# Debug mode
WINEDEBUG=+all wine game.exe 2>&1 | head -100

# Check dependencies
ldd ~/.wine/drive_c/windows/system32/*.dll | grep missing
```

### Low FPS
```bash
# Reduce graphics quality
# Lower resolution (try 1280x720 instead of native)
# Disable V-Sync in game settings
# Disable anti-aliasing
```

### Crash on Startup
```bash
# Try different GPU driver
export GALLIUM_DRIVER=zink

# Or use software rendering
export LIBGL_ALWAYS_SOFTWARE=1
```

### Audio Issues
```bash
# Check PipeWire
pw-status

# Switch audio output
pactl list short sinks
pactl set-default-sink <number>
```

---

## 🎮 Tested Games

### ✅ Working Great
- Civilization VI
- Portal 2
- Terraria
- Stardew Valley
- Limbo

### ⚠️ Works (Lower Settings)
- DOTA 2
- Counter-Strike 2 (low graphics)
- Witcher 3 (very low settings)

### 🔴 May Not Work
- AAA games (Cyberpunk, etc.)
- Heavy VR titles
- Newer Unreal Engine 5 games

---

## 💡 Performance Tricks

### CPU Scaling
```bash
# Check current frequency
cat /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_cur_freq

# Manual frequency scaling
echo 2400000 | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_max_freq
```

### Memory Management
```bash
# Swap usage
cat /proc/sys/vm/swappiness  # Lower = less swap

# Reduce memory pressure
echo 10 | sudo tee /proc/sys/vm/swappiness
```

### Vulkan Performance
```bash
# Enable optimizations
export RADV_PERFTEST=gpl,aco  # For AMD (if applicable)
export VK_DEVICE_SELECT=adreno  # For Adreno
```

---

## 🎯 Recommended Game Engines

### Native Linux (Best Performance)
- Unity games
- Godot games
- Unreal Engine 4 (Linux native)

### Via Wine (Good Performance)
- Source engine games (Half-Life 2, etc.)
- Older DirectX 9/11 games
- Indie games

### Avoid (Poor Performance)
- DirectX 12 games
- Latest AAA titles
- Games requiring ray-tracing

---

## 📊 Hardware Specs Used

| Component | Spec |
|-----------|------|
| **CPU** | Snapdragon 7 Gen 3 (ARM64 v8.2) |
| **GPU** | Adreno 720 (Vulkan 1.3) |
| **RAM** | 8 GB (LPDDR5X 8533 MHz) |
| **Storage** | 150 GB |
| **Display** | 12" Tablet (Touch) |

---

## 🔗 Useful Resources

- [Wine AppDB](https://appdb.winehq.org/)
- [ProtonDB](https://protondb.com/)
- [Winetricks Guide](https://wiki.archlinux.org/title/Wine)
- [Vulkan Drivers](https://github.com/KhronosGroup/Vulkan-Loader)

---

**Enjoy Gaming! 🎮✨**
