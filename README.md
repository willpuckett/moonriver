# Moonriver 🌙

<img src="docs/public/logo.svg" align="right" width="200" alt="Moonriver Logo" />

A terminal-based console for connecting to and interacting with **Klipper**
instances via the **Moonraker** WebSocket API.

**Moonriver** provides a fast, efficient, and color-coded way to monitor and
control your 3D printer, all from the comfort of your command line. Built
entirely in **Rust** for speed and reliability.

In the tradition of [klipper-repl](https://github.com/Annex-Engineering/klipper_estimator) and [krui](https://github.com/ballaswag/krui), Moonriver brings command-line control to Klipper 3D printers.

---

## ✨ Features

**Moonriver** aims to bring the core functionality of a Klipper interface
directly to your terminal, similar in spirit to a command-line Read-Eval-Print
Loop (REPL).

### 🚀 Connectivity & Performance

- **Moonraker WebSocket Integration** — Connects directly to a remote Moonraker
  instance using its WebSocket API, providing real-time data without polling
- **Built with Rust** — Leverages Rust's memory safety and performance
  characteristics for a fast, low-latency experience
- **Simple Configuration** — Easily configure the Moonraker host address and API
  key via a simple configuration file or command-line arguments
- **Automatic Reconnection** — Reconnects automatically if Klipper restarts or is unavailable
- **Scripting Support** — Run commands non-interactively for automation
- **Multiple Commands** — Execute multiple G-Code commands per line using `,` as a separator
- **Tab Autocompletion** — Tab completion for user-defined macros
- **Multiple Printers** — Control multiple printers via
  [GNU Parallel](https://www.gnu.org/software/parallel/)

### 💻 Terminal Experience

- **Rich Terminal Coloring** — Color-coded output categorizes different types
  of information (green for G-code responses, yellow for warnings, red for
  errors) for easy readability at a glance
- **Interactive Command Line** — Familiar, interactive command-line interface
  for sending commands
- **Command History** — Access and reuse previous commands using the up and
  down arrow keys
- **Syntax Highlighting** — Syntax highlighting for both G-Code and user-defined macros

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
- **Emergency Stop** — M112 emergency stop processing

---

## 🛠️ Usage

### Running interactively

To get started, simply run the `moonriver` executable:

```bash
moonriver --host <moonraker-url> --port <port>
```

Once connected, you can type your G-code commands:

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
