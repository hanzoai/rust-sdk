//! Core types for MCP protocol

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tool definition in MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Unique name of the tool
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// JSON Schema for input parameters
    #[serde(default)]
    pub input_schema: serde_json::Value,
    /// Whether tool requires confirmation before execution
    #[serde(default)]
    pub requires_confirmation: bool,
}

impl ToolDefinition {
    /// Create a new tool definition
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            input_schema: serde_json::json!({"type": "object"}),
            requires_confirmation: false,
        }
    }

    /// Set the input schema
    pub fn with_schema(mut self, schema: serde_json::Value) -> Self {
        self.input_schema = schema;
        self
    }

    /// Set whether confirmation is required
    pub fn with_confirmation(mut self, required: bool) -> Self {
        self.requires_confirmation = required;
        self
    }
}

/// Result of tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// Content returned by the tool
    pub content: Vec<ContentBlock>,
    /// Whether the tool execution errored
    #[serde(default)]
    pub is_error: bool,
}

impl ToolResult {
    /// Create a successful text result
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: vec![ContentBlock::Text {
                text: content.into(),
            }],
            is_error: false,
        }
    }

    /// Create an error result
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            content: vec![ContentBlock::Text {
                text: message.into(),
            }],
            is_error: true,
        }
    }

    /// Create a result with image content
    pub fn image(data: String, mime_type: String) -> Self {
        Self {
            content: vec![ContentBlock::Image { data, mime_type }],
            is_error: false,
        }
    }
}

/// Content block in MCP responses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentBlock {
    /// Text content
    Text { text: String },
    /// Image content (base64 encoded)
    Image { data: String, mime_type: String },
    /// Resource reference
    Resource { uri: String, text: Option<String> },
}

impl ContentBlock {
    /// Get text content if this is a text block
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ContentBlock::Text { text } => Some(text),
            _ => None,
        }
    }
}

/// Resource definition in MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceDefinition {
    /// URI for the resource
    pub uri: String,
    /// Human-readable name
    pub name: String,
    /// Description of the resource
    pub description: Option<String>,
    /// MIME type of the resource
    pub mime_type: Option<String>,
}

/// Prompt definition in MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptDefinition {
    /// Unique name of the prompt
    pub name: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Arguments the prompt accepts
    #[serde(default)]
    pub arguments: Vec<PromptArgument>,
}

/// Argument for a prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    /// Argument name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Whether the argument is required
    #[serde(default)]
    pub required: bool,
}

/// Server capabilities advertised during initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Tools capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    /// Resources capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    /// Prompts capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    /// Logging capability
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingCapability>,
}

/// Tools capability details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolsCapability {
    /// Whether the server supports tool list changes notification
    #[serde(default)]
    pub list_changed: bool,
}

/// Resources capability details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourcesCapability {
    /// Whether the server supports subscriptions
    #[serde(default)]
    pub subscribe: bool,
    /// Whether the server supports list changes notification
    #[serde(default)]
    pub list_changed: bool,
}

/// Prompts capability details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PromptsCapability {
    /// Whether the server supports list changes notification
    #[serde(default)]
    pub list_changed: bool,
}

/// Logging capability details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoggingCapability {}

/// Client information for initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Client name
    pub name: String,
    /// Client version
    pub version: String,
}

/// Server information returned during initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
    /// Protocol version supported
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
}

/// Connection configuration for MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum McpTransport {
    /// HTTP-based connection
    Http {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
    /// Server-Sent Events connection
    Sse {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
    /// WebSocket connection
    WebSocket {
        url: String,
        #[serde(default)]
        headers: HashMap<String, String>,
    },
    /// Subprocess (stdio) connection
    Process {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        #[serde(default)]
        env: HashMap<String, String>,
    },
}
