# Contributing to Moonriver 🌙

Thank you for your interest in contributing to Moonriver! We welcome
contributions from everyone.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/willpuckett/moonriver.git
   cd moonriver
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- A Moonraker/Klipper instance for testing (optional but recommended)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- --host localhost --port 7125
```

## Making Changes

1. Make your changes in your feature branch
2. Ensure your code compiles without warnings:
   ```bash
   cargo clippy -- -D warnings
   ```
3. Format your code:
   ```bash
   cargo fmt
   ```
4. Test your changes thoroughly

## Commit Guidelines

- Write clear, concise commit messages
- Start commit messages with a verb (e.g., "Add", "Fix", "Update")
- Reference issues when applicable (e.g., "Fixes #123")

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update the CHANGELOG.md if you're adding features or fixing bugs
3. Push your changes to your fork
4. Create a Pull Request from your fork to the main repository
5. Ensure all CI checks pass
6. Wait for review from maintainers

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write documentation for public APIs
- Add tests for new functionality

## Feature Requests and Bug Reports

- Use GitHub Issues to report bugs or request features
- For bugs, include:
  - Steps to reproduce
  - Expected behavior
  - Actual behavior
  - Your environment (OS, Rust version, etc.)
- For features, explain:
  - The use case
  - Why it would be beneficial
  - Any implementation ideas

## Questions?

Feel free to open an issue with the `question` label if you need help or
clarification.

## License

By contributing to Moonriver, you agree that your contributions will be licensed
under the MIT License.
