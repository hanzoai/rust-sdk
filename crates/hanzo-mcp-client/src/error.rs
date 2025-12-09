use thiserror::Error;

/// Error type for MCP client operations
#[derive(Debug, Error)]
#[error("{message}")]
pub struct McpError {
    pub message: String,
}

impl McpError {
    /// Create a new MCP error with the given message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}
