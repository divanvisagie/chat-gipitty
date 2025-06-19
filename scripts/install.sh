#!/bin/sh
set -e

UPDATE=0
if [ "$1" = "--update" ]; then
  UPDATE=1
fi

BINARY_PATH="/usr/local/bin/cgip"
MAN_DIR="/usr/local/share/man/man1"
MAN_PATH="$MAN_DIR/cgip.1"

if [ -f "$BINARY_PATH" ] && [ "$UPDATE" -eq 0 ]; then
  echo "cgip already installed at $BINARY_PATH"
  echo "Use --update to replace the existing installation." >&2
  exit 0
fi

PLATFORM=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
case "$ARCH" in
  x86_64|amd64)
    ARCH="x86_64"
    ;;
  arm64|aarch64)
    ARCH="aarch64"
    ;;
  *)
    echo "Unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

ASSET="cgip-${PLATFORM}-${ARCH}.tar.gz"
URL="https://github.com/divanvisagie/chat-gipity/releases/latest/download/${ASSET}"

TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading $ASSET from $URL" 
if ! curl -fL "$URL" -o "$TMP_DIR/$ASSET"; then
  echo "Failed to download release asset" >&2
  exit 1
fi

tar -xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR"

install -d "$MAN_DIR"
install -m 755 "$TMP_DIR/cgip" "$BINARY_PATH"
install -m 644 "$TMP_DIR/cgip.1" "$MAN_PATH"

echo "cgip installed to $BINARY_PATH"
