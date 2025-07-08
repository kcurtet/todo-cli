#!/bin/bash
# Build script for todo-cli that generates completions

set -e

echo "Building todo-cli..."

# Build the release binary
cargo build --release

echo "Generating shell completions..."

# Create completions directory if it doesn't exist
mkdir -p completions

# Generate completions for all supported shells
./target/release/todo-cli completions bash > completions/todo.bash
./target/release/todo-cli completions zsh > completions/todo.zsh
./target/release/todo-cli completions fish > completions/todo.fish

echo "✓ Generated completions for bash, zsh, and fish"
echo "✓ Build complete!"
echo ""
echo "To install completions, run: ./scripts/install-completions.sh"
echo "Binary location: ./target/release/todo-cli"
