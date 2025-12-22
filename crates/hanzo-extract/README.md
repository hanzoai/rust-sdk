# hanzo-extract

[![Crates.io](https://img.shields.io/crates/v/hanzo-extract.svg)](https://crates.io/crates/hanzo-extract)
[![Documentation](https://docs.rs/hanzo-extract/badge.svg)](https://docs.rs/hanzo-extract)
[![CI](https://github.com/hanzoai/rust-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/hanzoai/rust-sdk/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Content extraction library for Rust with built-in sanitization via hanzo-guard. Extract clean text from web pages and PDF documents with automatic PII redaction and safety filtering.

## Features

- **Web Extraction**: Fetch and extract clean text from web pages with smart content detection
- **PDF Extraction**: Extract text from PDF files with metadata preservation
- **Built-in Sanitization**: Optional PII redaction and safety filtering via hanzo-guard
- **Async/Await**: Non-blocking I/O for high-performance applications
- **Configurable**: Timeout, redirect handling, content length limits, user agent

## Quick Start

```bash
cargo add hanzo-extract
```

```rust
use hanzo_extract::{Extractor, WebExtractor, ExtractorConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor = WebExtractor::new(ExtractorConfig::default());

    // Extract text from a web page
    let result = extractor.extract("https://example.com").await?;

    println!("Title: {:?}", result.title);
    println!("Text: {} characters", result.text_length);
    println!("Content: {}", result.text);

    Ok(())
}
```

## Extractors

### Web Extractor

Extracts clean text from HTML web pages:

```rust
use hanzo_extract::{WebExtractor, ExtractorConfig};

let config = ExtractorConfig {
    timeout_secs: 30,
    max_length: 1_000_000,
    clean_text: true,
    follow_redirects: true,
    max_redirects: 5,
    user_agent: "Hanzo-Extract/0.1".into(),
    ..Default::default()
};

let extractor = WebExtractor::new(config);
let result = extractor.extract("https://example.com").await?;
```

Features:
- Smart content area detection (article, main, content divs)
- Script/style tag removal
- Whitespace normalization
- Title extraction

### PDF Extractor

Extracts text from PDF documents:

```rust
use hanzo_extract::{PdfExtractor, Extractor};

let extractor = PdfExtractor::default();

// From file path
let result = extractor.extract("/path/to/document.pdf").await?;

// From URL (requires 'web' feature)
let result = extractor.extract("https://example.com/doc.pdf").await?;

println!("Pages: {:?}", result.metadata.get("page_count"));
println!("Author: {:?}", result.metadata.get("author"));
```

Features:
- Page-by-page text extraction
- PDF metadata extraction (title, author)
- URL fetching support
- Whitespace normalization

## Sanitized Extraction

Enable the `sanitize` feature for automatic PII redaction:

```toml
hanzo-extract = { version = "0.1", features = ["sanitize"] }
```

```rust
use hanzo_extract::{WebExtractor, Extractor};

let extractor = WebExtractor::default();

// Extract with automatic sanitization
let result = extractor.extract_sanitized("https://example.com").await?;

if result.sanitized {
    println!("Sanitization applied:");
    if let Some(info) = &result.sanitization {
        println!("  PII redacted: {}", info.pii_redacted);
        println!("  PII types: {:?}", info.pii_types);
    }
}
```

## Configuration

```rust
use hanzo_extract::ExtractorConfig;

let config = ExtractorConfig {
    // Request settings
    timeout_secs: 30,
    max_length: 1_000_000,
    user_agent: "MyApp/1.0".into(),

    // Redirect handling
    follow_redirects: true,
    max_redirects: 5,

    // Text processing
    clean_text: true,

    // Sanitization (when 'sanitize' feature enabled)
    redact_pii: true,
    detect_injection: true,
};
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `web` | Yes | Web page extraction with reqwest |
| `pdf` | Yes | PDF extraction with lopdf |
| `sanitize` | Yes | PII redaction via hanzo-guard |

```toml
# Web only
hanzo-extract = { version = "0.1", default-features = false, features = ["web"] }

# PDF only
hanzo-extract = { version = "0.1", default-features = false, features = ["pdf"] }

# No sanitization
hanzo-extract = { version = "0.1", default-features = false, features = ["web", "pdf"] }
```

## Extraction Result

```rust
pub struct ExtractResult {
    /// Extracted/sanitized text content
    pub text: String,

    /// Original source URL or file path
    pub source: String,

    /// Content type (e.g., "text/html", "application/pdf")
    pub content_type: Option<String>,

    /// Extracted title (from HTML or PDF metadata)
    pub title: Option<String>,

    /// Length of extracted text
    pub text_length: usize,

    /// Original content length before processing
    pub original_length: usize,

    /// Whether sanitization was applied
    pub sanitized: bool,

    /// Sanitization details (when applied)
    pub sanitization: Option<SanitizationInfo>,

    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
```

## Error Handling

```rust
use hanzo_extract::{ExtractError, Extractor};

match extractor.extract(url).await {
    Ok(result) => println!("Extracted: {}", result.text_length),
    Err(ExtractError::InvalidUrl(url)) => println!("Bad URL: {url}"),
    Err(ExtractError::Http { status, message }) => {
        println!("HTTP {status}: {message}");
    }
    Err(ExtractError::ContentTooLarge { size, max }) => {
        println!("Content too large: {size} > {max}");
    }
    Err(ExtractError::Blocked(reason)) => {
        println!("Content blocked: {reason}");
    }
    Err(e) => println!("Error: {e}"),
}
```

## Performance

| Operation | Latency | Notes |
|-----------|---------|-------|
| Web fetch + extract | ~100-500ms | Network dependent |
| HTML parsing | ~1-5ms | Content size dependent |
| PDF extraction | ~10-50ms | Page count dependent |
| Sanitization | ~100μs | Via hanzo-guard |

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

## Related

- [hanzo-guard](../hanzo-guard) - LLM I/O sanitization layer
- [Zen Guard](https://github.com/zenlm/zen-guard) - ML-based safety classification
