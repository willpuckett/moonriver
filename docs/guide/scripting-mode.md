# Scripting Mode

Moonriver can be used in non-interactive scripting mode for automation and
integration with other tools.

## Basic Usage

Execute commands without entering interactive mode:

```bash
moonriver --host 192.168.1.100 --port 7125 COMMAND
```

## Single Commands

```bash
# Home the printer
moonriver --host 192.168.1.100 --port 7125 G28

# Check temperature
moonriver --host 192.168.1.100 --port 7125 M105

# Get position
moonriver --host 192.168.1.100 --port 7125 GET_POSITION
```

## Multiple Commands

Use commas to execute multiple commands:

```bash
moonriver --host 192.168.1.100 --port 7125 "G28, M105, GET_POSITION"
```

::: tip Quote the commands when using commas to prevent shell interpretation.
:::

## Shell Scripts

### Basic Script

```bash
#!/bin/bash

HOST="192.168.1.100"
PORT="7125"

# Home and check temperature
moonriver --host "$HOST" --port "$PORT" "G28, M105"

# Heat the extruder
moonriver --host "$HOST" --port "$PORT" "M104 S200"

# Wait a bit
sleep 5

# Check temperature again
moonriver --host "$HOST" --port "$PORT" "M105"
```

### Maintenance Script

```bash
#!/bin/bash
# Daily printer maintenance

PRINTER_HOST="printer.local"
PRINTER_PORT="7125"

echo "Starting daily maintenance..."

# Home all axes
moonriver --host "$PRINTER_HOST" --port "$PRINTER_PORT" G28

# Run bed leveling
moonriver --host "$PRINTER_HOST" --port "$PRINTER_PORT" BED_MESH_CALIBRATE

# Save configuration
moonriver --host "$PRINTER_HOST" --port "$PRINTER_PORT" SAVE_CONFIG

echo "Maintenance complete!"
```

### Pre-Print Script

```bash
#!/bin/bash
# Prepare printer for printing

PRINTER="$1"
EXTRUDER_TEMP="$2"
BED_TEMP="$3"

if [ -z "$PRINTER" ] || [ -z "$EXTRUDER_TEMP" ] || [ -z "$BED_TEMP" ]; then
    echo "Usage: $0 <printer-host> <extruder-temp> <bed-temp>"
    exit 1
fi

echo "Preparing $PRINTER for printing..."

# Home and heat
moonriver --host "$PRINTER" --port 7125 \
    "G28, M104 S$EXTRUDER_TEMP, M140 S$BED_TEMP"

# Wait for temperatures
moonriver --host "$PRINTER" --port 7125 \
    "M109 S$EXTRUDER_TEMP, M190 S$BED_TEMP"

echo "Printer ready!"
```

Usage:

```bash
./pre-print.sh printer.local 200 60
```

## Environment Variables

Create aliases or functions for convenience:

```bash
# In ~/.bashrc or ~/.zshrc

# Environment variables
export MOONRIVER_HOST="192.168.1.100"
export MOONRIVER_PORT="7125"

# Function for quick commands
moon() {
    moonriver --host "$MOONRIVER_HOST" --port "$MOONRIVER_PORT" "$@"
}

# Specific command aliases
alias moon-home='moon G28'
alias moon-temp='moon M105'
alias moon-status='moon STATUS'
```

Usage:

```bash
moon G28
moon-home
moon "G28, M105"
```

## Exit Codes

Moonriver returns standard exit codes:

- `0`: Success
- `1`: Error (connection failed, command failed, etc.)

Use in scripts:

```bash
if moonriver --host printer.local --port 7125 G28; then
    echo "Homing successful"
else
    echo "Homing failed!" >&2
    exit 1
fi
```

## Integration Examples

### Cron Jobs

Schedule regular tasks:

```bash
# crontab -e

# Check printer status every hour
0 * * * * moonriver --host printer.local --port 7125 M105

# Run bed mesh calibration daily at 3 AM
0 3 * * * moonriver --host printer.local --port 7125 "G28, BED_MESH_CALIBRATE"
```

### Make Tasks

```makefile
PRINTER_HOST = 192.168.1.100
PRINTER_PORT = 7125

.PHONY: home temp status prepare

home:
	moonriver --host $(PRINTER_HOST) --port $(PRINTER_PORT) G28

temp:
	moonriver --host $(PRINTER_HOST) --port $(PRINTER_PORT) M105

status:
	moonriver --host $(PRINTER_HOST) --port $(PRINTER_PORT) STATUS

prepare:
	moonriver --host $(PRINTER_HOST) --port $(PRINTER_PORT) "G28, M105"
```

Usage:

```bash
make home
make prepare
```

### CI/CD Integration

```yaml
# .github/workflows/test-printer.yml
name: Test Printer

on:
  schedule:
    - cron: "0 0 * * *"

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - name: Install Moonriver
        run: cargo install moonriver

      - name: Test Printer Connection
        run: |
          moonriver --host ${{ secrets.PRINTER_HOST }} --port 7125 STATUS

      - name: Home Printer
        run: |
          moonriver --host ${{ secrets.PRINTER_HOST }} --port 7125 G28
```

## Tips

### Timeout Handling

Add timeouts to prevent hanging:

```bash
timeout 30 moonriver --host printer.local --port 7125 G28 || {
    echo "Command timed out after 30 seconds"
    exit 1
}
```

### Error Handling

```bash
#!/bin/bash
set -e  # Exit on error

moonriver --host printer.local --port 7125 G28 || {
    echo "Homing failed, attempting recovery..."
    moonriver --host printer.local --port 7125 FIRMWARE_RESTART
    exit 1
}
```

### Logging

```bash
# Log to file
moonriver --host printer.local --port 7125 "G28, M105" >> /var/log/moonriver.log 2>&1

# Log with timestamp
echo "[$(date)] Starting maintenance" >> maintenance.log
moonriver --host printer.local --port 7125 G28 >> maintenance.log 2>&1
```

## Next Steps

- [Multiple Printers](/guide/multiple-printers) - Control many printers
- [Configuration](/guide/configuration) - Advanced configuration
- [Examples](https://github.com/willpuckett/moonriver/tree/main/examples) - More
  script examples
