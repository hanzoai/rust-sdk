# Hanzo Rust SDK

Official Rust SDK for Hanzo AI infrastructure, providing secure key management, post-quantum cryptography, and core primitives for building AI applications.

## Crates

This workspace contains the following crates:

### [`hanzo-kbs`](crates/hanzo-kbs)
Key Broker Service (KBS) and Key Management Service (KMS) for confidential computing with privacy tiers from open access to GPU TEE-I/O.

### [`hanzo-pqc`](crates/hanzo-pqc)
Post-Quantum Cryptography implementation supporting ML-KEM, ML-DSA, and hybrid modes with privacy tier optimization.

### [`hanzo-message-primitives`](crates/hanzo-message-primitives) 
Core message types and schemas for Hanzo AI systems (minimal stub, to be expanded).

## Getting Started

Add the desired crates to your `Cargo.toml`:

```toml
[dependencies]
hanzo-kbs = "0.1"
hanzo-pqc = "0.1"
```

## Examples

### Key Management with Privacy Tiers

```rust
use hanzo_kbs::{
    kbs::Kbs,
    kms::{Kms, MemoryKms},
    types::PrivacyTier,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kms = MemoryKms::new().await?;
    let kbs = Kbs::new(kms);
    
    // Initialize for GPU Confidential Computing
    kbs.initialize_vault(PrivacyTier::GpuCc, None).await?;
    
    Ok(())
}
```

### Post-Quantum Cryptography

```rust
use hanzo_pqc::{
    kem::{Kem, KemAlgorithm, MlKem},
    signature::{Signature, SignatureAlgorithm, MlDsa},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate ML-KEM keypair
    let ml_kem = MlKem::new();
    let keypair = ml_kem.generate_keypair(KemAlgorithm::MlKem768).await?;
    
    // Generate ML-DSA keypair
    let ml_dsa = MlDsa::new();
    let (verifying_key, signing_key) = ml_dsa.generate_keypair(SignatureAlgorithm::MlDsa65).await?;
    
    Ok(())
}
```

## Development

```bash
# Build all crates
cargo build --all

# Run tests
cargo test --all

# Build for release
cargo build --release --all
```

## Releasing

This monorepo supports per-package releases:

```bash
# Release a single package
./scripts/release-package.sh hanzo-kbs 0.1.1

# Release all packages
./scripts/release-package.sh all 0.2.0
```

See [PUBLISHING.md](PUBLISHING.md) for detailed release instructions.

## License

All crates are dual licensed under MIT OR Apache-2.0.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [Hanzo AI](https://hanzo.ai)
- [Documentation](https://docs.rs/hanzo-kbs)
- [GitHub](https://github.com/hanzoai/rust-sdk)