//! # Hanzo MCP - Unified Model Context Protocol
//!
//! This crate provides a unified interface to the Hanzo MCP ecosystem:
//!
//! - **Core types and traits** (`hanzo-mcp-core`) - Shared abstractions
//! - **Client** (`hanzo-mcp-client`) - Connect to MCP servers
//! - **Server** (`hanzo-mcp-server`) - Build MCP servers with tools
//!
//! ## Features
//!
//! - `client` (default) - Include the MCP client
//! - `server` - Include the MCP server implementation
//! - `full` - Include everything
//!
//! ## Quick Start
//!
//! ### Using the unified McpClient (recommended)
//!
//! ```rust,ignore
//! use hanzo_mcp::{McpClient, McpClientConfig, McpServerConfig, McpServerSource};
//!
//! let config = McpClientConfig {
//!     servers: vec![
//!         McpServerConfig {
//!             name: "example".to_string(),
//!             source: McpServerSource::Http {
//!                 url: "http://localhost:3333/mcp".to_string(),
//!                 timeout_secs: Some(30),
//!                 headers: None,
//!             },
//!             ..Default::default()
//!         },
//!     ],
//!     max_concurrent_calls: Some(5),
//!     ..Default::default()
//! };
//!
//! let mut client = McpClient::new(config);
//! client.initialize().await?;
//!
//! let tools = client.list_tools().await;
//! let result = client.call_tool("tool_name", serde_json::json!({})).await?;
//! ```
//!
//! ### Using low-level mcp_methods
//!
//! ```rust,ignore
//! use hanzo_mcp::mcp_methods;
//!
//! // List tools from an MCP server
//! let tools = mcp_methods::list_tools_via_http("http://localhost:3333", None).await?;
//!
//! // Execute a tool
//! let result = mcp_methods::run_tool_via_http(
//!     "http://localhost:3333".to_string(),
//!     "tool_name".to_string(),
//!     params,
//! ).await?;
//! ```
//!
//! ### Building a Server
//!
//! Enable the `server` feature and use `hanzo_mcp::server`:
//!
//! ```rust,ignore
//! use hanzo_mcp::server::{Config, MCPServer};
//!
//! let server = MCPServer::new(Config::default(), 3333)?;
//! server.run().await?;
//! ```

// Re-export core types - these are always available
pub use hanzo_mcp_core::*;

/// MCP client functionality
///
/// Connect to external MCP servers and execute tools.
pub mod client {
    pub use hanzo_mcp_client::*;
}

/// MCP server functionality (requires `server` feature)
#[cfg(feature = "server")]
pub mod server {
    pub use hanzo_mcp_server::*;
}

// Convenience re-exports for common usage patterns
pub use client::mcp_methods;
pub use client::McpClient;
pub use client::McpResourceInfo;
