# Migration: CSL to Pallas

This guide covers migrating from the legacy `cardano-serialization-lib` (CSL) backend to the `pallas`-based backend.

## Why Migrate?

Both serialization libraries have similar functionality, but **Pallas** offers:

- **Better maintenance** — actively updated for Cardano hard forks
- **Pure Rust** — no C/WASM dependencies
- **Ecosystem alignment** — used widely in the Cardano Rust ecosystem

The `whisky` main crate now defaults to Pallas. CSL has been removed from the default feature set.

## Transaction Building

### Before (CSL)

```rust,ignore
use whisky::*;
use whisky_csl::WhiskyCSL;

let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyCSL::new(None)),
    evaluator: None,
    fetcher: None,
    submitter: None,
    params: None,
});
```

### After (Pallas)

```rust,ignore
use whisky::*;
use whisky_pallas::WhiskyPallas;

// Option 1: Explicit
let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),
    evaluator: None,
    fetcher: None,
    submitter: None,
    params: None,
});

// Option 2: Use the convenience constructor (defaults to Pallas)
let mut tx_builder = TxBuilder::new_core();
```

The transaction building API remains **identical** — only the serializer instantiation changes:

```rust,ignore
let signed_tx = tx_builder
    .tx_in(tx_hash, tx_index, amount, address)
    .change_address(my_address)
    .signing_key(skey_hex)
    .complete_sync(None)?
    .complete_signing()?;
```

## Transaction Parsing

### Before (CSL)

```rust,ignore
use whisky_csl::CSLParser;

let mut parser = CSLParser::new();
parser.parse(tx_hex, &utxos)?;
let body = parser.get_builder_body();
```

### After (Pallas)

```rust,ignore
use whisky_pallas::tx_parser::parse;

let body = parse(tx_hex, &utxos)?;
```

Or using the trait-based approach:

```rust,ignore
use whisky_pallas::tx_parser::PallasParser;
use whisky_common::TxParsable;

let mut parser = PallasParser::new();
parser.parse(tx_hex, &utxos)?;
let body = parser.get_builder_body();
```

## Transaction Evaluation

```rust,ignore
use uplc::tx::script_context::SlotConfig;
use whisky_common::{Network, UTxO};
use whisky_pallas::utils::evaluate_tx_scripts;

let result = evaluate_tx_scripts(
    tx_hex,
    &utxos,
    &[],                      // additional chained transactions
    &Network::Mainnet,
    &SlotConfig::default(),
);
```

## Dependency Changes

If you were depending on `whisky-csl` directly, update your `Cargo.toml`:

### Before

```toml
[dependencies]
whisky = "1.0.17"
# CSL was included by default
```

### After

```toml
[dependencies]
whisky = "1.0.28-beta.1"
# Pallas is now the default — no extra dependency needed

# Only if you still need CSL:
# whisky-csl = "1.0.28-beta.1"
```

## Summary of Changes

| Component | CSL | Pallas |
|-----------|-----|--------|
| Serializer | `WhiskyCSL::new(None)` | `WhiskyPallas::new(None)` |
| Default constructor | N/A | `TxBuilder::new_core()` |
| Parser | `CSLParser::new()` | `parse(tx_hex, &utxos)` |
| Crate | `whisky-csl` | `whisky-pallas` (included via `whisky`) |
| Feature flag | `features = ["csl"]` | Included by default |

The transaction building API (`tx_in`, `tx_out`, `change_address`, `complete`, etc.) is **unchanged** — migration only requires swapping the serializer backend.
