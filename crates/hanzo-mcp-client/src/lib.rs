//! Hanzo MCP Client - Model Context Protocol client implementation
//!
//! This crate provides a comprehensive client for connecting to MCP servers,
//! supporting multiple transport protocols and multi-server orchestration.
//!
//! # Features
//!
//! - **Multiple Transports**: HTTP, SSE, Process (stdio)
//! - **Multi-Server Management**: Connect to multiple MCP servers simultaneously
//! - **Automatic Tool Discovery**: Discover and register tools from servers
//! - **Concurrency Control**: Limit concurrent tool calls
//! - **Timeout Support**: Per-tool execution timeouts
//!
//! # Quick Start
//!
//! ## Using the high-level McpClient
//!
//! ```rust,no_run
//! use hanzo_mcp_client::McpClient;
//! use hanzo_mcp_core::{McpClientConfig, McpServerConfig, McpServerSource};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = McpClientConfig {
//!         servers: vec![
//!             McpServerConfig {
//!                 name: "example".to_string(),
//!                 source: McpServerSource::Http {
//!                     url: "http://localhost:3000/mcp".to_string(),
//!                     timeout_secs: Some(30),
//!                     headers: None,
//!                 },
//!                 ..Default::default()
//!             },
//!         ],
//!         ..Default::default()
//!     };
//!
//!     let mut client = McpClient::new(config);
//!     client.initialize().await?;
//!
//!     let tools = client.list_tools().await;
//!     println!("Found {} tools", tools.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Using low-level mcp_methods
//!
//! For simple one-off operations:
//!
//! ```rust,no_run
//! use hanzo_mcp_client::mcp_methods;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let tools = mcp_methods::list_tools_via_http("http://localhost:3000/mcp", None).await?;
//!     println!("Found {} tools", tools.len());
//!     Ok(())
//! }
//! ```

mod client;
mod command;
pub mod error;
pub mod mcp_methods;
mod utils;

// Re-export the high-level client
pub use client::{CalledFunction, McpClient, McpResourceInfo, ToolCallback, ToolCallbackWithTool};

// Re-export core types for convenience
pub use hanzo_mcp_core::{
    McpClientConfig, McpServerConfig, McpServerSource, McpToolInfo, ResourceDefinition,
    ToolDefinition, ToolResult,
};
