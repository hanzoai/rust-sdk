//! Agent execution runner

use crate::agent::Agent;
use crate::context::RunContext;
use crate::errors::{AgentError, Result};
use crate::result::RunResult;
use crate::types::{InputItem, ModelResponse, ModelSettings, RunItem, Usage};
use serde_json::{json, Value};
use tracing::{debug, info, warn};

/// Default maximum turns for agent execution
pub const DEFAULT_MAX_TURNS: usize = 10;

/// Configuration for agent run
#[derive(Debug, Clone)]
pub struct RunConfig {
    /// Maximum number of turns (LLM invocations)
    pub max_turns: usize,

    /// The API base URL for the LLM provider
    pub api_base: String,

    /// API key for authentication
    pub api_key: Option<String>,

    /// Global model settings override
    pub model_settings: Option<ModelSettings>,

    /// Whether to include tool calls in the context
    pub include_tool_calls: bool,
}

impl RunConfig {
    /// Create a new run config with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the API base URL
    pub fn with_api_base(mut self, url: impl Into<String>) -> Self {
        self.api_base = url.into();
        self
    }

    /// Set the API key
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set maximum turns
    pub fn with_max_turns(mut self, turns: usize) -> Self {
        self.max_turns = turns;
        self
    }
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            max_turns: DEFAULT_MAX_TURNS,
            api_base: std::env::var("OPENAI_API_BASE")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            model_settings: None,
            include_tool_calls: true,
        }
    }
}

/// Runner executes the agent loop
pub struct Runner;

impl Runner {
    /// Run the agent with the given input
    ///
    /// This executes the agent loop:
    /// 1. Send input to LLM
    /// 2. If tool calls are returned, execute them
    /// 3. If a final output is returned, complete
    /// 4. If handoff occurs, switch to new agent
    /// 5. Repeat until max_turns or final output
    pub async fn run(agent: &Agent, input: String, config: &RunConfig) -> Result<RunResult> {
        let mut ctx = RunContext::new();
        let current_agent = agent;
        let mut turn = 0;
        let original_input = vec![InputItem::user_message(input)];
        let mut generated_items: Vec<RunItem> = Vec::new();
        let mut model_responses: Vec<ModelResponse> = Vec::new();

        info!("Starting agent run: {}", agent.name);

        loop {
            turn += 1;
            if turn > config.max_turns {
                warn!("Max turns ({}) exceeded", config.max_turns);
                return Err(AgentError::MaxTurnsExceeded(config.max_turns));
            }

            debug!("Turn {}: Running agent {}", turn, current_agent.name);

            // Build the messages for this turn
            let mut messages = original_input.clone();
            messages.extend(generated_items.iter().map(|item| item.to_input_item()));

            // Add system prompt if present
            let mut system_messages = Vec::new();
            if let Some(prompt) = current_agent.system_prompt() {
                system_messages.push(InputItem::system_message(prompt));
            }

            // Get response from LLM
            let response =
                Self::call_llm(current_agent, &system_messages, &messages, config, &mut ctx)
                    .await?;

            model_responses.push(response.clone());
            ctx.add_usage(&response.usage);

            // Process the response
            let (next_step, new_items) =
                Self::process_response(current_agent, response, &mut ctx, config).await?;

            generated_items.extend(new_items);

            match next_step {
                NextStep::FinalOutput(output) => {
                    info!("Agent completed with output");
                    return Ok(RunResult::new(
                        original_input,
                        generated_items,
                        model_responses,
                        output,
                        ctx.usage().clone(),
                    ));
                }
                NextStep::RunAgain => {
                    debug!("Continuing agent loop (tools executed)");
                    continue;
                }
                NextStep::Handoff(_new_agent) => {
                    // TODO: Implement handoff logic
                    warn!("Handoff not yet implemented");
                    return Err(AgentError::Configuration(
                        "Handoff not yet implemented".to_string(),
                    ));
                }
            }
        }
    }

