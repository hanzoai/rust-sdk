# hanzo-mcp-server

MCP (Model Context Protocol) server implementation with search, tools, and code analysis.

## Features

- **JSON-RPC Based**: Standard MCP protocol over HTTP
- **Built-in Tools**: Search, AST analysis, code navigation
- **Extensible**: Add custom tools easily
- **Multi-Language AST**: Support for Rust, Python, JavaScript, TypeScript, Go, Java, C/C++

## Usage

```toml
[dependencies]
hanzo-mcp-server = "0.1"
```

### Running the Server

```rust
use hanzo_mcp_server::{Config, MCPServer};

let config = Config::default();
let server = MCPServer::new(config, 3333)?;
server.run().await?;
```

### As a Binary

```bash
cargo install hanzo-mcp-server
hanzo-mcp-server --port 3333
```

## Built-in Tools

- **search**: Full-text search across files
- **grep**: Pattern matching with regex
- **ast**: AST-based code structure analysis
- **read_file**: Read file contents
- **list_directory**: List directory contents

## Part of Hanzo MCP

This crate is part of the Hanzo MCP family:

- `hanzo-mcp` - Unified crate (recommended)
- `hanzo-mcp-core` - Core types
- `hanzo-mcp-client` - MCP client
- `hanzo-mcp-server` - MCP server (this crate)

## License

MIT OR Apache-2.0
