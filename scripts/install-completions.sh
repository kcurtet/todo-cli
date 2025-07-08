#!/bin/bash
# Installation script for todo-cli shell completions

set -e

COMPLETIONS_DIR="$(dirname "$0")/../completions"
SCRIPT_NAME="todo"

echo "Installing todo-cli shell completions..."

# Function to detect the user's shell
detect_shell() {
    local shell_name
    shell_name=$(basename "$SHELL")
    echo "$shell_name"
}

# Function to get completion directory for different shells
get_completion_dir() {
    local shell=$1
    case "$shell" in
        bash)
            # Try common bash completion directories
            if [[ -d "/usr/share/bash-completion/completions" ]]; then
                echo "/usr/share/bash-completion/completions"
            elif [[ -d "/usr/local/share/bash-completion/completions" ]]; then
                echo "/usr/local/share/bash-completion/completions"
            elif [[ -d "$HOME/.local/share/bash-completion/completions" ]]; then
                echo "$HOME/.local/share/bash-completion/completions"
            else
                # Create user-local directory
                mkdir -p "$HOME/.local/share/bash-completion/completions"
                echo "$HOME/.local/share/bash-completion/completions"
            fi
            ;;
        zsh)
            # For zsh, we'll install to a user directory and add it to fpath
            local zsh_completions_dir="$HOME/.zsh/completions"
            mkdir -p "$zsh_completions_dir"
            echo "$zsh_completions_dir"
            ;;
        fish)
            # Fish completions go to the user config directory
            local fish_completions_dir="$HOME/.config/fish/completions"
            mkdir -p "$fish_completions_dir"
            echo "$fish_completions_dir"
            ;;
        *)
            echo ""
            ;;
    esac
}

# Function to install completions for a specific shell
install_completion() {
    local shell=$1
    local completion_file="$COMPLETIONS_DIR/$SCRIPT_NAME.$shell"
    local completion_dir
    
    if [[ ! -f "$completion_file" ]]; then
        echo "Warning: Completion file $completion_file not found"
        return 1
    fi
    
    completion_dir=$(get_completion_dir "$shell")
    
    if [[ -z "$completion_dir" ]]; then
        echo "Warning: Could not determine completion directory for $shell"
        return 1
    fi
    
    # Copy the completion file
    if [[ -w "$completion_dir" ]] || [[ "$completion_dir" == "$HOME"* ]]; then
        cp "$completion_file" "$completion_dir/"
        echo "✓ Installed $shell completions to $completion_dir/"
        
        # Special handling for zsh
        if [[ "$shell" == "zsh" ]]; then
            echo ""
            echo "For zsh completions to work, add this line to your ~/.zshrc:"
            echo "fpath=(~/.zsh/completions \$fpath)"
            echo "autoload -U compinit && compinit"
        fi
        
        return 0
    else
        echo "Warning: No write permission to $completion_dir"
        echo "You may need to run this script with sudo or copy manually:"
        echo "sudo cp $completion_file $completion_dir/"
        return 1
    fi
}

# Main installation logic
main() {
    local shell_arg=""
    local install_all=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --shell)
                shell_arg="$2"
                shift 2
                ;;
            --all)
                install_all=true
                shift
                ;;
            --help|-h)
                echo "Usage: $0 [--shell SHELL] [--all] [--help]"
                echo ""
                echo "Options:"
                echo "  --shell SHELL    Install completions for specific shell (bash, zsh, fish)"
                echo "  --all           Install completions for all supported shells"
                echo "  --help          Show this help message"
                echo ""
                echo "If no options are provided, completions will be installed for your current shell."
                exit 0
                ;;
            *)
                echo "Unknown option: $1"
                exit 1
                ;;
        esac
    done
    
    if [[ "$install_all" == true ]]; then
        echo "Installing completions for all supported shells..."
        for shell in bash zsh fish; do
            echo ""
            echo "Installing $shell completions..."
            install_completion "$shell"
        done
    elif [[ -n "$shell_arg" ]]; then
        echo "Installing completions for $shell_arg..."
        install_completion "$shell_arg"
    else
        # Install for current shell
        local current_shell
        current_shell=$(detect_shell)
        echo "Detected shell: $current_shell"
        echo "Installing completions for $current_shell..."
        install_completion "$current_shell"
    fi
    
    echo ""
    echo "Installation complete!"
    echo "You may need to restart your shell or run 'source ~/.bashrc' (or equivalent) for completions to take effect."
}

# Check if completions directory exists
if [[ ! -d "$COMPLETIONS_DIR" ]]; then
    echo "Error: Completions directory not found at $COMPLETIONS_DIR"
    echo "Please run this script from the todo-cli project directory."
    exit 1
fi

main "$@"
