# hanzo-agent-proxy

Multi-backend OpenAI-compatible API proxy for AI agents and CLIs.

## Features

- **OpenAI-compatible API** - Drop-in replacement for OpenAI API endpoints
- **Multiple backends** - Route requests to different AI providers
- **Subscription mode** - Use CLI tools with your existing subscriptions (no API keys needed)
- **HTTP passthrough** - Direct HTTP forwarding for local inference servers

## Supported Backends

| Backend | Model Aliases | Method | Description |
|---------|--------------|--------|-------------|
| claude | claude-cli, claude-opus | CLI | Anthropic subscription |
| codex | codex, gpt-4o, o1 | CLI | OpenAI subscription |
| gemini | gemini, gemini-2.5-pro | CLI | Google account |
| vibe | vibe, mistral | CLI | Mistral subscription |
| qwen | qwen, qwen-plus | CLI | Alibaba account |
| ollama | ollama, llama3.2 | CLI | Local (no keys) |
| hanzo | hanzo, hanzo-engine, default | HTTP | Local inference |

## Usage

```bash
# Start the proxy
hanzo-proxy --port 9998

# With custom hanzo engine URL
HANZO_ENGINE_URL="http://localhost:8080" hanzo-proxy --port 9998

# With custom default model for hanzo backend
HANZO_DEFAULT_MODEL="llama3.2" hanzo-proxy --port 9998
```

## API Endpoints

- `POST /v1/chat/completions` - Chat completions (OpenAI-compatible)
- `GET /v1/models` - List available models
- `GET /health` - Health check

## Example Request

```bash
curl http://localhost:9998/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "claude-cli",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `HANZO_ENGINE_URL` | `http://localhost:8080` | URL of hanzo inference engine |
| `HANZO_DEFAULT_MODEL` | `default` | Default model for hanzo backend |

## Integration

This proxy is designed to be used with:
- **hanzo-node** - Full AI node with inference and embeddings
- **hanzo-engine** - Local LLM inference server
- **hanzo-dev** - AI-powered development tools
- **Any OpenAI-compatible client** - Works with any client expecting OpenAI API

## License

MIT OR Apache-2.0
