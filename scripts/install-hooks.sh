#!/bin/bash
# Install git hooks

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOKS_DIR="$PROJECT_ROOT/.githooks"
GIT_HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo "Installing git hooks..."

# Check if .git directory exists
if [ ! -d "$GIT_HOOKS_DIR" ]; then
    echo "Error: .git/hooks directory not found."
    echo "Are you in a git repository?"
    exit 1
fi

# Check if .githooks directory exists
if [ ! -d "$HOOKS_DIR" ]; then
    echo "Error: .githooks directory not found."
    exit 1
fi

# Install pre-commit hook
if [ -f "$HOOKS_DIR/pre-commit" ]; then
    echo "Installing pre-commit hook..."
    cp "$HOOKS_DIR/pre-commit" "$GIT_HOOKS_DIR/pre-commit"
    chmod +x "$GIT_HOOKS_DIR/pre-commit"
    echo "  ✓ pre-commit hook installed"
else
    echo "Warning: pre-commit hook not found in .githooks/"
fi

# Install pre-push hook
if [ -f "$HOOKS_DIR/pre-push" ]; then
    echo "Installing pre-push hook..."
    cp "$HOOKS_DIR/pre-push" "$GIT_HOOKS_DIR/pre-push"
    chmod +x "$GIT_HOOKS_DIR/pre-push"
    echo "  ✓ pre-push hook installed"
else
    echo "Warning: pre-push hook not found in .githooks/"
fi

echo ""
echo "Git hooks installed successfully!"
echo ""
echo "To skip hooks (emergencies only):"
echo "  git commit --no-verify"
echo "  git push --no-verify"
