//! Agent registry for managing and accessing specialized agents

use crate::agents::*;
use crate::tools::ToolRegistry;
use crate::traits::{AgentError, Result, SpecializedAgent};
use std::collections::HashMap;
use std::sync::Arc;

/// Types of specialized agents available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentType {
    /// High-level system design and architecture
    Architect,
    /// Technical leadership and code quality
    Cto,
    /// Code review and quality assurance
    Reviewer,
    /// Codebase exploration and documentation
    Explorer,
    /// Task planning and implementation strategy
    Planner,
    /// Research and analysis
    Scientist,
}

impl AgentType {
    /// Get the string name of this agent type
    pub fn name(&self) -> &'static str {
        match self {
            AgentType::Architect => "architect",
            AgentType::Cto => "cto",
            AgentType::Reviewer => "reviewer",
            AgentType::Explorer => "explorer",
            AgentType::Planner => "planner",
            AgentType::Scientist => "scientist",
        }
    }

    /// Parse agent type from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "architect" => Some(AgentType::Architect),
            "cto" => Some(AgentType::Cto),
            "reviewer" | "review" => Some(AgentType::Reviewer),
            "explorer" | "explore" => Some(AgentType::Explorer),
            "planner" | "plan" => Some(AgentType::Planner),
            "scientist" | "research" => Some(AgentType::Scientist),
            _ => None,
        }
    }

    /// List all available agent types
    pub fn all() -> Vec<AgentType> {
        vec![
            AgentType::Architect,
            AgentType::Cto,
            AgentType::Reviewer,
            AgentType::Explorer,
            AgentType::Planner,
            AgentType::Scientist,
        ]
    }
}

impl std::fmt::Display for AgentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Registry for managing specialized agents
///
/// The registry provides a central point for:
/// - Creating and accessing agents
/// - Managing shared tool registry
/// - Configuring agent defaults
pub struct AgentRegistry {
    tool_registry: Arc<ToolRegistry>,
    agents: HashMap<AgentType, Arc<dyn SpecializedAgent>>,
}

impl AgentRegistry {
    /// Create a new agent registry with default tool registry
    pub fn new() -> Self {
        let tool_registry = Arc::new(ToolRegistry::with_defaults());
        Self {
            tool_registry,
            agents: HashMap::new(),
        }
    }

    /// Create a registry with a custom tool registry
    pub fn with_tools(tool_registry: Arc<ToolRegistry>) -> Self {
        Self {
            tool_registry,
            agents: HashMap::new(),
        }
    }

    /// Get the shared tool registry
    pub fn tool_registry(&self) -> Arc<ToolRegistry> {
        self.tool_registry.clone()
    }

    /// Get or create an agent of the specified type
    pub fn get(&mut self, agent_type: AgentType) -> Result<Arc<dyn SpecializedAgent>> {
        if let Some(agent) = self.agents.get(&agent_type) {
            return Ok(agent.clone());
        }

        let agent: Arc<dyn SpecializedAgent> = match agent_type {
            AgentType::Architect => Arc::new(ArchitectAgent::new(self.tool_registry.clone())),
            AgentType::Cto => Arc::new(CtoAgent::new(self.tool_registry.clone())),
            AgentType::Reviewer => Arc::new(ReviewerAgent::new(self.tool_registry.clone())),
            AgentType::Explorer => Arc::new(ExplorerAgent::new(self.tool_registry.clone())),
            AgentType::Planner => Arc::new(PlannerAgent::new(self.tool_registry.clone())),
            AgentType::Scientist => Arc::new(ScientistAgent::new(self.tool_registry.clone())),
        };

        self.agents.insert(agent_type, agent.clone());
        Ok(agent)
    }

    /// Get an agent by name
    pub fn get_by_name(&mut self, name: &str) -> Result<Arc<dyn SpecializedAgent>> {
        let agent_type = AgentType::from_str(name)
            .ok_or_else(|| AgentError::ConfigError(format!("Unknown agent type: {}", name)))?;
        self.get(agent_type)
    }

    /// List all available agent types with descriptions
    pub fn list_agents(&self) -> Vec<(AgentType, &'static str)> {
        vec![
            (
                AgentType::Architect,
                "High-level system design and architecture",
            ),
            (AgentType::Cto, "Technical leadership and code quality"),
            (AgentType::Reviewer, "Code review and quality assurance"),
            (
                AgentType::Explorer,
                "Codebase exploration and documentation",
            ),
            (
                AgentType::Planner,
                "Task planning and implementation strategy",
            ),
            (AgentType::Scientist, "Research and analysis"),
        ]
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_type_from_str() {
        assert_eq!(AgentType::from_str("architect"), Some(AgentType::Architect));
        assert_eq!(AgentType::from_str("CTO"), Some(AgentType::Cto));
        assert_eq!(AgentType::from_str("review"), Some(AgentType::Reviewer));
        assert_eq!(AgentType::from_str("unknown"), None);
    }

    #[test]
    fn test_registry_creation() {
        let mut registry = AgentRegistry::new();
        let agent = registry.get(AgentType::Architect).unwrap();
        assert_eq!(agent.name(), "architect");
    }

    #[test]
    fn test_registry_caching() {
        let mut registry = AgentRegistry::new();
        let agent1 = registry.get(AgentType::Cto).unwrap();
        let agent2 = registry.get(AgentType::Cto).unwrap();
        // Both should be the same Arc
        assert!(Arc::ptr_eq(&agent1, &agent2));
    }
}
