#!/bin/bash
set -e

REPO="tusharkhatriofficial/dotkeep"
LATEST=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" \
  | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS-$ARCH" in
  linux-x86_64)  TARGET="x86_64-unknown-linux-musl" ;;
  darwin-x86_64) TARGET="x86_64-apple-darwin" ;;
  darwin-arm64)  TARGET="aarch64-apple-darwin" ;;
  *) echo "Unsupported: $OS-$ARCH"; exit 1 ;;
esac

URL="https://github.com/$REPO/releases/download/$LATEST/dotkeep-$TARGET.tar.gz"
echo "Downloading dotkeep $LATEST for $TARGET..."
curl -sL "$URL" | tar xz -C /tmp/
chmod +x /tmp/dotkeep

DEST="/usr/local/bin/dotkeep"
if [ -w "$(dirname $DEST)" ]; then
  mv /tmp/dotkeep "$DEST"
else
  sudo mv /tmp/dotkeep "$DEST"
fi

echo "Installed dotkeep to $DEST"
dotkeep --help