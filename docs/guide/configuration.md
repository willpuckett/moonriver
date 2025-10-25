# Configuration

Learn how to configure Moonriver for your setup.

## Command-Line Options

All configuration can be done via command-line arguments:

```bash
moonriver [OPTIONS] [COMMANDS]...
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `--host <HOST>` | Moonraker host address | `localhost` |
| `--port <PORT>` | Moonraker port | `7125` |
| `--api-key <KEY>` | API key for authentication | None |
| `-h, --help` | Print help information | - |
| `-V, --version` | Print version | - |

### Examples

```bash
# Connect to remote printer
moonriver --host 192.168.1.100 --port 7125

# Use with authentication
moonriver --host printer.local --port 7125 --api-key "your-api-key"

# Execute commands
moonriver --host printer.local --port 7125 G28

# Multiple commands
moonriver --host printer.local --port 7125 "G28, M105"
```

## Configuration File

Create a `moonriver.toml` file for default settings:

```toml
[connection]
host = "192.168.1.100"
port = 7125
api_key = "your-api-key-here"  # Optional
```

::: warning
Configuration file support is planned for a future release. Currently, use command-line options or shell aliases.
:::

## Shell Aliases

The easiest way to save configuration:

### Bash/Zsh

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# Single printer
alias moon='moonriver --host 192.168.1.100 --port 7125'

# Multiple printers
alias moon-ender='moonriver --host ender3.local --port 7125'
alias moon-prusa='moonriver --host prusa.local --port 7125'
alias moon-voron='moonriver --host voron.local --port 7125'

# With API key
alias moon-secure='moonriver --host printer.local --port 7125 --api-key "your-key"'
```

Usage:
```bash
moon
moon-ender
moon-prusa G28
```

### Fish Shell

Add to `~/.config/fish/config.fish`:

```fish
# Single printer
alias moon 'moonriver --host 192.168.1.100 --port 7125'

# Multiple printers
alias moon-ender 'moonriver --host ender3.local --port 7125'
alias moon-prusa 'moonriver --host prusa.local --port 7125'
```

## Environment Variables

Use environment variables for default values:

```bash
# In ~/.bashrc or ~/.zshrc
export MOONRIVER_HOST="192.168.1.100"
export MOONRIVER_PORT="7125"
export MOONRIVER_API_KEY="your-api-key"

# Wrapper function
moon() {
    moonriver --host "${MOONRIVER_HOST}" --port "${MOONRIVER_PORT}" "$@"
}
```

## Shell Functions

Create advanced configurations with shell functions:

### Smart Wrapper

```bash
moon() {
    local host="${MOONRIVER_HOST:-localhost}"
    local port="${MOONRIVER_PORT:-7125}"
    
    # Override with arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --host) host="$2"; shift 2 ;;
            --port) port="$2"; shift 2 ;;
            *) break ;;
        esac
    done
    
    moonriver --host "$host" --port "$port" "$@"
}
```

### Multi-Printer Manager

```bash
# Define printers
declare -A PRINTERS=(
    [ender]="ender3.local:7125"
    [prusa]="prusa.local:7125"
    [voron]="voron.local:7125"
)

# Function to connect to printer by name
moonp() {
    local printer_name="$1"
    shift
    
    if [[ -z "${PRINTERS[$printer_name]}" ]]; then
        echo "Unknown printer: $printer_name"
        echo "Available: ${!PRINTERS[@]}"
        return 1
    fi
    
    local host_port="${PRINTERS[$printer_name]}"
    local host="${host_port%:*}"
    local port="${host_port#*:}"
    
    moonriver --host "$host" --port "$port" "$@"
}

# Usage: moonp ender G28
# Usage: moonp prusa M105
```

## History File

Command history is automatically saved to:
```
~/.moonriver_history
```

### Disable History

```bash
# Remove history file
rm ~/.moonriver_history

# Prevent creation
touch ~/.moonriver_history
chmod 000 ~/.moonriver_history
```

### Share History Across Machines

Symlink to a shared location:

```bash
# Move to shared location
mv ~/.moonriver_history ~/Dropbox/moonriver_history

# Create symlink
ln -s ~/Dropbox/moonriver_history ~/.moonriver_history
```

## Network Configuration

### DNS Resolution

Use hostnames with mDNS:

```bash
moonriver --host printer.local --port 7125
```

Or use IP addresses:

```bash
moonriver --host 192.168.1.100 --port 7125
```

### SSH Tunneling

Access printers behind firewalls:

```bash
# Create SSH tunnel
ssh -L 7125:localhost:7125 pi@printer.local

# In another terminal, connect through tunnel
moonriver --host localhost --port 7125
```

### Firewall Rules

Ensure port 7125 is accessible:

```bash
# On the printer (Raspberry Pi)
sudo ufw allow 7125/tcp
```

## API Authentication

If your Moonraker instance requires authentication:

```bash
moonriver --host printer.local --port 7125 --api-key "your-api-key-here"
```

Generate API keys in Moonraker's web interface or configuration.

## Tips

### Quick Config Test

Test your configuration:

```bash
moonriver --host printer.local --port 7125 STATUS
```

### Profile Switching

Use different shell profiles:

```bash
# ~/.moonriver-home
export MOONRIVER_HOST="home-printer.local"

# ~/.moonriver-work  
export MOONRIVER_HOST="work-printer.local"

# Source the profile you need
source ~/.moonriver-home
moon G28
```

### Config Management

Store configurations in a git repository:

```bash
~/moonriver-configs/
├── home.env
├── work.env
└── lab.env
```

Source as needed:
```bash
source ~/moonriver-configs/home.env
moon
```

## Next Steps

- [Interactive Mode](/guide/interactive-mode) - Use the REPL
- [Scripting Mode](/guide/scripting-mode) - Automate tasks
- [Multiple Printers](/guide/multiple-printers) - Manage many printers
