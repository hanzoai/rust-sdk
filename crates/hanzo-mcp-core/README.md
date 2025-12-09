# hanzo-mcp-core

Core traits and types for Hanzo MCP (Model Context Protocol) implementations.

## Overview

This crate provides the foundational types used across all Hanzo MCP crates:

- **Configuration types**: `McpClientConfig`, `McpServerConfig`, `McpServerSource`
- **Tool types**: `ToolDefinition`, `ToolResult`, `McpToolInfo`
- **Resource types**: `ResourceDefinition`
- **Protocol types**: Server capabilities, client info, etc.

## Usage

```toml
[dependencies]
hanzo-mcp-core = "0.1"
```

```rust
use hanzo_mcp_core::{McpClientConfig, McpServerConfig, McpServerSource};

let config = McpClientConfig {
    servers: vec![
        McpServerConfig {
            name: "example".to_string(),
            source: McpServerSource::Http {
                url: "http://localhost:3333".to_string(),
                timeout_secs: Some(30),
                headers: None,
            },
            ..Default::default()
        },
    ],
    ..Default::default()
};
```

## Part of Hanzo MCP

This crate is part of the Hanzo MCP family:

- `hanzo-mcp` - Unified crate (recommended)
- `hanzo-mcp-core` - Core types (this crate)
- `hanzo-mcp-client` - MCP client
- `hanzo-mcp-server` - MCP server

## License

MIT OR Apache-2.0
