# What is Moonriver?

**Moonriver** is a terminal-based console for connecting to and interacting with **Klipper** instances via the **Moonraker** WebSocket API.

## Overview

Moonriver provides a fast, efficient, and color-coded way to monitor and control your 3D printer, all from the comfort of your command line. Built entirely in **Rust** for speed and reliability.

## The Problem

While web interfaces like Mainsail and Fluidd are excellent, there are scenarios where a terminal-based solution is preferable:

- **Remote Access**: SSH into your printer's host without needing X forwarding or VNC
- **Automation**: Integrate printer commands into scripts and workflows
- **Speed**: Keyboard-driven interface is often faster than clicking through a web UI
- **Multiple Printers**: Manage many printers simultaneously using tools like GNU Parallel
- **Resource Usage**: Minimal overhead compared to running a web browser

## Key Features

### 🚀 Connectivity & Performance

- **Real-Time WebSocket**: Direct connection to Moonraker's WebSocket API
- **No Polling**: Efficient real-time updates without constant HTTP requests
- **Rust Performance**: Fast, memory-safe implementation
- **Auto-Reconnect**: Automatically reconnects if Klipper restarts

### 💻 Terminal Experience

- **Rich Coloring**: Color-coded output for different message types
- **Command History**: Navigate previous commands with arrow keys
- **Tab Completion**: Autocomplete G-code commands and macros
- **Syntax Highlighting**: Visual feedback as you type

### 🖨️ Printer Control

- **G-code Execution**: Submit any G-code command
- **Macro Support**: Execute user-defined Klipper macros
- **Status Monitoring**: Real-time temperature and position data
- **Emergency Stop**: Quick M112 support

## Use Cases

### System Administration
Manage printers via SSH without needing a GUI:
```bash
ssh pi@printer.local
moonriver --host localhost --port 7125
```

### Automation Scripts
Integrate printer control into maintenance scripts:
```bash
#!/bin/bash
moonriver --host printer.local --port 7125 "G28, M105"
```

### Printer Farms
Control multiple printers efficiently:
```bash
parallel moonriver --host {} --port 7125 G28 ::: printer{1..10}.local
```

### Development & Debugging
Quick access to printer internals:
```bash
moonriver --host test-printer --port 7125
> GET_POSITION
> QUERY_ENDSTOPS
> FIRMWARE_RESTART
```

## Architecture

Moonriver is built with modern async Rust:

- **tokio**: Async runtime for non-blocking I/O
- **tokio-tungstenite**: WebSocket client implementation
- **rustyline**: Interactive line editing with history
- **clap**: Command-line argument parsing
- **colored**: Terminal color output

## Comparison with Other Tools

| Feature | Moonriver | Mainsail/Fluidd | SSH + G-code |
|---------|-----------|-----------------|--------------|
| Terminal-based | ✅ | ❌ | ✅ |
| Real-time updates | ✅ | ✅ | ❌ |
| Tab completion | ✅ | ❌ | ❌ |
| Command history | ✅ | ❌ | ❌ |
| Syntax highlighting | ✅ | ✅ | ❌ |
| Scriptable | ✅ | ❌ | ✅ |
| Multiple printers | ✅ | ❌ | ⚠️ |
| Resource usage | Low | Medium | Low |

## Next Steps

- [Getting Started](/guide/getting-started) - Install and configure Moonriver
- [Quick Start](/guide/quick-start) - Jump right in with examples
- [Interactive Mode](/guide/interactive-mode) - Learn the REPL interface
