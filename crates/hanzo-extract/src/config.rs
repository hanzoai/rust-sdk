//! Extractor configuration

use serde::{Deserialize, Serialize};

/// Configuration for content extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractorConfig {
    /// Maximum content length to extract (in characters)
    pub max_length: usize,

    /// Request timeout in seconds
    pub timeout_secs: u64,

    /// Whether to extract clean text (remove HTML tags, scripts, etc.)
    pub clean_text: bool,

    /// Whether to preserve whitespace formatting
    pub preserve_whitespace: bool,

    /// User agent for web requests
    pub user_agent: String,

    /// Whether to follow redirects
    pub follow_redirects: bool,

    /// Maximum redirects to follow
    pub max_redirects: usize,

    /// Whether to sanitize output via hanzo-guard
    #[cfg(feature = "sanitize")]
    pub sanitize: bool,

    /// Whether to redact PII
    #[cfg(feature = "sanitize")]
    pub redact_pii: bool,

    /// Whether to detect injection attempts
    #[cfg(feature = "sanitize")]
    pub detect_injection: bool,
}

impl Default for ExtractorConfig {
    fn default() -> Self {
        Self {
            max_length: 100_000,
            timeout_secs: 30,
            clean_text: true,
            preserve_whitespace: false,
            user_agent: format!(
                "HanzoExtract/{} (https://hanzo.ai)",
                env!("CARGO_PKG_VERSION")
            ),
            follow_redirects: true,
            max_redirects: 5,
            #[cfg(feature = "sanitize")]
            sanitize: true,
            #[cfg(feature = "sanitize")]
            redact_pii: true,
            #[cfg(feature = "sanitize")]
            detect_injection: true,
        }
    }
}

impl ExtractorConfig {
    /// Create a new config with custom max length
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    /// Create a new config with custom timeout
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Enable or disable text cleaning
    pub fn with_clean_text(mut self, clean: bool) -> Self {
        self.clean_text = clean;
        self
    }

    #[cfg(feature = "sanitize")]
    /// Enable or disable sanitization
    pub fn with_sanitize(mut self, sanitize: bool) -> Self {
        self.sanitize = sanitize;
        self
    }

    #[cfg(feature = "sanitize")]
    /// Enable or disable PII redaction
    pub fn with_redact_pii(mut self, redact: bool) -> Self {
        self.redact_pii = redact;
        self
    }

    #[cfg(feature = "sanitize")]
    /// Enable or disable injection detection
    pub fn with_detect_injection(mut self, detect: bool) -> Self {
        self.detect_injection = detect;
        self
    }
}
