# Makefile for Bury development

.PHONY: help install fmt lint check test all clean pre-commit-install pre-commit-run

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install dependencies and tools
	@echo "📦 Installing Rust tools..."
	rustup component add rustfmt clippy
	cargo install pmat || echo "⚠️  pmat installation failed (optional)"
	@echo "📦 Installing pre-commit..."
	pip install pre-commit || echo "⚠️  pre-commit requires Python/pip"
	@echo "✅ Installation complete!"

fmt: ## Format code with rustfmt
	@echo "🎨 Formatting Rust code..."
	cargo fmt --all
	@echo "✅ Formatting complete!"

lint: ## Lint code with clippy
	@echo "🔍 Linting with clippy..."
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Linting complete!"

check: ## Check code compiles
	@echo "🔨 Checking compilation..."
	cargo check --all-targets --all-features
	@echo "✅ Compilation check complete!"

test: ## Run unit tests
	@echo "🧪 Running tests..."
	cargo test --all-features
	@echo "✅ Tests complete!"

complexity: ## Analyze code complexity
	@echo "📊 Analyzing complexity..."
	pmat --threshold 10 src/ || echo "⚠️  High complexity detected"
	@echo "✅ Complexity analysis complete!"

all: fmt lint check test ## Run all checks (fmt, lint, check, test)
	@echo "✅ All checks passed!"

clean: ## Clean build artifacts
	@echo "🧹 Cleaning..."
	cargo clean
	@echo "✅ Clean complete!"

build-release: ## Build release binary
	@echo "🚀 Building release..."
	cargo build --release
	@echo "✅ Release build complete!"
	@echo "Binary: ./target/release/bury"

pre-commit-install: ## Install pre-commit hooks
	@echo "🪝 Installing pre-commit hooks..."
	pre-commit install
	@echo "✅ Pre-commit hooks installed!"

pre-commit-run: ## Run pre-commit on all files
	@echo "🪝 Running pre-commit checks..."
	pre-commit run --all-files
	@echo "✅ Pre-commit checks complete!"

ci: fmt lint check test ## Run CI pipeline locally
	@echo "✅ CI pipeline passed!"

watch: ## Watch for changes and run tests
	@echo "👀 Watching for changes..."
	cargo watch -x test

dev: ## Start development mode (format on save + tests)
	@echo "🔧 Starting development mode..."
	cargo watch -x fmt -x clippy -x test
