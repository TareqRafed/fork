#!/bin/sh
set -e

REPO="TareqRafed/fork"
BIN="fork"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    case "$ARCH" in
      x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
      aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
      arm64)   TARGET="aarch64-unknown-linux-gnu" ;;
      *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
    esac
    EXT="tar.gz"
    ;;
  Darwin)
    case "$ARCH" in
      x86_64) TARGET="x86_64-apple-darwin" ;;
      arm64)  TARGET="aarch64-apple-darwin" ;;
      *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
    esac
    EXT="tar.gz"
    ;;
  *)
    echo "Unsupported OS: $OS"
    echo "On Windows, download the .zip from https://github.com/$REPO/releases/latest"
    exit 1
    ;;
esac

# Resolve version
if [ -z "$VERSION" ]; then
  VERSION="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": "\(.*\)".*/\1/')"
fi

if [ -z "$VERSION" ]; then
  echo "Could not determine latest version. Set VERSION env var manually."
  exit 1
fi

FILENAME="${BIN}-${VERSION}-${TARGET}.${EXT}"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${FILENAME}"

echo "Installing $BIN $VERSION for $TARGET..."

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

curl -fsSL "$URL" -o "$TMP/$FILENAME"
tar -xzf "$TMP/$FILENAME" -C "$TMP"

mkdir -p "$INSTALL_DIR"
install -m 755 "$TMP/$BIN" "$INSTALL_DIR/$BIN"

case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) echo "Note: add $INSTALL_DIR to your PATH if it isn't already" ;;
esac

echo "$BIN $VERSION installed to $INSTALL_DIR/$BIN"
