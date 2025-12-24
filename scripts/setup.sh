#!/bin/bash
# Setup development environment for Bury

set -e

echo "🚀 Setting up Bury development environment..."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust is installed"

# Install Rust components
echo "📦 Installing Rust components..."
rustup component add rustfmt clippy

# Try to install pmat (optional)
echo "📦 Installing pmat (code complexity analyzer)..."
cargo install pmat 2>/dev/null || echo "⚠️  pmat installation skipped (optional)"

# Check for Python/pip for pre-commit
if command -v pip &> /dev/null || command -v pip3 &> /dev/null; then
    echo "📦 Installing pre-commit..."
    pip install pre-commit 2>/dev/null || pip3 install pre-commit 2>/dev/null || echo "⚠️  pre-commit installation failed"
    
    if command -v pre-commit &> /dev/null; then
        echo "🪝 Installing pre-commit hooks..."
        pre-commit install
        echo "✅ Pre-commit hooks installed!"
    fi
else
    echo "⚠️  Python/pip not found. Skipping pre-commit installation."
    echo "   To install pre-commit: pip install pre-commit && pre-commit install"
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "Available commands:"
echo "  make help              - Show all available make targets"
echo "  make fmt               - Format code"
echo "  make lint              - Run clippy linter"
echo "  make test              - Run tests"
echo "  make all               - Run all checks"
echo "  cargo run -- analyze . - Run bury"
echo ""
echo "Pre-commit hooks will run automatically on git commit"
echo "To run manually: pre-commit run --all-files"
echo ""
echo "Happy coding! 🎉"
