//! Unified MCP Client for multi-server orchestration
//!
//! This module provides a high-level client that manages connections to multiple
//! MCP servers simultaneously, with support for:
//! - Multiple transport protocols (HTTP, SSE, Process)
//! - Automatic tool discovery and registration
//! - Concurrency control via semaphore
//! - Per-tool timeouts
//! - Tool name prefixing to avoid conflicts

use crate::error::McpError;
use crate::mcp_methods;
use hanzo_mcp_core::{
    McpClientConfig, McpServerConfig, McpServerSource, McpToolInfo, ToolDefinition, ToolResult,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Semaphore};
use tracing::{debug, error, info, warn};

type Result<T> = std::result::Result<T, McpError>;

/// Callback type for tool execution (sync, used by some integrations)
pub type ToolCallback = dyn Fn(&CalledFunction) -> anyhow::Result<String> + Send + Sync;

/// A tool callback with its associated Tool definition
#[derive(Clone)]
pub struct ToolCallbackWithTool {
    pub callback: Arc<ToolCallback>,
    pub tool: ToolDefinition,
}

/// Called function with name and arguments
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CalledFunction {
    pub name: String,
    pub arguments: String,
}

/// Connection state for an MCP server
struct ServerConnection {
    config: McpServerConfig,
    tools: Vec<McpToolInfo>,
}

/// Unified MCP client that manages connections to multiple MCP servers
///
/// # Features
///
/// - **Multi-server Management**: Connects to and manages multiple MCP servers simultaneously
/// - **Automatic Tool Discovery**: Discovers available tools from connected servers
/// - **Tool Name Prefixing**: Prefixes tool names to avoid conflicts between servers
/// - **Concurrency Control**: Limits concurrent tool calls via semaphore
/// - **Timeout Support**: Per-tool execution timeouts
///
/// # Example
///
/// ```rust,ignore
/// use hanzo_mcp_client::McpClient;
/// use hanzo_mcp_core::{McpClientConfig, McpServerConfig, McpServerSource};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = McpClientConfig {
///         servers: vec![
///             McpServerConfig {
///                 name: "local".to_string(),
///                 source: McpServerSource::Process {
///                     command: "mcp-server".to_string(),
///                     args: vec![],
///                     work_dir: None,
///                     env: None,
///                 },
///                 ..Default::default()
///             },
///         ],
///         max_concurrent_calls: Some(5),
///         ..Default::default()
///     };
///
///     let mut client = McpClient::new(config);
///     client.initialize().await?;
///
///     let tools = client.list_tools().await;
///     println!("Found {} tools", tools.len());
///
///     Ok(())
/// }
/// ```
pub struct McpClient {
    config: McpClientConfig,
    servers: RwLock<HashMap<String, ServerConnection>>,
    tools: RwLock<HashMap<String, McpToolInfo>>,
    concurrency_semaphore: Arc<Semaphore>,
}

impl McpClient {
    /// Create a new MCP client with the given configuration
    pub fn new(config: McpClientConfig) -> Self {
        let max_concurrent = config.max_concurrent_calls.unwrap_or(10);
        Self {
            config,
            servers: RwLock::new(HashMap::new()),
            tools: RwLock::new(HashMap::new()),
            concurrency_semaphore: Arc::new(Semaphore::new(max_concurrent)),
        }
    }

    /// Initialize connections to all configured servers
    ///
    /// This method connects to each enabled server and discovers available tools.
    pub async fn initialize(&mut self) -> Result<()> {
        for server_config in &self.config.servers {
            if !server_config.enabled {
                debug!("Skipping disabled server: {}", server_config.name);
                continue;
            }

            match self.connect_server(server_config.clone()).await {
                Ok(()) => {
                    info!("Connected to MCP server: {}", server_config.name);
                }
                Err(e) => {
                    error!(
                        "Failed to connect to MCP server {}: {}",
                        server_config.name, e
                    );
                    // Continue with other servers
                }
            }
        }

        if self.config.auto_register_tools {
            self.discover_tools().await?;
        }

        Ok(())
    }

    /// Connect to a single server and discover its tools
    async fn connect_server(&self, config: McpServerConfig) -> Result<()> {
        let tools = self.list_server_tools(&config).await?;

        let mut servers = self.servers.write().await;
        servers.insert(
            config.id.clone(),
            ServerConnection {
                config,
                tools,
            },
        );

        Ok(())
    }

    /// List tools from a specific server
    async fn list_server_tools(&self, config: &McpServerConfig) -> Result<Vec<McpToolInfo>> {
        let raw_tools = match &config.source {
            McpServerSource::Http { url, .. } => {
                mcp_methods::list_tools_via_http(url, None).await?
            }
            McpServerSource::Sse { url, .. } => {
                mcp_methods::list_tools_via_sse(url, None).await?
            }
            McpServerSource::Process { command, args, env, .. } => {
                let cmd_str = if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                };
                mcp_methods::list_tools_via_command(&cmd_str, env.clone()).await?
            }
            McpServerSource::WebSocket { .. } => {
                // WebSocket not yet implemented in rmcp-based client
                warn!("WebSocket transport not yet supported, skipping server: {}", config.name);
                return Ok(Vec::new());
            }
        };

