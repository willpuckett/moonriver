# Interactive Mode

Moonriver's interactive mode provides a powerful REPL (Read-Eval-Print Loop) for controlling your printer.

## Starting Interactive Mode

Launch Moonriver with connection parameters:

```bash
moonriver --host 192.168.1.100 --port 7125
```

You'll see the welcome screen:

```
🌙 Moonriver - Klipper Console 🌙
Type your G-code commands below. Use Ctrl+D or 'exit' to quit.
Use ',' to separate multiple commands on one line.
Type 'M112' for emergency stop.

> 
```

## Command Input

### Basic Commands

Simply type G-code commands or Klipper macros:

```bash
> G28
> M105
> QUERY_ENDSTOPS
```

### Multiple Commands

Separate commands with commas:

```bash
> G28, M105, GET_POSITION
```

All commands execute sequentially.

### Case Insensitivity

G-code commands are case-insensitive:

```bash
> g28      # Same as G28
> m105     # Same as M105
```

Klipper macros preserve their case.

## Tab Completion

Press `Tab` to autocomplete commands:

```bash
> G2<Tab>
G28  G29

> SET_<Tab>
SET_HEATER_TEMPERATURE  SET_PRESSURE_ADVANCE  SET_VELOCITY_LIMIT
```

### Macro Completion

Tab completion includes your custom Klipper macros:

```bash
> PRINT_<Tab>
PRINT_START  PRINT_END  PRINT_PAUSE
```

## Command History

### Navigation

- `↑` - Previous command
- `↓` - Next command
- `Home` - Beginning of line
- `End` - End of line

### Search

Press `Ctrl+R` to search history:

```bash
(reverse-i-search)`temp': M105
```

Type to filter, press `Enter` to execute, or `Esc` to cancel.

### Persistence

Command history is automatically saved to `~/.moonriver_history` and loaded on startup.

## Syntax Highlighting

Commands are color-coded as you type:

- **Green**: G-code commands (G0, M105, etc.)
- **Cyan**: Klipper macros (PRINT_START, etc.)
- **White**: Unknown/unrecognized commands

## Output Coloring

Responses are color-coded for easy readability:

- **Green**: Successful responses, "ok" messages
- **Cyan**: Informational messages
- **Yellow**: Warnings, notices starting with "//"
- **Red**: Errors, messages containing "error" or "!!"

Example:

```bash
> G28
// Homing X Y Z                      (cyan)
ok                                   (green)

> M105
T:20.5 /0.0 B:21.1 /0.0             (cyan)

> INVALID_COMMAND
!! Unknown command: "INVALID_COMMAND" (red)
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `↑` / `↓` | Navigate history |
| `Tab` | Autocomplete |
| `Ctrl+R` | Search history |
| `Ctrl+C` | Cancel current input (doesn't exit) |
| `Ctrl+D` | Exit Moonriver |
| `Ctrl+L` | Clear screen |
| `Home` / `Ctrl+A` | Beginning of line |
| `End` / `Ctrl+E` | End of line |
| `Ctrl+W` | Delete word backward |
| `Ctrl+K` | Delete to end of line |
| `Ctrl+U` | Delete entire line |

## Special Commands

### Exit

Exit Moonriver:

```bash
> exit
```

Or press `Ctrl+D`.

### Emergency Stop

Immediately stop the printer:

```bash
> M112
```

This sends an emergency stop signal and clears the command buffer.

## Real-Time Updates

Moonriver subscribes to printer status updates and displays them automatically:

```bash
> M104 S200
// Setting extruder temperature to 200.0
// T:25.3 /200.0
// T:45.2 /200.0
// T:75.8 /200.0
...
// T:200.1 /200.0
```

## Tips

### Quick Temperature Check

```bash
> M105
```

### Monitor While Heating

Set temperature and watch updates:

```bash
> M104 S200
```

Real-time temperature updates appear automatically.

### Combine Common Tasks

```bash
> G28, M105, GET_POSITION, QUERY_ENDSTOPS
```

### Clear Screen

Press `Ctrl+L` to clear the terminal while keeping your connection.

## Next Steps

- [Scripting Mode](/guide/scripting-mode) - Automate tasks
- [Configuration](/guide/configuration) - Customize behavior
- [Features](/features/tab-completion) - Learn advanced features
