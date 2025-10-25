# Setup Guide for Moonriver

This guide covers setting up Moonriver for both publishing to crates.io and
deploying documentation to GitHub Pages.

## Part 1: Publishing to crates.io

### Step 1: Prepare Your Account

1. **Create a crates.io account** at https://crates.io/
2. **Get your API token**:
   - Go to https://crates.io/me
   - Under "API Tokens", click "New Token"
   - Give it a name (e.g., "moonriver-publish")
   - Copy the token (you won't see it again!)

### Step 2: Configure GitHub Repository

1. **Add the crates.io token to GitHub**:
   - Go to your repository: https://github.com/willpuckett/moonriver
   - Click Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: paste your crates.io token
   - Click "Add secret"

### Step 3: Verify Package Metadata

Check that `Cargo.toml` has all required fields:

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

### Step 4: Test Locally (Optional)

```bash
# Dry run to test packaging
cargo publish --dry-run

# Check what will be included
cargo package --list

# Build the actual package
cargo package
```

### Step 5: Publish to crates.io

You have two options:

#### Option A: Manual Publish

```bash
# Login with your token
cargo login

# Publish
cargo publish
```

#### Option B: Automated via GitHub Release

1. **Update version** in `Cargo.toml` (e.g., `0.1.0`)
2. **Update CHANGELOG.md** with release notes
3. **Commit and push**:
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "Release v0.1.0"
   git push
   ```
4. **Create and push tag**:
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```
5. **Create GitHub Release**:
   - Go to https://github.com/willpuckett/moonriver/releases
   - Click "Create a new release"
   - Choose tag `v0.1.0`
   - Add release title: "v0.1.0"
   - Copy release notes from CHANGELOG
   - Click "Publish release"

The GitHub Action will automatically publish to crates.io!

## Part 2: GitHub Pages Documentation

### Step 1: Enable GitHub Pages

1. Go to https://github.com/willpuckett/moonriver/settings/pages
2. Under "Source", select **"GitHub Actions"**
3. Click Save

### Step 2: Verify Workflow

The workflow is already set up in `.github/workflows/docs.yml`. It will:

- Trigger on pushes to `main` branch
- Build the VitePress documentation
- Deploy to GitHub Pages

### Step 3: Push to Trigger Deployment

```bash
git add .
git commit -m "Add documentation site"
git push origin main
```

### Step 4: Wait for Deployment

1. Go to https://github.com/willpuckett/moonriver/actions
2. Watch the "Deploy VitePress Documentation" workflow
3. Once complete (green checkmark), your site is live!

### Step 5: Visit Your Documentation

Your documentation will be available at:

```
https://moonriver.rs/
```

## Part 3: Testing Locally

### Test Documentation Site

```bash
cd docs

# Install dependencies (first time only)
npm install

# Run dev server
npm run docs:dev
# Visit http://localhost:5173

# Build for production
npm run docs:build

# Preview production build
npm run docs:preview
```

### Test Rust Package

```bash
# Build
cargo build --release

# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Format code
cargo fmt

# Test installation
cargo install --path .
```

## Part 4: Maintenance

### Updating Documentation

1. Edit files in `docs/` directory
2. Test locally: `npm run docs:dev`
3. Commit and push to `main` branch
4. GitHub Actions automatically deploys

### Publishing New Versions

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit changes
4. Create and push tag
5. Create GitHub Release
6. Automated workflow publishes to crates.io

### Monitoring

- **crates.io**: https://crates.io/crates/moonriver
- **GitHub Actions**: https://github.com/willpuckett/moonriver/actions
- **Documentation**: https://moonriver.rs/

## Troubleshooting

### crates.io Publishing Fails

**"crate name already exists"**

- The name is taken, choose a different name

**"repository not found"**

- Ensure repository URL is correct and public

**"missing required fields"**

- Check all required fields in Cargo.toml

### GitHub Pages Not Updating

**Check Actions Tab**

- Look for errors in workflow runs
- Ensure Pages is set to "GitHub Actions" source

**Clear Cache**

- Sometimes GitHub Pages caches aggressively
- Try hard refresh: Ctrl+Shift+R (Cmd+Shift+R on Mac)

**Check base URL**

- Ensure `.vitepress/config.mts` has correct base path
- Should be `/moonriver/` for username.github.io/moonriver

### Documentation Build Fails

**Dead links**

- Already configured to ignore with `ignoreDeadLinks: true`

**Missing dependencies**

- Run `npm install` in docs directory

**Node version**

- Ensure Node.js 18+ is installed

## Next Steps

After setup:

1. **Announce the release**:
   - Reddit (r/3Dprinting, r/klippers)
   - Klipper Discord
   - Twitter/X

2. **Monitor feedback**:
   - GitHub Issues
   - crates.io download stats
   - GitHub Discussions

3. **Iterate**:
   - Fix bugs
   - Add features
   - Improve documentation

## Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [GitHub Pages Docs](https://docs.github.com/en/pages)
- [VitePress Guide](https://vitepress.dev/guide/getting-started)
- [Semantic Versioning](https://semver.org/)

## Quick Checklist

Before first publish:

- [ ] crates.io account created
- [ ] API token generated
- [ ] GitHub secret `CARGO_REGISTRY_TOKEN` configured
- [ ] Cargo.toml metadata complete
- [ ] README.md is comprehensive
- [ ] LICENSE file exists
- [ ] CHANGELOG.md has v0.1.0 entry
- [ ] GitHub Pages enabled (source: GitHub Actions)
- [ ] Documentation builds locally
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted

Ready to publish! 🚀
