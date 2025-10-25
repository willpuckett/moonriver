# Quick Start

Get up and running with Moonriver in minutes.

## Installation

```bash
cargo install moonriver
```

## Connect to Your Printer

```bash
moonriver --host 192.168.1.100 --port 7125
```

## Common Commands

### Homing

```bash
# Home all axes
> G28

# Home specific axes
> G28 X Y
> G28 Z
```

### Temperature Control

```bash
# Check current temperature
> M105

# Set extruder temperature
> M104 S200

# Set bed temperature
> M140 S60

# Set and wait for temperature
> M109 S200  # Extruder
> M190 S60   # Bed
```

### Movement

```bash
# Set to absolute positioning
> G90

# Move to position
> G0 X100 Y100 Z10 F3000

# Set to relative positioning
> G91

# Move relative
> G0 X10 Y10
```

### Klipper-Specific Commands

```bash
# Get current position
> GET_POSITION

# Check endstop status
> QUERY_ENDSTOPS

# Get printer status
> STATUS

# Firmware restart
> FIRMWARE_RESTART

# Emergency stop
> M112
```

## Multiple Commands

Use commas to execute multiple commands:

```bash
> G28, M105, GET_POSITION
```

## Scripting Mode

Execute commands without interactive mode:

```bash
# Single command
moonriver --host 192.168.1.100 --port 7125 G28

# Multiple commands
moonriver --host 192.168.1.100 --port 7125 "G28, M105"
```

## Shell Aliases

Create shortcuts for your printers:

```bash
# Add to ~/.bashrc or ~/.zshrc
alias moon='moonriver --host 192.168.1.100 --port 7125'
alias moon-test='moonriver --host test.local --port 7125'
```

Use them:
```bash
moon
moon-test
```

## Tips & Tricks

### Tab Completion

Press `Tab` while typing to autocomplete commands:

```bash
> G2<Tab>
G28  G29
```

### Command History

Use arrow keys to navigate history:
- `↑` - Previous command
- `↓` - Next command

### Search History

Press `Ctrl+R` and start typing to search history.

### Multiple Printers

Use with GNU Parallel:

```bash
# Home all printers
parallel moonriver --host {} --port 7125 G28 ::: printer1 printer2 printer3

# Check all temperatures
parallel moonriver --host {} --port 7125 M105 ::: printer{1..5}.local
```

## Next Steps

- [Interactive Mode](/guide/interactive-mode) - Master the REPL
- [Scripting Mode](/guide/scripting-mode) - Automate workflows
- [Multiple Printers](/guide/multiple-printers) - Manage printer farms
