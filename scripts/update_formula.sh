#!/usr/bin/env bash
# update_formula.sh - Script to update the Homebrew formula for a new release

set -e

if [ $# -ne 1 ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

VERSION="$1"
TAG="v${VERSION}"
FORMULA_FILE="moonriver.rb"
TARBALL_URL="https://github.com/willpuckett/moonriver/archive/refs/tags/${TAG}.tar.gz"
TEMP_FILE=$(mktemp)

echo "Updating Homebrew formula for moonriver ${VERSION}"
echo "================================================"
echo ""

# Download the tarball
echo "→ Downloading release tarball..."
if ! curl -sL "$TARBALL_URL" -o "$TEMP_FILE"; then
    echo "✗ Error: Failed to download tarball from $TARBALL_URL"
    echo "  Make sure the tag ${TAG} exists and is pushed to GitHub"
    rm -f "$TEMP_FILE"
    exit 1
fi

# Calculate SHA256
echo "→ Calculating SHA256 hash..."
if command -v shasum >/dev/null 2>&1; then
    SHA256=$(shasum -a 256 "$TEMP_FILE" | awk '{print $1}')
elif command -v sha256sum >/dev/null 2>&1; then
    SHA256=$(sha256sum "$TEMP_FILE" | awk '{print $1}')
else
    echo "✗ Error: Neither shasum nor sha256sum found"
    rm -f "$TEMP_FILE"
    exit 1
fi

rm -f "$TEMP_FILE"

echo "→ SHA256: $SHA256"
echo ""

# Update the formula file
echo "→ Updating $FORMULA_FILE..."

# Backup the original
cp "$FORMULA_FILE" "${FORMULA_FILE}.bak"

# Update URL and SHA256
sed -i.tmp "s|url \".*\"|url \"$TARBALL_URL\"|" "$FORMULA_FILE"
sed -i.tmp "s|sha256 \".*\"|sha256 \"$SHA256\"|" "$FORMULA_FILE"
rm -f "${FORMULA_FILE}.tmp"

echo "✓ Formula updated successfully!"
echo ""
echo "Changes:"
echo "--------"
echo "URL:    $TARBALL_URL"
echo "SHA256: $SHA256"
echo ""
echo "Next steps:"
echo "1. Review the changes: git diff $FORMULA_FILE"
echo "2. Test the formula: brew install --build-from-source $FORMULA_FILE"
echo "3. Run audit: brew audit --strict --online moonriver"
echo "4. Commit changes: git add $FORMULA_FILE && git commit -m 'Update formula to ${VERSION}'"
echo ""
echo "Backup saved to: ${FORMULA_FILE}.bak"
