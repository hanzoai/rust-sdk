//! Hanzo Agents - Specialized AI agents for development workflows
//!
//! This crate provides pre-built agent types that can be used across
//! hanzo-dev (CLI) and hanzo-node for consistent AI-powered development assistance.
//!
//! # Agent Types
//!
//! - **Architect Agent**: High-level system design and architectural decisions
//! - **CTO Agent**: Technical leadership, code quality, and best practices
//! - **Reviewer Agent**: Code review, quality assurance, and suggestions
//! - **Explorer Agent**: Codebase exploration and documentation
//! - **Planner Agent**: Task planning and implementation strategy
//!
//! # Architecture
//!
//! The agent system follows patterns from hanzo-engine:
//! - **Trait-based extensibility**: Tools implement a consistent callback interface
//! - **MCP integration**: Tools can be provided via Model Context Protocol servers
//! - **OpenAI compatibility**: Tool calling uses OpenAI-compatible format
//! - **Async-first design**: All APIs are async with proper concurrency control
//!
//! # Example
//!
//! ```no_run
//! use hanzo_agents::{AgentRegistry, AgentType};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let registry = AgentRegistry::new();
//!     let architect = registry.get(AgentType::Architect)?;
//!
//!     let result = architect
//!         .run("Design a microservices architecture for a payment system")
//!         .await?;
//!
//!     println!("{}", result.output);
//!     Ok(())
//! }
//! ```

pub mod agents;
pub mod prompts;
pub mod registry;
pub mod traits;
pub mod tools;

pub use agents::*;
pub use registry::{AgentRegistry, AgentType};
pub use traits::SpecializedAgent;

/// Re-exports from hanzo-agent for convenience
pub mod prelude {
    pub use crate::registry::{AgentRegistry, AgentType};
    pub use crate::traits::SpecializedAgent;
    pub use crate::tools::ToolRegistry;
    pub use hanzo_agent::prelude::*;
}
