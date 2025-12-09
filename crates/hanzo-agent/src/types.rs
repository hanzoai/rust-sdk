//! Core types for agent framework

use serde::{Deserialize, Serialize};

/// Input item for the agent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum InputItem {
    #[serde(rename = "message")]
    Message { role: String, content: String },

    #[serde(rename = "tool_result")]
    ToolResult {
        tool_call_id: String,
        content: String,
    },
}

impl InputItem {
    /// Create a user message
    pub fn user_message(content: impl Into<String>) -> Self {
        InputItem::Message {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    /// Create an assistant message
    pub fn assistant_message(content: impl Into<String>) -> Self {
        InputItem::Message {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    /// Create a system message
    pub fn system_message(content: impl Into<String>) -> Self {
        InputItem::Message {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

/// Item generated during agent run
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RunItem {
    #[serde(rename = "message")]
    Message { role: String, content: String },

    #[serde(rename = "tool_call")]
    ToolCall {
        id: String,
        name: String,
        arguments: String,
    },

    #[serde(rename = "tool_result")]
    ToolResult {
        tool_call_id: String,
        content: String,
    },
}

impl RunItem {
    /// Convert to input item for next turn
    pub fn to_input_item(&self) -> InputItem {
        match self {
            RunItem::Message { role, content } => InputItem::Message {
                role: role.clone(),
                content: content.clone(),
            },
            RunItem::ToolResult {
                tool_call_id,
                content,
            } => InputItem::ToolResult {
                tool_call_id: tool_call_id.clone(),
                content: content.clone(),
            },
            RunItem::ToolCall { .. } => {
                // Tool calls are converted to messages in the context
                InputItem::assistant_message("")
            }
        }
    }
}

/// Model response from LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    /// The output items (messages, tool calls, etc.)
    pub output: Vec<RunItem>,

    /// Usage statistics
    pub usage: Usage,

    /// Response ID for reference
    pub id: Option<String>,
}

/// Usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    pub requests: usize,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub total_tokens: usize,
}

impl Usage {
    /// Add usage from another instance
    pub fn add(&mut self, other: &Usage) {
        self.requests += other.requests;
        self.input_tokens += other.input_tokens;
        self.output_tokens += other.output_tokens;
        self.total_tokens += other.total_tokens;
    }
}

/// Model settings for tuning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

impl Default for ModelSettings {
    fn default() -> Self {
        Self {
            temperature: None,
            top_p: None,
            max_tokens: None,
            stop: None,
        }
    }
}
