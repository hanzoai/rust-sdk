//! Hanzo Agent SDK - Core agent functionality
//!
//! This crate provides the core agent framework for building AI agents with tools, handoffs,
//! and structured output support.
//!
//! # Example
//!
//! ```no_run
//! use hanzo_agent::{Agent, RunConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let agent = Agent::builder("assistant")
//!         .instructions("You are a helpful assistant.")
//!         .model("gpt-4")
//!         .build();
//!
//!     let result = agent.run("Hello!", &RunConfig::default()).await?;
//!     println!("Response: {}", result.final_output);
//!     Ok(())
//! }
//! ```

pub mod agent;
pub mod context;
pub mod errors;
pub mod result;
pub mod runner;
pub mod tool;
pub mod types;

pub use agent::{Agent, AgentBuilder};
pub use context::RunContext;
pub use errors::AgentError;
pub use result::{RunResult, RunResultStreaming};
pub use runner::{RunConfig, Runner};
pub use tool::{FunctionTool, Tool};
pub use types::{InputItem, ModelResponse, RunItem};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::types::Usage;
    pub use crate::{
        Agent, AgentBuilder, AgentError, FunctionTool, RunConfig, RunContext, RunResult, Runner,
        Tool,
    };
}
