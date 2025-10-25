# Command History

Moonriver maintains a persistent command history for easy reuse and navigation.

## Basic Navigation

Use arrow keys to navigate through previously executed commands:

- `↑` - Previous command
- `↓` - Next command

## Example Usage

```bash
> G28
> M105
> GET_POSITION
> ↑        # Shows: GET_POSITION
> ↑        # Shows: M105
> ↑        # Shows: G28
> ↓        # Shows: M105
```

## History Search

Press `Ctrl+R` to search your command history:

```bash
(reverse-i-search)`temp': M105
```

- **Type** to filter matching commands
- **Enter** to execute the found command
- **Esc** to cancel and return to normal mode
- **Ctrl+R again** to cycle through matches

### Search Examples

Search for temperature commands:

```bash
Ctrl+R
(reverse-i-search)`temp': M105
Ctrl+R
(reverse-i-search)`temp': M104 S200
Ctrl+R
(reverse-i-search)`temp': SET_HEATER_TEMPERATURE HEATER=extruder TARGET=200
```

Search for homing:

```bash
Ctrl+R
(reverse-i-search)`home': G28
Ctrl+R
(reverse-i-search)`home': G28 X Y
```

## Persistence

Command history is automatically saved to:

```
~/.moonriver_history
```

### Features

- **Automatic Saving**: History is saved after each session
- **Automatic Loading**: Previous history is loaded on startup
- **Persistent**: History survives across restarts and reboots
- **Unlimited**: No practical limit on history size

## History Management

### View History File

```bash
cat ~/.moonriver_history
```

### Clear History

```bash
rm ~/.moonriver_history
```

Next time you run Moonriver, a fresh history will be created.

### Share History Across Machines

Sync history between machines using a cloud service:

```bash
# Move to Dropbox
mv ~/.moonriver_history ~/Dropbox/moonriver_history

# Create symlink
ln -s ~/Dropbox/moonriver_history ~/.moonriver_history
```

### Backup History

```bash
# Create backup
cp ~/.moonriver_history ~/.moonriver_history.backup

# Restore from backup
cp ~/.moonriver_history.backup ~/.moonriver_history
```

## Duplicate Handling

Moonriver intelligently handles duplicates:

- Consecutive duplicate commands are not re-added
- Non-consecutive duplicates are preserved (useful for recurring tasks)

Example:

```bash
> G28
> M105
> G28        # Not added (consecutive duplicate)
> M105
> G28        # Added (not consecutive)
```

## Empty Commands

Empty commands (just pressing Enter) are not added to history.

## Session Behavior

### Within a Session

All commands are available via arrow keys and `Ctrl+R` immediately after execution.

### Between Sessions

History from previous sessions is loaded on startup, giving you access to commands from days, weeks, or months ago.

## Tips & Tricks

### Quick Repetition

```bash
> G28
> ↑ Enter    # Quickly re-execute G28
```

### Modify Previous Command

```bash
> M104 S200
> ↑
> # Edit to: M104 S210
> Enter
```

### Find Long Commands

Use `Ctrl+R` to find complex commands you don't want to retype:

```bash
Ctrl+R
(reverse-i-search)`mesh': BED_MESH_CALIBRATE PROFILE=default
```

### Common Command Patterns

History makes it easy to repeat common workflows:

```bash
# First time
> G28
> BED_MESH_CALIBRATE
> M104 S200
> M140 S60

# Later, press ↑ four times to repeat entire sequence
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `↑` | Previous command |
| `↓` | Next command |
| `Ctrl+R` | Search history backward |
| `Ctrl+S` | Search history forward |
| `Alt+<` | Beginning of history |
| `Alt+>` | End of history |
| `Ctrl+P` | Previous command (same as ↑) |
| `Ctrl+N` | Next command (same as ↓) |

## Privacy Considerations

The history file is stored in plain text in your home directory. If you execute sensitive commands (like those with API keys), be aware they will be stored in the history file.

To prevent saving:

```bash
# Execute without saving to history
# (Not yet implemented - future feature)
```

## History Limits

By default, there is no hard limit on history size. The history file will grow over time. To manage size:

```bash
# Keep only last 1000 lines
tail -n 1000 ~/.moonriver_history > ~/.moonriver_history.tmp
mv ~/.moonriver_history.tmp ~/.moonriver_history
```

## Next Steps

- [Tab Completion](/features/tab-completion) - Autocomplete commands
- [Syntax Highlighting](/features/syntax-highlighting) - Visual feedback
- [Interactive Mode](/guide/interactive-mode) - Full REPL features
