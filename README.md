# Moonriver 🌙

[![Crates.io](https://img.shields.io/crates/v/moonriver.svg)](https://crates.io/crates/moonriver)
[![Release](https://github.com/willpuckett/moonriver/actions/workflows/release.yml/badge.svg)](https://github.com/willpuckett/moonriver/actions/workflows/release.yml)
[![Docs](https://github.com/willpuckett/moonriver/actions/workflows/docs.yml/badge.svg)](https://github.com/willpuckett/moonriver/actions/workflows/docs.yml)

<img src="docs/public/logo.svg" align="right" width="200" alt="Moonriver Logo" />

A terminal-based console for connecting to and interacting with **Klipper**
instances via the **Moonraker** WebSocket API.

**Moonriver** provides a fast, efficient, and color-coded way to monitor and
control your 3D printer, all from the comfort of your command line. Built
entirely in **Rust** for speed and reliability.

In the tradition of
[klipper-repl](https://github.com/Annex-Engineering/klipper_estimator) and
[krui](https://github.com/jfoucher/krui), Moonriver brings command-line control
to Klipper 3D printers.

---

## ✨ Features

**Moonriver** offers three modes of operation: **TUI (Terminal User Interface)**, 
**Interactive REPL**, and **Scripting** mode.

### 🖥️ TUI Mode (NEW in v0.2.0!)

The **Terminal User Interface** is now the default mode, providing a comprehensive 
dashboard for monitoring and controlling your 3D printer.

#### Real-Time Monitoring
- **Temperature Display** — Compact single-line display showing all temperatures
  - Live extruder, bed, and chamber temperatures with clickable setpoints
  - Click on any temperature setpoint to edit and press Enter to apply
  - Color-coded by proximity to target: Green (at temp), Yellow (approaching), Cyan (heating/cooling)
  - Shows MCU temperatures and fan speeds with RPM
- **Position Display** — Compact single-line position bar (toggleable)
  - Live X/Y/Z coordinates with homed status indicators (✓/✗)
  - Click on any position value to edit and press Enter to move axis
  - Click the 🏠 Home All button to home all axes
  - Floating point precision for accurate positioning
- **Print Status** — Active print job details including filename, duration, and filament used
- **Connection Status** — Visual feedback of Moonraker connection state

#### Interactive Navigation
- **Tab Navigation** — Switch between views using semantic keys:
  - `m` - Main dashboard with temperatures and job status
  - `c` - Console for GCode commands (coming soon)
  - `p` - Position display with homed status
  - `j` - Print job history browser (coming soon)
  - `h` - Help screen
- **Toggle Panels** — Control visibility with:
  - `t` - Toggle temperature bar on/off
  - `p` - Toggle position bar on/off
- **Mouse Support** — Click to interact with UI elements:
  - Click temperature setpoints to edit target values
  - Click position coordinates to move axes
  - Click 🏠 Home All button to home all axes
  - Click footer tabs to switch views
- **Context-Sensitive Help** — Footer shows available keys for current view

#### Emergency Stop
- Press `Ctrl+C` to trigger emergency stop (M112) and exit
- Press `q` or `Esc` to exit normally

### 🚀 Connectivity & Performance

- **Moonraker WebSocket Integration** — Connects directly to a remote Moonraker
  instance using its WebSocket API, providing real-time data without polling
- **Built with Rust** — Leverages Rust's memory safety and performance
  characteristics for a fast, low-latency experience
- **Simple Configuration** — Easily configure the Moonraker host address and API
  key via a simple configuration file or command-line arguments
- **Automatic Reconnection** — Reconnects automatically if Klipper restarts or
  is unavailable
- **Scripting Support** — Run commands non-interactively for automation
- **Multiple Commands** — Execute multiple G-Code commands per line using `,` as
  a separator
- **Tab Autocompletion** — Tab completion for user-defined macros
- **Multiple Printers** — Control multiple printers via
  [GNU Parallel](https://www.gnu.org/software/parallel/)

### 💻 Terminal Experience

- **Rich Terminal Coloring** — Color-coded output categorizes different types of
  information (green for G-code responses, yellow for warnings, red for errors)
  for easy readability at a glance
- **Interactive Command Line** — Familiar, interactive command-line interface
  for sending commands
- **Command History** — Access and reuse previous commands using the up and down
  arrow keys
- **Syntax Highlighting** — Syntax highlighting for both G-Code and user-defined
  macros

### 🖨️ Klipper & Printer Control

- **G-code Submission** — Directly submit G-code commands to your Klipper
  instance and view immediate responses
- **Configuration File Access** — Easily retrieve and potentially modify Klipper
  configuration files (`printer.cfg`, etc.) directly through the console
- **Printer Status Monitoring** — Display key status information:
  - Tool/Bed Temperatures: Real-time temperature readings and target values
  - Extruder Position/State: Current position and state information
  - Homing/Mainsail Status: Quick access to the printer's operational state
- **Macro Execution** — Send and execute defined Klipper macros instantly
- **Emergency Stop** — Ctrl+C emergency stop (TUI) or M112 (REPL)

---

## 🛠️ Usage

### TUI Mode (Default)

Launch the full Terminal User Interface (this is now the default):

```bash
moonriver --host <moonraker-url> --port <port>
# or simply:
moonriver
```

**Key Bindings:**
- `m` - Main dashboard
- `c` - Console
- `p` - Toggle position bar
- `j` - Jobs
- `h` or `?` - Help
- `q` - Quit
- `Ctrl+C` - Emergency stop
- `t` - Toggle temperature bar

### REPL Mode (Classic Interactive)

To use the classic REPL interface:

```bash
moonriver --repl --host <moonraker-url> --port <port>
```

Once connected, you can type your G-code commands:

```
> G28
> M104 S200
```

### Scripting Mode

Execute commands non-interactively for automation:

```bash
moonriver --command "G28, M104 S200, G1 X100 Y100" --host <moonraker-url>
# or short form:
moonriver -c "G28, M104 S200" --host <moonraker-url>
```

```
> G28
// Printer response: X axis homed
// Printer response: Y axis homed
// Printer response: Z axis homed
> M105
// Printer response: T:20.5 /0.0 B:21.1 /0.0
> GET_TEMP SENSOR=heater_bed
// Printer response: // Current temperature of heater_bed is 21.1
```

### Usage in scripts

You can evaluate one line of G-Code as follows. Note that you can use the `,`
character to incorporate multiple G-Code commands into one line, and that G-Code
is case-insensitive (but generally gets converted to uppercase by Klipper).

```sh
moonriver --host <moonraker-url> --port <port> g28, screws_tilt_calculate
```

### Emergency stop

Typing the command `m112` into `moonriver` will immediately discard the rest of
the command buffer and send an emergency stop signal to Klipper.

---

## � Documentation

**Full documentation is available at: [moonriver.rs](https://moonriver.rs/)**

- [What is Moonriver?](https://moonriver.rs/guide/what-is-moonriver)
- [Getting Started Guide](https://moonriver.rs/guide/getting-started)
- [Quick Start](https://moonriver.rs/guide/quick-start)
- [API Reference](https://moonriver.rs/api/)

---

## 💡 Installation

### Homebrew (macOS)

```bash
brew install moonriver
```

### From crates.io

```bash
cargo install moonriver
```

### Pre-built Binaries

```bash
cargo binstall moonriver
```

### Debian/Ubuntu (.deb packages)

Download the appropriate `.deb` file from the
[releases page](https://github.com/willpuckett/moonriver/releases):

```bash
# Download for your architecture (amd64 or arm64)
wget https://github.com/willpuckett/moonriver/releases/latest/download/moonriver_VERSION_amd64.deb

# Install
sudo dpkg -i moonriver_VERSION_amd64.deb
```

### From Source

```bash
git clone https://github.com/willpuckett/moonriver.git
cd moonriver
cargo build --release
sudo cp target/release/moonriver /usr/local/bin/
```

---

## 🤝 Contributing

We welcome contributions! If you have suggestions or want to report a bug,
please open an issue on GitHub. If you're interested in contributing code, feel
free to submit a Pull Request!
