#!/bin/bash
# Setup script to configure git hooks
# Run this after cloning the repository

set -e

echo "Setting up git hooks..."

# Configure git to use .githooks directory
git config core.hooksPath .githooks

echo "✓ Git hooks configured successfully!"
echo ""
echo "The following hooks are now active:"
echo "  - pre-commit: Runs rustfmt and clippy"
echo "  - pre-push: Runs test suite"
echo ""
echo "To bypass hooks (use sparingly):"
echo "  git commit --no-verify"
echo "  git push --no-verify"
