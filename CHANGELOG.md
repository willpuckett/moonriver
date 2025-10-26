# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

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

- Extended WebSocket implementation with real-time printer state updates
- Improved keyboard navigation with semantic keys (m/c/p/j/h instead of F-keys)
- Enhanced footer with context-sensitive key hints
- Updated help documentation with new TUI features

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
