//! Error types for the agent framework

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Max turns exceeded: {0}")]
    MaxTurnsExceeded(usize),

    #[error("Model error: {0}")]
    ModelError(String),

    #[error("Tool error: {tool_name}: {message}")]
    ToolError { tool_name: String, message: String },

    #[error("Invalid JSON: {0}")]
    InvalidJson(String),

    #[error("Model behavior error: {0}")]
    ModelBehavior(String),

    #[error("Agent configuration error: {0}")]
    Configuration(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Generic error: {0}")]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, AgentError>;
