//! Runtime context for agent execution

use crate::types::Usage;
use std::any::Any;
use std::sync::Arc;

/// Runtime context wrapper for agent execution
///
/// The context holds user-provided state and usage statistics.
/// It is passed to tools, hooks, and other callbacks during execution.
#[derive(Debug, Clone)]
pub struct RunContext {
    /// User-provided context data
    context: Option<Arc<dyn Any + Send + Sync>>,
    
    /// Usage statistics
    usage: Usage,
}

impl RunContext {
    /// Create a new run context
    pub fn new() -> Self {
        Self {
            context: None,
            usage: Usage::default(),
        }
    }

    /// Create a run context with user data
    pub fn with_context<T: Any + Send + Sync + 'static>(data: T) -> Self {
        Self {
            context: Some(Arc::new(data)),
            usage: Usage::default(),
        }
    }

    /// Get the user context data
    pub fn context<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.context
            .as_ref()
            .and_then(|c| c.downcast_ref::<T>())
    }

    /// Get mutable access to usage statistics
    pub fn usage_mut(&mut self) -> &mut Usage {
        &mut self.usage
    }

    /// Get usage statistics
    pub fn usage(&self) -> &Usage {
        &self.usage
    }

    /// Add usage from a model response
    pub fn add_usage(&mut self, usage: &Usage) {
        self.usage.add(usage);
    }
}

impl Default for RunContext {
    fn default() -> Self {
        Self::new()
    }
}
