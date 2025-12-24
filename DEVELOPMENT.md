# Development Guide

This guide covers the development workflow for Bury.

## Quick Start

```bash
# Clone and setup
git clone https://github.com/paolorechia/bury
cd bury
./scripts/setup.sh

# Or manually:
make install
make pre-commit-install
```

## Development Workflow

### 1. Format Code
```bash
make fmt
# or
cargo fmt
```

### 2. Lint Code
```bash
make lint
# or
cargo clippy --all-targets --all-features -- -D warnings
```

### 3. Check Compilation
```bash
make check
# or
cargo check
```

### 4. Run Tests
```bash
make test
# or
cargo test
```

### 5. Analyze Complexity
```bash
make complexity
# or
pmat --threshold 10 src/
```

### 6. Run All Checks
```bash
make all
```

## Pre-commit Hooks

Pre-commit hooks run automatically before each commit. They will:

1. ✅ Format code with `rustfmt`
2. ✅ Lint code with `clippy`
3. ✅ Check compilation
4. ✅ Run unit tests
5. ⚠️  Analyze complexity (warning only)

### Install Pre-commit Hooks
```bash
pip install pre-commit
pre-commit install
```

### Run Manually
```bash
pre-commit run --all-files
```

### Skip Hooks (Not Recommended)
```bash
git commit --no-verify
```

## Make Targets

Run `make help` to see all available targets:

```
make help              Show help
make install           Install dependencies and tools
make fmt               Format code
make lint              Lint code with clippy
make check             Check compilation
make test              Run unit tests
make complexity        Analyze code complexity
make all               Run all checks
make clean             Clean build artifacts
make build-release     Build release binary
make pre-commit-install Install pre-commit hooks
make pre-commit-run    Run pre-commit on all files
make ci                Run CI pipeline locally
make watch             Watch for changes and run tests
make dev               Development mode (auto-format, lint, test)
```

## Continuous Integration

GitHub Actions runs on every push and PR:

- ✅ Format check
- ✅ Clippy lint
- ✅ Compilation check
- ✅ Unit tests (Ubuntu, macOS, Windows)
- ✅ Complexity analysis (warning only)
- ✅ Code coverage
- ✅ Release builds

## Code Style

### Rust
- Use `rustfmt` for formatting (enforced by CI)
- Follow Rust naming conventions
- Use `clippy` recommendations
- Keep functions under 50 lines when possible
- Keep cyclomatic complexity under 10

### Commits
- Use conventional commits format:
  - `feat:` - New feature
  - `fix:` - Bug fix
  - `docs:` - Documentation
  - `refactor:` - Code refactoring
  - `test:` - Tests
  - `chore:` - Maintenance

Example:
```
feat: add Python parser for class definitions

- Implement AST traversal for classes
- Extract method definitions
- Add tests for class parsing
```

## Testing

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test '*'
```

### Test Coverage
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --all-features
```

### Test Specific Module
```bash
cargo test parser::python
```

## Debugging

### Verbose Output
```bash
./target/debug/bury --verbose analyze ./tests/fixtures/
```

### Enable Rust Backtrace
```bash
RUST_BACKTRACE=1 cargo test
RUST_BACKTRACE=full cargo run
```

## Performance Profiling

### Build with Optimizations
```bash
cargo build --release
```

### Benchmark (Future)
```bash
cargo bench
```

## Tools

### Required
- Rust 1.70+ (`rustup`)
- `rustfmt` (`rustup component add rustfmt`)
- `clippy` (`rustup component add clippy`)

### Optional
- `pre-commit` (`pip install pre-commit`)
- `pmat` (`cargo install pmat`) - Complexity analysis
- `cargo-watch` (`cargo install cargo-watch`) - Auto-rebuild
- `cargo-tarpaulin` (`cargo install cargo-tarpaulin`) - Coverage

## Troubleshooting

### Pre-commit Fails
```bash
# Run checks manually to see details
make all

# Update pre-commit hooks
pre-commit autoupdate
```

### Clippy Warnings
```bash
# See all warnings
cargo clippy --all-targets --all-features

# Allow specific warning (not recommended)
#[allow(clippy::warning_name)]
```

### Test Failures
```bash
# Run specific test with output
cargo test test_name -- --nocapture --test-threads=1
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Questions?

Open an issue or discussion on GitHub!