        // Convert to McpToolInfo with server context
        let tools: Vec<McpToolInfo> = raw_tools
            .into_iter()
            .map(|t| McpToolInfo {
                name: t.name.to_string(),
                description: t.description.map(|d| d.to_string()),
                input_schema: serde_json::Value::Object((*t.input_schema).clone()),
                server_id: config.id.clone(),
                server_name: config.name.clone(),
            })
            .collect();

        Ok(tools)
    }

    /// Discover and register tools from all connected servers
    async fn discover_tools(&self) -> Result<()> {
        let servers = self.servers.read().await;
        let mut all_tools = self.tools.write().await;

        for (server_id, connection) in servers.iter() {
            for tool in &connection.tools {
                // Apply tool prefix if configured
                let tool_name = if let Some(prefix) = &connection.config.tool_prefix {
                    format!("{}_{}", prefix, tool.name)
                } else {
                    tool.name.clone()
                };

                let mut tool_info = tool.clone();
                tool_info.name = tool_name.clone();
                all_tools.insert(tool_name, tool_info);
            }
            debug!(
                "Registered {} tools from server {}",
                connection.tools.len(),
                server_id
            );
        }

        info!("Total tools registered: {}", all_tools.len());
        Ok(())
    }

    /// Get all discovered tools
    pub async fn list_tools(&self) -> Vec<McpToolInfo> {
        self.tools.read().await.values().cloned().collect()
    }

    /// Get tools as ToolDefinition format (for use with agent systems)
    pub async fn get_tool_definitions(&self) -> Vec<ToolDefinition> {
        self.tools
            .read()
            .await
            .values()
            .map(|t| ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone().unwrap_or_default(),
                input_schema: t.input_schema.clone(),
                requires_confirmation: false,
            })
            .collect()
    }

    /// Call a tool by its prefixed name
    ///
    /// This method handles routing to the correct server, concurrency control,
    /// and timeout enforcement.
    pub async fn call_tool(
        &self,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<ToolResult> {
        // Find the tool and its server
        let (original_name, server_config) = {
            let tools = self.tools.read().await;
            let tool_info = tools
                .get(tool_name)
                .ok_or_else(|| McpError::new(format!("Tool not found: {}", tool_name)))?;

            let servers = self.servers.read().await;
            let connection = servers
                .get(&tool_info.server_id)
                .ok_or_else(|| McpError::new(format!("Server not found: {}", tool_info.server_id)))?;

            // Extract original tool name (remove prefix if present)
            let original_name = if let Some(prefix) = &connection.config.tool_prefix {
                let prefix_with_underscore = format!("{}_", prefix);
                if tool_name.starts_with(&prefix_with_underscore) {
                    tool_name[prefix_with_underscore.len()..].to_string()
                } else {
                    tool_name.to_string()
                }
            } else {
                tool_name.to_string()
            };

            (original_name, connection.config.clone())
        };

        // Acquire concurrency permit
        let _permit = self
            .concurrency_semaphore
            .acquire()
            .await
            .map_err(|_| McpError::new("Failed to acquire concurrency permit"))?;

        // Execute with timeout
        let timeout_duration = Duration::from_secs(self.config.tool_timeout_secs.unwrap_or(30));

        let result = tokio::time::timeout(
            timeout_duration,
            self.execute_tool(&server_config, &original_name, arguments),
        )
        .await
        .map_err(|_| {
            McpError::new(format!(
                "Tool call timed out after {} seconds",
                timeout_duration.as_secs()
            ))
        })??;

        Ok(result)
    }

    /// Execute a tool on a specific server
    async fn execute_tool(
        &self,
        config: &McpServerConfig,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<ToolResult> {
        let params = arguments.as_object().cloned().unwrap_or_default();

        let result = match &config.source {
            McpServerSource::Http { url, .. } => {
                mcp_methods::run_tool_via_http(url.clone(), tool_name.to_string(), params).await?
            }
            McpServerSource::Sse { url, .. } => {
                mcp_methods::run_tool_via_sse(url.clone(), tool_name.to_string(), params).await?
            }
            McpServerSource::Process { command, args, env, .. } => {
                let cmd_str = if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                };
                mcp_methods::run_tool_via_command(
                    cmd_str,
                    tool_name.to_string(),
                    env.clone().unwrap_or_default(),
                    params,
                )
                .await?
            }
            McpServerSource::WebSocket { .. } => {
                return Err(McpError::new("WebSocket transport not yet implemented"));
            }
        };

        // Convert CallToolResult to ToolResult
        let content: Vec<hanzo_mcp_core::ContentBlock> = result
            .content
            .iter()
            .filter_map(|c| {
                c.as_text().map(|t| hanzo_mcp_core::ContentBlock::Text {
                    text: t.text.clone(),
                })
            })
            .collect();

        Ok(ToolResult {
            content,
            is_error: result.is_error.unwrap_or(false),
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &McpClientConfig {
        &self.config
    }

    /// Get server count
    pub async fn server_count(&self) -> usize {
        self.servers.read().await.len()
    }

    /// Get tool count
    pub async fn tool_count(&self) -> usize {
        self.tools.read().await.len()
    }

    /// Check if a specific server is connected
    pub async fn is_server_connected(&self, server_id: &str) -> bool {
        self.servers.read().await.contains_key(server_id)
    }

    // =========================================================================
    // Resource Methods
    // =========================================================================

    /// List all resources from all connected servers
    pub async fn list_resources(&self) -> Result<Vec<McpResourceInfo>> {
        let servers = self.servers.read().await;
        let mut all_resources = Vec::new();

        for (server_id, connection) in servers.iter() {
            match self.list_server_resources(&connection.config).await {
                Ok(resources) => {
                    for resource in resources {
                        all_resources.push(McpResourceInfo {
                            uri: resource.uri.to_string(),
                            name: resource.name.to_string(),
                            description: resource.description.as_ref().map(|d| d.to_string()),
                            mime_type: resource.mime_type.as_ref().map(|m| m.to_string()),
                            server_id: server_id.clone(),
                            server_name: connection.config.name.clone(),
                        });
                    }
                }
                Err(e) => {
                    warn!("Failed to list resources from server {}: {}", server_id, e);
                }
            }
        }

        Ok(all_resources)
    }

    /// List resources from a specific server
    async fn list_server_resources(&self, config: &McpServerConfig) -> Result<Vec<rmcp::model::Resource>> {
        match &config.source {
            McpServerSource::Http { url, .. } => {
                mcp_methods::list_resources_via_http(url, None).await
            }
            McpServerSource::Sse { url, .. } => {
                mcp_methods::list_resources_via_sse(url, None).await
            }
            McpServerSource::Process { command, args, env, .. } => {
                let cmd_str = if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                };
                mcp_methods::list_resources_via_command(&cmd_str, env.clone()).await
            }
            McpServerSource::WebSocket { .. } => {
                warn!("WebSocket transport not yet supported for resources");
                Ok(Vec::new())
            }
        }
    }

    /// Read a resource by URI
    ///
    /// The URI must match a resource from one of the connected servers.
    pub async fn read_resource(&self, uri: &str) -> Result<String> {
        // First, find which server has this resource
        let resources = self.list_resources().await?;
        let resource_info = resources
            .iter()
            .find(|r| r.uri == uri)
            .ok_or_else(|| McpError::new(format!("Resource not found: {}", uri)))?;

        let servers = self.servers.read().await;
        let connection = servers
            .get(&resource_info.server_id)
            .ok_or_else(|| McpError::new(format!("Server not found: {}", resource_info.server_id)))?;

        self.read_server_resource(&connection.config, uri).await
    }

    /// Read a resource from a specific server
    async fn read_server_resource(&self, config: &McpServerConfig, uri: &str) -> Result<String> {
        match &config.source {
            McpServerSource::Http { url, .. } => {
                mcp_methods::read_resource_via_http(url, uri).await
            }
            McpServerSource::Sse { url, .. } => {
                mcp_methods::read_resource_via_sse(url, uri).await
            }
            McpServerSource::Process { command, args, env, .. } => {
                let cmd_str = if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                };
                mcp_methods::read_resource_via_command(&cmd_str, uri, env.clone()).await
            }
            McpServerSource::WebSocket { .. } => {
                Err(McpError::new("WebSocket transport not yet supported for resources"))
            }
        }
    }
}

/// Information about an MCP resource with server context
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpResourceInfo {
    /// URI for the resource
    pub uri: String,
    /// Human-readable name
    pub name: String,
    /// Description of the resource
    pub description: Option<String>,
    /// MIME type of the resource
    pub mime_type: Option<String>,
    /// ID of the server providing this resource
    pub server_id: String,
    /// Name of the server providing this resource
    pub server_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let config = McpClientConfig::default();
        let client = McpClient::new(config);
        assert_eq!(client.server_count().await, 0);
        assert_eq!(client.tool_count().await, 0);
    }

    #[tokio::test]
    async fn test_custom_concurrency() {
        let config = McpClientConfig {
            max_concurrent_calls: Some(5),
            ..Default::default()
        };
        let client = McpClient::new(config);
        assert_eq!(client.concurrency_semaphore.available_permits(), 5);
    }
}
