# Getting Started

This guide will help you install and configure Moonriver.

## Prerequisites

Before installing Moonriver, ensure you have:

- A working Klipper installation with Moonraker
- Rust and Cargo installed (for building from source)
- Network access to your Moonraker instance

## Installation

### Method 1: Install from crates.io

The simplest method is to install directly from crates.io:

```bash
cargo install moonriver
```

This will download, compile, and install Moonriver to your Cargo bin directory
(usually `~/.cargo/bin/`).

### Method 2: Using cargo-binstall

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
installed:

```bash
cargo binstall moonriver
```

This downloads pre-built binaries if available, which is faster than compiling.

### Method 3: Build from Source

For the latest development version:

```bash
# Clone the repository
git clone https://github.com/willpuckett/moonriver.git
cd moonriver

# Build in release mode
cargo build --release

# The binary will be at target/release/moonriver
# Optionally, copy it to your PATH
sudo cp target/release/moonriver /usr/local/bin/
```

## Verify Installation

Check that Moonriver is installed correctly:

```bash
moonriver --version
```

You should see output like:

```
moonriver 0.1.0
```

## First Connection

Connect to your printer for the first time:

```bash
moonriver --host <your-printer-ip> --port 7125
```

For example:

```bash
moonriver --host 192.168.1.100 --port 7125
```

If successful, you'll see:

```
Connecting to ws://192.168.1.100:7125/websocket...
Connected to Moonraker!

🌙 Moonriver - Klipper Console 🌙
Type your G-code commands below. Use Ctrl+D or 'exit' to quit.
Use ',' to separate multiple commands on one line.
Type 'M112' for emergency stop.

>
```

## Basic Usage

Try some basic commands:

```bash
# Home all axes
> G28

# Check temperature
> M105

# Get current position
> GET_POSITION

# Exit
> exit
```

## Troubleshooting

### Cannot Connect

If you see connection errors:

1. **Verify Moonraker is running**:
   ```bash
   curl http://<printer-ip>:7125/printer/info
   ```

2. **Check the port**: Default is 7125, but verify in your Moonraker config

3. **Try IP address**: Use the IP address instead of hostname

4. **Check firewall**: Ensure port 7125 is not blocked

### Command Not Found

If the `moonriver` command isn't found after installation:

1. Ensure `~/.cargo/bin` is in your PATH:
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

2. Or use the full path:
   ```bash
   ~/.cargo/bin/moonriver --host <printer-ip> --port 7125
   ```

## Configuration

For convenience, you can create shell aliases:

```bash
# Add to ~/.bashrc or ~/.zshrc
alias moon='moonriver --host 192.168.1.100 --port 7125'
alias moon-test='moonriver --host test-printer.local --port 7125'
```

Then simply run:

```bash
moon
```

See the [Configuration Guide](/guide/configuration) for more options.

## Next Steps

- [Quick Start](/guide/quick-start) - Common commands and workflows
- [Interactive Mode](/guide/interactive-mode) - REPL features
- [Scripting Mode](/guide/scripting-mode) - Automation and scripts
