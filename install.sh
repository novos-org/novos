#!/bin/sh
# novos - build at the speed of thought
# POSIX-compliant installer for FreeBSD, OmniOS, and Linux

# Default Configuration
PREFIX="/usr/local"
BINARY_NAME="novos"
GH_LINK="https://github.com/novos-org/novos"

# System Detection
OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

# Map FreeBSD/System architecture to standard naming
if [ "$ARCH_TYPE" = "amd64" ] || [ "$ARCH_TYPE" = "x86_64" ]; then
    ARCH_TYPE="x86_64"
fi

TARGET_BINARY="novos-${OS_TYPE}-${ARCH_TYPE}"

show_help() {
    cat <<EOF
novos installer
Usage: install.sh [options]

Options:
  --prefix <path>       Set installation prefix (default: $PREFIX)
  --binary-name <name>  Set binary name (default: $BINARY_NAME)
  --info                Show detected system information
  --ghub-link           Show GitHub repository link
  --help                Show this help message
EOF
}

show_info() {
    echo "--- System Information ---"
    echo "Detected OS:   $OS_TYPE"
    echo "Detected Arch: $ARCH_TYPE"
    echo "Target Binary: $TARGET_BINARY"
    echo "Install Path:  $PREFIX/bin/$BINARY_NAME"
    echo "--------------------------"
}

# Manual Flag Parsing
while [ $# -gt 0 ]; do
    case "$1" in
        --prefix)
            PREFIX="$2"
            shift 2
            ;;
        --binary-name)
            BINARY_NAME="$2"
            shift 2
            ;;
        --info)
            show_info
            exit 0
            ;;
        --ghub-link)
            echo "$GH_LINK"
            exit 0
            ;;
        --help)
            show_help
            exit 0
            ;;
        *)
            echo "Error: Unknown option $1"
            show_help
            exit 1
            ;;
    esac
done

perform_install() {
    echo "Installing $BINARY_NAME for $OS_TYPE ($ARCH_TYPE)..."
    
    curl -L "$GH_LINK/releases/latest/download/$TARGET_BINARY" -o "$PREFIX/bin/$BINARY_NAME"
    chmod +x "$PREFIX/bin/$BINARY_NAME"
    
    echo "Successfully installed to $PREFIX/bin/$BINARY_NAME"
    echo "Build at the speed of thought."
}

show_menu() {
    echo ""
    echo "  novos installer"
    echo "  1) Proceed with installation (Default)"
    echo "  2) Customize Prefix (Current: $PREFIX)"
    echo "  3) Customize Binary Name (Current: $BINARY_NAME)"
    echo "  4) Show System Info"
    echo "  5) Abort"
    echo ""
    printf "Selection: "
}

# Main Loop
while true; do
    show_menu
    read sel
    # Handle default enter press
    if [ -z "$sel" ]; then sel=1; fi

    case "$sel" in
        1)
            perform_install
            exit 0
            ;;
        2)
            printf "New Prefix: "
            read PREFIX
            ;;
        3)
            printf "New Binary Name: "
            read BINARY_NAME
            ;;
        4)
            show_info
            ;;
        5)
            echo "Installation cancelled."
            exit 0
            ;;
        *)
            echo "Invalid selection: $sel"
            ;;
    esac
done
