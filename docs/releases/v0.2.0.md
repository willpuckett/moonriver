# Changelog - v0.2.0

## Major Release: Terminal User Interface (TUI)

This release introduces a comprehensive **Terminal User Interface (TUI)** as the default mode for Moonriver, transforming it from a simple REPL into a full-featured printer monitoring and control dashboard.

---

## 🎯 New Default Experience

**TUI mode is now the default!** Simply run:
```bash
moonriver --host <printer-ip>
```

Access other modes with flags:
- `--repl` - Classic REPL mode
- `--command` / `-c` - Scripting mode for automation

---

## ✨ TUI Features

### 📊 Main Dashboard (`m` key)
- **Real-time Temperature Monitoring**
  - Extruder and bed temperatures with live updates
  - Power gauges showing heater output (0-100%)
  - Color-coded by power level: Green (0-30%), Yellow (30-60%), Red (60-100%)
  - Toggle with `t` key
  
- **System Information Panel** 
  - Connection status with color coding
  - Printer state (ready/printing/paused/error)
  - Server URL display
  - Toggle with `s` key

- **Print Job Status**
  - Current filename when printing
  - Print duration tracking
  - Filament usage statistics

### 💻 Console Tab (`c` key)
- **Full GCode Terminal**
  - Press `i` to enter edit mode
  - Type commands and press Enter to send
  - Color-coded output:
    - Commands: Green prompt `>`
    - Responses: Cyan text
    - Errors: Red text with ✗ prefix
    - Info: Gray status messages

- **Command History**
  - Navigate with ↑↓ arrow keys
  - Last 100 commands stored
  - Smart duplicate avoidance

- **Real-time Response Display**
  - Captures `notify_gcode_response` from Klipper
  - Error handling for failed commands
  - Scrollable message history

### 📍 Position Tab (`p` key)
- **Live Toolhead Position**
  - X, Y, Z, E coordinates with 2 decimal precision
  - Homed axis indicators (✓/✗)
  - Real-time updates from Moonraker

- **Homing Controls**
  - `x` - Home X axis
  - `y` - Home Y axis  
  - `z` - Home Z axis
  - `a` - Home all axes
  - Feedback in console

### 📋 Jobs Tab (`j` key)
- Placeholder for future job history browser
- Tab navigation functional

### ❓ Help Tab (`h` key)
- Comprehensive keyboard shortcut reference
- Context-sensitive help
- Feature documentation
- Current tab indicator

---

## 🎮 Keyboard Controls

### Global
- `m` - Main dashboard
- `c` - Console
- `p` - Position
- `j` - Jobs
- `h` - Help
- `q` / `Esc` - Quit
- `Ctrl+C` - **Emergency stop** (M112) and quit

### Main Dashboard
- `t` - Toggle temperature panel
- `s` - Toggle system info panel

### Console
- `i` - Enter edit mode
- `Enter` - Send command
- `Esc` - Cancel editing
- `↑↓` - Navigate command history

### Position
- `x/y/z` - Home individual axis
- `a` - Home all axes

---

## 🏗️ Technical Architecture

### Module Structure
```
src/tui/
├── mod.rs           # TUI initialization & event loop
├── app.rs           # Application state & event handling
├── tabs.rs          # Tab navigation with semantic keys
├── event.rs         # Keyboard/mouse/tick event handling
├── ui.rs            # Main render coordinator
├── printer.rs       # Printer state structures & JSON parser
├── modal.rs         # Modal dialog system (future)
├── stateful_list.rs # List widget pattern (future)
└── widgets/
    ├── header.rs       # Top bar with connection status
    ├── footer.rs       # Context-sensitive key hints
    ├── main.rs         # Main dashboard layout
    ├── console.rs      # GCode console terminal
    ├── position.rs     # Position display & controls
    ├── jobs.rs         # Job history (placeholder)
    ├── help.rs         # Help screen
    ├── temperatures.rs # Temperature gauges
    └── system_info.rs  # System information panel
```

### Data Flow
```
Moonraker WebSocket (JSON-RPC)
    ↓
MoonrakerClient.try_receive_message()
    ↓
App.update() → process_message()
    ↓
printer::update_from_json() / console messages
    ↓
Widget rendering (ratatui)
    ↓
Live UI updates (100ms tick rate)
```

