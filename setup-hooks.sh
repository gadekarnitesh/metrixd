#!/bin/bash

# Setup script for MetrixD git hooks
# This script configures git to use the custom pre-commit hook

set -e

echo "🔧 Setting up git hooks for MetrixD..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

# Check if .githooks directory exists
if [ ! -d ".githooks" ]; then
    echo "❌ Error: .githooks directory not found"
    exit 1
fi

# Check if pre-commit hook exists
if [ ! -f ".githooks/pre-commit" ]; then
    echo "❌ Error: .githooks/pre-commit not found"
    exit 1
fi

# Make sure the hook is executable
chmod +x .githooks/pre-commit
print_info "Made pre-commit hook executable"

# Configure git to use the custom hooks directory
git config core.hooksPath .githooks
print_success "Configured git to use .githooks directory"

# Install required Rust components if not already installed
print_info "Checking Rust components..."

if ! cargo fmt --version &> /dev/null; then
    print_info "Installing rustfmt..."
    rustup component add rustfmt
    print_success "rustfmt installed"
else
    print_success "rustfmt already available"
fi

if ! cargo clippy --version &> /dev/null; then
    print_info "Installing clippy..."
    rustup component add clippy
    print_success "clippy installed"
else
    print_success "clippy already available"
fi

print_success "Git hooks setup complete! ✨"
echo ""
echo "📋 What happens now:"
echo "  • Every commit will run formatting checks (cargo fmt)"
echo "  • Clippy lints will be checked for warnings"
echo "  • All tests must pass before committing"
echo "  • Common issues will be detected (TODO comments, debug prints)"
echo ""
echo "💡 Tips:"
echo "  • Run 'cargo fmt --all' to fix formatting issues"
echo "  • Run 'cargo clippy --fix' to auto-fix some clippy warnings"
echo "  • Use 'git commit --no-verify' to bypass hooks (not recommended)"
echo ""
print_success "Happy coding! 🚀"
