# Hanzo Rust SDK

Shared Rust infrastructure for Hanzo AI services - used by `hanzo-dev` (CLI), `hanzo-node` (blockchain node), and `hanzo-desktop` (Tauri app).

## Architecture Overview

```
rust-sdk/
├── crates/
│   ├── hanzo-pqc/               # Post-quantum cryptography
│   ├── hanzo-crypto/            # General crypto primitives
│   ├── hanzo-did/               # W3C DID support
│   ├── hanzo-message-primitives/ # Core message schemas
│   ├── hanzo-config/            # Configuration management
│   ├── hanzo-agent/             # Agent framework (tool calling)
│   ├── hanzo-mcp-core/          # MCP shared traits and types
│   ├── hanzo-mcp-client/        # MCP client (connects to servers)
│   ├── hanzo-mcp-server/        # MCP server (exposes tools)
│   ├── hanzo-mcp/               # Unified MCP re-export crate
│   ├── hanzo-agents/            # Specialized agents (architect, cto, reviewer)
│   └── [disabled crates]        # Need refactoring for clean separation
```

## Key Crates

### Core Infrastructure

| Crate | Purpose | Status |
|-------|---------|--------|
| `hanzo-pqc` | Post-quantum cryptography (ML-KEM, ML-DSA, SLH-DSA) | Active |
| `hanzo-crypto` | General cryptographic primitives | Active |
| `hanzo-did` | W3C Decentralized Identifiers | Active |
| `hanzo-message-primitives` | Core message schemas for node communication | Active |
| `hanzo-config` | Configuration file management | Active |

### MCP (Model Context Protocol)

The MCP implementation is the **canonical Rust implementation** for all Hanzo projects. It consolidates functionality from `mistralrs-mcp` and provides a unified interface.

| Crate | Purpose | Status |
|-------|---------|--------|
| `hanzo-mcp-core` | Core traits, types, and **McpClientConfig/McpServerConfig** | Active |
| `hanzo-mcp-client` | Client with **McpClient** orchestrator + low-level methods | Active |
| `hanzo-mcp-server` | Server implementation with tools (jsonrpc-based) | Active |
| `hanzo-mcp` | Unified crate that re-exports all MCP functionality | Active |

### AI/ML Integration

| Crate | Purpose | Status |
|-------|---------|--------|
| `hanzo-agent` | Base agent framework with tool calling | Active |
| `hanzo-agents` | Specialized agents (architect, cto, reviewer, etc.) | Active |

### Disabled Crates (need refactoring)

These crates have dependencies on hanzo-node internals and need refactoring:
- `hanzo-crypto-identities` - needs `hanzo_non_rust_code`
- `hanzo-messages` - deep dependencies
- `hanzo-embedding` - blocking reqwest issues
- `hanzo-llm` - depends on sqlite
- `hanzo-tools-runner` - Deno/Python execution (node-specific)

## MCP Architecture

### Design Philosophy

One crate, one responsibility. All MCP crates follow a clean separation:

```
hanzo-mcp-core     ─────────────────────────────────────
    │              Shared types, traits, error handling
    │              (no external MCP dependencies)
    │
    ├─── hanzo-mcp-client ──────────────────────────────
    │         │            Connect TO MCP servers
    │         │            Uses: rmcp crate
    │         │
    │         └─── Transports: HTTP, SSE, Process (stdio)
    │
    └─── hanzo-mcp-server ──────────────────────────────
              │            BE an MCP server
              │            Uses: jsonrpc-core
              │
              └─── Tools: Search, AST, Personalities
```

### Using the Unified Crate

For most use cases, depend on `hanzo-mcp`:

```toml
[dependencies]
hanzo-mcp = { version = "0.1" }                    # Client only (default)
hanzo-mcp = { version = "0.1", features = ["server"] }  # Include server
hanzo-mcp = { version = "0.1", features = ["full"] }    # Everything
```

### As a Client (Recommended: McpClient)

