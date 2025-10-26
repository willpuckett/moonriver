# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **REPL Mode Enhancements**:
  - Comprehensive autocomplete with 350+ commands (G-codes, Klipper commands, macros)
  - Dynamic command loading from printer's HELP output at startup
  - Enhanced syntax highlighting by command type (green for G-codes, blue for Klipper commands, cyan for macros)
- **TUI Mode**: Full-screen terminal user interface with multiple tabs
  - Main dashboard with real-time temperature and position monitoring
  - Console tab with GCode command entry and history
  - Position tab with toolhead coordinates and homing controls
  - Jobs tab with print history browser
  - Help screen with keyboard shortcuts
- **Job History Browser**: View and manage print jobs
  - Browse up to 50 recent print jobs with status indicators
  - View job details (duration, filament used, completion time)
  - Start new prints directly from job history
  - Automatic job list refresh when entering Jobs tab
  - Manual refresh capability
- **Enhanced Console**:
  - Command history navigation with arrow keys
  - Real-time command/response display
  - Error highlighting
  - Scrollable message history
- **Emergency Stop**: Ctrl+C sends M112 emergency stop command
- **Homing Controls**: Individual axis homing (x/y/z) and home all (a)
- **System Information Panel**: Toggleable display of connection status and printer state
- **HTTP API Integration**: Added reqwest for REST API calls to Moonraker
- **Date/Time Formatting**: Added chrono for human-readable timestamps

### Changed

- **REPL Mode Improvements**:
  - Optimized response timing to 330ms with intelligent polling
  - Automatic output display without requiring Enter presses
- Extended WebSocket implementation with real-time printer state updates
- Improved keyboard navigation with semantic keys (m/c/p/j/h instead of F-keys)
- Enhanced footer with context-sensitive key hints
- Updated help documentation with new TUI features

### Fixed

- Filtered unnecessary "ok" messages from REPL command responses

## [0.2.0] - 2025-10-25

See [detailed release notes](docs/releases/v0.2.0.md) for comprehensive v0.2.0 documentation.

### Added

- **TUI Mode**: Full-screen terminal user interface as the new default mode
  - Main dashboard with real-time temperature monitoring and power gauges
  - Console tab with GCode terminal and command history
  - Position tab with live toolhead coordinates and homing controls
  - Jobs tab placeholder for future job history browser
  - Help screen with comprehensive keyboard shortcuts
- **Temperature Monitoring**:
  - Real-time extruder and bed temperature display
  - Power gauges showing heater output (0-100%)
  - Color-coded by power level (green/yellow/red)
  - Toggleable temperature panel
- **Console Terminal**:
  - Full GCode command entry with edit mode
  - Color-coded output (commands, responses, errors)
  - Command history with arrow key navigation
  - Real-time response capture from Klipper
- **Position Display**:
  - Live X, Y, Z, E coordinates
  - Homed axis indicators
  - Individual and all-axis homing controls
- **Emergency Stop**: Ctrl+C sends M112 and quits safely
- **System Information Panel**: Connection status and printer state display

### Changed

- **Default mode is now TUI** (use `--repl` flag for old behavior)
- New CLI flags: `--repl` for REPL mode, `--command`/`-c` for scripting
- Non-blocking I/O architecture for smooth UI updates
- 100ms tick rate for real-time monitoring

### Technical

- Added ratatui 0.29 for TUI framework
- Added crossterm 0.28 for terminal backend
- Added toml 0.8 for configuration file support
- Modular TUI architecture with separate widget modules

## [0.1.0] - 2025-10-25

### Added

- Initial release of Moonriver
- WebSocket connection to Moonraker API
- Interactive REPL with command history
- Tab completion for G-code commands and macros
- Syntax highlighting for commands
- Color-coded output (green for responses, yellow for warnings, red for errors)
- Support for multiple commands per line using comma separator
- Emergency stop (M112) support
- Scripting mode for non-interactive command execution
- Command-line arguments for host, port, and API key configuration
- Automatic reconnection handling
- Support for user-defined Klipper macros
- History file saving and loading

### Features

- Real-time printer status monitoring
- G-code command submission and response
- Macro execution support
- Multiple printer support via GNU Parallel compatibility
- Fast, memory-safe implementation in Rust

[Unreleased]: https://github.com/willpuckett/moonriver/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/willpuckett/moonriver/releases/tag/v0.1.0
