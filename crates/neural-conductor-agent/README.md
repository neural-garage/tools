# neural-conductor-agent

> Remote agent for Neural Conductor orchestration platform

[![Crates.io](https://img.shields.io/crates/v/neural-conductor-agent.svg)](https://crates.io/crates/neural-conductor-agent)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

## Overview

`neural-conductor-agent` is the remote agent component of the Neural Conductor platform. It runs on development machines and executes commands on behalf of the orchestration server.

## Features (Planned)

- **Session Management** - Manage isolated work sessions
- **Command Execution** - Execute commands in specific workspaces
- **Secure Communication** - TLS-encrypted communication with server
- **Resource Monitoring** - Track CPU, memory, and disk usage
- **Tool Integration** - Seamless integration with Neural Garage analysis tools

## Architecture

Neural Conductor consists of three components:

1. **Server** (private) - Orchestrates agents, manages sessions, provides UI
2. **Agent** (this crate) - Runs on remote machines, executes commands
3. **Shared** - Communication protocol and types

## Installation

```bash
# From crates.io (coming soon)
cargo install neural-conductor-agent

# From source
git clone https://github.com/neural-garage/tools
cd tools
cargo install --path crates/neural-conductor-agent
```

## Usage (Coming Soon)

```bash
# Start agent and connect to server
neural-conductor-agent --server https://conductor.example.com

# Start agent with custom workspace
neural-conductor-agent --server https://conductor.example.com --workspace /path/to/workspace

# Run in daemon mode
neural-conductor-agent --server https://conductor.example.com --daemon
```

## Status

🚧 **Early Development** - This crate reserves the name and provides basic structure. Full agent functionality is in development.

## Security

The agent will implement:
- TLS encryption for all communication
- Command allowlisting/denylisting
- Workspace isolation
- Resource limits
- Audit logging

## Part of Neural Garage 🧠🔧

This agent is part of the [Neural Garage](https://github.com/neural-garage/tools) suite.

**Related crates:**
- **[neural-conductor-shared](https://crates.io/crates/neural-conductor-shared)** - Protocol definitions
- **neural-conductor-server** *(private)* - Orchestration server

**Works with:**
- **[bury](https://crates.io/crates/bury)** - Dead code detector
- **[neural-complexity](https://crates.io/crates/neural-complexity)** - Complexity analyzer
- **[neural-shared](https://crates.io/crates/neural-shared)** - Code analysis utilities

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
