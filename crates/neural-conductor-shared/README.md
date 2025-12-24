# neural-conductor-shared

> Shared protocol and types for Neural Conductor

[![Crates.io](https://img.shields.io/crates/v/neural-conductor-shared.svg)](https://crates.io/crates/neural-conductor-shared)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

## Overview

`neural-conductor-shared` provides the communication protocol between the Neural Conductor server and remote agents. It defines:

- **Protocol Messages** - Request/Response types for RPC communication
- **Session Management** - Session types and lifecycle management
- **Message Framing** - Serialization and versioning for network messages
- **Agent Info** - Agent identification and metadata

## Architecture

Neural Conductor is a multi-agent orchestration platform with three components:

1. **Server** (private) - Orchestrates agents and manages sessions
2. **Agent** (public) - Runs on remote machines and executes commands
3. **Shared** (this crate) - Communication protocol used by both

## Usage

### Agent Information

```rust
use neural_conductor_shared::AgentInfo;

let agent = AgentInfo {
    id: "agent-001".to_string(),
    hostname: "dev-machine".to_string(),
    platform: "linux".to_string(),
    version: "0.1.0".to_string(),
};
```

### Protocol Messages

```rust
use neural_conductor_shared::protocol::{Request, Response};
use neural_conductor_shared::SessionId;

// Server sends request
let request = Request::ExecuteCommand {
    session_id: SessionId::new(),
    command: "cargo".to_string(),
    args: vec!["build".to_string()],
    workdir: Some("/path/to/project".to_string()),
};

// Agent responds
let response = Response::CommandResult {
    session_id: session_id.clone(),
    exit_code: 0,
    stdout: "Finished build".to_string(),
    stderr: String::new(),
};
```

### Message Framing

```rust
use neural_conductor_shared::message::Message;
use neural_conductor_shared::protocol::Request;

let msg = Message::new(Request::Ping);
let json = msg.to_json()?;

// Send over network...

let received: Message<Request> = Message::from_json(&json)?;
```

## Status

🚧 **Early Development** - This crate defines the initial protocol. The API may change as we develop the conductor platform.

## Part of Neural Garage 🧠🔧

This crate is part of the [Neural Garage](https://github.com/neural-garage/tools) suite.

**Related crates:**
- **neural-conductor-agent** - Agent runtime for remote execution
- **neural-conductor-server** *(private)* - Orchestration server

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
