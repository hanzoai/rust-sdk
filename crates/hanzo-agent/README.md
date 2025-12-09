# hanzo-agent

Core agent framework for Hanzo AI, ported from the Python agent SDK.

## Overview

`hanzo-agent` provides a flexible, type-safe agent framework for building AI applications with:

- **Agents**: Configured with instructions, tools, and model settings
- **Tools**: Functions that agents can call to perform actions
- **Handoffs**: Delegate to specialized sub-agents
- **Runner**: Execute the agent loop with tool execution and handoff support
- **OpenAI-compatible API**: Works with any OpenAI-compatible LLM endpoint

## Features

- ✅ Async/await with tokio
- ✅ Type-safe tool system with JSON schema
- ✅ Handoff mechanism (TODO: full implementation)
- ✅ Usage tracking
- ✅ Error handling with Result types
- ✅ Builder pattern for easy configuration
- 🔜 Streaming support
- 🔜 Guardrails
- 🔜 Lifecycle hooks

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
hanzo-agent = { path = "path/to/hanzo-agent" }
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use hanzo_agent::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an agent
    let agent = Agent::builder("assistant")
        .instructions("You are a helpful assistant.")
        .model("gpt-4")
        .build();

    // Configure the run
    let config = RunConfig::new()
        .with_api_key(std::env::var("OPENAI_API_KEY")?)
        .with_max_turns(10);

    // Run the agent
    let result = agent.run("What is 2+2?", &config).await?;
    println!("Response: {}", result.final_output);
    println!("Usage: {:?}", result.usage);

    Ok(())
}
```

## Adding Tools

```rust
use hanzo_agent::prelude::*;
use serde_json::json;

// Create a simple tool
let calculator = FunctionTool::builder("calculator")
    .description("Performs arithmetic operations")
    .schema(json!({
        "type": "object",
        "properties": {
            "operation": {
                "type": "string",
                "enum": ["add", "subtract", "multiply", "divide"]
            },
            "a": { "type": "number" },
            "b": { "type": "number" }
        },
        "required": ["operation", "a", "b"]
    }))
    .handler(|_ctx, args| {
        let op = args["operation"].as_str().unwrap();
        let a = args["a"].as_f64().unwrap();
        let b = args["b"].as_f64().unwrap();
        
        let result = match op {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => a / b,
            _ => return Err(AgentError::ToolError {
                tool_name: "calculator".to_string(),
                message: "Invalid operation".to_string(),
            }),
        };
        
        Ok(result.to_string())
    })
    .build()?;

// Add to agent
let agent = Agent::builder("math_assistant")
    .instructions("You are a math assistant. Use the calculator for all arithmetic.")
    .tool(calculator)
    .build();
```

## Architecture

The agent framework follows a simple execution loop:

1. **Build messages**: Combine original input with any generated items
2. **Call LLM**: Send messages to the model with available tools
3. **Process response**:
   - If tool calls: Execute tools and continue loop
   - If text message: Return as final output
   - If handoff: Switch to new agent (TODO)
4. **Repeat** until max_turns or final output

### Core Types

- **Agent**: The main configuration object (name, instructions, model, tools, handoffs)
- **Tool**: Trait for tools with name, description, JSON schema, and invoke method
- **RunContext**: Runtime context with user data and usage tracking
- **RunResult**: Contains input, generated items, responses, output, and usage
- **RunConfig**: Configuration for the run (max_turns, API settings)

## Comparison with Python SDK

This Rust implementation mirrors the Python agent SDK core functionality:

| Feature | Python SDK | Rust SDK | Status |
|---------|-----------|----------|--------|
| Agent struct | ✅ | ✅ | Complete |
| Tool trait | ✅ | ✅ | Complete |
| Runner loop | ✅ | ✅ | Complete |
| Result types | ✅ | ✅ | Complete |
| Context | ✅ | ✅ | Complete |
| Handoffs | ✅ | 🔜 | Partial |
| Streaming | ✅ | 🔜 | TODO |
| Guardrails | ✅ | 🔜 | TODO |
| Hooks | ✅ | 🔜 | TODO |
| Output types | ✅ | 🔜 | TODO |

## Examples

See the Python SDK examples for inspiration:
- `/Users/z/work/hanzo/agent/examples/`

Rust examples coming soon!

## Environment Variables

- `OPENAI_API_BASE`: API base URL (default: https://api.openai.com/v1)
- `OPENAI_API_KEY`: API key for authentication

## Testing

Run tests:

```bash
cargo test -p hanzo-agent
```

Run with output:

```bash
cargo test -p hanzo-agent -- --nocapture
```

## Development

### Building

```bash
cargo build -p hanzo-agent
```

### Linting

```bash
cargo clippy -p hanzo-agent
```

### Formatting

```bash
cargo fmt -p hanzo-agent
```

## License

MIT OR Apache-2.0

## Links

- [Hanzo AI](https://hanzo.ai)
- [Python Agent SDK](/Users/z/work/hanzo/agent)
- [Rust SDK](/Users/z/work/hanzo/rust-sdk)
