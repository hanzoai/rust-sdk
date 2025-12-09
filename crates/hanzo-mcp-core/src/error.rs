//! Error types for MCP operations

use thiserror::Error;

/// Errors that can occur during MCP operations
#[derive(Debug, Error)]
pub enum McpError {
    /// Tool not found
    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    /// Tool execution failed
    #[error("Tool execution failed: {tool} - {message}")]
    ToolExecutionFailed { tool: String, message: String },

    /// Resource not found
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    /// Invalid parameters
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    /// Timeout
    #[error("Operation timed out")]
    Timeout,

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for McpError {
    fn from(err: serde_json::Error) -> Self {
        McpError::SerializationError(err.to_string())
    }
}

/// Result type for MCP operations
pub type McpResult<T> = Result<T, McpError>;
