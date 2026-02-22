#!/bin/bash
set -e

REPO="tusharkhatriofficial/envkeep"
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

URL="https://github.com/$REPO/releases/download/$LATEST/envkeep-$TARGET.tar.gz"
echo "Downloading envkeep $LATEST for $TARGET..."
curl -sL "$URL" | tar xz -C /tmp/
chmod +x /tmp/envkeep

DEST="/usr/local/bin/envkeep"
if [ -w "$(dirname $DEST)" ]; then
  mv /tmp/envkeep "$DEST"
else
  sudo mv /tmp/envkeep "$DEST"
fi

echo "Installed envkeep to $DEST"
envkeep --help