//! Architect Agent - High-level system design and architectural decisions

use crate::prompts::ARCHITECT_PROMPT;
use crate::tools::ToolRegistry;
use crate::traits::{
    AgentConfig, AgentError, AgentOutput, Result, SpecializedAgent, ToolDefinition, Usage,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Architect Agent for system design and architectural decisions
pub struct ArchitectAgent {
    #[allow(dead_code)] // Reserved for tool execution integration
    tool_registry: Arc<ToolRegistry>,
}

impl ArchitectAgent {
    /// Create a new architect agent
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    /// Create with default tool registry
    pub fn default_agent() -> Self {
        Self::new(Arc::new(ToolRegistry::with_defaults()))
    }
}

#[async_trait]
impl SpecializedAgent for ArchitectAgent {
    fn name(&self) -> &str {
        "architect"
    }

    fn description(&self) -> &str {
        "Expert software architect for system design and architectural decisions"
    }

    fn system_prompt(&self) -> &str {
        ARCHITECT_PROMPT
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        // Architect typically needs: file reading, search, documentation tools
        vec![
            ToolDefinition::new("read_file", "Read contents of a file").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to read"
                        }
                    },
                    "required": ["path"]
                }),
            ),
            ToolDefinition::new("search_code", "Search for patterns in the codebase")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Search pattern (regex supported)"
                        },
                        "path": {
                            "type": "string",
                            "description": "Directory to search in"
                        }
                    },
                    "required": ["pattern"]
                })),
            ToolDefinition::new("list_directory", "List files in a directory").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Directory path"
                        },
                        "recursive": {
                            "type": "boolean",
                            "description": "Whether to list recursively"
                        }
                    },
                    "required": ["path"]
                }),
            ),
        ]
    }

    async fn run(&self, input: &str, _config: &AgentConfig) -> Result<AgentOutput> {
        // This is a placeholder implementation
        // The actual implementation would use hanzo-agent's Runner
        // with the configured model and tools

        tracing::info!("Architect agent processing: {}", input);

        // For now, return a placeholder output
        // In production, this would:
        // 1. Build messages with system prompt
        // 2. Call the LLM API
        // 3. Execute any tool calls
        // 4. Return the final result

        Ok(AgentOutput {
            output: format!(
                "Architect analysis for: {}\n\n[This is a placeholder - implement LLM integration]",
                input
            ),
            data: None,
            tool_calls: vec![],
            usage: Usage::default(),
            metadata: [("agent".to_string(), "architect".to_string())]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_architect_agent_creation() {
        let agent = ArchitectAgent::default_agent();
        assert_eq!(agent.name(), "architect");
        assert!(!agent.tools().is_empty());
    }
}
