# bury 🪦

> A blazingly fast dead code detector using reachability analysis

**Bury the dead code before it haunts your codebase!**

Part of the [Neural Garage](https://github.com/neural-garage) toolkit - AI-powered code analysis tools built in Rust.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/neural-garage/bury/workflows/CI/badge.svg)](https://github.com/neural-garage/bury/actions)

## What is Bury?

Bury finds unused code in your Python and TypeScript projects by performing **reachability analysis** from entry points. Unlike simple pattern matching tools, Bury builds a complete call graph and identifies code that's truly unreachable.

### Key Features

- 🚀 **Blazingly Fast** - Written in Rust with parallel processing
- 🎯 **Accurate** - Uses reachability analysis, not simple pattern matching
- 🌍 **Multi-Language** - Supports Python and TypeScript (more coming!)
- 🤖 **LLM-Friendly** - Outputs structured JSON perfect for AI tools
- ⚙️ **Configurable** - Define entry points and ignore patterns
- 📊 **Multiple Output Formats** - JSON, Markdown, or terminal

## Installation

```bash
# From crates.io (coming soon)
cargo install bury

# From source
git clone https://github.com/neural-garage/bury
cd bury
cargo install --path .
```

## Quick Start

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

## How It Works

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

## Roadmap

### Phase 1 - MVP ✅ Complete
- [x] Project structure
- [x] Python parser implementation
- [x] TypeScript parser implementation
- [x] Reachability analysis with entry point detection
- [x] JSON output
- [x] CLI commands
- [x] Call graph-based dead code detection

### Phase 2 - Enhanced Features
- [ ] Configuration file support
- [ ] Cross-file analysis
- [ ] Import/export tracking
- [ ] Dynamic code pattern detection
- [ ] Performance optimization (parallel processing)

### Phase 3 - Neural Garage Integration
- [ ] Integration with `neural-complexity` analyzer
- [ ] LLM context generation
- [ ] AI-powered refactoring suggestions
- [ ] Unified `neural` CLI

### Phase 4 - Premium Features (Separate crate)
- [ ] Additional languages (Java, Go, Rust, C#)
- [ ] CI/CD integrations
- [ ] Team dashboards
- [ ] Historical tracking
- [ ] Custom rules engine

## Architecture

```
bury/                          # Core (Open Source - MIT/Apache-2.0)
├── src/
│   ├── scanner/              # File discovery
│   ├── parser/               # AST parsing (tree-sitter)
│   │   ├── python.rs
│   │   └── typescript.rs
│   ├── analyzer/             # Reachability analysis
│   └── report/               # Output generation
│
bury-pro/                      # Premium features (Future)
├── languages/                # Additional language support
├── integrations/             # CI/CD plugins
└── dashboard/                # Web UI
```

## Why Open Core?

Bury's core is **open source** (MIT/Apache-2.0) to:
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

- 🐛 [Report bugs](https://github.com/neural-garage/bury/issues)
- 💡 [Request features](https://github.com/neural-garage/bury/issues)
- 💬 [Discussions](https://github.com/neural-garage/bury/discussions)

## Part of Neural Garage 🧠🔧

Bury is part of the [Neural Garage](https://github.com/neural-garage) toolkit - next-generation developer tools built for the AI era.

**Other Neural Garage tools:**
- **neural-complexity** *(coming soon)* - AI-powered complexity analysis
- **neural-context** *(coming soon)* - LLM context generator
- **neural-inspect** *(coming soon)* - Comprehensive code health diagnostics

---

**Built with ❤️ and 🦀 by [Paolo Rechia](https://github.com/paolorechia) and the Neural Garage community**
