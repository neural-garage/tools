# bury 🪦

> A blazingly fast dead code detector using reachability analysis

**Bury the dead code before it haunts your codebase!**

[![Crates.io](https://img.shields.io/crates/v/bury.svg)](https://crates.io/crates/bury)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

## What is Bury?

Bury finds unused code in your Python and TypeScript projects by performing **reachability analysis** from entry points. Unlike simple pattern matching tools, Bury builds a complete call graph and identifies code that's truly unreachable.

## Key Features

- 🚀 **Blazingly Fast** - Written in Rust with parallel processing
- 🎯 **Accurate** - Uses reachability analysis, not simple pattern matching
- 🌍 **Multi-Language** - Supports Python and TypeScript
- 🤖 **LLM-Friendly** - Outputs structured JSON perfect for AI tools
- 📊 **Multiple Output Formats** - JSON, Markdown, or terminal

## Installation

```bash
# From crates.io
cargo install bury

# From source
git clone https://github.com/neural-garage/tools
cd tools
cargo install --path crates/bury
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
  "summary": {
    "total_findings": 1
  },
  "findings": [
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

## Part of Neural Garage 🧠🔧

Bury is part of the [Neural Garage](https://github.com/neural-garage) toolkit - AI-powered code analysis tools built in Rust.

**Other Neural Garage tools:**
- **complexity** *(coming soon)* - Code complexity analyzer
- **conductor** *(private)* - Multi-agent orchestration platform

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

See the [main repository](https://github.com/neural-garage/tools) for contribution guidelines.
