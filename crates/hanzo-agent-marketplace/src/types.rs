//! Type definitions for the marketplace

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of services agents can offer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    /// Data collection and processing
    Data,
    /// Compute resources
    Compute,
    /// AI/ML inference
    Inference,
    /// Data analysis
    Analysis,
    /// Code generation and development
    Coding,
    /// Research and information gathering
    Research,
    /// Custom service type
    Custom,
}

impl std::fmt::Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::Data => write!(f, "data"),
            ServiceType::Compute => write!(f, "compute"),
            ServiceType::Inference => write!(f, "inference"),
            ServiceType::Analysis => write!(f, "analysis"),
            ServiceType::Coding => write!(f, "coding"),
            ServiceType::Research => write!(f, "research"),
            ServiceType::Custom => write!(f, "custom"),
        }
    }
}

/// Service offered by an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOffer {
    /// Unique offer ID
    pub id: String,
    /// Agent's blockchain address
    pub agent_address: String,
    /// Agent's name
    pub agent_name: String,
    /// Type of service offered
    pub service_type: ServiceType,
    /// Service description
    pub description: String,
    /// Price in ETH
    pub price_eth: f64,
    /// Minimum reputation required from requesters
    #[serde(default)]
    pub min_reputation: f64,
    /// Maximum duration in hours
    #[serde(default = "default_duration")]
    pub max_duration_hours: f64,
    /// Whether TEE is required
    #[serde(default)]
    pub requires_tee: bool,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Creation timestamp
    pub created_at: f64,
    /// Optional expiration timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
}

fn default_duration() -> f64 {
    24.0
}

impl ServiceOffer {
    /// Check if offer has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            now > expires_at
        } else {
            false
        }
    }

    /// Check if offer matches a request
    pub fn matches_request(&self, request: &ServiceRequest) -> bool {
        // Check expiration
        if self.is_expired() {
            return false;
        }

        // Check service type (CUSTOM matches any)
        if request.service_type != ServiceType::Custom {
            if self.service_type != request.service_type {
                return false;
            }
        }

        // Check price
        if self.price_eth > request.max_price_eth {
            return false;
        }

        // Check duration
        if request.duration_hours > self.max_duration_hours {
            return false;
        }

        // Check TEE requirement
        if request.requires_tee && !self.requires_tee {
            return false;
        }

        true
    }
}

/// Request for a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    /// Unique request ID
    pub id: String,
    /// Requester's blockchain address
    pub requester_address: String,
    /// Requester's name
    pub requester_name: String,
    /// Type of service needed
    pub service_type: ServiceType,
    /// Service description
    pub description: String,
    /// Maximum price willing to pay in ETH
    pub max_price_eth: f64,
    /// Expected duration in hours
    #[serde(default = "default_request_duration")]
    pub duration_hours: f64,
    /// Minimum reputation required from providers
    #[serde(default)]
    pub min_reputation: f64,
    /// Whether TEE is required
    #[serde(default)]
    pub requires_tee: bool,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Creation timestamp
    pub created_at: f64,
    /// Optional expiration timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
}

fn default_request_duration() -> f64 {
    1.0
}

impl ServiceRequest {
    /// Check if request has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            now > expires_at
        } else {
            false
        }
    }
}

