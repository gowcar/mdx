#!/bin/sh
set -e

REPO="gowcar/mdx"
INSTALL_DIR="/usr/local/bin"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Darwin)
        case "$ARCH" in
            arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
            x86_64)        TARGET="x86_64-apple-darwin" ;;
            *)             echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Linux)
        case "$ARCH" in
            x86_64|amd64)  TARGET="x86_64-unknown-linux-gnu" ;;
            aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
            *)             echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"; exit 1
        ;;
esac

# Get latest release tag
LATEST=$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')

if [ -z "$LATEST" ]; then
    echo "Failed to fetch latest release"
    exit 1
fi

URL="https://github.com/$REPO/releases/download/$LATEST/mdx-$TARGET.tar.gz"

echo "Downloading mdx $LATEST for $TARGET..."

TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

curl -fsSL "$URL" -o "$TMPDIR/mdx.tar.gz"
tar xzf "$TMPDIR/mdx.tar.gz" -C "$TMPDIR"

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMPDIR/mdx" "$INSTALL_DIR/mdx"
else
    sudo mv "$TMPDIR/mdx" "$INSTALL_DIR/mdx"
fi

echo "mdx $LATEST installed to $INSTALL_DIR/mdx"
echo "Run 'mdx --help' to get started"

# Detect yazi and suggest integration
if command -v yazi >/dev/null 2>&1; then
    echo ""
    echo "Yazi detected! Set up mdx as markdown previewer?"
    printf "  Run: mdx --setup-yazi [Y/n] "
    read -r REPLY
    case "$REPLY" in
        [nN]*) echo "Skipped. You can run 'mdx --setup-yazi' later." ;;
        *)     mdx --setup-yazi ;;
    esac
fi
