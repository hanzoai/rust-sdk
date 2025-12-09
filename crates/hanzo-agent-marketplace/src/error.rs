//! Error types for the marketplace

use thiserror::Error;

/// Result type for marketplace operations
pub type Result<T> = std::result::Result<T, MarketplaceError>;

/// Errors that can occur in marketplace operations
#[derive(Debug, Error)]
pub enum MarketplaceError {
    /// Match not found
    #[error("Match not found: {0}")]
    MatchNotFound(String),

    /// Offer not found
    #[error("Offer not found: {0}")]
    OfferNotFound(String),

    /// Request not found
    #[error("Request not found: {0}")]
    RequestNotFound(String),

    /// Invalid price
    #[error("Invalid price: {0}")]
    InvalidPrice(String),

    /// Invalid duration
    #[error("Invalid duration: {0}")]
    InvalidDuration(String),

    /// Insufficient reputation
    #[error("Insufficient reputation: required {required}, got {actual}")]
    InsufficientReputation { required: f64, actual: f64 },

    /// Match already completed
    #[error("Match already completed: {0}")]
    MatchAlreadyCompleted(String),

    /// Match already disputed
    #[error("Match already disputed: {0}")]
    MatchAlreadyDisputed(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Generic error
    #[error("{0}")]
    Other(String),
}
