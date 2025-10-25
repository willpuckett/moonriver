# Publishing to crates.io

This guide explains how to publish Moonriver to crates.io.

## Prerequisites

1. **Create a crates.io account**: https://crates.io/
2. **Get API token**: Go to https://crates.io/me and create a new token
3. **Configure GitHub secret**:
   - Go to your GitHub repository settings
   - Navigate to Secrets and variables > Actions
   - Add a new secret named `CARGO_REGISTRY_TOKEN`
   - Paste your crates.io API token as the value

## Pre-Publishing Checklist

Before publishing, ensure:

- [ ] All tests pass: `cargo test`
- [ ] Code compiles without warnings: `cargo clippy -- -D warnings`
- [ ] Code is formatted: `cargo fmt --check`
- [ ] README.md is up to date
- [ ] CHANGELOG.md includes version notes
- [ ] Version number is updated in `Cargo.toml`
- [ ] Documentation builds: `cargo doc --no-deps`
- [ ] All examples work
- [ ] License files are included

## Manual Publishing

### 1. Login to crates.io

```bash
cargo login
```

Paste your API token when prompted.

### 2. Dry Run

Test the publish process without actually publishing:

```bash
cargo publish --dry-run
```

This will:

- Check that all files are included
- Verify the package builds
- Show what would be published

### 3. Publish

When ready:

```bash
cargo publish
```

::: warning Once published, a version cannot be deleted or modified! Make sure
everything is correct. :::

## Automated Publishing (GitHub Actions)

The repository includes a GitHub Action that automatically publishes to
crates.io when you create a release.

### Create a Release

1. **Update version** in `Cargo.toml`:
   ```toml
   version = "0.2.0"
   ```

2. **Update CHANGELOG.md** with release notes

3. **Commit changes**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release v0.2.0"
   git push
   ```

4. **Create and push tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

5. **Create GitHub Release**:
   - Go to GitHub repository
   - Click "Releases" → "Create a new release"
   - Select your tag (v0.2.0)
   - Add release notes from CHANGELOG
   - Click "Publish release"

The GitHub Action will automatically:

- Build the project
- Run tests
- Run clippy
- Publish to crates.io

### Manual Workflow Trigger

You can also manually trigger the publish workflow:

1. Go to Actions tab in GitHub
2. Select "Publish to crates.io"
3. Click "Run workflow"

## Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backwards compatible
- **Patch** (0.0.1): Bug fixes, backwards compatible

Examples:

- `0.1.0` → `0.1.1`: Bug fix
- `0.1.0` → `0.2.0`: New feature
- `0.9.0` → `1.0.0`: Stable release

## Package Metadata

Ensure `Cargo.toml` has complete metadata:

```toml
[package]
name = "moonriver"
version = "0.1.0"
edition = "2021"
authors = ["Moonriver Contributors"]
description = "A terminal-based console for connecting to and interacting with Klipper instances via the Moonraker WebSocket API"
readme = "README.md"
homepage = "https://moonriver.rs/"
documentation = "https://moonriver.rs/"
repository = "https://github.com/willpuckett/moonriver"
license = "MIT"
keywords = ["klipper", "moonraker", "3dprinting", "terminal", "repl"]
categories = ["command-line-utilities"]
```

## What Gets Published

Files included in the package:

- ✅ `src/`
- ✅ `Cargo.toml`
- ✅ `README.md`
- ✅ `LICENSE-MIT`
- ✅ `CHANGELOG.md`

Files excluded (see `Cargo.toml`):

- ❌ `docs/`
- ❌ `.github/`
- ❌ `target/`
- ❌ `examples/`

## After Publishing

### Verify Package

Check the published package:

```bash
cargo search moonriver
```

View on crates.io:

```
https://crates.io/crates/moonriver
```

### Test Installation

Test that others can install it:

```bash
cargo install moonriver
moonriver --version
```

### Update Documentation

Update links in README and docs to point to:

- crates.io page
- docs.rs documentation

### Announce

Announce the release:

- GitHub Discussions
- Reddit r/3Dprinting, r/klippers
- Twitter/X
- Discord communities

## Troubleshooting

### "crate name already exists"

The name `moonriver` is taken. Choose a different name in `Cargo.toml`.

### "repository not found"

Ensure the repository URL in `Cargo.toml` is correct and public.

### "failed to verify"

Check that:

- All dependencies are specified correctly
- No path dependencies (use version dependencies)
- Code compiles: `cargo build --release`

### "missing required fields"

Ensure `Cargo.toml` has all required fields:

- `description`
- `license`
- `repository`

## Yanking a Release

If you need to yank (hide) a published version:

```bash
cargo yank --vers 0.1.0
```

This prevents new projects from using it, but doesn't delete it.

Undo a yank:

```bash
cargo yank --vers 0.1.0 --undo
```

## Best Practices

1. **Test thoroughly** before publishing
2. **Use semantic versioning** correctly
3. **Write clear changelogs** for each release
4. **Keep README current** with installation instructions
5. **Respond to issues** from users
6. **Maintain backwards compatibility** when possible
7. **Document breaking changes** clearly

## Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io Policies](https://crates.io/policies)
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