```rust
use hanzo_mcp::{McpClient, McpClientConfig, McpServerConfig, McpServerSource};

// Configure multi-server client
let config = McpClientConfig {
    servers: vec![
        McpServerConfig {
            name: "local-tools".to_string(),
            source: McpServerSource::Process {
                command: "mcp-server".to_string(),
                args: vec![],
                work_dir: None,
                env: None,
            },
            ..Default::default()
        },
        McpServerConfig {
            name: "remote-api".to_string(),
            source: McpServerSource::Http {
                url: "https://api.example.com/mcp".to_string(),
                timeout_secs: Some(30),
                headers: None,
            },
            bearer_token: Some("your-api-key".to_string()),
            ..Default::default()
        },
    ],
    max_concurrent_calls: Some(5),
    tool_timeout_secs: Some(30),
    ..Default::default()
};

let mut client = McpClient::new(config);
client.initialize().await?;

// List all tools from all servers
let tools = client.list_tools().await;

// Call a tool (automatically routed to correct server)
let result = client.call_tool("mcp_xxx_search", json!({"query": "foo"})).await?;

// List resources from all servers
let resources = client.list_resources().await?;

// Read a specific resource
let content = client.read_resource("file:///path/to/resource").await?;
```

### As a Client (Low-level mcp_methods)

For simple one-off operations:

```rust
use hanzo_mcp::mcp_methods;

// List tools from an MCP server
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

### As a Server

```rust
use hanzo_mcp::server::{Config, MCPServer};

let config = Config::default();
let server = MCPServer::new(config, 3333)?;
server.run().await?;
```

### Supported Transports

**Client (hanzo-mcp-client):**
- HTTP (StreamableHttpClientTransport)
- SSE (SseClientTransport)
- Child process (TokioChildProcess/stdio)

**Server (hanzo-mcp-server):**
- HTTP (jsonrpc-http-server)

## Agent System

### Specialized Agents

The `hanzo-agents` crate provides pre-built agent types:

```rust
use hanzo_agents::{AgentRegistry, AgentType};

let mut registry = AgentRegistry::new();
let architect = registry.get(AgentType::Architect)?;
let result = architect.run("Design a microservices architecture").await?;
```

**Available Agents:**
- **Architect**: System design and architectural decisions
- **CTO**: Technical leadership and code quality
- **Reviewer**: Code review and quality assurance
- **Explorer**: Codebase exploration and documentation
- **Planner**: Task planning and implementation strategy
- **Scientist**: Research and evidence-based analysis

### Tool Registry

Tools can be registered from multiple sources:

```rust
use hanzo_agents::tools::{ToolRegistry, McpServerConfig, McpConnection};

let registry = ToolRegistry::new();

// Register MCP server tools
registry.register_mcp_server(McpServerConfig {
    name: "hanzo-mcp".to_string(),
    connection: McpConnection::Http { url: "http://localhost:3333".to_string() },
    env: None,
    timeout_secs: Some(30),
}).await?;

// Execute a tool
let result = registry.execute("hanzo-mcp:search", json!({"query": "foo"})).await?;
```

## Building

```bash
cd ~/work/hanzo/rust-sdk
cargo build              # Build all active crates
cargo check              # Type check
cargo test               # Run tests
```

## Integration Points

### hanzo-dev Integration

The CLI uses these crates for:
- Agent execution (specialized agents for coding tasks)
- MCP tool calling (connect to tool servers)
- Configuration management

### hanzo-node Integration

The node uses these crates for:
- Post-quantum cryptography for secure messaging
- DID-based identities
- Message primitives for protocol communication
- Tool execution (via hanzo-tools-runner, currently disabled)

### hanzo-engine Integration

The engine (mistralrs fork) should migrate to use `hanzo-mcp` from rust-sdk. The `mistralrs-mcp` crate's configuration types (`McpClientConfig`, `McpServerConfig`, `McpServerSource`) have been merged into `hanzo-mcp-core`.

**Migration path for hanzo-engine:**
1. Add `hanzo-mcp` dependency with workspace reference
2. Replace `mistralrs_mcp::McpClientConfig` → `hanzo_mcp::McpClientConfig`
3. Replace transport creation with `hanzo_mcp::McpClient`
4. Keep engine-specific tool callback integration in `mistralrs-mcp` (PyO3 bindings, etc.)

### Architecture Diagram

```
┌─────────────┐     MCP/gRPC     ┌─────────────┐
│ hanzo-node  │ ◄──────────────► │  hanzo-dev  │
│ (blockchain)│                  │    (CLI)    │
└─────────────┘                  └─────────────┘
       │                                │
       │                                │
       └────────────────┬───────────────┘
                        │
              ┌─────────┴─────────┐
              │    rust-sdk       │
              │  (shared crates)  │
              └───────────────────┘
                        │
    ┌───────────────────┼───────────────────┐
    │                   │                   │
