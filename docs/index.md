---
layout: home

hero:
  name: "Moonriver"
  text: "Terminal Console for Klipper"
  tagline: Fast, efficient, and color-coded printer control from the command line
  image:
    src: /logo.svg
    alt: Moonriver
  actions:
    - theme: brand
      text: Get Started
      link: /guide/getting-started
    - theme: alt
      text: Quick Start
      link: /guide/quick-start
    - theme: alt
      text: View on GitHub
      link: https://github.com/willpuckett/moonriver

features:
  - icon: �️
    title: Terminal User Interface (TUI)
    details: Full-featured dashboard with real-time monitoring, interactive controls, and mouse support
  - icon: 🌡️
    title: Interactive Temperature & Fan Control
    details: Compact display with clickable setpoints and fan speeds - click any value to edit and press Enter to apply
  - icon: 📍
    title: Interactive Position Control
    details: Click-to-move position bar with coordinates and home button for quick axis control
  - icon: �🚀
    title: Real-Time WebSocket Connection
    details: Connects directly to Moonraker using WebSocket API for real-time data without polling
  - icon: ⚡
    title: Built with Rust 2024
    details: Leverages Rust's memory safety and performance with modern edition features for maximum speed
  - icon: 🎨
    title: Rich Terminal Coloring
    details: Color-coded output for easy readability - green for responses, yellow for warnings, red for errors
  - icon: 📝
    title: Interactive Command Line
    details: Familiar REPL interface with command history, tab completion, and syntax highlighting
  - icon: 🖨️
    title: Full Klipper Control
    details: Submit G-code, execute macros, monitor temperatures, control fans, and manage your 3D printer
  - icon: �
    title: File Browser with Mouse Support
    details: Browse available G-code files, scroll with mouse wheel, click to select, and start prints instantly
  - icon: 🔧
    title: Scriptable
    details: Use in scripts or with GNU Parallel for managing multiple printers simultaneously
  - icon: 🚨
    title: Emergency Stop
    details: Quick Ctrl+C emergency stop support for immediate printer shutdown
  - icon: �
    title: Dynamic Sensor Discovery
    details: Automatically detects all temperature sensors, fans, and MCU temps - no manual configuration needed
---

## Quick Example

::: code-group

```bash [TUI Mode (Default)]
# Launch the Terminal User Interface
moonriver --host 192.168.1.100 --port 7125

# Interactive dashboard with:
# - Temperature bar (click setpoints to edit)
# - Fan speed control (click percentages to edit)
# - Position bar (click coordinates to move)
# - Job browser with mouse scroll and click (press j)
# - Console (press c)
# - Global temperature/position toggles (t/l keys)
# - Mouse support throughout
```

```bash [Interactive Mode]
# Connect to your printer
moonriver --repl --host 192.168.1.100 --port 7125

# Start typing commands
> G28
> M105
> SET_HEATER_TEMPERATURE HEATER=extruder TARGET=200
```

```bash [Scripting Mode]
# Execute commands directly
moonriver --host printer.local --port 7125 G28

# Multiple commands with comma separator
moonriver --host printer.local --port 7125 "G28, M105, GET_POSITION"
```

```bash [Multiple Printers]
# Use with GNU Parallel
parallel -j 0 moonriver --host {} --port 7125 G28 ::: printer1 printer2 printer3
```

:::

## Why Moonriver?

Moonriver brings the power of Klipper control directly to your terminal, perfect
for:

- **System Administrators**: Manage printers via SSH without a GUI
- **Power Users**: Faster workflows with keyboard-driven interface
- **Automation**: Integrate printer control into scripts and workflows
- **Multiple Printers**: Efficiently manage printer farms
- **Debugging**: Quick access to printer status and commands

## Installation

::: code-group

```bash [Cargo]
cargo install moonriver
```

```bash [From Source]
git clone https://github.com/willpuckett/moonriver.git
cd moonriver
cargo build --release
```

```bash [Cargo Binstall]
cargo binstall moonriver
```

:::

## Community

- [GitHub Discussions](https://github.com/willpuckett/moonriver/discussions) -
  Ask questions and share ideas
- [Issue Tracker](https://github.com/willpuckett/moonriver/issues) - Report bugs
  and request features
- [Contributing Guide](/contributing/development) - Learn how to contribute

## License

Moonriver is
[MIT licensed](https://github.com/willpuckett/moonriver/blob/main/LICENSE-MIT).
