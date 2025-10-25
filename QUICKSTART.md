# Quick Start Guide

This guide will help you get started with Moonriver quickly.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/moonriver.git
cd moonriver

# Build the project
cargo build --release

# The binary will be at target/release/moonriver
```

### Install with Cargo

```bash
cargo install moonriver
```

## Basic Usage

### Interactive Mode

Connect to your Moonraker instance and start the interactive console:

```bash
moonriver --host 192.168.1.100 --port 7125
```

Once connected, you can type G-code commands:

```
> G28
> M105
> M104 S200
> GET_POSITION
```

### Scripting Mode

Execute commands without entering interactive mode:

```bash
# Single command
moonriver --host 192.168.1.100 --port 7125 G28

# Multiple commands (separated by commas)
moonriver --host 192.168.1.100 --port 7125 "G28, M105, G0 Z10"
```

### Command Line Options

- `--host <HOST>`: Moonraker host address (default: localhost)
- `--port <PORT>`: Moonraker port (default: 7125)
- `--api-key <KEY>`: API key for authentication (if required)

## Features

### Tab Completion

Press TAB while typing to autocomplete G-code commands and macros.

### Command History

Use the UP and DOWN arrow keys to navigate through command history.

### Multiple Commands

Use commas to send multiple commands in one line:

```
> G28, M105, SET_HEATER_TEMPERATURE HEATER=extruder TARGET=200
```

### Emergency Stop

Type `M112` to send an emergency stop to your printer:

```
> M112
```

### Syntax Highlighting

Commands are color-coded:
- **Green**: G-code commands (G0, M105, etc.)
- **Cyan**: Klipper macros
- **Yellow**: Warnings
- **Red**: Errors

## Tips

1. **Save Commands**: Your command history is automatically saved to `~/.moonriver_history`
2. **Exit**: Type `exit` or press Ctrl+D to quit
3. **Interrupt**: Press Ctrl+C to cancel current input (doesn't exit)
4. **Case Insensitive**: G-code commands are case-insensitive (G28 = g28)

## Common Commands

### Homing
```
G28        # Home all axes
G28 X Y    # Home X and Y only
```

### Temperature
```
M105                              # Get current temperature
M104 S200                         # Set extruder temperature
M140 S60                          # Set bed temperature
M109 S200                         # Set and wait for extruder temperature
M190 S60                          # Set and wait for bed temperature
```

### Movement
```
G0 X100 Y100 Z10 F3000   # Move to position
G28                       # Home all axes
G90                       # Absolute positioning
G91                       # Relative positioning
```

### Klipper Specific
```
GET_POSITION              # Get current position
QUERY_ENDSTOPS            # Check endstop status
FIRMWARE_RESTART          # Restart firmware
STATUS                    # Get printer status
```

## Troubleshooting

### Connection Issues

If you can't connect:

1. Verify Moonraker is running: `curl http://<host>:<port>/printer/info`
2. Check the host and port are correct
3. Ensure there's no firewall blocking the connection
4. Try the IP address instead of hostname

### Command Not Working

- Check if the command is valid G-code or a defined macro
- Use `HELP` command to see available commands
- Check Moonraker logs for errors

## Next Steps

- Read the full [README](README.md) for more features
- Check [CONTRIBUTING](CONTRIBUTING.md) to contribute
- See [examples/](examples/) for usage examples
- Report issues on [GitHub Issues](https://github.com/yourusername/moonriver/issues)
