//! Tool system for agents

use crate::context::RunContext;
use crate::errors::{AgentError, Result};
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

/// Tool trait for agent tools
///
/// Tools are functions that agents can call to perform actions.
/// They have a name, description, JSON schema, and an invoke method.
#[async_trait]
pub trait Tool: Send + Sync {
    /// The name of the tool
    fn name(&self) -> &str;

    /// A description of what the tool does
    fn description(&self) -> &str;

    /// JSON schema for the tool's parameters
    fn json_schema(&self) -> Value;

    /// Invoke the tool with the given context and arguments
    ///
    /// # Arguments
    /// * `ctx` - The runtime context
    /// * `args` - JSON string containing the tool arguments
    ///
    /// # Returns
    /// The tool result as a string, or an error
    async fn invoke(&self, ctx: &RunContext, args: &str) -> Result<String>;
}

/// A function-based tool implementation
pub struct FunctionTool {
    name: String,
    description: String,
    json_schema: Value,
    handler: Arc<dyn Fn(&RunContext, Value) -> Result<String> + Send + Sync>,
}

impl FunctionTool {
    /// Create a new function tool
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        json_schema: Value,
        handler: impl Fn(&RunContext, Value) -> Result<String> + Send + Sync + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            json_schema,
            handler: Arc::new(handler),
        }
    }

    /// Builder for creating function tools
    pub fn builder(name: impl Into<String>) -> FunctionToolBuilder {
        FunctionToolBuilder {
            name: name.into(),
            description: String::new(),
            json_schema: Value::Null,
            handler: None,
        }
    }
}

#[async_trait]
impl Tool for FunctionTool {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn json_schema(&self) -> Value {
        self.json_schema.clone()
    }

    async fn invoke(&self, ctx: &RunContext, args: &str) -> Result<String> {
        let value: Value = serde_json::from_str(args)
            .map_err(|e| AgentError::InvalidJson(e.to_string()))?;
        (self.handler)(ctx, value)
    }
}

/// Builder for FunctionTool
pub struct FunctionToolBuilder {
    name: String,
    description: String,
    json_schema: Value,
    handler: Option<Arc<dyn Fn(&RunContext, Value) -> Result<String> + Send + Sync>>,
}

impl FunctionToolBuilder {
    /// Set the tool description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Set the JSON schema
    pub fn schema(mut self, schema: Value) -> Self {
        self.json_schema = schema;
        self
    }

    /// Set the handler function
    pub fn handler<F>(mut self, f: F) -> Self
    where
        F: Fn(&RunContext, Value) -> Result<String> + Send + Sync + 'static,
    {
        self.handler = Some(Arc::new(f));
        self
    }

    /// Build the function tool
    pub fn build(self) -> Result<FunctionTool> {
        let handler = self
            .handler
            .ok_or_else(|| AgentError::Configuration("Tool handler not set".to_string()))?;

        Ok(FunctionTool {
            name: self.name,
            description: self.description,
            json_schema: self.json_schema,
            handler,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_function_tool() {
        let tool = FunctionTool::builder("test_tool")
            .description("A test tool")
            .schema(json!({"type": "object"}))
            .handler(|_ctx, _args| Ok("test result".to_string()))
            .build()
            .unwrap();

        assert_eq!(tool.name(), "test_tool");
        assert_eq!(tool.description(), "A test tool");

        let ctx = RunContext::new();
        let result = tool.invoke(&ctx, "{}").await.unwrap();
        assert_eq!(result, "test result");
    }
}
