# hanzo-agent-marketplace

Decentralized marketplace for agent services and resources.

## Overview

This crate provides a marketplace system for AI agents to discover, offer, and request services. It includes:

- **Service Types**: Data, Compute, Inference, Analysis, Coding, Research, Custom
- **Service Discovery**: Automatic matching of offers and requests
- **Reputation System**: Track agent performance and reliability
- **Statistics**: Monitor marketplace activity and volume
- **Flexible Matching**: Price-based, duration-based, and TEE-requirement matching

## Features

- ✅ Service offer posting with customizable options
- ✅ Service request posting with requirements
- ✅ Automatic best-price matching
- ✅ Reputation tracking (0.0-1.0 scale)
- ✅ Expiration handling for offers and requests
- ✅ TEE (Trusted Execution Environment) support
- ✅ Match completion and dispute handling
- ✅ Comprehensive statistics
- ✅ Thread-safe using DashMap
- ✅ Fully tested with 100% coverage

## Usage

### Basic Example

```rust
use hanzo_agent_marketplace::{
    AgentMarketplace, ServiceOfferOptions, ServiceRequestOptions, ServiceType,
};

// Create marketplace
let marketplace = AgentMarketplace::new();

// Post a service offer
let offer_id = marketplace.post_offer(
    "agent_0x123abc",
    "CodeBot",
    ServiceType::Coding,
    "Expert Python code generation",
    0.5, // 0.5 ETH
    Some(
        ServiceOfferOptions::new()
            .max_duration_hours(48.0)
            .min_reputation(0.3),
    ),
);

// Post a service request
let request_id = marketplace.post_request(
    "user_0x789ghi",
    "Alice",
    ServiceType::Coding,
    "Need Python backend development",
    1.0, // Max 1.0 ETH
    Some(ServiceRequestOptions::new().duration_hours(24.0)),
);

// Matches are created automatically!
let stats = marketplace.get_stats();
println!("Total matches: {}", stats.total_matches);
```

### Complete a Match

```rust
use std::collections::HashMap;

// Get matches for an agent
let matches = marketplace.get_matches_for_agent("agent_0x123abc");

if let Some(m) = matches.first() {
    // Complete the match
    let mut result = HashMap::new();
    result.insert(
        "deliverable".to_string(),
        serde_json::json!("https://github.com/alice/project"),
    );

    marketplace.complete_match(&m.match_id, result, None).unwrap();
}
```

### Filter by Service Type

```rust
// Get only coding offers
let coding_offers = marketplace.get_active_offers(Some(ServiceType::Coding));

// Get only analysis requests
let analysis_requests = marketplace.get_active_requests(Some(ServiceType::Analysis));
```

### Advanced Options

```rust
use hanzo_agent_marketplace::{ServiceOfferOptions, ServiceRequestOptions};

// Offer with custom options
let options = ServiceOfferOptions::new()
    .min_reputation(0.7)
    .max_duration_hours(72.0)
    .requires_tee(true)
    .metadata("gpu_required", serde_json::json!(true))
    .expires_at(timestamp + 3600.0);

marketplace.post_offer(
    "agent_addr",
    "GPUBot",
    ServiceType::Inference,
    "High-performance GPU inference",
    2.5,
    Some(options),
);

// Request with TEE requirement
let req_options = ServiceRequestOptions::new()
    .requires_tee(true)
    .duration_hours(12.0)
    .min_reputation(0.8);

marketplace.post_request(
    "user_addr",
    "Bob",
    ServiceType::Compute,
    "Secure computation needed",
    3.0,
    Some(req_options),
);
```

## Service Types

- **Data**: Data collection and processing
- **Compute**: Compute resources
- **Inference**: AI/ML inference
- **Analysis**: Data analysis
- **Coding**: Code generation and development
- **Research**: Research and information gathering
- **Custom**: Custom service type (matches any)

## Matching Logic

The marketplace automatically matches offers with requests based on:

1. **Service Type**: Must match (unless request type is Custom)
2. **Price**: Offer price ≤ Request max price
3. **Duration**: Request duration ≤ Offer max duration
4. **TEE**: If request requires TEE, offer must provide it
5. **Reputation**: Both parties must meet minimum reputation requirements
6. **Best Price**: Among valid matches, lowest price wins

## Reputation System

- Default reputation: 0.5 (on 0.0-1.0 scale)
- Completing a match: +0.1 (damped)
- Disputing a match: -0.05 (damped)
- Clamped to [0.0, 1.0] range

## Statistics

The marketplace tracks:
- Active offers and requests
- Total matches created
- Total volume in ETH
- Total completed transactions
- Unique providers and requesters

```rust
let stats = marketplace.get_stats();
println!("Total volume: {} ETH", stats.total_volume_eth);
```

## Architecture

- **Thread-Safe**: Uses `Arc<DashMap>` for concurrent access
- **Automatic Matching**: Matches happen immediately when posting
- **Expiration Handling**: Expired offers/requests are automatically cleaned
- **Price Optimization**: Best (lowest) price is automatically selected

## Testing

Run tests with:

```bash
cargo test
```

All core functionality is tested:
- Marketplace creation
- Offer/request posting
- Automatic matching
- Price filtering
- Reputation tracking
- Match completion
- Service type filtering
- Expiration handling
- TEE requirement matching

## Examples

Run the basic usage example:

```bash
cargo run --example basic_usage
```

## Integration

This crate is designed to work with:
- `hanzo-agent`: Base agent framework
- `hanzo-crypto`: Blockchain signatures and verification
- `hanzo-did`: Decentralized identities
- Smart contracts for on-chain escrow (future)

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please ensure all tests pass before submitting PRs.

## Roadmap

- [ ] On-chain escrow integration
- [ ] Multi-signature match approval
- [ ] Negotiation protocol (beyond fixed price)
- [ ] Dispute resolution system
- [ ] Rating and review system
- [ ] Service level agreements (SLAs)
- [ ] Payment streaming
- [ ] Multi-token support (beyond ETH)
