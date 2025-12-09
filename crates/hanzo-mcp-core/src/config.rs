//! Configuration types for MCP clients and servers
//!
//! These types define how to configure MCP client connections to multiple servers,
//! including transport selection, authentication, and execution policies.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Generate a UUID string (simple implementation to avoid dependency)
fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:032x}", timestamp)
}

/// Generate a UUID-based prefix for tool names
fn generate_uuid_prefix() -> Option<String> {
    Some(format!("mcp_{}", &generate_uuid()[..12]))
}

fn default_true() -> bool {
    true
}

/// Configuration for MCP client integration
///
/// This structure defines how the MCP client should connect to and manage
/// multiple MCP servers, including authentication, tool registration, and
/// execution policies.
///
/// # Example
///
/// ```rust
/// use hanzo_mcp_core::{McpClientConfig, McpServerConfig, McpServerSource};
///
/// let config = McpClientConfig {
///     servers: vec![
///         McpServerConfig {
///             name: "filesystem".to_string(),
///             source: McpServerSource::Process {
///                 command: "mcp-server-filesystem".to_string(),
///                 args: vec!["--root".to_string(), "/tmp".to_string()],
///                 work_dir: None,
///                 env: None,
///             },
///             ..Default::default()
///         },
///     ],
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpClientConfig {
    /// List of MCP servers to connect to
    pub servers: Vec<McpServerConfig>,
    /// Whether to automatically register discovered tools with the model
    ///
    /// When enabled, tools from MCP servers are automatically converted to
    /// the internal Tool format and registered for automatic tool calling.
    #[serde(default = "default_true")]
    pub auto_register_tools: bool,
    /// Timeout for individual tool execution in seconds
    ///
    /// Controls how long to wait for a tool call to complete before timing out.
    /// Defaults to no timeout if not specified.
    pub tool_timeout_secs: Option<u64>,
    /// Maximum number of concurrent tool calls across all MCP servers
    ///
    /// Limits resource usage and prevents overwhelming servers with too many
    /// simultaneous requests. Defaults to 1 if not specified.
    pub max_concurrent_calls: Option<usize>,
}

impl Default for McpClientConfig {
    fn default() -> Self {
        Self {
            servers: Vec::new(),
            auto_register_tools: true,
            tool_timeout_secs: None,
            max_concurrent_calls: Some(1),
        }
    }
}

/// Configuration for an individual MCP server
///
/// Defines connection parameters, authentication, and tool management
/// settings for a single MCP server instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct McpServerConfig {
    /// Unique identifier for this server
    ///
    /// Used internally to track connections and route tool calls.
    /// Must be unique across all servers in a single MCP client configuration.
    /// Defaults to a UUID if not specified.
    #[serde(default = "generate_uuid")]
    pub id: String,
    /// Human-readable name for this server
    ///
    /// Used for logging, debugging, and user-facing displays.
    pub name: String,
    /// Transport-specific connection configuration
    pub source: McpServerSource,
    /// Whether this server should be activated
    ///
    /// Disabled servers are ignored during client initialization.
    /// Defaults to true if not specified.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Optional prefix to add to all tool names from this server
    ///
    /// Helps prevent naming conflicts when multiple servers provide
    /// tools with similar names. For example, with prefix "web",
    /// a tool named "search" becomes "web_search".
    /// Defaults to a UUID-based prefix if not specified.
    #[serde(default = "generate_uuid_prefix")]
    pub tool_prefix: Option<String>,
    /// Optional resource URI patterns this server provides
    ///
    /// Used for resource discovery and subscription.
    /// Supports glob patterns like "file://**" for filesystem access.
    pub resources: Option<Vec<String>>,
    /// Optional Bearer token for authentication
    ///
    /// Automatically included as `Authorization: Bearer <token>` header
    /// for HTTP and WebSocket connections. Process connections typically
    /// don't require authentication tokens.
    pub bearer_token: Option<String>,
}

impl Default for McpServerConfig {
    fn default() -> Self {
        Self {
            id: generate_uuid(),
            name: String::new(),
            source: McpServerSource::Http {
                url: String::new(),
                timeout_secs: None,
                headers: None,
            },
            enabled: true,
            tool_prefix: generate_uuid_prefix(),
            resources: None,
            bearer_token: None,
        }
    }
}

/// Supported MCP server transport sources
///
/// Defines the different ways to connect to MCP servers, each optimized for
/// specific use cases and deployment scenarios.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpServerSource {
    /// HTTP-based MCP server using JSON-RPC over HTTP
    ///
    /// Best for: Public APIs, RESTful services, servers behind load balancers
    /// Features: SSE support, standard HTTP semantics, easy debugging
    Http {
        /// Base URL of the MCP server (http:// or https://)
        url: String,
        /// Optional timeout in seconds for HTTP requests
        /// Defaults to no timeout if not specified.
        timeout_secs: Option<u64>,
        /// Optional headers to include in requests (e.g., API keys, custom headers)
        headers: Option<HashMap<String, String>>,
    },
    /// Server-Sent Events (SSE) based connection
    ///
    /// Best for: Streaming responses, real-time updates
    /// Features: One-way server push, automatic reconnection
    Sse {
        /// SSE endpoint URL
        url: String,
        /// Optional timeout in seconds
        timeout_secs: Option<u64>,
        /// Optional headers for the connection
        headers: Option<HashMap<String, String>>,
    },
    /// Local process-based MCP server using stdin/stdout communication
    ///
    /// Best for: Local tools, development servers, sandboxed environments
    /// Features: Process isolation, no network overhead, easy deployment
    Process {
        /// Command to execute (e.g., "mcp-server-filesystem")
        command: String,
        /// Arguments to pass to the command
        #[serde(default)]
        args: Vec<String>,
        /// Optional working directory for the process
        work_dir: Option<String>,
        /// Optional environment variables for the process
        env: Option<HashMap<String, String>>,
    },
    /// WebSocket-based MCP server for real-time bidirectional communication
    ///
    /// Best for: Interactive applications, real-time data, low-latency requirements
    /// Features: Persistent connections, server-initiated notifications, minimal overhead
    WebSocket {
        /// WebSocket URL (ws:// or wss://)
        url: String,
        /// Optional timeout in seconds for connection establishment
        /// Defaults to no timeout if not specified.
        timeout_secs: Option<u64>,
        /// Optional headers for the WebSocket handshake
        headers: Option<HashMap<String, String>>,
    },
}

/// Information about a tool discovered from an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    /// Name of the tool as reported by the MCP server
    pub name: String,
    /// Optional human-readable description of what the tool does
    pub description: Option<String>,
    /// JSON schema describing the tool's input parameters
    pub input_schema: serde_json::Value,
    /// ID of the server this tool comes from
    ///
    /// Used to route tool calls to the correct MCP server connection.
    pub server_id: String,
    /// Display name of the server for logging and debugging
    pub server_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = McpClientConfig::default();
        assert!(config.servers.is_empty());
        assert!(config.auto_register_tools);
        assert_eq!(config.max_concurrent_calls, Some(1));
    }

    #[test]
    fn test_server_config_default() {
        let server = McpServerConfig::default();
        assert!(!server.id.is_empty());
        assert!(server.enabled);
        assert!(server.tool_prefix.is_some());
    }

    #[test]
    fn test_server_source_serialization() {
        let http = McpServerSource::Http {
            url: "https://example.com/mcp".to_string(),
            timeout_secs: Some(30),
            headers: None,
        };
        let json = serde_json::to_string(&http).unwrap();
        assert!(json.contains("http"));
        assert!(json.contains("example.com"));
    }
}
