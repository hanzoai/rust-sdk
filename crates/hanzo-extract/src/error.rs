//! Error types for content extraction

use thiserror::Error;

/// Result type for extraction operations
pub type Result<T> = std::result::Result<T, ExtractError>;

/// Errors that can occur during content extraction
#[derive(Error, Debug)]
pub enum ExtractError {
    /// Network error during fetch
    #[error("Network error: {0}")]
    Network(String),

    /// HTTP error response
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Parse error
    #[error("Parse error: {0}")]
    Parse(String),

    /// PDF extraction error
    #[error("PDF error: {0}")]
    Pdf(String),

    /// Content too large
    #[error("Content too large: {size} bytes exceeds max {max} bytes")]
    ContentTooLarge { size: usize, max: usize },

    /// Timeout error
    #[error("Request timeout after {0} seconds")]
    Timeout(u64),

    /// Sanitization blocked content
    #[cfg(feature = "sanitize")]
    #[error("Content blocked by sanitization: {0}")]
    Blocked(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Other error
    #[error("{0}")]
    Other(String),
}

#[cfg(feature = "web")]
impl From<reqwest::Error> for ExtractError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            ExtractError::Timeout(30)
        } else if let Some(status) = err.status() {
            ExtractError::Http {
                status: status.as_u16(),
                message: err.to_string(),
            }
        } else {
            ExtractError::Network(err.to_string())
        }
    }
}

impl From<url::ParseError> for ExtractError {
    fn from(err: url::ParseError) -> Self {
        ExtractError::InvalidUrl(err.to_string())
    }
}

#[cfg(feature = "pdf")]
impl From<lopdf::Error> for ExtractError {
    fn from(err: lopdf::Error) -> Self {
        ExtractError::Pdf(err.to_string())
    }
}
