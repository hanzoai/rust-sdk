# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Hanzo Rust SDK
- `hanzo-kbs` - Key Broker Service and Key Management Service implementation
  - Support for privacy tiers (0-4) from open access to GPU TEE-I/O
  - Hardware attestation for SEV-SNP, TDX, H100 CC, and Blackwell TEE-I/O
  - Key hierarchy with root keys, tenant KEKs, agent DEKs, and session keys
  - In-memory KMS implementation for development
  - Vault implementations for each privacy tier
  - X402 payment protocol integration
- `hanzo-pqc` - Post-Quantum Cryptography implementation
  - ML-KEM (Kyber) support with 512, 768, and 1024-bit security levels
  - ML-DSA (Dilithium) support with 44, 65, and 87 security levels
  - Hybrid classical/PQC modes combining X25519 with ML-KEM
  - Privacy tier-based algorithm selection
  - FIPS-compliant mode support
  - GPU optimization features
- `hanzo-message-primitives` - Core message types (minimal stub)
  - Basic structure for future expansion
  - Placeholder modules for schemas, messages, and utilities

### Security
- Post-quantum cryptographic primitives for quantum resistance
- Hardware-based attestation verification
- Secure key derivation and management
- Privacy tier enforcement

### Infrastructure
- Comprehensive CI/CD pipeline with GitHub Actions
- Multi-platform build support (Linux, macOS, Windows)
- Automated security audits and dependency checks
- Release automation with changelog generation
- Documentation deployment to GitHub Pages

## [0.1.0] - TBD

Initial release - see Unreleased section above.

[Unreleased]: https://github.com/hanzoai/rust-sdk/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/hanzoai/rust-sdk/releases/tag/v0.1.0