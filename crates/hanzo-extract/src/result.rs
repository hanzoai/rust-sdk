//! Extraction result types

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Result of content extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResult {
    /// The extracted text content
    pub text: String,
    
    /// Source URL or path
    pub source: String,
    
    /// Content type (e.g., "text/html", "application/pdf")
    pub content_type: Option<String>,
    
    /// Original content length in bytes
    pub original_length: usize,
    
    /// Extracted text length in characters
    pub text_length: usize,
    
    /// Page title (for web pages)
    pub title: Option<String>,
    
    /// Metadata extracted from the source
    pub metadata: HashMap<String, String>,
    
    /// Whether the content was sanitized
    pub sanitized: bool,
    
    /// Sanitization details
    #[cfg(feature = "sanitize")]
    pub sanitization: Option<SanitizationInfo>,
}

/// Information about sanitization applied
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg(feature = "sanitize")]
pub struct SanitizationInfo {
    /// Number of PII items redacted
    pub pii_redacted: usize,
    
    /// Types of PII found
    pub pii_types: Vec<String>,
    
    /// Whether injection was detected
    pub injection_detected: bool,
    
    /// Injection detection confidence (0.0-1.0)
    pub injection_confidence: f32,
    
    /// Whether content was blocked
    pub blocked: bool,
    
    /// Block reason if blocked
    pub block_reason: Option<String>,
}

impl ExtractResult {
    /// Create a new extraction result
    pub fn new(text: String, source: String) -> Self {
        let text_length = text.len();
        Self {
            text,
            source,
            content_type: None,
            original_length: 0,
            text_length,
            title: None,
            metadata: HashMap::new(),
            sanitized: false,
            #[cfg(feature = "sanitize")]
            sanitization: None,
        }
    }
    
    /// Set the content type
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }
    
    /// Set the original length
    pub fn with_original_length(mut self, length: usize) -> Self {
        self.original_length = length;
        self
    }
    
    /// Set the title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    
    /// Mark as sanitized
    #[cfg(feature = "sanitize")]
    pub fn with_sanitization(mut self, info: SanitizationInfo) -> Self {
        self.sanitized = true;
        self.sanitization = Some(info);
        self
    }
    
    /// Check if content is safe to use with LLM
    pub fn is_safe(&self) -> bool {
        #[cfg(feature = "sanitize")]
        {
            if let Some(ref info) = self.sanitization {
                return !info.blocked && !info.injection_detected;
            }
        }
        true
    }
    
    /// Get a truncated version of the text
    pub fn truncate(&self, max_chars: usize) -> String {
        if self.text.len() <= max_chars {
            self.text.clone()
        } else {
            let mut result = self.text.chars().take(max_chars).collect::<String>();
            result.push_str("...");
            result
        }
    }
}

#[cfg(feature = "sanitize")]
impl Default for SanitizationInfo {
    fn default() -> Self {
        Self {
            pii_redacted: 0,
            pii_types: Vec::new(),
            injection_detected: false,
            injection_confidence: 0.0,
            blocked: false,
            block_reason: None,
        }
    }
}
