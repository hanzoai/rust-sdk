# Publishing Guide

## Overview

This monorepo contains publishable Rust crates for Hanzo AI infrastructure:

- `hanzo-message-primitives` - Core message types and schemas
- `hanzo-pqc` - Post-Quantum Cryptography implementation with privacy tiers
- `hanzo-kbs` - Key Broker Service and Key Management Service for confidential computing

## Release Strategy

This repository follows a **per-package release strategy** similar to our Python SDK:

### Tag Patterns

- `hanzo-message-primitives-v0.1.0` - Release only hanzo-message-primitives
- `hanzo-pqc-v0.1.0` - Release only hanzo-pqc
- `hanzo-kbs-v0.1.0` - Release only hanzo-kbs
- `v0.1.0` - Release all packages with the same version

### Release Commands

```bash
# Check everything is ready
./scripts/check-release.sh

# Release a single package
./scripts/release-package.sh hanzo-kbs 0.1.1

# Release all packages
./scripts/release-package.sh all 0.2.0

# Dry run (see what would happen)
./scripts/release-package.sh hanzo-pqc 0.1.0 --dry-run
```

## Automated Publishing

When you push a tag, GitHub Actions will automatically:

1. Run the full test suite across multiple platforms
2. Build release artifacts
3. Publish to crates.io in the correct order:
   - hanzo-message-primitives (first)
   - hanzo-pqc (depends on message-primitives)
   - hanzo-kbs (depends on both)
4. Create a GitHub release with changelog

## Manual Publishing

If you need to publish manually:

```bash
# Set your crates.io token
export CARGO_REGISTRY_TOKEN=your-token-here

# Publish in dependency order
cd crates/hanzo-message-primitives && cargo publish
cd ../hanzo-pqc && cargo publish
cd ../hanzo-kbs && cargo publish
```

## Version Management

### Current Versions
- hanzo-message-primitives: 0.1.0
- hanzo-pqc: 0.1.0
- hanzo-kbs: 0.1.0

### Version Guidelines
- Packages can have different versions
- Use semantic versioning
- Update inter-crate dependencies when needed
- Consider compatibility when releasing

## Pre-Release Checklist

- [ ] Run `./scripts/check-release.sh`
- [ ] Update CHANGELOG.md for the package(s)
- [ ] Verify inter-crate dependency versions
- [ ] Run tests locally
- [ ] Check documentation builds
- [ ] Review security audit results

## Monitoring Releases

- GitHub Actions: https://github.com/hanzoai/rust-sdk/actions
- Crates.io: https://crates.io/users/hanzoai

## Troubleshooting

### Failed Publication
If a package fails to publish:
1. Check the GitHub Actions logs
2. Verify your CARGO_REGISTRY_TOKEN secret
3. Ensure no duplicate version on crates.io
4. Try manual publication after fixing issues

### Dependency Issues
- Always publish in dependency order
- Wait ~30 seconds between dependent packages
- Use exact versions for inter-crate dependencies

## Notes

- The `hanzo-message-primitives` crate is currently a minimal stub
- All crates are dual-licensed under MIT OR Apache-2.0
- Security vulnerabilities should be reported to security@hanzo.ai