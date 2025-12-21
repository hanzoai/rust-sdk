//! # Hanzo Extract
//!
//! Content extraction with built-in sanitization via `hanzo-guard`.
//!
//! This crate provides utilities for extracting text content from various sources
//! (web pages, PDFs, etc.) and sanitizing the output for safe use with LLMs.
//!
//! ## Features
//!
//! - **Web Extraction**: Fetch and extract clean text from web pages
//! - **PDF Extraction**: Extract text from PDF documents
//! - **Sanitization**: Automatic PII redaction and injection detection via `hanzo-guard`
//!
//! ## Example
//!
//! ```rust,ignore
//! use hanzo_extract::{WebExtractor, ExtractorConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let extractor = WebExtractor::new(ExtractorConfig::default());
//!     let result = extractor.extract("https://example.com").await?;
//!     println!("Extracted: {}", result.text);
//!     Ok(())
//! }
//! ```
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
//! │   Source    │ ──► │  Extractor   │ ──► │  Hanzo Guard    │
//! │ (URL/PDF)   │     │ (Text Parse) │     │ (Sanitization)  │
//! └─────────────┘     └──────────────┘     └─────────────────┘
//!                                                   │
//!                                                   ▼
//!                                          ┌─────────────────┐
//!                                          │  Clean Output   │
//!                                          │ (LLM-Ready)     │
//!                                          └─────────────────┘
//! ```

pub mod config;
pub mod error;
pub mod result;

#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "pdf")]
pub mod pdf;

#[cfg(feature = "sanitize")]
pub mod sanitize;

pub use config::ExtractorConfig;
pub use error::{ExtractError, Result};
pub use result::ExtractResult;

#[cfg(feature = "web")]
pub use web::WebExtractor;

#[cfg(feature = "pdf")]
pub use pdf::PdfExtractor;

/// Common trait for all extractors
#[async_trait::async_trait]
pub trait Extractor: Send + Sync {
    /// Extract text content from the given source
    async fn extract(&self, source: &str) -> Result<ExtractResult>;
    
    /// Extract and sanitize content
    #[cfg(feature = "sanitize")]
    async fn extract_sanitized(&self, source: &str) -> Result<ExtractResult>;
}
