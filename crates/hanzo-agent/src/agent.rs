//! Agent implementation

use crate::errors::Result;
use crate::result::RunResult;
use crate::runner::{RunConfig, Runner};
use crate::tool::Tool;
use crate::types::ModelSettings;
use std::sync::Arc;

/// Instructions for the agent (system prompt)
#[derive(Debug, Clone)]
pub enum Instructions {
    /// Static instructions
    Static(String),
    
    /// Dynamic instructions generated at runtime
    /// TODO: Add support for dynamic instructions via function
    Dynamic(String),
}

impl Instructions {
    /// Get the instructions as a string
    pub fn as_str(&self) -> &str {
        match self {
            Instructions::Static(s) | Instructions::Dynamic(s) => s,
        }
    }
}

impl From<String> for Instructions {
    fn from(s: String) -> Self {
        Instructions::Static(s)
    }
}

impl From<&str> for Instructions {
    fn from(s: &str) -> Self {
        Instructions::Static(s.to_string())
    }
}

/// An AI agent configured with instructions, tools, and settings
///
/// Agents are the core abstraction for building AI applications.
/// They encapsulate a model, system prompt (instructions), tools, and other configuration.
#[derive(Clone)]
pub struct Agent {
    /// The name of the agent
    pub name: String,
    
    /// Instructions (system prompt) for the agent
    pub instructions: Option<Instructions>,
    
    /// The model to use (e.g., "gpt-4", "claude-3-5-sonnet")
    pub model: String,
    
    /// Tools available to the agent
    pub tools: Vec<Arc<dyn Tool>>,
    
    /// Handoff agents (sub-agents the agent can delegate to)
    pub handoffs: Vec<Agent>,
    
    /// Model settings (temperature, max_tokens, etc.)
    pub model_settings: ModelSettings,
    
    /// Description for when this agent is used as a handoff
    pub handoff_description: Option<String>,
}

impl Agent {
    /// Create a new agent with the given name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instructions: None,
            model: "gpt-4".to_string(),
            tools: Vec::new(),
            handoffs: Vec::new(),
            model_settings: ModelSettings::default(),
            handoff_description: None,
        }
    }

    /// Create a builder for the agent
    pub fn builder(name: impl Into<String>) -> AgentBuilder {
        AgentBuilder::new(name)
    }

    /// Clone the agent with modifications
    pub fn clone_with(&self) -> AgentBuilder {
        AgentBuilder {
            name: self.name.clone(),
            instructions: self.instructions.clone(),
            model: self.model.clone(),
            tools: self.tools.clone(),
            handoffs: self.handoffs.clone(),
            model_settings: self.model_settings.clone(),
            handoff_description: self.handoff_description.clone(),
        }
    }

    /// Get the system prompt for the agent
    pub fn system_prompt(&self) -> Option<&str> {
        self.instructions.as_ref().map(|i| i.as_str())
    }

    /// Run the agent with the given input
    ///
    /// This is a convenience method that creates a default RunConfig.
    /// For more control, use `Runner::run` directly.
    pub async fn run(&self, input: impl Into<String>, config: &RunConfig) -> Result<RunResult> {
        Runner::run(self, input.into(), config).await
    }

    /// Add a tool to the agent
    pub fn with_tool(mut self, tool: impl Tool + 'static) -> Self {
        self.tools.push(Arc::new(tool));
        self
    }

    /// Add a handoff agent
    pub fn with_handoff(mut self, agent: Agent) -> Self {
        self.handoffs.push(agent);
        self
    }
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("name", &self.name)
            .field("model", &self.model)
            .field("tools", &self.tools.len())
            .field("handoffs", &self.handoffs.len())
            .finish()
    }
}

/// Builder for creating agents
pub struct AgentBuilder {
    name: String,
    instructions: Option<Instructions>,
    model: String,
    tools: Vec<Arc<dyn Tool>>,
    handoffs: Vec<Agent>,
    model_settings: ModelSettings,
    handoff_description: Option<String>,
}

impl AgentBuilder {
    /// Create a new agent builder
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            instructions: None,
            model: "gpt-4".to_string(),
            tools: Vec::new(),
            handoffs: Vec::new(),
            model_settings: ModelSettings::default(),
            handoff_description: None,
        }
    }

    /// Set the agent instructions
    pub fn instructions(mut self, instructions: impl Into<Instructions>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Set the model
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    /// Add a tool
    pub fn tool(mut self, tool: impl Tool + 'static) -> Self {
        self.tools.push(Arc::new(tool));
        self
    }

    /// Add multiple tools
    pub fn tools(mut self, tools: Vec<Arc<dyn Tool>>) -> Self {
        self.tools.extend(tools);
        self
    }

    /// Add a handoff agent
    pub fn handoff(mut self, agent: Agent) -> Self {
        self.handoffs.push(agent);
        self
    }

    /// Set model settings
    pub fn model_settings(mut self, settings: ModelSettings) -> Self {
        self.model_settings = settings;
        self
    }

    /// Set handoff description
    pub fn handoff_description(mut self, desc: impl Into<String>) -> Self {
        self.handoff_description = Some(desc.into());
        self
    }

    /// Build the agent
    pub fn build(self) -> Agent {
        Agent {
            name: self.name,
            instructions: self.instructions,
            model: self.model,
            tools: self.tools,
            handoffs: self.handoffs,
            model_settings: self.model_settings,
            handoff_description: self.handoff_description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_builder() {
        let agent = Agent::builder("test")
            .instructions("You are a helpful assistant")
            .model("gpt-4")
            .build();

        assert_eq!(agent.name, "test");
        assert_eq!(agent.model, "gpt-4");
        assert!(agent.system_prompt().is_some());
    }

    #[test]
    fn test_agent_clone_with() {
        let agent = Agent::builder("test")
            .instructions("Original instructions")
            .build();

        let modified = agent
            .clone_with()
            .instructions("New instructions")
            .build();

        assert_eq!(modified.name, "test");
        assert_eq!(
            modified.system_prompt(),
            Some("New instructions")
        );
    }
}
