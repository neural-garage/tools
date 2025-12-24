# neural-complexity

> Code complexity analyzer for Python and TypeScript

[![Crates.io](https://img.shields.io/crates/v/neural-complexity.svg)](https://crates.io/crates/neural-complexity)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

## Overview

`neural-complexity` analyzes code complexity using industry-standard metrics including cyclomatic complexity and cognitive complexity. It helps identify complex code that may be hard to maintain, test, or understand.

## Status

🚧 **Early Development** - This crate is currently in active development. The initial release reserves the name and provides a basic CLI structure. Full complexity analysis features are coming soon!

## Planned Features

- **Cyclomatic Complexity** - Measure code paths and branching
- **Cognitive Complexity** - Assess how hard code is to understand
- **Multi-Language Support** - Python and TypeScript initially
- **LLM-Friendly Output** - JSON reports for AI-powered analysis
- **Threshold Enforcement** - Fail builds when complexity exceeds limits
- **Visual Reports** - Markdown and HTML output formats

## Roadmap

- [ ] v0.2.0 - Cyclomatic complexity for Python
- [ ] v0.3.0 - Cyclomatic complexity for TypeScript
- [ ] v0.4.0 - Cognitive complexity metrics
- [ ] v0.5.0 - Threshold enforcement and CI/CD integration
- [ ] v1.0.0 - Stable release with full feature set

## Installation

```bash
# From crates.io
cargo install neural-complexity

# From source
git clone https://github.com/neural-garage/tools
cd tools
cargo install --path crates/neural-complexity
```

## Quick Start

```bash
# Analyze current directory (coming soon)
neural-complexity

# Analyze specific path (coming soon)
neural-complexity ./src

# Output as JSON (coming soon)
neural-complexity --format json ./src
```

## Part of Neural Garage 🧠🔧

This tool is part of the [Neural Garage](https://github.com/neural-garage/tools) suite.

**Other Neural Garage tools:**
- **[bury](https://crates.io/crates/bury)** - Dead code detector
- **[neural-shared](https://crates.io/crates/neural-shared)** - Shared parsing utilities

## Contributing

We're actively building this tool! Contributions are welcome. See the [main repository](https://github.com/neural-garage/tools) for contribution guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
