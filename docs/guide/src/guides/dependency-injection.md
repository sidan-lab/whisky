# Dependency Injection

Whisky uses a trait-based dependency injection pattern that lets you swap out core components: the serializer backend, blockchain data fetcher, script evaluator, and transaction submitter.

## TxBuilderParam

When creating a `TxBuilder`, you can inject dependencies via `TxBuilderParam`:

```rust,ignore
use whisky::*;
use whisky_pallas::WhiskyPallas;

let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),  // Required
    evaluator: None,    // Optional — defaults to OfflineTxEvaluator
    fetcher: None,      // Optional — for blockchain data
    submitter: None,    // Optional — for tx submission
    params: None,       // Optional — protocol parameters
});
```

Or use the convenience constructor with all defaults:

```rust,ignore
let mut tx_builder = TxBuilder::new_core();
// Equivalent to: WhiskyPallas serializer, offline evaluator, no fetcher/submitter
```

## The TxBuilder Struct

```rust,ignore
pub struct TxBuilder {
    pub serializer: Box<dyn TxBuildable>,       // Serializes tx body to CBOR
    pub fetcher: Option<Box<dyn Fetcher>>,      // Fetches blockchain data
    pub evaluator: Option<Box<dyn Evaluator>>,  // Evaluates Plutus scripts
    pub submitter: Option<Box<dyn Submitter>>,  // Submits transactions
    pub protocol_params: Option<Protocol>,      // Network parameters
    // ... internal state fields
}
```

## Why Dependency Injection?

This design enables:

- **Swappable serializers**: Use Pallas (recommended) or CSL backend without changing transaction logic
- **Testing**: Mock fetchers and evaluators in unit tests
- **Custom providers**: Implement your own blockchain data source
- **Offline mode**: Build transactions without any network dependency (the default)
- **Full pipeline**: Wire up fetcher + evaluator + submitter for end-to-end transaction handling

## Trait Overview

| Trait | Purpose | Built-in Implementations |
|-------|---------|------------------------|
| `TxBuildable` | Serialize transaction body to CBOR | `WhiskyPallas`, `WhiskyCSL` |
| `Fetcher` | Fetch UTxOs, protocol params, block info | `MaestroProvider`, `BlockfrostProvider` |
| `Evaluator` | Evaluate Plutus script execution units | `OfflineTxEvaluator` |
| `Submitter` | Submit signed transactions | `MaestroProvider`, `BlockfrostProvider` |

## Chapters

- [Serializer Backends](./di/serializers.md) — `TxBuildable` trait, WhiskyPallas vs WhiskyCSL
- [Providers](./di/providers.md) — Fetcher, Evaluator, Submitter traits and implementations
