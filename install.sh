#!/bin/sh
# novos - build at the speed of thought
# POSIX-compliant installer for FreeBSD, OmniOS, and Linux

# Default Configuration
PREFIX="/usr/local"
BINARY_NAME="novos"
GH_LINK="https://github.com/novos-org/novos"
TAG="v0.1.4"

# System Detection
OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

# Map FreeBSD/System architecture to standard naming
if [ "$ARCH_TYPE" = "amd64" ] || [ "$ARCH_TYPE" = "x86_64" ]; then
    ARCH_TYPE="x86_64"
fi

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
    # We calculate these on the fly so they reflect manual changes
    echo "--- System Information ---"
    echo "Detected OS:    $OS_TYPE"
    echo "Detected Arch:  $ARCH_TYPE"
    echo "Target Binary:  novos-${OS_TYPE}-${ARCH_TYPE}"
    echo "Install Path:   $PREFIX/bin/$BINARY_NAME"
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
    # Re-evaluate variables at time of execution
    TARGET_BINARY="novos-${OS_TYPE}-${ARCH_TYPE}"
    FULL_PATH="$PREFIX/bin/$BINARY_NAME"
    
    echo "Installing $BINARY_NAME for $OS_TYPE ($ARCH_TYPE)..."
    
    # Ensure directory exists
    mkdir -p "$PREFIX/bin" || { echo "Error: Cannot create $PREFIX/bin. Try sudo?"; exit 1; }

    if ! command -v curl >/dev/null 2>&1; then
        echo "Error: curl is required but not installed."
        exit 1
    fi

    curl -s -A "Mozilla/5.0" -L "$GH_LINK/releases/download/$TAG/$TARGET_BINARY" -o "$FULL_PATH"
    
    if [ $? -eq 0 ]; then
        chmod +x "$FULL_PATH"
        echo "Successfully installed to $FULL_PATH"
        echo "Build at the speed of thought."
    else
        echo "Download failed. Please check your connection or the TAG version."
        exit 1
    fi
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
    read -r sel
    if [ -z "$sel" ]; then sel=1; fi

    case "$sel" in
        1)
            perform_install
            exit 0
            ;;
        2)
            printf "New Prefix: "
            read -r PREFIX
            ;;
        3)
            printf "New Binary Name: "
            read -r BINARY_NAME
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
