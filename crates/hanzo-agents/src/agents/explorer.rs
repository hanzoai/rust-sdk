//! Explorer Agent - Codebase exploration and documentation

use crate::prompts::EXPLORER_PROMPT;
use crate::tools::ToolRegistry;
use crate::traits::{
    AgentConfig, AgentError, AgentOutput, Result, SpecializedAgent, ToolDefinition, Usage,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Explorer Agent for codebase exploration and understanding
pub struct ExplorerAgent {
    #[allow(dead_code)] // Reserved for tool execution integration
    tool_registry: Arc<ToolRegistry>,
}

impl ExplorerAgent {
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    pub fn default_agent() -> Self {
        Self::new(Arc::new(ToolRegistry::with_defaults()))
    }
}

#[async_trait]
impl SpecializedAgent for ExplorerAgent {
    fn name(&self) -> &str {
        "explorer"
    }

    fn description(&self) -> &str {
        "Expert codebase analyst for exploration and documentation"
    }

    fn system_prompt(&self) -> &str {
        EXPLORER_PROMPT
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition::new("read_file", "Read contents of a file").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" }
                    },
                    "required": ["path"]
                }),
            ),
            ToolDefinition::new("list_directory", "List files in a directory").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "recursive": { "type": "boolean" },
                        "pattern": { "type": "string", "description": "Glob pattern" }
                    },
                    "required": ["path"]
                }),
            ),
            ToolDefinition::new("search_code", "Search for patterns").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "pattern": { "type": "string" },
                        "path": { "type": "string" },
                        "file_type": { "type": "string", "description": "File extension filter" }
                    },
                    "required": ["pattern"]
                }),
            ),
            ToolDefinition::new("find_definition", "Find definition of a symbol").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "symbol": { "type": "string" },
                        "path": { "type": "string" }
                    },
                    "required": ["symbol"]
                }),
            ),
            ToolDefinition::new("find_references", "Find all references to a symbol")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "symbol": { "type": "string" },
                        "path": { "type": "string" }
                    },
                    "required": ["symbol"]
                })),
            ToolDefinition::new("get_ast", "Get AST structure of a file").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "depth": { "type": "integer", "description": "Max depth to show" }
                    },
                    "required": ["path"]
                }),
            ),
        ]
    }

    async fn run(&self, input: &str, _config: &AgentConfig) -> Result<AgentOutput> {
        tracing::info!("Explorer agent processing: {}", input);

        Ok(AgentOutput {
            output: format!(
                "Exploration results for: {}\n\n[This is a placeholder - implement LLM integration]",
                input
            ),
            data: None,
            tool_calls: vec![],
            usage: Usage::default(),
            metadata: [("agent".to_string(), "explorer".to_string())]
                .into_iter()
                .collect(),
        })
    }

    async fn run_streaming(
        &self,
        _input: &str,
        _config: &AgentConfig,
    ) -> Result<Box<dyn futures::Stream<Item = Result<String>> + Send + Unpin>> {
        Err(AgentError::Other(
            "Streaming not yet implemented".to_string(),
        ))
    }
}
