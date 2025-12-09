//! Reviewer Agent - Code review, quality assurance, and suggestions

use crate::prompts::REVIEWER_PROMPT;
use crate::tools::ToolRegistry;
use crate::traits::{AgentConfig, AgentError, AgentOutput, Result, SpecializedAgent, ToolDefinition, Usage};
use async_trait::async_trait;
use std::sync::Arc;

/// Reviewer Agent for code review and quality assurance
pub struct ReviewerAgent {
    tool_registry: Arc<ToolRegistry>,
}

impl ReviewerAgent {
    /// Create a new reviewer agent
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    /// Create with default tool registry
    pub fn default_agent() -> Self {
        Self::new(Arc::new(ToolRegistry::with_defaults()))
    }
}

#[async_trait]
impl SpecializedAgent for ReviewerAgent {
    fn name(&self) -> &str {
        "reviewer"
    }

    fn description(&self) -> &str {
        "Meticulous code reviewer focused on quality, correctness, and maintainability"
    }

    fn system_prompt(&self) -> &str {
        REVIEWER_PROMPT
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition::new("read_file", "Read contents of a file")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Path to the file" }
                    },
                    "required": ["path"]
                })),
            ToolDefinition::new("git_diff", "Get git diff for the changes being reviewed")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "target": { "type": "string", "description": "Branch or commit to diff against" },
                        "file": { "type": "string", "description": "Specific file to diff" }
                    }
                })),
            ToolDefinition::new("git_blame", "Get git blame for a file")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File path" },
                        "line_start": { "type": "integer", "description": "Start line" },
                        "line_end": { "type": "integer", "description": "End line" }
                    },
                    "required": ["path"]
                })),
            ToolDefinition::new("search_code", "Search for patterns in codebase")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "pattern": { "type": "string", "description": "Search pattern" },
                        "path": { "type": "string", "description": "Directory to search" }
                    },
                    "required": ["pattern"]
                })),
            ToolDefinition::new("check_tests", "Check test coverage for changes")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Path to check" }
                    }
                })),
            ToolDefinition::new("suggest_edit", "Suggest a code edit")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File path" },
                        "line": { "type": "integer", "description": "Line number" },
                        "original": { "type": "string", "description": "Original code" },
                        "suggested": { "type": "string", "description": "Suggested replacement" },
                        "reason": { "type": "string", "description": "Reason for suggestion" }
                    },
                    "required": ["path", "suggested", "reason"]
                })),
        ]
    }

    async fn run(&self, input: &str, config: &AgentConfig) -> Result<AgentOutput> {
        tracing::info!("Reviewer agent processing: {}", input);

        Ok(AgentOutput {
            output: format!(
                "Code review for: {}\n\n[This is a placeholder - implement LLM integration]",
                input
            ),
            data: None,
            tool_calls: vec![],
            usage: Usage::default(),
            metadata: [("agent".to_string(), "reviewer".to_string())]
                .into_iter()
                .collect(),
        })
    }

    async fn run_streaming(
        &self,
        _input: &str,
        _config: &AgentConfig,
    ) -> Result<Box<dyn futures::Stream<Item = Result<String>> + Send + Unpin>> {
        Err(AgentError::Other("Streaming not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reviewer_agent_creation() {
        let agent = ReviewerAgent::default_agent();
        assert_eq!(agent.name(), "reviewer");
        assert!(!agent.tools().is_empty());
    }
}
