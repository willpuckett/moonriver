# Syntax Highlighting

Moonriver provides real-time syntax highlighting as you type commands.

## Color Coding

Different command types are highlighted with distinct colors:

### G-code Commands
**Bright Green** - Standard G-code commands

```bash
> G28
> M105
> G0 X100 Y100 Z10
```

### Klipper Macros
**Bright Cyan** - User-defined Klipper macros

```bash
> PRINT_START
> BED_MESH_CALIBRATE
> SCREWS_TILT_CALCULATE
```

### Unknown Commands
**White** - Unrecognized commands

```bash
> UNKNOWN_COMMAND
```

## Response Coloring

Responses from the printer are also color-coded:

### Success Messages
**Green** - Successful responses, "ok" messages

```
ok
// Homing complete
```

### Information
**Cyan** - Normal informational messages

```
T:200.5 /200.0 B:60.2 /60.0
// Current position: X:100.0 Y:100.0 Z:10.0
```

### Warnings
**Yellow** - Warning messages, comments starting with "//"

```
// Warning: Temperature variance detected
!! Endstop not triggered
```

### Errors
**Red** - Error messages, messages containing "error" or "!!"

```
!! Error: Unknown command
!! Emergency stop
Error: Connection lost
```

## Visual Feedback

Syntax highlighting provides immediate visual feedback:

1. **Command Recognition**: Green/cyan highlighting confirms the command is recognized
2. **Typo Detection**: White highlighting suggests a potential typo
3. **Status Understanding**: Color-coded responses help you quickly understand printer state

## Examples

### Successful Command Flow

```bash
> G28                           # Green (recognized)
// Homing X Y Z                 # Cyan (info)
ok                              # Green (success)

> M104 S200                     # Green (recognized)
// Setting extruder to 200.0    # Cyan (info)
ok                              # Green (success)
```

### Error Handling

```bash
> INVALID_COMMAND               # White (not recognized)
!! Unknown command              # Red (error)

> M999                          # Green (recognized but causes error)
!! Error: Invalid command      # Red (error)
```

### Macro Execution

```bash
> PRINT_START                   # Cyan (macro)
// Executing PRINT_START        # Cyan (info)
// Homing all axes              # Cyan (info)
// Heating bed to 60            # Cyan (info)
ok                              # Green (success)
```

## Benefits

### Quick Recognition
- **Spot typos instantly** before pressing Enter
- **Distinguish macros from G-code** at a glance
- **Understand command type** immediately

### Error Prevention
- White highlighting warns you before executing unknown commands
- Helps catch case-sensitive macro names
- Visual confirmation of command recognition

### Faster Workflow
- No need to wait for error messages
- Quick visual scanning of command history
- Immediate feedback reduces mistakes

## Technical Details

Highlighting is applied in real-time using the `colored` crate:

- Processed character-by-character as you type
- Zero latency on modern systems
- Works in all terminal emulators with color support

## Terminal Compatibility

Syntax highlighting works in:
- ✅ Terminal.app (macOS)
- ✅ iTerm2 (macOS)
- ✅ GNOME Terminal (Linux)
- ✅ Konsole (Linux)
- ✅ Windows Terminal
- ✅ xterm
- ✅ Most modern terminal emulators

## Customization

::: tip Future Feature
Custom color schemes are planned for a future release.
:::

## Next Steps

- [Tab Completion](/features/tab-completion) - Command autocompletion
- [Command History](/features/command-history) - Navigate previous commands
- [Interactive Mode](/guide/interactive-mode) - Full REPL features
