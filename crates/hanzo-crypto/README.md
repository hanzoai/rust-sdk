# Hanzo PQC

Post-Quantum Cryptography implementation for Hanzo AI with privacy tier support.

## Features

- **NIST PQC Standards**: ML-KEM (Kyber) and ML-DSA (Dilithium) implementations
- **Hybrid Cryptography**: Combines classical and post-quantum algorithms
- **Privacy Tiers**: Optimized configurations for different security levels
- **Hardware Integration**: Support for GPU CC and TEE-I/O environments
- **FIPS Compliance**: Optional FIPS-compliant mode

## Algorithms

### Key Encapsulation (KEM)
- ML-KEM-512 (NIST Level 1)
- ML-KEM-768 (NIST Level 3)
- ML-KEM-1024 (NIST Level 5)

### Digital Signatures
- ML-DSA-44 (NIST Level 2)
- ML-DSA-65 (NIST Level 3)
- ML-DSA-87 (NIST Level 5)

### Hybrid Modes
- X25519 + ML-KEM for key exchange
- Ed25519 + ML-DSA for signatures

## Usage

```rust
use hanzo_pqc::{
    kem::{Kem, KemAlgorithm, MlKem},
    signature::{Signature, SignatureAlgorithm, MlDsa},
    privacy_tiers::PrivacyTier,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // KEM example
    let ml_kem = MlKem::new();
    let keypair = ml_kem.generate_keypair(KemAlgorithm::MlKem768).await?;
    
    // Encapsulation
    let output = ml_kem.encapsulate(&keypair.encap_key).await?;
    
    // Decapsulation
    let shared_secret = ml_kem.decapsulate(&keypair.decap_key, &output.ciphertext).await?;
    
    // Signature example
    let ml_dsa = MlDsa::new();
    let (verifying_key, signing_key) = ml_dsa.generate_keypair(SignatureAlgorithm::MlDsa65).await?;
    
    let message = b"Hello, quantum world!";
    let signature = ml_dsa.sign(&signing_key, message).await?;
    let valid = ml_dsa.verify(&verifying_key, message, &signature).await?;
    
    assert!(valid);
    Ok(())
}
```

## Privacy Tier Configuration

The crate automatically selects appropriate algorithm strengths based on privacy tiers:

- **Tier 0-1**: ML-KEM-512, ML-DSA-44
- **Tier 2**: ML-KEM-768, ML-DSA-65  
- **Tier 3-4**: ML-KEM-1024, ML-DSA-87

## Features

- `default`: Enables ML-KEM, ML-DSA, and hybrid mode
- `ml-kem`: ML-KEM (Kyber) support
- `ml-dsa`: ML-DSA (Dilithium) support
- `slh-dsa`: SLH-DSA (SPHINCS+) support
- `hybrid`: Hybrid classical/PQC modes
- `fips-mode`: FIPS compliance mode
- `gpu-cc`: GPU confidential computing optimizations
- `tee-io`: TEE-I/O support for Blackwell

## License

Dual licensed under MIT OR Apache-2.0