/// Matched offer and request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMatch {
    /// Unique match ID
    pub match_id: String,
    /// Matched offer
    pub offer: ServiceOffer,
    /// Matched request
    pub request: ServiceRequest,
    /// Agreed price in ETH
    pub agreed_price_eth: f64,
    /// Optional escrow transaction hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub escrow_tx: Option<String>,
    /// Match status: pending, active, completed, disputed
    pub status: String,
    /// Creation timestamp
    pub created_at: f64,
    /// Optional completion timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// Optional result data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<HashMap<String, serde_json::Value>>,
    /// Optional TEE attestation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation: Option<HashMap<String, serde_json::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_type_serialization() {
        let st = ServiceType::Coding;
        let json = serde_json::to_string(&st).unwrap();
        assert_eq!(json, r#""coding""#);

        let deserialized: ServiceType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ServiceType::Coding);
    }

    #[test]
    fn test_service_offer_expiry() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut offer = ServiceOffer {
            id: "test".to_string(),
            agent_address: "addr".to_string(),
            agent_name: "bot".to_string(),
            service_type: ServiceType::Data,
            description: "test".to_string(),
            price_eth: 1.0,
            min_reputation: 0.0,
            max_duration_hours: 24.0,
            requires_tee: false,
            metadata: HashMap::new(),
            created_at: now,
            expires_at: Some(now - 100.0), // Expired
        };

        assert!(offer.is_expired());

        offer.expires_at = Some(now + 100.0); // Not expired
        assert!(!offer.is_expired());

        offer.expires_at = None; // Never expires
        assert!(!offer.is_expired());
    }

    #[test]
    fn test_offer_request_matching() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let offer = ServiceOffer {
            id: "offer1".to_string(),
            agent_address: "addr1".to_string(),
            agent_name: "bot1".to_string(),
            service_type: ServiceType::Coding,
            description: "Python coding".to_string(),
            price_eth: 0.5,
            min_reputation: 0.0,
            max_duration_hours: 24.0,
            requires_tee: false,
            metadata: HashMap::new(),
            created_at: now,
            expires_at: None,
        };

        let request = ServiceRequest {
            id: "req1".to_string(),
            requester_address: "addr2".to_string(),
            requester_name: "user1".to_string(),
            service_type: ServiceType::Coding,
            description: "Need Python code".to_string(),
            max_price_eth: 1.0,
            duration_hours: 2.0,
            min_reputation: 0.0,
            requires_tee: false,
            metadata: HashMap::new(),
            created_at: now,
            expires_at: None,
        };

        assert!(offer.matches_request(&request));
    }

    #[test]
    fn test_price_mismatch() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let offer = ServiceOffer {
            id: "offer1".to_string(),
            agent_address: "addr1".to_string(),
            agent_name: "bot1".to_string(),
            service_type: ServiceType::Coding,
            description: "Expensive coding".to_string(),
            price_eth: 2.0, // Too expensive
            min_reputation: 0.0,
            max_duration_hours: 24.0,
            requires_tee: false,
            metadata: HashMap::new(),
            created_at: now,
            expires_at: None,
        };

        let request = ServiceRequest {
            id: "req1".to_string(),
            requester_address: "addr2".to_string(),
            requester_name: "user1".to_string(),
            service_type: ServiceType::Coding,
            description: "Need Python code".to_string(),
            max_price_eth: 1.0,
            duration_hours: 2.0,
            min_reputation: 0.0,
            requires_tee: false,
            metadata: HashMap::new(),
            created_at: now,
            expires_at: None,
        };

        assert!(!offer.matches_request(&request));
    }

    #[test]
    fn test_tee_requirement() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let offer = ServiceOffer {
            id: "offer1".to_string(),
            agent_address: "addr1".to_string(),
            agent_name: "bot1".to_string(),
            service_type: ServiceType::Compute,
            description: "Regular compute".to_string(),
            price_eth: 0.5,
            min_reputation: 0.0,
            max_duration_hours: 24.0,
            requires_tee: false, // No TEE
            metadata: HashMap::new(),
            created_at: now,
            expires_at: None,
        };

        let request = ServiceRequest {
            id: "req1".to_string(),
            requester_address: "addr2".to_string(),
            requester_name: "user1".to_string(),
            service_type: ServiceType::Compute,
            description: "Need secure compute".to_string(),
            max_price_eth: 1.0,
            duration_hours: 2.0,
            min_reputation: 0.0,
            requires_tee: true, // Requires TEE
            metadata: HashMap::new(),
            created_at: now,
            expires_at: None,
        };

        assert!(!offer.matches_request(&request));
    }
}
