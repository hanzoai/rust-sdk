# hanzo-mcp-client

MCP (Model Context Protocol) client for connecting to MCP servers.

## Features

- **Multiple Transports**: HTTP, SSE, Process (stdio)
- **Multi-Server Management**: Connect to multiple MCP servers simultaneously
- **Automatic Tool Discovery**: Discover and register tools from servers
- **Resource Management**: List and read resources from servers
- **Concurrency Control**: Limit concurrent tool calls
- **Timeout Support**: Per-tool execution timeouts

## Usage

```toml
[dependencies]
hanzo-mcp-client = "0.1"
```

### High-Level McpClient (Recommended)

```rust
use hanzo_mcp_client::{McpClient, McpClientConfig, McpServerConfig, McpServerSource};

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

// List all tools
let tools = client.list_tools().await;

// Call a tool
let result = client.call_tool("search", json!({"query": "foo"})).await?;

// List resources
let resources = client.list_resources().await?;

// Read a resource
let content = client.read_resource("file:///path").await?;
```

### Low-Level mcp_methods

For simple one-off operations:

```rust
use hanzo_mcp_client::mcp_methods;

// List tools
let tools = mcp_methods::list_tools_via_http("http://localhost:3333", None).await?;

// Execute a tool
let result = mcp_methods::run_tool_via_http(
    "http://localhost:3333".to_string(),
    "search".to_string(),
    params,
).await?;

// List resources
let resources = mcp_methods::list_resources_via_http("http://localhost:3333", None).await?;

// Read a resource
let content = mcp_methods::read_resource_via_http("http://localhost:3333", "file:///path").await?;
```

## Supported Transports

- **HTTP**: `McpServerSource::Http` - Streamable HTTP transport
- **SSE**: `McpServerSource::Sse` - Server-Sent Events transport
- **Process**: `McpServerSource::Process` - Child process with stdio
- **WebSocket**: `McpServerSource::WebSocket` - (planned)

## Part of Hanzo MCP

This crate is part of the Hanzo MCP family:

- `hanzo-mcp` - Unified crate (recommended)
- `hanzo-mcp-core` - Core types
- `hanzo-mcp-client` - MCP client (this crate)
- `hanzo-mcp-server` - MCP server

## License

MIT OR Apache-2.0
