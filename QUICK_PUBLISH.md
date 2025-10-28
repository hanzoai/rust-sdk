# 🚀 Quick Publishing Reference

## Prerequisites
- [ ] `CARGO_REGISTRY_TOKEN` in GitHub Secrets
- [ ] Clean working directory on `main` branch

## Commands

### 1️⃣ Verify Setup
```bash
./scripts/verify-setup.sh
```

### 2️⃣ Pre-Release Check
```bash
./scripts/check-release.sh
```

### 3️⃣ Release Options

#### Single Package
```bash
# Release hanzo-kbs version 0.1.1
./scripts/release-package.sh hanzo-kbs 0.1.1

# Release hanzo-pqc version 0.2.0-beta.1
./scripts/release-package.sh hanzo-pqc 0.2.0-beta.1

# Dry run (no changes)
./scripts/release-package.sh hanzo-kbs 0.1.1 --dry-run
```

#### All Packages
```bash
# Release all packages as version 0.2.0
./scripts/release-package.sh all 0.2.0
```

### 4️⃣ Monitor Release
- Actions: https://github.com/hanzoai/rust-sdk/actions
- Look for "Release & Publish" workflow

## Tag Patterns
- `hanzo-kbs-v0.1.0` → Publishes only hanzo-kbs
- `hanzo-pqc-v0.1.0` → Publishes only hanzo-pqc  
- `hanzo-message-primitives-v0.1.0` → Publishes only hanzo-message-primitives
- `v0.1.0` → Publishes all packages

## Troubleshooting

### Token Issues
```bash
# Check if token is set in GitHub
gh secret list --repo hanzoai/rust-sdk | grep CARGO_REGISTRY_TOKEN
```

### Manual Publish (Emergency)
```bash
export CARGO_REGISTRY_TOKEN=your-token-here
cd crates/hanzo-message-primitives && cargo publish
cd ../hanzo-pqc && cargo publish
cd ../hanzo-kbs && cargo publish
```

## Publishing Order
Always publishes in dependency order:
1. hanzo-message-primitives
2. hanzo-pqc (depends on message-primitives)
3. hanzo-kbs (depends on both)

## Quick Links
- 📦 [Crates.io](https://crates.io/users/hanzoai)
- 🔧 [GitHub Actions](https://github.com/hanzoai/rust-sdk/actions)
- 📚 [Docs.rs Queue](https://docs.rs/releases/queue)
- 🔑 [Add Token](https://github.com/hanzoai/rust-sdk/settings/secrets/actions)