# Neural Garage Tools 🧠🔧

> AI-powered code analysis tools built in Rust

Part of the [Neural Garage](https://github.com/neural-garage) suite of developer tools for the AI era.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/neural-garage/tools/workflows/CI/badge.svg)](https://github.com/neural-garage/tools/actions)

## Tools

This monorepo contains multiple code analysis tools:

### bury 🪦 - Dead Code Detector

**Bury the dead code before it haunts your codebase!**

Finds unused code in your Python and TypeScript projects using reachability analysis.

### bury 🪦 - Dead Code Detector

**Bury the dead code before it haunts your codebase!**

Finds unused code in your Python and TypeScript projects using reachability analysis.

#### Key Features

- 🚀 **Blazingly Fast** - Written in Rust with parallel processing
- 🎯 **Accurate** - Uses reachability analysis, not simple pattern matching
- 🌍 **Multi-Language** - Supports Python and TypeScript (more coming!)
- 🤖 **LLM-Friendly** - Outputs structured JSON perfect for AI tools
- ⚙️ **Configurable** - Define entry points and ignore patterns
- 📊 **Multiple Output Formats** - JSON, Markdown, or terminal

### complexity 📊 - Complexity Analyzer (Coming Soon)

Analyzes code complexity using cyclomatic and cognitive complexity metrics.

## Repository Structure

This is a Cargo workspace containing multiple crates:

```
neural-garage/tools/
├── crates/
│   ├── shared/          # Shared library (neural-shared)
│   │   ├── parser/      # AST parsing (tree-sitter)
│   │   ├── scanner/     # File discovery
│   │   └── report/      # Output generation
│   │
│   ├── bury/            # Dead code detector
│   │   ├── analyzer/    # Reachability analysis
│   │   └── cli/         # CLI interface
│   │
│   └── complexity/      # Complexity analyzer (WIP)
│       └── analyzer/    # Complexity metrics
│
├── Cargo.toml           # Workspace configuration
└── README.md
```

## Installation

```bash
# Install bury
cargo install --path crates/bury

# Install complexity (when ready)
cargo install --path crates/complexity

# Or build all tools
cargo build --workspace --release
```

## Quick Start

### Using bury

```bash
# Analyze current directory
bury

# Analyze specific path
bury ./src

# Output as JSON
bury --format json ./src

# Verbose mode
bury --verbose ./src
```

## How Bury Works

Bury uses a three-phase reachability analysis:

1. **Scan** - Find all source files (respecting .gitignore)
2. **Parse** - Build AST using tree-sitter for each language
3. **Analyze** - Perform reachability analysis from entry points
4. **Report** - Output dead code findings

### Reachability Analysis

```
Entry Points (main, tests, exports)
    ↓
Build Call Graph (function → callees)
    ↓
Mark Reachable Code (BFS/DFS traversal)
    ↓
Dead Code = Definitions - Reachable
```

### Example

```python
# module.py

class Calculator:
    def add(self, a, b):      # ✅ Used
        return a + b
    
    def multiply(self, a, b):  # ❌ DEAD CODE
        return a * b

def main():
    calc = Calculator()
    result = calc.add(1, 2)  # Only calls add()
```

Output:
```json
{
  "dead_code": [
    {
      "kind": "Method",
      "name": "multiply",
      "file": "module.py",
      "line": 6,
      "reason": "Not reachable from any entry point",
      "confidence": "High"
    }
  ]
}
```

## Configuration

Create a `.bury.json` file:

```json
{
  "entry_points": {
    "patterns": [
      "**/main.py",
      "**/test_*.py",
      "src/index.ts"
    ],
    "functions": [
      "main",
      "test_*"
    ]
  },
  "ignore": [
    "**/node_modules/**",
    "**/__pycache__/**"
  ]
}
```

## Development

### Building

```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p bury
cargo build -p complexity
cargo build -p neural-shared

# Run tests
cargo test --workspace

# Run clippy
cargo clippy --workspace --all-targets
```

### Running

```bash
# Run bury
cargo run -p bury -- --help
cargo run -p bury -- ./src

# Run complexity
cargo run -p complexity
```

## Roadmap

### Phase 1 - Monorepo Migration ✅ Complete
- [x] Create workspace structure
- [x] Extract shared library (neural-shared)
- [x] Migrate bury to workspace
- [x] Create complexity placeholder
- [x] Generic reporter trait
- [x] All tests passing

### Phase 2 - Bury Enhancements
- [ ] Configuration file support
- [ ] Cross-file analysis
- [ ] Import/export tracking
- [ ] Dynamic code pattern detection
- [ ] Performance optimization (parallel processing)

### Phase 3 - Complexity Analyzer
- [ ] Implement cyclomatic complexity
- [ ] Implement cognitive complexity
- [ ] CLI interface
- [ ] Reporter integration
- [ ] Documentation

### Phase 4 - Conductor Platform
- [ ] Multi-agent orchestration system (private repo)
- [ ] Remote execution infrastructure
- [ ] Session management
- [ ] Dashboard and UI
- [ ] Integration with analysis tools
- [ ] LLM context generation
- [ ] AI-powered refactoring suggestions

### Phase 5 - Premium Features
- [ ] Additional languages (Java, Go, Rust, C#)
- [ ] CI/CD integrations
- [ ] Team dashboards
- [ ] Historical tracking
- [ ] Custom rules engine

## Architecture Overview

### Shared Library (neural-shared)

The `neural-shared` crate provides common functionality for all analysis tools:

- **Parser Module** - Tree-sitter-based AST parsing
  - Language detection from file extensions
  - Pluggable parser architecture (Python, TypeScript)
  - Symbol extraction (definitions, usages, entry points)

- **Scanner Module** - File system traversal
  - .gitignore support
  - Parallel file scanning
  - Language-specific file filtering

- **Report Module** - Generic reporting framework
  - `Finding` trait for all analysis results
  - JSON and Markdown reporters
  - Extensible for custom formats

### Tool-Specific Analyzers

Each tool (bury, complexity, etc.) implements its own analysis logic:

- **Bury** - Reachability analysis for dead code detection
- **Complexity** - Cyclomatic and cognitive complexity metrics (WIP)

## Why a Monorepo?

## Why a Monorepo?

- **Code Sharing** - All tools share parser, scanner, and reporter code
- **Consistent Versioning** - Coordinated releases across tools
- **Easier Development** - Test changes across all tools simultaneously
- **Better CI/CD** - Unified testing and deployment

Each tool can still be:
- Published independently to crates.io
- Installed separately via `cargo install`
- Used as a library in other projects

## Why Open Source?
- Build a strong community
- Enable contributions
- Ensure transparency
- Provide value to individual developers

Premium features (additional languages, enterprise integrations) will be available separately to support continued development.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Areas where we need help:
- Parser improvements (AST traversal)
- Language support (Java, Go, Rust, C#)
- Documentation
- Test fixtures
- Performance optimizations

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Inspiration

Bury was inspired by excellent tools like:
- [Knip](https://github.com/webpro-nl/knip) - TypeScript dead code finder
- [Vulture](https://github.com/jendrikseipp/vulture) - Python dead code finder
- [cargo-udeps](https://github.com/est31/cargo-udeps) - Rust unused dependencies

## Support

- 🐛 [Report bugs](https://github.com/neural-garage/tools/issues)
- 💡 [Request features](https://github.com/neural-garage/tools/issues)
- 💬 [Discussions](https://github.com/neural-garage/tools/discussions)

## Part of Neural Garage 🧠🔧

This toolset is part of the [Neural Garage](https://github.com/neural-garage) suite - next-generation developer tools built for the AI era.

**Neural Garage Ecosystem:**
- **Analysis Tools** (this repo) - Open source CLI tools (bury, complexity, etc.)
- **Conductor** *(private repo)* - Multi-agent orchestration platform
- **Context Generator** *(coming soon)* - LLM context optimization

---

**Built with ❤️ and 🦀 by [Paolo Rechia](https://github.com/paolorechia) and the Neural Garage community**
