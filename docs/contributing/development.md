# Contributing

Thank you for your interest in contributing to Moonriver!

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- A Moonraker/Klipper instance for testing (optional but recommended)

### Clone and Build

```bash
# Fork and clone the repository
git clone https://github.com/willpuckett/moonriver.git
cd moonriver

# Build the project
cargo build

# Run tests
cargo test

# Run locally
cargo run -- --host localhost --port 7125
```

## Making Changes

1. **Create a branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**

3. **Format code**:
   ```bash
   cargo fmt
   ```

4. **Lint code**:
   ```bash
   cargo clippy -- -D warnings
   ```

5. **Test changes**:
   ```bash
   cargo test
   cargo build --release
   ```

6. **Commit**:
   ```bash
   git commit -m "Add: your feature description"
   ```

## Code Style

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write documentation for public APIs
- Add tests for new functionality

### Naming Conventions

- Use snake_case for functions and variables
- Use PascalCase for types and traits
- Use SCREAMING_SNAKE_CASE for constants

### Documentation

Document public APIs:

````rust
/// Connects to a Moonraker instance via WebSocket
///
/// # Arguments
///
/// * `url` - WebSocket URL of the Moonraker instance
///
/// # Examples
///
/// ```
/// let client = MoonrakerClient::connect("ws://localhost:7125/websocket").await?;
/// ```
pub async fn connect(url: &str) -> Result<Self> {
    // Implementation
}
````

## Commit Guidelines

Write clear, concise commit messages:

- **Add**: New features
- **Fix**: Bug fixes
- **Update**: Changes to existing features
- **Refactor**: Code improvements without behavior changes
- **Docs**: Documentation changes
- **Test**: Test additions or changes
- **Chore**: Maintenance tasks

Examples:

```
Add: tab completion for user macros
Fix: connection timeout on slow networks
Update: improve error messages
Docs: add scripting mode examples
```

## Pull Request Process

1. **Update documentation** if needed
2. **Update CHANGELOG.md** with your changes
3. **Ensure all tests pass**:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

4. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create a Pull Request** on GitHub

6. **Wait for review** - maintainers will review your PR

### PR Checklist

- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Code is formatted (`cargo fmt`)
- [ ] Code passes clippy (`cargo clippy`)
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Commit messages are clear

## Testing

### Unit Tests

Write unit tests for new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gcode() {
        let result = parse_gcode("G28");
        assert_eq!(result, Command::Home);
    }
}
```

### Integration Tests

Add integration tests in `tests/`:

```rust
// tests/integration_test.rs
use moonriver::moonraker::MoonrakerClient;

#[tokio::test]
async fn test_connection() {
    // Test code
}
```

### Manual Testing

Test with a real Moonraker instance:

```bash
cargo run -- --host localhost --port 7125
```

## Feature Requests

Have an idea? Open an issue on GitHub:

1. Go to [Issues](https://github.com/willpuckett/moonriver/issues)
2. Click "New Issue"
3. Select "Feature Request"
4. Describe:
   - The use case
   - Why it would be beneficial
   - Implementation ideas (if any)

## Bug Reports

Found a bug? Report it:

1. Go to [Issues](https://github.com/willpuckett/moonriver/issues)
2. Click "New Issue"
3. Select "Bug Report"
4. Include:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Your environment (OS, Rust version, etc.)
   - Relevant logs or screenshots

## Questions

Have questions? Feel free to:

- Open a [Discussion](https://github.com/willpuckett/moonriver/discussions)
- Join our community chat (if available)
- Ask in an issue with the `question` label

## Code of Conduct

Be respectful and inclusive. We welcome contributors from all backgrounds.

## License

By contributing, you agree that your contributions will be licensed under the
MIT License.

## Recognition

Contributors will be recognized in:

- The project README
- Release notes
- CHANGELOG.md

Thank you for contributing to Moonriver! 🌙
