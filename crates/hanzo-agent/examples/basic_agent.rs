//! Basic agent example
//!
//! This example shows how to create and run a basic agent with tools.
//!
//! Run with:
//! ```bash
//! export OPENAI_API_KEY=sk-...
//! cargo run --example basic_agent
//! ```

use hanzo_agent::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a calculator tool
    let calculator = FunctionTool::builder("calculator")
        .description("Performs basic arithmetic operations: add, subtract, multiply, divide")
        .schema(json!({
            "type": "object",
            "properties": {
                "operation": {
                    "type": "string",
                    "enum": ["add", "subtract", "multiply", "divide"],
                    "description": "The arithmetic operation to perform"
                },
                "a": {
                    "type": "number",
                    "description": "The first number"
                },
                "b": {
                    "type": "number",
                    "description": "The second number"
                }
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
                "divide" => {
                    if b == 0.0 {
                        return Err(AgentError::ToolError {
                            tool_name: "calculator".to_string(),
                            message: "Division by zero".to_string(),
                        });
                    }
                    a / b
                }
                _ => {
                    return Err(AgentError::ToolError {
                        tool_name: "calculator".to_string(),
                        message: format!("Invalid operation: {}", op),
                    })
                }
            };

            Ok(result.to_string())
        })
        .build()?;

    // Create the agent
    let agent = Agent::builder("math_assistant")
        .instructions(
            "You are a helpful math assistant. Use the calculator tool for all arithmetic operations. \
             Always show your work and explain the calculation.",
        )
        .model("gpt-4")
        .tool(calculator)
        .build();

    // Configure the run
    let config = RunConfig::new()
        .with_api_key(
            std::env::var("OPENAI_API_KEY")
                .expect("OPENAI_API_KEY environment variable not set"),
        )
        .with_max_turns(10);

    // Run the agent
    println!("Running agent...\n");
    let result = agent
        .run("What is 123 * 456? Then divide that by 2.", &config)
        .await?;

    // Display results
    println!("\n=== Results ===");
    println!("Final output: {}", result.final_output);
    println!("\nUsage:");
    println!("  Requests: {}", result.usage.requests);
    println!("  Input tokens: {}", result.usage.input_tokens);
    println!("  Output tokens: {}", result.usage.output_tokens);
    println!("  Total tokens: {}", result.usage.total_tokens);

    println!("\n=== Generated Items ===");
    for (i, item) in result.new_items.iter().enumerate() {
        println!("{}. {:?}", i + 1, item);
    }

    Ok(())
}
