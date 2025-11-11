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
FUNCTION_NAME="wt"

echo "🔧 Worktree Installation Script"
echo "================================"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed. Please install Rust first.${NC}"
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Build the project
echo "📦 Building the project..."
cargo build --release

if [ ! -f "target/release/${BINARY_NAME}" ]; then
    echo -e "${RED}Error: Build failed. Binary not found at target/release/${BINARY_NAME}${NC}"
    exit 1
fi

# Create installation directory if it doesn't exist
echo "📁 Creating installation directory: ${INSTALL_DIR}"
mkdir -p "${INSTALL_DIR}"

# Copy binary to installation directory
echo "📋 Installing binary to ${INSTALL_DIR}/${BINARY_NAME}"
cp "target/release/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

# Check if install dir is in PATH
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    echo -e "${YELLOW}Warning: ${INSTALL_DIR} is not in your PATH${NC}"
    echo "The installer will add it to your shell configuration."
fi

# Detect shell and add wrapper function
detect_and_configure_shell() {
    local shell_name=$(basename "$SHELL")
    local config_file=""
    local function_code=""

    case "$shell_name" in
        bash)
            config_file="${HOME}/.bashrc"
            function_code=$(cat << 'EOF'
# Worktree wrapper function - installed by worktree installer
wt() {
    local output
    output=$(worktree "$@")
    local exit_code=$?

    # Check if the output contains a cd command
    if echo "$output" | grep -q "^cd "; then
        # Execute the cd command
        eval "$output"
    else
        echo "$output"
    fi

    return $exit_code
}
EOF
)
            ;;
        zsh)
            config_file="${HOME}/.zshrc"
            function_code=$(cat << 'EOF'
# Worktree wrapper function - installed by worktree installer
wt() {
    local output
    output=$(worktree "$@")
    local exit_code=$?

    # Check if the output contains a cd command
    if echo "$output" | grep -q "^cd "; then
        # Execute the cd command
        eval "$output"
    else
        echo "$output"
    fi

    return $exit_code
}
EOF
)
            ;;
        fish)
            config_file="${HOME}/.config/fish/config.fish"
            mkdir -p "$(dirname "$config_file")"
            function_code=$(cat << 'EOF'
# Worktree wrapper function - installed by worktree installer
function wt
    set output (worktree $argv)
    set exit_code $status

    if string match -q "cd *" $output
        eval $output
    else
        echo $output
    end

    return $exit_code
end
EOF
)
            ;;
        *)
            echo -e "${YELLOW}Warning: Unsupported shell: $shell_name${NC}"
            echo "Please manually add the wrapper function to your shell configuration."
            return 1
            ;;
    esac

    # Check if function already exists in config file
    if [ -f "$config_file" ] && grep -q "# Worktree wrapper function" "$config_file"; then
        echo -e "${YELLOW}ℹ️  Wrapper function already exists in $config_file${NC}"
        echo "Skipping shell configuration..."
        return 0
    fi

    # Add PATH if needed
    local path_export=""
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        path_export="export PATH=\"\${HOME}/.local/bin:\$PATH\""
    fi

    # Add function to config file
    echo ""
    echo "🐚 Configuring $shell_name..."
    echo "   Config file: $config_file"

    {
        echo ""
        if [ -n "$path_export" ]; then
            echo "# Add ~/.local/bin to PATH - installed by worktree installer"
            echo "$path_export"
            echo ""
        fi
        echo "$function_code"
    } >> "$config_file"

    echo -e "${GREEN}✅ Shell configuration updated${NC}"
    return 0
}

# Configure the current shell
if detect_and_configure_shell; then
    echo ""
    echo -e "${GREEN}✅ Installation complete!${NC}"
    echo ""
    echo "📝 Next steps:"
    echo "   1. Restart your shell or run: source ~/.$(basename $SHELL)rc"
    echo "   2. Set the WORKTREE_ROOT_DIR environment variable:"
    echo "      export WORKTREE_ROOT_DIR=\"\$HOME/worktrees\""
    echo "   3. Add that export to your shell config to make it permanent"
    echo "   4. Use the 'wt' command to manage worktrees"
    echo ""
    echo "📚 Example usage:"
    echo "   wt add feature-branch    # Create new worktree"
    echo "   wt list                  # List all worktrees"
    echo "   wt master                # Return to main repo"
    echo "   wt rm feature-branch     # Remove worktree"
    echo ""
else
    echo ""
    echo -e "${YELLOW}⚠️  Manual configuration required${NC}"
    echo ""
    echo "Add this function to your shell configuration:"
    echo ""
    echo "For Bash/Zsh (~/.bashrc or ~/.zshrc):"
    echo "----------------------------------------"
    echo 'wt() {'
    echo '    local output'
    echo '    output=$(worktree "$@")'
    echo '    local exit_code=$?'
    echo '    if echo "$output" | grep -q "^cd "; then'
    echo '        eval "$output"'
    echo '    else'
    echo '        echo "$output"'
    echo '    fi'
    echo '    return $exit_code'
    echo '}'
    echo ""
    echo "For Fish (~/.config/fish/config.fish):"
    echo "----------------------------------------"
    echo 'function wt'
    echo '    set output (worktree $argv)'
    echo '    set exit_code $status'
    echo '    if string match -q "cd *" $output'
    echo '        eval $output'
    echo '    else'
    echo '        echo $output'
    echo '    end'
    echo '    return $exit_code'
    echo 'end'
    echo ""
fi
