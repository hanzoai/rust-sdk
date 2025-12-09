//! Result types for agent runs

use crate::types::{InputItem, ModelResponse, RunItem, Usage};
use serde::{Deserialize, Serialize};

/// Result of an agent run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    /// The original input
    pub input: Vec<InputItem>,

    /// New items generated during the run
    pub new_items: Vec<RunItem>,

    /// Raw model responses
    pub raw_responses: Vec<ModelResponse>,

    /// The final output
    pub final_output: String,

    /// Total usage statistics
    pub usage: Usage,
}

impl RunResult {
    /// Create a new run result
    pub fn new(
        input: Vec<InputItem>,
        new_items: Vec<RunItem>,
        raw_responses: Vec<ModelResponse>,
        final_output: String,
        usage: Usage,
    ) -> Self {
        Self {
            input,
            new_items,
            raw_responses,
            final_output,
            usage,
        }
    }

    /// Convert the result back to a list of input items
    ///
    /// This merges the original input with all new items,
    /// useful for continuing a conversation.
    pub fn to_input_list(&self) -> Vec<InputItem> {
        let mut items = self.input.clone();
        items.extend(self.new_items.iter().map(|item| item.to_input_item()));
        items
    }

    /// Get the last message content from the result
    pub fn last_message(&self) -> Option<&str> {
        self.new_items.iter().rev().find_map(|item| {
            if let RunItem::Message { content, .. } = item {
                Some(content.as_str())
            } else {
                None
            }
        })
    }
}

/// Streaming result (placeholder for future implementation)
#[derive(Debug)]
pub struct RunResultStreaming {
    // TODO: Implement streaming support
    _placeholder: (),
}

impl RunResultStreaming {
    pub(crate) fn _new() -> Self {
        Self { _placeholder: () }
    }
}
