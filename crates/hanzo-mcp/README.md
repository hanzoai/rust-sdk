# hanzo-mcp

Unified Model Context Protocol (MCP) for Hanzo - the canonical Rust MCP implementation for all Hanzo projects.

## Overview

`hanzo-mcp` provides a complete MCP implementation for:
- **hanzo-node** - AI infrastructure node
- **hanzo-dev** - Developer CLI tool
- **hanzo-engine** - LLM inference engine
- Any Rust project needing MCP support

## Features

- **Client**: Connect to MCP servers (HTTP, SSE, Process)
- **Server**: Build MCP servers with tools (optional feature)
- **Multi-Server**: Manage multiple servers simultaneously
- **Resource Management**: List and read server resources
- **Tool Discovery**: Automatic tool registration
- **Concurrency Control**: Rate limiting and timeouts

## Quick Start

```toml
[dependencies]
hanzo-mcp = "0.1"                         # Client only (default)
hanzo-mcp = { version = "0.1", features = ["server"] }  # Include server
hanzo-mcp = { version = "0.1", features = ["full"] }    # Everything
```

### Using McpClient (Recommended)

```rust
use hanzo_mcp::{McpClient, McpClientConfig, McpServerConfig, McpServerSource};

let config = McpClientConfig {
    servers: vec![
        McpServerConfig {
            name: "tools".to_string(),
            source: McpServerSource::Http {
                url: "http://localhost:3333".to_string(),
                timeout_secs: Some(30),
                headers: None,
            },
            ..Default::default()
        },
    ],
    max_concurrent_calls: Some(5),
    ..Default::default()
};

let mut client = McpClient::new(config);
client.initialize().await?;

// List tools
let tools = client.list_tools().await;

// Call a tool
let result = client.call_tool("search", json!({"query": "foo"})).await?;

// List resources
let resources = client.list_resources().await?;

// Read a resource
let content = client.read_resource("file:///path").await?;
```

### Using Low-Level Methods

```rust
use hanzo_mcp::mcp_methods;

let tools = mcp_methods::list_tools_via_http("http://localhost:3333", None).await?;
let result = mcp_methods::run_tool_via_http(url, "search".into(), params).await?;
```

### Building a Server

```rust
use hanzo_mcp::server::{Config, MCPServer};

let server = MCPServer::new(Config::default(), 3333)?;
server.run().await?;
```

## Migration Guide

### From hanzo-node's hanzo_mcp

```rust
// Before
use hanzo_mcp::mcp_methods;

// After
use hanzo_mcp::mcp_methods;  // Same API!
```

### From mistralrs-mcp

```rust
// Before
use mistralrs_mcp::{McpClientConfig, McpServerConfig};

// After
use hanzo_mcp::{McpClientConfig, McpServerConfig, McpClient};
```

## Crate Structure

```
hanzo-mcp (unified)
├── hanzo-mcp-core (types, traits)
├── hanzo-mcp-client (connect to servers)
└── hanzo-mcp-server (be a server) [optional]
```

## License

MIT OR Apache-2.0
