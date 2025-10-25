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
      link: https://github.com/yourusername/moonriver

features:
  - icon: 🚀
    title: Real-Time WebSocket Connection
    details: Connects directly to Moonraker using WebSocket API for real-time data without polling
  - icon: ⚡
    title: Built with Rust
    details: Leverages Rust's memory safety and performance for a fast, low-latency experience
  - icon: 🎨
    title: Rich Terminal Coloring
    details: Color-coded output for easy readability - green for responses, yellow for warnings, red for errors
  - icon: 📝
    title: Interactive Command Line
    details: Familiar REPL interface with command history, tab completion, and syntax highlighting
  - icon: 🖨️
    title: Full Klipper Control
    details: Submit G-code, execute macros, monitor temperatures, and manage your 3D printer
  - icon: 🔧
    title: Scriptable
    details: Use in scripts or with GNU Parallel for managing multiple printers simultaneously
  - icon: 🚨
    title: Emergency Stop
    details: Quick M112 emergency stop support for immediate printer shutdown
  - icon: 📦
    title: Simple Configuration
    details: Easy setup via command-line arguments or configuration file
---

## Quick Example

::: code-group

```bash [Interactive Mode]
# Connect to your printer
moonriver --host 192.168.1.100 --port 7125

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

Moonriver brings the power of Klipper control directly to your terminal, perfect for:

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
git clone https://github.com/yourusername/moonriver.git
cd moonriver
cargo build --release
```

```bash [Cargo Binstall]
cargo binstall moonriver
```

:::

## Community

- [GitHub Discussions](https://github.com/yourusername/moonriver/discussions) - Ask questions and share ideas
- [Issue Tracker](https://github.com/yourusername/moonriver/issues) - Report bugs and request features
- [Contributing Guide](/contributing/development) - Learn how to contribute

## License

Moonriver is [MIT licensed](https://github.com/yourusername/moonriver/blob/main/LICENSE-MIT).
