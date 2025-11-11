#!/usr/bin/env bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Installation configuration
BINARY_NAME="worktree"
INSTALL_DIR="${HOME}/.local/bin"

echo "🗑️  Worktree Uninstallation Script"
echo "==================================="
echo ""

# Remove binary
if [ -f "${INSTALL_DIR}/${BINARY_NAME}" ]; then
    echo "🗑️  Removing binary: ${INSTALL_DIR}/${BINARY_NAME}"
    rm "${INSTALL_DIR}/${BINARY_NAME}"
    echo -e "${GREEN}✅ Binary removed${NC}"
else
    echo -e "${YELLOW}ℹ️  Binary not found at ${INSTALL_DIR}/${BINARY_NAME}${NC}"
fi

# Function to remove configuration from a file
remove_from_config() {
    local config_file=$1
    local shell_name=$2

    if [ ! -f "$config_file" ]; then
        echo -e "${YELLOW}ℹ️  Config file not found: $config_file${NC}"
        return
    fi

    if ! grep -q "# Worktree wrapper function" "$config_file"; then
        echo -e "${YELLOW}ℹ️  No worktree configuration found in $config_file${NC}"
        return
    fi

    echo "🧹 Removing configuration from $config_file"

    # Create a temporary file
    local temp_file=$(mktemp)

    # Remove the worktree sections
    # This removes lines between the marker comments and the function/export
    awk '
        /# Add ~\/.local\/bin to PATH - installed by worktree installer/ {
            skip = 1
            next
        }
        /# Worktree wrapper function - installed by worktree installer/ {
            skip = 1
            next
        }
        skip && /^export PATH=.*\.local\/bin/ {
            next
        }
        skip && /^wt\(\)/ {
            in_function = 1
            next
        }
        skip && in_function && /^}$/ {
            skip = 0
            in_function = 0
            next
        }
        skip && /^function wt$/ {
            in_function = 1
            next
        }
        skip && in_function && /^end$/ {
            skip = 0
            in_function = 0
            next
        }
        in_function {
            next
        }
        skip {
            next
        }
        {
            print
        }
    ' "$config_file" > "$temp_file"

    # Replace original file with cleaned version
    mv "$temp_file" "$config_file"

    echo -e "${GREEN}✅ Configuration removed from $config_file${NC}"
}

# Detect shell and remove configuration
echo ""
echo "🐚 Detecting shell configuration..."

shell_name=$(basename "$SHELL")

case "$shell_name" in
    bash)
        remove_from_config "${HOME}/.bashrc" "bash"
        ;;
    zsh)
        remove_from_config "${HOME}/.zshrc" "zsh"
        ;;
    fish)
        remove_from_config "${HOME}/.config/fish/config.fish" "fish"
        ;;
    *)
        echo -e "${YELLOW}⚠️  Unsupported shell: $shell_name${NC}"
        echo "Please manually remove the 'wt' function from your shell configuration."
        ;;
esac

echo ""
echo -e "${GREEN}✅ Uninstallation complete!${NC}"
echo ""
echo "📝 Note: You may need to restart your shell or run:"
echo "   source ~/.$(basename $SHELL)rc"
echo ""
echo "💡 If you set WORKTREE_ROOT_DIR in your shell config, you may want to remove that too."
echo ""
