#!/bin/bash
set -e

# bgustreadimg - Universal Installer
# Compatible with direct curl execution:
# curl -sSf https://raw.githubusercontent.com/B-GUST/bgustreadimg/main/install.sh | sh

echo "========================================="
echo "   BGUSTREADIMG - UNIVERSAL INSTALLER    "
echo "========================================="
echo ""

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     echo "System: Linux detected";;
    Darwin*)    echo "System: macOS detected";;
    *)          echo "System: Other (${OS}) detected";;
esac

# Check dependencies
HAS_NODE=0
HAS_PIP=0
HAS_CARGO=0

if command -v node >/dev/null 2>&1; then
    HAS_NODE=1
    NODE_VERSION=$(node -v)
    echo "✔ Found Node.js: ${NODE_VERSION}"
fi

if command -v pip >/dev/null 2>&1; then
    HAS_PIP=1
    PIP_VERSION=$(pip --version | awk '{print $2}')
    echo "✔ Found Python pip: ${PIP_VERSION}"
fi

if command -v cargo >/dev/null 2>&1; then
    HAS_CARGO=1
    CARGO_VERSION=$(cargo --version | awk '{print $2}')
    echo "✔ Found Rust cargo: ${CARGO_VERSION}"
fi

echo ""
echo "Select installation target:"
echo "1) NPM Package (Node.js Backend native addon)"
echo "2) NPM Package (JS WebAssembly/Browser module)"
echo "3) PyPI Package (Python Module via pip)"
echo "4) Clone & Build from source (Rust & Cargo)"
echo "5) Exit"
echo ""

# Default to auto-detect if run non-interactively
if [ ! -t 0 ]; then
    echo "Non-interactive shell detected. Auto-installing based on available tools..."
    if [ ${HAS_NODE} -eq 1 ]; then
        CHOICE=1
    elif [ ${HAS_PIP} -eq 1 ]; then
        CHOICE=3
    elif [ ${HAS_CARGO} -eq 1 ]; then
        CHOICE=4
    else
        echo "Error: No compatible developer environment (Node, Pip, Cargo) found."
        exit 1
    fi
else
    read -p "Enter choice [1-5]: " CHOICE
fi

case $CHOICE in
    1)
        echo "Installing NPM Backend package..."
        if [ ${HAS_NODE} -eq 1 ]; then
            npm install bgustreadimg
            echo "✔ bgustreadimg successfully installed in node_modules."
        else
            echo "Error: Node.js/NPM is not installed."
            exit 1
        fi
        ;;
    2)
        echo "Installing NPM WebAssembly package..."
        if [ ${HAS_NODE} -eq 1 ]; then
            npm install bgustreadimg-wasm
            echo "✔ bgustreadimg-wasm successfully installed in node_modules."
        else
            echo "Error: Node.js/NPM is not installed."
            exit 1
        fi
        ;;
    3)
        echo "Installing Python package..."
        if [ ${HAS_PIP} -eq 1 ]; then
            pip install bgustreadimg --break-system-packages || pip install bgustreadimg
            echo "✔ bgustreadimg successfully installed in Python site-packages."
        else
            echo "Error: Python/pip is not installed."
            exit 1
        fi
        ;;
    4)
        echo "Cloning and building from source..."
        if [ ${HAS_CARGO} -eq 1 ]; then
            if command -v git >/dev/null 2>&1; then
                git clone https://github.com/B-GUST/bgustreadimg.git
                cd bgustreadimg
                cargo build --release
                echo "✔ Compiled release binary at: target/release/libbgustreadimg.so"
            else
                echo "Error: Git is required to clone the source."
                exit 1
            fi
        else
            echo "Error: Rust/Cargo is not installed."
            exit 1
        fi
        ;;
    *)
        echo "Installation aborted."
        exit 0
        ;;
esac

echo "========================================="
echo "Installation complete!"
echo "========================================="
