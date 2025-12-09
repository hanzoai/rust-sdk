pub mod did;
pub mod document;
pub mod error;
pub mod proof;
pub mod resolver;
pub mod service;
pub mod verification_method;

pub use did::{Network, DID};
pub use document::DIDDocument;
pub use error::DIDError;
pub use proof::Proof;
pub use resolver::DIDResolver;
pub use service::{Service, ServiceEndpoint};
pub use verification_method::{VerificationMethod, VerificationMethodType};

// Re-export commonly used types
pub use serde_json::Value as JsonValue;
