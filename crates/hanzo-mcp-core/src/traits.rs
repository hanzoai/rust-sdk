//! Core traits for MCP implementations

use async_trait::async_trait;
use std::collections::HashMap;

use crate::{
    McpResult, ToolDefinition, ToolResult, ResourceDefinition, PromptDefinition,
    ServerInfo, ClientInfo, ServerCapabilities,
};

/// Trait for MCP tool implementations
///
/// Tools are the primary way MCP servers expose functionality to clients.
/// Each tool has a name, description, and can be invoked with parameters.
#[async_trait]
pub trait McpTool: Send + Sync {
    /// Get the tool definition
    fn definition(&self) -> ToolDefinition;

    /// Execute the tool with the given parameters
    async fn execute(&self, params: serde_json::Value) -> McpResult<ToolResult>;
}

/// Trait for MCP clients
///
/// Clients connect to MCP servers and can discover/invoke tools.
#[async_trait]
pub trait McpClient: Send + Sync {
    /// Initialize the connection with server
    async fn initialize(&mut self, client_info: ClientInfo) -> McpResult<ServerInfo>;

    /// List all available tools
    async fn list_tools(&self) -> McpResult<Vec<ToolDefinition>>;

    /// Call a tool by name with parameters
    async fn call_tool(
        &self,
        name: &str,
        params: serde_json::Value,
    ) -> McpResult<ToolResult>;

    /// List available resources (if supported)
    async fn list_resources(&self) -> McpResult<Vec<ResourceDefinition>> {
        Ok(vec![])
    }

    /// Read a resource by URI (if supported)
    async fn read_resource(&self, _uri: &str) -> McpResult<String> {
        Err(crate::McpError::ResourceNotFound("Resources not supported".to_string()))
    }

    /// List available prompts (if supported)
    async fn list_prompts(&self) -> McpResult<Vec<PromptDefinition>> {
        Ok(vec![])
    }

    /// Get a prompt by name (if supported)
    async fn get_prompt(
        &self,
        _name: &str,
        _args: HashMap<String, String>,
    ) -> McpResult<String> {
        Err(crate::McpError::ResourceNotFound("Prompts not supported".to_string()))
    }

    /// Close the connection
    async fn close(&mut self) -> McpResult<()>;
}

/// Trait for MCP servers
///
/// Servers expose tools, resources, and prompts to clients.
#[async_trait]
pub trait McpServer: Send + Sync {
    /// Get server information
    fn info(&self) -> ServerInfo;

    /// Get server capabilities
    fn capabilities(&self) -> ServerCapabilities;

    /// Handle initialization from client
    async fn handle_initialize(&self, client_info: ClientInfo) -> McpResult<ServerInfo>;

    /// List all available tools
    async fn list_tools(&self) -> McpResult<Vec<ToolDefinition>>;

    /// Execute a tool
    async fn call_tool(
        &self,
        name: &str,
        params: serde_json::Value,
    ) -> McpResult<ToolResult>;

    /// List available resources
    async fn list_resources(&self) -> McpResult<Vec<ResourceDefinition>> {
        Ok(vec![])
    }

    /// Read a resource
    async fn read_resource(&self, _uri: &str) -> McpResult<String> {
        Err(crate::McpError::ResourceNotFound("Resources not supported".to_string()))
    }

    /// List available prompts
    async fn list_prompts(&self) -> McpResult<Vec<PromptDefinition>> {
        Ok(vec![])
    }

    /// Get a prompt
    async fn get_prompt(
        &self,
        _name: &str,
        _args: HashMap<String, String>,
    ) -> McpResult<String> {
        Err(crate::McpError::ResourceNotFound("Prompts not supported".to_string()))
    }
}

/// Registry for managing multiple tools
pub trait ToolRegistry: Send + Sync {
    /// Register a tool
    fn register(&mut self, tool: Box<dyn McpTool>);

    /// Get a tool by name
    fn get(&self, name: &str) -> Option<&dyn McpTool>;

    /// List all registered tools
    fn list(&self) -> Vec<ToolDefinition>;

    /// Check if a tool exists
    fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
}
