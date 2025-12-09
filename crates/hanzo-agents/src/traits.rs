//! Core traits for specialized agents

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result type for agent operations
pub type Result<T> = std::result::Result<T, AgentError>;

/// Agent execution error
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Tool execution failed: {tool_name}: {message}")]
    ToolError { tool_name: String, message: String },

    #[error("Model error: {0}")]
    ModelError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Max iterations exceeded: {0}")]
    MaxIterations(usize),

    #[error("MCP error: {0}")]
    McpError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

/// Output from an agent run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    /// The final text output
    pub output: String,

    /// Structured data if any
    pub data: Option<serde_json::Value>,

    /// Tool calls that were made
    pub tool_calls: Vec<ToolCall>,

    /// Token usage statistics
    pub usage: Usage,

    /// Metadata about the run
    pub metadata: HashMap<String, String>,
}

/// A tool call made during agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
    pub result: Option<String>,
}

/// Token usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_tokens: u64,
}

/// Configuration for agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// The model to use (e.g., "gpt-4", "claude-3-5-sonnet")
    pub model: String,

    /// API base URL
    pub api_base: Option<String>,

    /// API key
    pub api_key: Option<String>,

    /// Maximum iterations for tool loops
    pub max_iterations: usize,

    /// Temperature for sampling
    pub temperature: Option<f32>,

    /// Maximum tokens for response
    pub max_tokens: Option<u32>,

    /// Additional model settings
    pub settings: HashMap<String, serde_json::Value>,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            api_base: None,
            api_key: None,
            max_iterations: 10,
            temperature: None,
            max_tokens: None,
            settings: HashMap::new(),
        }
    }
}

impl AgentConfig {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            ..Default::default()
        }
    }

    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    pub fn with_api_base(mut self, base: impl Into<String>) -> Self {
        self.api_base = Some(base.into());
        self
    }

    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
}

/// Core trait for specialized agents
///
/// All agent types (Architect, CTO, Reviewer, etc.) implement this trait.
/// This provides a consistent interface for running agents across different
/// contexts (CLI, node, desktop).
#[async_trait]
pub trait SpecializedAgent: Send + Sync {
    /// Get the agent's name
    fn name(&self) -> &str;

    /// Get the agent's description
    fn description(&self) -> &str;

    /// Get the system prompt for this agent
    fn system_prompt(&self) -> &str;

    /// Get available tools for this agent
    fn tools(&self) -> Vec<ToolDefinition>;

    /// Run the agent with the given input
    async fn run(&self, input: &str, config: &AgentConfig) -> Result<AgentOutput>;

    /// Run with streaming output (returns a stream of partial results)
    async fn run_streaming(
        &self,
        input: &str,
        config: &AgentConfig,
    ) -> Result<Box<dyn futures::Stream<Item = Result<String>> + Send + Unpin>>;
}

/// Definition of a tool available to agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Tool name (must be unique)
    pub name: String,

    /// Human-readable description
    pub description: String,

    /// JSON schema for parameters
    pub parameters: serde_json::Value,

    /// Whether this tool requires confirmation before execution
    pub requires_confirmation: bool,
}

impl ToolDefinition {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
            requires_confirmation: false,
        }
    }

    pub fn with_parameters(mut self, schema: serde_json::Value) -> Self {
        self.parameters = schema;
        self
    }

    pub fn with_confirmation(mut self, requires: bool) -> Self {
        self.requires_confirmation = requires;
        self
    }
}