### State Management
- **PrinterState**: Temperatures, toolhead position, print stats
- **ConsoleMessages**: Command/response history with types
- **Command Queue**: Non-blocking command execution
- **Panel Visibility**: Toggleable UI sections
- **Input Modes**: Normal vs Editing for console

---

## 🔧 Dependencies

### New Dependencies (v0.2.0)
- `ratatui = "0.29"` - Modern TUI framework
- `crossterm = "0.28"` - Terminal backend
- `toml = "0.8"` - Configuration file support

### Existing
- `tokio = "1.48"` - Async runtime
- `tokio-tungstenite = "0.28"` - WebSocket client
- `serde_json = "1.0"` - JSON parsing
- `anyhow = "1.0"` - Error handling
- `clap = "4.5"` - CLI argument parsing

---

## 📈 Performance

- **Non-blocking I/O**: UI never freezes waiting for printer
- **100ms tick rate**: Smooth updates without CPU waste
- **Efficient rendering**: Only redraws on state change
- **Command queuing**: Prevents blocking on send operations
- **Minimal allocations**: Hot paths optimized for performance

---

## 🐛 Known Limitations

- Console doesn't auto-scroll to absolute bottom (shows last 100 messages)
- No reconnection logic if WebSocket drops (restart required)
- System info panel shows limited data (placeholder for future)
- Job history browser not yet implemented
- No jogging controls yet (coming soon)

---

## 🔄 Migration from v0.1.x

### Breaking Changes
- **Default mode changed**: TUI is now default instead of REPL
- **New flags**: Use `--repl` for old behavior, `--command` for scripting

### Compatible Changes
- All v0.1.x commands still work in console tab
- REPL mode unchanged when using `--repl` flag
- Configuration file format unchanged

### Example Migration
```bash
# Old way (v0.1.x)
moonriver --host printer.local

# New equivalent (v0.2.0)
moonriver --host printer.local --repl

# New default (v0.2.0)
moonriver --host printer.local  # Launches TUI!
```

---

## 🚀 Future Roadmap

### Phase 4 (Planned)
- [ ] Job history browser with StatefulList
- [ ] Fetch recent prints from Moonraker
- [ ] Reprint/delete job actions
- [ ] Job statistics and filtering

### Phase 5 (Planned)
- [ ] Jogging controls with arrow keys
- [ ] Move distance selection (1/10/100mm)
- [ ] Quick position presets
- [ ] Enhanced temperature controls (set target)

### Phase 6 (Planned)
- [ ] Reconnection logic with auto-retry
- [ ] Connection status notifications
- [ ] Error recovery strategies
- [ ] Network diagnostic tools

### Phase 7 (Planned)
- [ ] Enhanced system info (CPU, RAM, MCU stats)
- [ ] Klipper version display
- [ ] Macro browser and execution
- [ ] Command auto-completion

---

## 🙏 Acknowledgments

Built with inspiration from:
- [krui](https://github.com/jfoucher/krui) - Klipper TUI interface
- [klipper-repl](https://github.com/Annex-Engineering/klipper_estimator) - Command-line Klipper control
- [ratatui](https://github.com/ratatui-org/ratatui) - Modern Rust TUI framework

---

## 📝 Testing

### Without Printer
```bash
cargo run
```
- TUI launches with "Not connected" status
- All tabs accessible
- Widgets display gracefully

### With Printer
```bash
cargo run -- --host <printer-ip> --port 7125
```
- Real-time temperature updates
- Live position tracking
- Console commands functional
- Homing controls active

### Key Sequences to Test
1. **Temperature monitoring**: Watch gauges update in real-time
2. **Console**: `c` → `i` → `M105` → Enter (check temp)
3. **History**: `c` → `i` → `G28` → Enter → `↑` (recalls command)
4. **Homing**: `p` → `a` (homes all axes)
5. **Panels**: `m` → `t` (toggle temp) → `s` (toggle sys info)
6. **Emergency**: `Ctrl+C` (emergency stop and quit)

---

## 🏁 Release Notes

**Version**: 0.2.0  
**Release Date**: October 25, 2025  
**License**: MIT  
**Repository**: [github.com/willpuckett/moonriver](https://github.com/willpuckett/moonriver)

This is a **major feature release** with significant new functionality. The TUI provides a much richer user experience compared to v0.1.x while maintaining full backward compatibility through the `--repl` flag.

**Upgrade recommended for all users!** 🎉
