#!/bin/sh

set -e

BIN_DIR="${BIN_DIR:-/usr/local/bin}"
REPO=https://github.com/leocm889/todo_list.git

echo "CLoning repo..."
git clone "$REPO" "$HOME/.todo_list"

echo "Building project..."
cd "$HOME/.todo_list"
cargo build --release

echo "Instally binary to $BIN_DIR..."
mkdir -p "$BIND_DIR"
cp target/release/todo "$BIND_DIR"

echo "✅ Installation complete! Run 'todo --help'"
# VERSION="${VERSION:-latest}"
#
# # Detect OS/Arch
# OS=$(uname -s | tr '[:upper:]' '[:lower:]')
# ARCH=$(uname -m)
#
# case "$ARCH" in
# x86_64) ARCH="x86_64" ;;
# aarch64 | arm64) ARCH="aarch64" ;;
# *)
#   echo "Unsupported architecture: $ARCH"
#   exit 1
#   ;;
# esac
#
# if [ "$VERSION" = "latest" ]; then
#   VERSION=$(curl -s https://api.github.com/repos/$REPO/releases/latest |
#     grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
# fi
#
# TARBALL="todo-${VERSION}-${ARCH}-${OS}.tar.gz"
# URL="https://github.com/$REPO/releases/download/${VERSION}/${TARBALL}"
#
# echo "Downloading $URL..."
# curl -fsSL "$URL" -o /tmp/todo.tar.gz
#
# echo "Installing to $BIN_DIR..."
# mkdir -p "$BIN_DIR"
# tar -xzf /tmp/todo.tar.gz -C "$BIN_DIR"
#
# echo "✅ Installation complete! Run 'todo --help'"
