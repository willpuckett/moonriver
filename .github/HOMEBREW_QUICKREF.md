# Homebrew Formula Quick Reference

## Testing the Formula Locally

```bash
# Install from local formula
brew install --build-from-source moonriver.rb

# Test it works
moonriver --version
moonriver --help

# Run formula tests
brew test moonriver

# Audit for issues
brew audit --strict --online moonriver

# Uninstall
brew uninstall moonriver
```

## Updating for a New Release

```bash
# Automated way (recommended)
./scripts/update_formula.sh 0.2.0

# Manual way
# 1. Download tarball
curl -L https://github.com/willpuckett/moonriver/archive/refs/tags/v0.2.0.tar.gz -o moonriver.tar.gz

# 2. Get SHA256
shasum -a 256 moonriver.tar.gz

# 3. Edit moonriver.rb - update url and sha256
```

## Submitting to Homebrew Core

```bash
# Fork homebrew-core on GitHub first

# Clone your fork
git clone https://github.com/YOUR_USERNAME/homebrew-core.git
cd homebrew-core

# Create branch
git checkout -b moonriver-0.2.0

# Copy formula
cp /path/to/moonriver/moonriver.rb Formula/moonriver.rb

# Test thoroughly
brew install --build-from-source Formula/moonriver.rb
brew test moonriver
brew audit --strict --online moonriver

# Commit and push
git add Formula/moonriver.rb
git commit -m "moonriver 0.2.0 (new formula)"
# or for updates:
git commit -m "moonriver: update 0.1.0 -> 0.2.0"

git push origin moonriver-0.2.0

# Create PR on GitHub
```

## Common Commands

```bash
# Reinstall from local formula
brew reinstall --build-from-source moonriver.rb

# Check formula syntax
brew audit moonriver.rb

# See formula info
brew info moonriver

# Edit formula
brew edit moonriver

# Check dependencies
brew deps moonriver

# See formula history (once in homebrew-core)
brew log moonriver
```

## Troubleshooting

### Formula doesn't install
- Verify SHA256 hash matches the tarball
- Check that the URL is accessible
- Ensure all Rust dependencies compile on macOS

### Audit fails
- Run `brew audit --strict --online moonriver` for details
- Common issues: wrong SHA256, invalid URL, missing test

### Build fails on Apple Silicon
- Test on both Intel and M1/M2 Macs if possible
- Ensure Rust dependencies support aarch64-apple-darwin

## Formula Structure

```ruby
class Moonriver < Formula
  desc "Short description"           # One-line description
  homepage "https://moonriver.rs/"   # Project homepage
  url "https://..."                  # Source tarball URL
  sha256 "..."                       # SHA256 of tarball
  license "MIT"                      # License
  head "...", branch: "main"        # Optional: install from HEAD

  depends_on "rust" => :build        # Build dependencies

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Tests to verify installation
    assert_match version.to_s, shell_output("#{bin}/moonriver --version")
  end
end
```

## Resources

- Formula Cookbook: https://docs.brew.sh/Formula-Cookbook
- Acceptable Formulae: https://docs.brew.sh/Acceptable-Formulae
- Contributing: https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md