┌───┴───┐         ┌─────┴─────┐       ┌─────┴─────┐
│hanzo- │         │ hanzo-mcp │       │  hanzo-   │
│agents │         │  (unified)│       │  crypto   │
└───────┘         └───────────┘       └───────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
   ┌────┴────┐    ┌─────┴─────┐   ┌─────┴─────┐
   │  core   │    │  client   │   │  server   │
   │ traits  │    │ (rmcp)    │   │(jsonrpc)  │
   └─────────┘    └───────────┘   └───────────┘
```

## Dependencies

Key external dependencies:
- `rmcp` v0.8 - Rust MCP client implementation
- `jsonrpc-core` v18.0 - JSON-RPC server for MCP server
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `oqs` / `saorsa-pqc` - Post-quantum cryptography
- `reqwest` - HTTP client
- `tree-sitter` - AST parsing (MCP server)

## Development Notes

### Package Naming

- **ALWAYS use hyphens** for crate names: `name = "hanzo-mcp-client"`
- Rust normalizes to underscores in code: `use hanzo_mcp_client::...`
- Workspace dependencies use hyphenated form only

### Adding New Crates

1. Create crate in `crates/hanzo-<name>/`
2. Add to workspace members in root `Cargo.toml`
3. Add workspace dependency entry (hyphenated form only)
4. Update this LLM.md

### Disabled Crates

To enable a disabled crate:
1. Identify and resolve dependencies on node internals
2. Extract shared functionality to new crates if needed
3. Uncomment in workspace members
4. Run `cargo check` to verify

## Related Projects

- `~/work/hanzo/engine/` - LLM inference engine with `mistralrs-mcp` client
- `~/work/hanzo/node/` - Blockchain node using rust-sdk crates
- `~/work/hanzo/dev/` - CLI using rust-sdk agents and MCP

## Migration Guide

All Hanzo Rust projects should use `hanzo-mcp` from rust-sdk as the canonical MCP implementation.

### For hanzo-node

Replace the local `hanzo_mcp` crate with the rust-sdk version:

```toml
# In hanzo-node/Cargo.toml workspace dependencies
# BEFORE
hanzo_mcp = { path = "./crates/hanzo-mcp" }

# AFTER
hanzo-mcp = { git = "https://github.com/hanzoai/rust-sdk", version = "0.1" }
# Or after publishing to crates.io:
hanzo-mcp = "0.1"
```

The API is compatible - `mcp_methods` functions have the same signatures.

### For hanzo-engine (mistralrs-mcp)

The config types have been merged into hanzo-mcp-core. Update imports:

```rust
// BEFORE
use mistralrs_mcp::{McpClientConfig, McpServerConfig, McpServerSource};

// AFTER
use hanzo_mcp::{McpClientConfig, McpServerConfig, McpServerSource, McpClient};
```

Keep engine-specific code (PyO3 bindings, utoipa) in mistralrs-mcp but depend on hanzo-mcp for shared types.

```toml
# In hanzo-engine/mistralrs-mcp/Cargo.toml
[dependencies]
hanzo-mcp = { git = "https://github.com/hanzoai/rust-sdk", version = "0.1" }
```

### For hanzo-dev (codex-rs)

The `codex-rmcp-client` can use hanzo-mcp for MCP operations:

```toml
# In codex-rs/Cargo.toml
hanzo-mcp = { git = "https://github.com/hanzoai/rust-sdk", version = "0.1" }
```

### Publishing to crates.io

Publish order (dependencies must be published first):
1. `cargo publish -p hanzo-mcp-core`
2. `cargo publish -p hanzo-mcp-client`
3. `cargo publish -p hanzo-mcp-server`
4. `cargo publish -p hanzo-mcp`

After publishing, update dependent projects to use crates.io version:
```toml
hanzo-mcp = "0.1"
```

## Future: Distributed Compute Network

Goal: Turn all devices in a home network into an AI compute cluster.

**Planned features:**
- Ring backend for heterogeneous multi-node inference (from hanzo-engine)
- QR code device pairing via CLI/app
- libp2p for P2P networking (from hanzo-node)
- WebGPU support for browser-based compute
