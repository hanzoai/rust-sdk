//! Planner Agent - Task planning and implementation strategy

use crate::prompts::PLANNER_PROMPT;
use crate::tools::ToolRegistry;
use crate::traits::{AgentConfig, AgentError, AgentOutput, Result, SpecializedAgent, ToolDefinition, Usage};
use async_trait::async_trait;
use std::sync::Arc;

/// Planner Agent for task breakdown and implementation planning
pub struct PlannerAgent {
    tool_registry: Arc<ToolRegistry>,
}

impl PlannerAgent {
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    pub fn default_agent() -> Self {
        Self::new(Arc::new(ToolRegistry::with_defaults()))
    }
}

#[async_trait]
impl SpecializedAgent for PlannerAgent {
    fn name(&self) -> &str {
        "planner"
    }

    fn description(&self) -> &str {
        "Technical planner for task breakdown and implementation strategy"
    }

    fn system_prompt(&self) -> &str {
        PLANNER_PROMPT
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition::new("read_file", "Read file contents for context")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" }
                    },
                    "required": ["path"]
                })),
            ToolDefinition::new("list_directory", "Understand project structure")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" },
                        "recursive": { "type": "boolean" }
                    },
                    "required": ["path"]
                })),
            ToolDefinition::new("search_code", "Find relevant code sections")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "pattern": { "type": "string" },
                        "path": { "type": "string" }
                    },
                    "required": ["pattern"]
                })),
            ToolDefinition::new("create_plan", "Create a structured implementation plan")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "title": { "type": "string", "description": "Plan title" },
                        "steps": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "description": { "type": "string" },
                                    "complexity": { "type": "string", "enum": ["low", "medium", "high"] },
                                    "dependencies": { "type": "array", "items": { "type": "integer" } },
                                    "verification": { "type": "string" }
                                },
                                "required": ["description"]
                            }
                        }
                    },
                    "required": ["title", "steps"]
                })),
            ToolDefinition::new("estimate_effort", "Estimate effort for a task")
                .with_parameters(serde_json::json!({
                    "type": "object",
                    "properties": {
                        "task": { "type": "string" },
                        "factors": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Factors affecting estimate"
                        }
                    },
                    "required": ["task"]
                })),
        ]
    }

    async fn run(&self, input: &str, config: &AgentConfig) -> Result<AgentOutput> {
        tracing::info!("Planner agent processing: {}", input);

        Ok(AgentOutput {
            output: format!(
                "Implementation plan for: {}\n\n[This is a placeholder - implement LLM integration]",
                input
            ),
            data: None,
            tool_calls: vec![],
            usage: Usage::default(),
            metadata: [("agent".to_string(), "planner".to_string())]
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