    /// Call the LLM with the current messages
    async fn call_llm(
        agent: &Agent,
        system_messages: &[InputItem],
        messages: &[InputItem],
        config: &RunConfig,
        _ctx: &mut RunContext,
    ) -> Result<ModelResponse> {
        let client = reqwest::Client::new();

        // Build the request body
        let mut all_messages = Vec::new();
        all_messages.extend(Self::items_to_openai_messages(system_messages));
        all_messages.extend(Self::items_to_openai_messages(messages));

        let mut body = json!({
            "model": agent.model,
            "messages": all_messages,
        });

        // Add tools if present
        if !agent.tools.is_empty() {
            let tools: Vec<Value> = agent
                .tools
                .iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "function": {
                            "name": t.name(),
                            "description": t.description(),
                            "parameters": t.json_schema(),
                        }
                    })
                })
                .collect();
            body["tools"] = json!(tools);
        }

        // Add model settings
        let settings = config
            .model_settings
            .as_ref()
            .unwrap_or(&agent.model_settings);
        if let Some(temp) = settings.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(top_p) = settings.top_p {
            body["top_p"] = json!(top_p);
        }
        if let Some(max_tokens) = settings.max_tokens {
            body["max_tokens"] = json!(max_tokens);
        }

        debug!("Calling LLM: {}", agent.model);

        // Make the request
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| AgentError::Configuration("API key not set".to_string()))?;

        let response = client
            .post(format!("{}/chat/completions", config.api_base))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AgentError::ModelError(format!(
                "LLM API error {}: {}",
                status, error_text
            )));
        }

        let response_json: Value = response.json().await?;
        debug!("LLM response: {:?}", response_json);

        Self::parse_llm_response(response_json)
    }

    /// Parse the LLM response into a ModelResponse
    fn parse_llm_response(response: Value) -> Result<ModelResponse> {
        let choice = response["choices"]
            .get(0)
            .ok_or_else(|| AgentError::ModelBehavior("No choices in response".to_string()))?;

        let message = &choice["message"];
        let mut output = Vec::new();

        // Check for text content
        if let Some(content) = message["content"].as_str() {
            if !content.is_empty() {
                output.push(RunItem::Message {
                    role: "assistant".to_string(),
                    content: content.to_string(),
                });
            }
        }

        // Check for tool calls
        if let Some(tool_calls) = message["tool_calls"].as_array() {
            for call in tool_calls {
                let id = call["id"]
                    .as_str()
                    .ok_or_else(|| AgentError::ModelBehavior("Missing tool call id".to_string()))?;
                let function = &call["function"];
                let name = function["name"]
                    .as_str()
                    .ok_or_else(|| AgentError::ModelBehavior("Missing tool name".to_string()))?;
                let args = function["arguments"].as_str().ok_or_else(|| {
                    AgentError::ModelBehavior("Missing tool arguments".to_string())
                })?;

                output.push(RunItem::ToolCall {
                    id: id.to_string(),
                    name: name.to_string(),
                    arguments: args.to_string(),
                });
            }
        }

        // Parse usage
        let usage = if let Some(u) = response["usage"].as_object() {
            Usage {
                requests: 1,
                input_tokens: u["prompt_tokens"].as_u64().unwrap_or(0) as usize,
                output_tokens: u["completion_tokens"].as_u64().unwrap_or(0) as usize,
                total_tokens: u["total_tokens"].as_u64().unwrap_or(0) as usize,
            }
        } else {
            Usage::default()
        };

        Ok(ModelResponse {
            output,
            usage,
            id: response["id"].as_str().map(|s| s.to_string()),
        })
    }

    /// Process the model response
    async fn process_response(
        agent: &Agent,
        response: ModelResponse,
        ctx: &mut RunContext,
        _config: &RunConfig,
    ) -> Result<(NextStep, Vec<RunItem>)> {
        let mut new_items = Vec::new();

        // Check if there are tool calls to execute
        let tool_calls: Vec<_> = response
            .output
            .iter()
            .filter_map(|item| {
                if let RunItem::ToolCall {
                    id,
                    name,
                    arguments,
                } = item
                {
                    Some((id.clone(), name.clone(), arguments.clone()))
                } else {
                    None
                }
            })
            .collect();

        if !tool_calls.is_empty() {
            debug!("Executing {} tool calls", tool_calls.len());

            // Add the tool calls to new items
            for (id, name, args) in &tool_calls {
                new_items.push(RunItem::ToolCall {
                    id: id.clone(),
                    name: name.clone(),
                    arguments: args.clone(),
                });
            }

            // Execute each tool
            for (id, name, args) in tool_calls {
                let tool = agent
                    .tools
                    .iter()
                    .find(|t| t.name() == name)
                    .ok_or_else(|| AgentError::ToolError {
                        tool_name: name.clone(),
                        message: "Tool not found".to_string(),
                    })?;

                debug!("Invoking tool: {}", name);
                let result = tool
                    .invoke(ctx, &args)
                    .await
                    .map_err(|e| AgentError::ToolError {
                        tool_name: name.clone(),
                        message: e.to_string(),
                    })?;

                new_items.push(RunItem::ToolResult {
                    tool_call_id: id,
                    content: result,
                });
            }

            return Ok((NextStep::RunAgain, new_items));
        }

        // Check for final output (text message)
        for item in &response.output {
            if let RunItem::Message { content, .. } = item {
                new_items.push(item.clone());
                return Ok((NextStep::FinalOutput(content.clone()), new_items));
            }
        }

        // No tool calls and no text - this is an error
        Err(AgentError::ModelBehavior(
            "Model produced no tool calls or text output".to_string(),
        ))
    }

    /// Convert InputItems to OpenAI message format
    fn items_to_openai_messages(items: &[InputItem]) -> Vec<Value> {
        items
            .iter()
            .map(|item| match item {
                InputItem::Message { role, content } => {
                    json!({
                        "role": role,
                        "content": content,
                    })
                }
                InputItem::ToolResult {
                    tool_call_id,
                    content,
                } => {
                    json!({
                        "role": "tool",
                        "tool_call_id": tool_call_id,
                        "content": content,
                    })
                }
            })
            .collect()
    }
}

/// Next step in the agent loop
#[derive(Debug)]
enum NextStep {
    /// Agent produced final output
    FinalOutput(String),

    /// Run the agent again (after tool execution)
    RunAgain,

    /// Handoff to another agent (planned for multi-agent workflows)
    #[allow(dead_code)]
    Handoff(Agent),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_config_builder() {
        let config = RunConfig::new()
            .with_max_turns(5)
            .with_api_base("https://example.com");

        assert_eq!(config.max_turns, 5);
        assert_eq!(config.api_base, "https://example.com");
    }
}
