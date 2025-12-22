# hanzo-guard

[![Crates.io](https://img.shields.io/crates/v/hanzo-guard.svg)](https://crates.io/crates/hanzo-guard)
[![Documentation](https://docs.rs/hanzo-guard/badge.svg)](https://docs.rs/hanzo-guard)
[![CI](https://github.com/hanzoai/rust-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/hanzoai/rust-sdk/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Fast, comprehensive LLM I/O sanitization layer for Rust. Provides PII detection/redaction, prompt injection detection, rate limiting, content filtering, and audit logging with sub-millisecond latency.

## Features

- **PII Detection & Redaction**: SSN, credit cards (Luhn validated), emails, phone numbers, IP addresses, API keys
- **Prompt Injection Detection**: Jailbreak attempts, system prompt leaks, role-play manipulation, encoding tricks
- **Rate Limiting**: Per-user request throttling with configurable burst handling
- **Content Filtering**: ML-based safety classification via external API (Zen Guard integration)
- **Audit Logging**: JSONL audit trails with content hashing for compliance
- **Sub-millisecond Latency**: Pure Rust implementation, no external API calls for core features

## Quick Start

```bash
cargo add hanzo-guard
```

```rust
use hanzo_guard::{Guard, GuardConfig, SanitizeResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create guard with default config (all features enabled)
    let guard = Guard::new(GuardConfig::default());

    // Sanitize user input
    let result = guard.sanitize_input("My SSN is 123-45-6789").await?;

    match result {
        SanitizeResult::Clean(text) => println!("Clean: {text}"),
        SanitizeResult::Redacted { text, redactions } => {
            println!("Redacted: {text}");
            println!("Found {} PII items", redactions.len());
        }
        SanitizeResult::Blocked { reason, .. } => {
            println!("Blocked: {reason}");
        }
    }

    Ok(())
}
```

## Configuration

### Minimal (PII only)

```rust
let guard = Guard::new(GuardConfig::minimal());
```

### Builder Pattern

```rust
let guard = Guard::builder()
    .pii_only()           // Only PII detection
    .with_injection()     // Add injection detection
    .with_rate_limit()    // Add rate limiting
    .build();
```

### Full Configuration

```rust
use hanzo_guard::config::*;

let config = GuardConfig {
    pii: PiiConfig {
        enabled: true,
        detect_ssn: true,
        detect_credit_card: true,
        detect_email: true,
        detect_phone: true,
        detect_ip: true,
        detect_api_keys: true,
        redaction_format: "[REDACTED:{TYPE}]".into(),
    },
    injection: InjectionConfig {
        enabled: true,
        block_on_detection: true,
        sensitivity: 0.7,
        custom_patterns: vec!["my-custom-pattern".into()],
    },
    rate_limit: RateLimitConfig {
        enabled: true,
        requests_per_minute: 60,
        burst_size: 10,
    },
    content_filter: ContentFilterConfig {
        enabled: false, // Requires external API
        api_url: "https://api.hanzo.ai/guard".into(),
        ..Default::default()
    },
    audit: AuditConfig {
        enabled: true,
        log_path: Some("/var/log/hanzo-guard.jsonl".into()),
        ..Default::default()
    },
};

let guard = Guard::new(config);
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `pii` | Yes | PII detection and redaction |
| `rate-limit` | Yes | Rate limiting with governor |
| `content-filter` | No | External ML content classification |
| `audit` | Yes | Audit logging |

```toml
# Minimal (just core types)
hanzo-guard = { version = "0.1", default-features = false }

# PII only
hanzo-guard = { version = "0.1", default-features = false, features = ["pii"] }

# Full features
hanzo-guard = { version = "0.1", features = ["content-filter"] }
```

## Context-Aware Sanitization

```rust
use hanzo_guard::GuardContext;

let context = GuardContext::new()
    .with_user_id("user123")
    .with_session_id("session456")
    .with_metadata("model", "gpt-4");

let result = guard.sanitize_input_with_context(input, &context).await?;
```

## Integration with Zen Guard

For ML-based content classification, hanzo-guard integrates with [Zen Guard](https://github.com/zenlm/zen-guard):

```
┌─────────────┐     ┌──────────────┐     ┌────────────┐
│ Application │ ──► │ Hanzo Guard  │ ──► │ Zen Guard  │
└─────────────┘     │ (Rust, <1ms) │     │ (ML Model) │
                    │              │     │            │
                    │ • PII Redact │     │ • Content  │
                    │ • Rate Limit │     │   Classify │
                    │ • Injection  │     │ • Severity │
                    │   Detect     │     │   Levels   │
                    │ • Audit Log  │     │ • 119 Lang │
                    └──────────────┘     └────────────┘
```

## Performance

| Operation | Latency | Throughput |
|-----------|---------|------------|
| PII Detection | ~50μs | 20K+ ops/sec |
| Injection Check | ~20μs | 50K+ ops/sec |
| Full Sanitize | ~100μs | 10K+ ops/sec |
| Rate Limit Check | ~1μs | 1M+ ops/sec |

*Benchmarked on Apple M1 Max, single-threaded

## Safety Categories

When using content filtering, content is classified into these categories:

- **Violent**: Violence instructions or depictions
- **Illegal**: Hacking, unauthorized activities
- **Sexual**: Adult content
- **PII**: Personal information disclosure
- **SelfHarm**: Self-harm encouragement
- **Unethical**: Bias, discrimination, hate
- **Political**: False political information
- **Copyright**: Copyrighted material
- **Jailbreak**: System prompt override attempts

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Related

- [hanzo-extract](../hanzo-extract) - Content extraction with hanzo-guard integration
- [Zen Guard](https://github.com/zenlm/zen-guard) - ML-based safety classification
