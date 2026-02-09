#!/bin/sh
set -e

BIN_NAME="novos"
DIST_DIR="dist"
RELEASE_BIN="target/release/$BIN_NAME"

# Detect Metadata
VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d '"' -f 2)
TARGET_OS=$(uname -s | tr '[:upper:]' '[:lower:]')
TARGET_ARCH=$(uname -m)
PKG_NAME="$BIN_NAME-$TARGET_ARCH-$TARGET_OS.tar.xz"

case "$1" in
    build)
        echo "Building $BIN_NAME v$VERSION..."
        cargo build --release
        echo "Stripping binary..."
        strip "$RELEASE_BIN"
        ;;
    dist)
        "$0" build
        mkdir -p "$DIST_DIR"
        echo "Packaging for $TARGET_OS..."
        tar -cJvf "$DIST_DIR/$PKG_NAME" -C target/release "$BIN_NAME"
        
        cd "$DIST_DIR"
        sha256sum "$PKG_NAME" > "$PKG_NAME".sha256
        sha512sum "$PKG_NAME" > "$PKG_NAME".sha512
        sha1sum   "$PKG_NAME" > "$PKG_NAME".sha1
        
        if command -v b3sum >/dev/null; then
            b3sum "$PKG_NAME" > "$PKG_NAME".b3sum
        else
            echo "Warning: b3sum not found, skipping..."
        fi
        ;;
    clean)
        cargo clean
        rm -rf "$DIST_DIR"
        ;;
    *)
        echo "Usage: $0 {build|dist|clean}"
        exit 1
        ;;
esac
