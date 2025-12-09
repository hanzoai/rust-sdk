//! Scientist Agent - Research, analysis, and evidence-based reasoning

use crate::prompts::SCIENTIST_PROMPT;
use crate::tools::ToolRegistry;
use crate::traits::{
    AgentConfig, AgentError, AgentOutput, Result, SpecializedAgent, ToolDefinition, Usage,
};
use async_trait::async_trait;
use std::sync::Arc;

/// Scientist Agent for research and analysis
pub struct ScientistAgent {
    tool_registry: Arc<ToolRegistry>,
}

impl ScientistAgent {
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    pub fn default_agent() -> Self {
        Self::new(Arc::new(ToolRegistry::with_defaults()))
    }
}

#[async_trait]
impl SpecializedAgent for ScientistAgent {
    fn name(&self) -> &str {
        "scientist"
    }

    fn description(&self) -> &str {
        "Research scientist for analysis, experimentation, and evidence-based reasoning"
    }

    fn system_prompt(&self) -> &str {
        SCIENTIST_PROMPT
    }

    fn tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition::new("web_search", "Search the web for information").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" },
                        "num_results": { "type": "integer", "default": 5 }
                    },
                    "required": ["query"]
                }),
            ),
            ToolDefinition::new("read_url", "Read content from a URL").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string" }
                    },
                    "required": ["url"]
                }),
            ),
            ToolDefinition::new("read_file", "Read a local file").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string" }
                    },
                    "required": ["path"]
                }),
            ),
            ToolDefinition::new("analyze_data", "Analyze data with statistics").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "data": { "type": "array", "items": { "type": "number" } },
                        "operations": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "enum": ["mean", "median", "std", "min", "max", "correlation"]
                            }
                        }
                    },
                    "required": ["data"]
                }),
            ),
            ToolDefinition::new("create_hypothesis", "Document a hypothesis").with_parameters(
                serde_json::json!({
                    "type": "object",
                    "properties": {
                        "hypothesis": { "type": "string" },
                        "evidence_for": { "type": "array", "items": { "type": "string" } },
                        "evidence_against": { "type": "array", "items": { "type": "string" } },
                        "confidence": { "type": "number", "minimum": 0, "maximum": 1 }
                    },
                    "required": ["hypothesis"]
                }),
            ),
        ]
    }

    async fn run(&self, input: &str, config: &AgentConfig) -> Result<AgentOutput> {
        tracing::info!("Scientist agent processing: {}", input);

        Ok(AgentOutput {
            output: format!(
                "Research analysis for: {}\n\n[This is a placeholder - implement LLM integration]",
                input
            ),
            data: None,
            tool_calls: vec![],
            usage: Usage::default(),
            metadata: [("agent".to_string(), "scientist".to_string())]
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
