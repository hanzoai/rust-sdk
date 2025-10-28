# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of Hanzo Rust SDK seriously. If you believe you have found a security vulnerability, please report it to us as described below.

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to [security@hanzo.ai](mailto:security@hanzo.ai).

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the requested information listed below (as much as you can provide) to help us better understand the nature and scope of the possible issue:

- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, cryptographic weakness, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

## Preferred Languages

We prefer all communications to be in English.

## Disclosure Policy

When we receive a security bug report, we will:

1. Confirm the receipt of your vulnerability report
2. Work with you to understand and validate the report
3. Keep you informed about the progress towards a fix and full announcement
4. Credit you in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices for Contributors

When contributing to this repository, please:

1. **Never commit secrets**: API keys, passwords, or certificates
2. **Use secure randomness**: Always use cryptographically secure RNGs
3. **Validate inputs**: Sanitize and validate all external inputs
4. **Handle errors safely**: Don't expose sensitive information in error messages
5. **Use safe dependencies**: Keep dependencies updated and audited
6. **Follow cryptographic best practices**: Don't roll your own crypto

## Security Features

Hanzo Rust SDK includes several security features:

- **Post-Quantum Cryptography**: ML-KEM and ML-DSA for quantum resistance
- **Hardware Attestation**: Support for SEV-SNP, TDX, and GPU TEE
- **Key Hierarchy**: Secure key derivation and management
- **Privacy Tiers**: Granular security levels from open to TEE-I/O
- **Audit Logging**: Comprehensive audit trail for all operations

## Security Audits

We conduct regular security audits:

- Automated scanning with `cargo audit`
- Dependency vulnerability checking
- Static analysis with `cargo clippy`
- Fuzzing of critical components

## Cryptographic Disclosure

This software includes cryptographic software. The country in which you currently reside may have restrictions on the import, possession, use, and/or re-export to another country, of encryption software. BEFORE using any encryption software, please check your country's laws, regulations and policies concerning the import, possession, or use, and re-export of encryption software.

## Contact

For any security-related questions, please contact:
- Email: [security@hanzo.ai](mailto:security@hanzo.ai)
- GPG Key: Available at [https://hanzo.ai/security.asc](https://hanzo.ai/security.asc)