# Homebrew Formula Submission Guide

This document explains how to submit the Moonriver formula to Homebrew core.

## Prerequisites

Before submitting to Homebrew core, ensure:

1. **At least one stable release** - You need at least one tagged release
   (v0.1.0 or higher)
2. **The formula works** - Test it thoroughly on macOS
3. **Notable project** - Homebrew core requires projects to be "notable" (have
   users, stars, activity)
4. **No vendored dependencies** - All dependencies must be properly declared

## Preparing the Formula

### 1. Calculate the SHA256 Hash

After creating a release (e.g., v0.1.0), calculate the SHA256 hash of the source
tarball:

```bash
# Download the release tarball
curl -L https://github.com/willpuckett/moonriver/archive/refs/tags/v0.1.0.tar.gz -o moonriver-0.1.0.tar.gz

# Calculate SHA256
shasum -a 256 moonriver-0.1.0.tar.gz
```

### 2. Update the Formula

Edit `moonriver.rb` and replace:

- `v0.1.0` with your actual release version
- `PLACEHOLDER_SHA256_HASH` with the calculated SHA256 hash

Example:

```ruby
url "https://github.com/willpuckett/moonriver/archive/refs/tags/v0.2.0.tar.gz"
sha256 "abc123def456..." # Your actual SHA256 hash
```

### 3. Test the Formula Locally

Before submitting, test the formula on your Mac:

```bash
# Install from local formula
brew install --build-from-source moonriver.rb

# Test it works
moonriver --version
moonriver --help

# Run the test block
brew test moonriver

# Check for issues
brew audit --strict --online moonriver

# Uninstall after testing
brew uninstall moonriver
```

## Submitting to Homebrew Core

### Option 1: Using `brew create` (Recommended)

1. **Fork homebrew-core**:
   ```bash
   # Fork https://github.com/Homebrew/homebrew-core on GitHub
   ```

2. **Create the formula**:
   ```bash
   cd $(brew --repository homebrew/core)
   git checkout -b moonriver

   # Copy your tested formula
   cp /path/to/moonriver.rb Formula/moonriver.rb
   ```

3. **Commit and push**:
   ```bash
   git add Formula/moonriver.rb
   git commit -m "moonriver: new formula"
   git push origin moonriver
   ```

4. **Create Pull Request**:
   - Go to https://github.com/Homebrew/homebrew-core
   - Create a PR from your fork
   - Title: `moonriver: new formula`

### Option 2: Manual Submission

1. **Fork and clone homebrew-core**:
   ```bash
   # Fork https://github.com/Homebrew/homebrew-core
   git clone https://github.com/YOUR_USERNAME/homebrew-core.git
   cd homebrew-core
   ```

2. **Create branch**:
   ```bash
   git checkout -b moonriver
   ```

3. **Add formula**:
   ```bash
   cp /path/to/moonriver.rb Formula/moonriver.rb
   ```

4. **Test thoroughly**:
   ```bash
   brew install --build-from-source Formula/moonriver.rb
   brew test moonriver
   brew audit --strict --online moonriver
   ```

5. **Commit and push**:
   ```bash
   git add Formula/moonriver.rb
   git commit -m "moonriver: new formula"
   git push origin moonriver
   ```

6. **Create PR on GitHub**

## PR Guidelines

Your PR description should include:

```markdown
## moonriver 0.1.0 (new formula)

A terminal-based console for connecting to and interacting with Klipper
instances via the Moonraker WebSocket API.

- Built with Rust
- Real-time WebSocket connection to Moonraker
- Interactive REPL with command history
- Syntax highlighting for G-code
- Scripting support

**Homepage**: https://moonriver.rs/ **Repository**:
https://github.com/willpuckett/moonriver
```

## Homebrew Requirements Checklist

- [ ] Formula follows Homebrew naming conventions (lowercase, no hyphens if
      possible)
- [ ] SHA256 hash is correct
- [ ] URL points to a stable release tarball
- [ ] License is correctly specified
- [ ] `test do` block includes meaningful tests
- [ ] `brew audit --strict --online` passes with no errors
- [ ] `brew install` works on macOS (both Intel and Apple Silicon if possible)
- [ ] `brew test` passes
- [ ] Formula builds from source successfully
- [ ] Project has a stable release (not pre-1.0 alpha/beta)
- [ ] Project is notable (has users, GitHub stars, activity)
- [ ] No vendored dependencies

## Common Issues

### Build Failures

If the build fails:

1. Check that all Rust dependencies compile on macOS
2. Ensure no system-specific dependencies are hardcoded
3. Test on both Intel and Apple Silicon Macs if possible

### Audit Failures

Common audit issues:

- Incorrect SHA256 hash
- URL doesn't exist or isn't a release tarball
- Missing license information
- Test block doesn't adequately test the formula

### Rejection Reasons

Homebrew core might reject if:

- Project is too new or not established
- No stable release (pre-1.0 often rejected)
- Duplicate functionality of existing formula
- Not enough community interest

## Alternative: Homebrew Tap (Not Required)

If you want users to install before/instead of homebrew-core submission:

```bash
# Create a tap repository (optional)
# Users can install with:
brew install willpuckett/tap/moonriver
```

However, since you mentioned you don't want a tap, focus on getting into
homebrew-core.

## After Acceptance

Once your formula is accepted into homebrew-core:

1. **Update installation docs**:
   ```bash
   brew install moonriver
   ```

2. **Maintain the formula**:
   - Submit updates for new versions
   - Respond to issues in homebrew-core

3. **Version updates** use this format:
   ```bash
   # For version bumps
   cd $(brew --repository homebrew/core)
   git checkout -b moonriver-0.2.0
   # Update Formula/moonriver.rb
   git commit -m "moonriver: update 0.1.0 -> 0.2.0"
   ```

## Testing on macOS

To ensure your formula works properly:

```bash
# Test installation
brew install --build-from-source moonriver.rb

# Test the binary
moonriver --version
moonriver --help

# Test formula audit
brew audit --strict --online moonriver

# Test formula test block
brew test moonriver

# Test uninstallation
brew uninstall moonriver

# Test installation from bottle (after initial acceptance)
brew install moonriver
```

## Resources

- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Homebrew Acceptable Formulae](https://docs.brew.sh/Acceptable-Formulae)
- [Homebrew Node for Python](https://docs.brew.sh/Python-for-Formula-Authors)
- [homebrew-core Contributing Guide](https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md)

## Timeline

Typical timeline for homebrew-core PRs:

- Initial review: 1-7 days
- Revisions (if needed): varies
- Acceptance: can take 1-4 weeks total

Be patient and responsive to maintainer feedback!
