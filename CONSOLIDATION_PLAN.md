# Hanzo Rust Ecosystem Consolidation Plan

## Current State

### rust-sdk (hanzoai/rust-sdk) - Currently has:
- ✅ hanzo-config
- ✅ hanzo-crypto
- ✅ hanzo-message-primitives
- ✅ hanzo-pqc

### Crates to Move to rust-sdk (Shared/Reusable):

**From ~/work/hanzo/node/crates & ~/work/shinkai/hanzo-node/hanzo-libs:**

#### High Priority (Core Infrastructure):
1. **hanzo-did** - DID (Decentralized Identifier) support
   - Used by: gateway, desktop app, node
   - Status: Already in rust-sdk as hanzo-crypto-identities partially
   
2. **hanzo-mcp** - Model Context Protocol
   - Used by: LLM services, agents, tools
   - Status: Core protocol, must be shared

3. **hanzo-model-discovery** - AI model discovery and registration
   - Used by: gateway, node, inference services
   - Status: Shared infrastructure

#### Medium Priority (Storage & Data):
4. **hanzo-db** - Database abstractions and utilities
   - Used by: node, services that need storage
   - Status: Shared utility layer

5. **hanzo-kbs** - Knowledge Base System
   - Used by: RAG systems, agents
   - Status: Shared AI infrastructure

6. **hanzo-embedding** - Vector embedding utilities
   - Used by: RAG, search, semantic systems
   - Status: Shared ML utility

7. **hanzo-fs** - Filesystem utilities
   - Used by: Any service needing file operations
   - Status: Shared utility

#### Lower Priority (Utilities):
8. **hanzo-http-api** - HTTP/REST API utilities
   - Used by: All web services
   - Status: Shared API layer

9. **hanzo-sqlite** - SQLite wrappers
   - Dependency of hanzo-db
   - Status: Shared storage

10. **hanzo-baml** - BAML (Behavioural AI Markup Language) support
    - Used by: Agent systems
    - Status: If shared across services

11. **hanzo-hmm** - Hidden Markov Model utilities
    - Used by: ML/AI systems
    - Status: If shared

### Crates that Should Stay in hanzo-node (Node-Specific):

1. **hanzo-libp2p-relayer** - P2P networking for distributed nodes
2. **hanzo-job-queue-manager** - Node-specific job orchestration  
3. **hanzo-mining** - Blockchain mining (if applicable)
4. **hanzo-runtime** - Node runtime environment
5. **hanzo-llm** - LLM provider implementations
6. **hanzo-non-rust-code** - FFI bindings to external code
7. **hanzo-runtime-tests** - Node runtime testing
8. **hanzo-test-framework** - Node testing utilities
9. **hanzo-test-macro** - Test macros
10. **hanzo-simulation** - Node simulation
11. **hanzo-sheet** - Spreadsheet functionality (?)
12. **hanzo-wasm-runtime** - WASM runtime (if node-specific)
13. **hanzo-hllm** - Local LLM support (if node-specific)
14. **hanzo-consensus** - Blockchain consensus (if applicable)

## Migration Strategy

### Phase 1: Setup Infrastructure (Week 1)
- [ ] Create crate template structure in rust-sdk
- [ ] Set up CI/CD for rust-sdk crate publishing
- [ ] Document crate guidelines and standards
- [ ] Set up crates.io publishing workflow

### Phase 2: Core Protocol Crates (Week 1-2)
Priority order:
1. **hanzo-did** - Identity foundation
2. **hanzo-mcp** - Protocol foundation  
3. **hanzo-model-discovery** - Service discovery

Steps per crate:
```bash
# 1. Copy latest version to rust-sdk
cp -r ~/work/hanzo/node/crates/hanzo-did ~/work/hanzo/rust-sdk/crates/

# 2. Update Cargo.toml dependencies to use workspace versions
# 3. Remove node-specific dependencies
# 4. Add comprehensive tests
# 5. Publish to crates.io as 0.1.0
```

### Phase 3: Storage & Data Crates (Week 2-3)
- hanzo-db
- hanzo-kbs
- hanzo-embedding
- hanzo-fs
- hanzo-sqlite

### Phase 4: Utility Crates (Week 3-4)
- hanzo-http-api
- hanzo-baml
- hanzo-hmm

### Phase 5: Update Dependents (Week 4)
1. Update `~/work/shinkai/hanzo-node` to use rust-sdk crates:
```toml
[dependencies]
hanzo-did = { version = "0.1", git = "https://github.com/hanzoai/rust-sdk" }
hanzo-mcp = { version = "0.1", git = "https://github.com/hanzoai/rust-sdk" }
# Or from crates.io once published:
# hanzo-did = "0.1"
```

2. Update `~/work/hanzo/node` to use rust-sdk crates

3. Update any other Hanzo Rust projects

## Dependency Strategy

### Development Phase (Now - 1 month):
Use git dependencies:
```toml
hanzo-did = { git = "https://github.com/hanzoai/rust-sdk", branch = "main" }
```

### Production Phase (After stabilization):
Use crates.io:
```toml
hanzo-did = "0.1"
```

## Versioning Strategy

- **rust-sdk workspace**: 0.1.x (pre-1.0 = API may change)
- **Individual crates**: Start at 0.1.0
- Use semantic versioning (SemVer)
- Synchronized releases across related crates

## Publishing Workflow

1. Update CHANGELOG.md
2. Bump version in Cargo.toml
3. Run tests: `cargo test --all-features`
4. Publish: `cargo publish -p hanzo-did`
5. Tag release: `git tag hanzo-did-v0.1.0`
6. Create GitHub release

## Benefits

1. **Single Source of Truth**: One canonical implementation
2. **Faster Development**: No code duplication
3. **Easier Maintenance**: Fix bugs once, benefit everywhere
4. **Better Testing**: Shared test infrastructure
5. **Clear Ownership**: rust-sdk team owns core crates
6. **Faster CI**: hanzo-node doesn't rebuild shared crates
7. **Version Control**: Explicit dependency versions
8. **Public API**: Published crates force good API design

## Implementation Checklist

### Immediate (This Week):
- [ ] Set up rust-sdk CI/CD for crate publishing
- [ ] Create CONTRIBUTING.md for crate development
- [ ] Move hanzo-did to rust-sdk
- [ ] Move hanzo-mcp to rust-sdk
- [ ] Move hanzo-model-discovery to rust-sdk

### Short Term (Next 2 Weeks):
- [ ] Move remaining shared crates (see Phase 3-4)
- [ ] Update hanzo-node to use rust-sdk crates
- [ ] Publish initial versions to crates.io
- [ ] Document migration guide

### Long Term (Next Month):
- [ ] Stabilize APIs for 1.0 release
- [ ] Set up automated dependency updates
- [ ] Create rust-sdk documentation site
- [ ] Establish governance model

## Questions to Answer

1. **hanzo-crypto vs hanzo-crypto-identities**: Merge or keep separate?
2. **hanzo-message-primitives**: Already in rust-sdk, ensure latest
3. **Crate naming**: Keep `hanzo-*` prefix or use `hanzo::*` namespace?
4. **Feature flags**: How to handle node-specific features?
5. **WASM support**: Which crates need wasm32 targets?

## Success Metrics

- [ ] Zero code duplication across repos
- [ ] hanzo-node build time reduced by 50%
- [ ] All shared crates published to crates.io
- [ ] Documentation at docs.rs for all crates
- [ ] CI green on all repos
- [ ] Gateway can use latest crates without recompiling node
