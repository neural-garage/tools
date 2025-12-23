# Contributing to Bury

Thank you for your interest in contributing to Bury! This document provides guidelines for contributing.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/bury`
3. Create a feature branch: `git checkout -b my-feature`
4. Make your changes
5. Test your changes: `cargo test && cargo build`
6. Commit your changes: `git commit -m "Add my feature"`
7. Push to your fork: `git push origin my-feature`
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- Git

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- analyze ./src --verbose
```

## Code Style

- Use `cargo fmt` to format code
- Use `cargo clippy` to lint code
- Follow Rust naming conventions

## Areas for Contribution

We especially welcome contributions in:

### 1. Parser Implementations
- Implement tree-sitter AST traversal for Python
- Implement tree-sitter AST traversal for TypeScript
- Add support for additional languages

### 2. Analysis Engine
- Improve reachability analysis algorithm
- Add support for dynamic imports
- Handle edge cases (eval, getattr, etc.)

### 3. Testing
- Add test fixtures (Python/TypeScript code samples)
- Write integration tests
- Add benchmarks

### 4. Documentation
- Improve README
- Add usage examples
- Write tutorials

### 5. Features
- Configuration file parsing
- Entry point auto-detection
- CI/CD integrations

## Pull Request Guidelines

- Keep PRs focused on a single feature or bug fix
- Include tests for new functionality
- Update documentation as needed
- Ensure `cargo test` and `cargo clippy` pass
- Write clear commit messages

## License

By contributing, you agree that your contributions will be dual-licensed under MIT and Apache-2.0, matching the project's license.

## Questions?

Open an issue or start a discussion on GitHub!
