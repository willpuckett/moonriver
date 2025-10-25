# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/yourusername/moonriver/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/moonriver/releases/tag/v0.1.0
