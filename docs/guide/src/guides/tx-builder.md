# Transaction Builder

The `TxBuilder` is the core API for constructing Cardano transactions in whisky. It uses a chainable builder pattern where you compose a transaction step by step, then finalize it.

## Overview

```rust,ignore
use whisky::*;

let mut tx_builder = TxBuilder::new_core();
tx_builder
    .tx_in(tx_hash, tx_index, amount, address)  // Add inputs
    .tx_out(address, assets)                      // Add outputs
    .change_address(my_address)                   // Set change address
    .complete_sync(None)?;                        // Balance and serialize

let tx_hex = tx_builder.tx_hex();
```

## Key Concepts

### Builder Pattern

Every method returns `&mut Self`, allowing you to chain calls fluently. The builder accumulates state until you call `complete_sync()` or `complete()`.

### Inputs and UTxO Selection

You can specify inputs explicitly with `.tx_in()` or let whisky select them automatically:

```rust,ignore
// Explicit input
tx_builder.tx_in(tx_hash, tx_index, amount, address);

// Automatic selection from a pool of UTxOs
tx_builder.select_utxos_from(&available_utxos, 5000000);
```

The `select_utxos_from` threshold (e.g., 5,000,000 lovelace) tells the selector to pick enough UTxOs to cover all outputs plus this extra amount for fees and change output min UTxO.

### Outputs

```rust,ignore
// Send lovelace
tx_builder.tx_out(address, &[Asset::new_from_str("lovelace", "2000000")]);

// Send native tokens
tx_builder.tx_out(address, &[
    Asset::new_from_str("lovelace", "2000000"),
    Asset::new("policy_id_hex".to_string() + "token_name_hex", "1".to_string()),
]);
```

### Completion

| Method | Sync/Async | Script Evaluation | Use Case |
|--------|-----------|-------------------|----------|
| `complete_sync(None)` | Sync | No | Simple transactions |
| `complete(None).await` | Async | Yes (offline) | Plutus script transactions |

### Common Methods

| Method | Purpose |
|--------|---------|
| `.tx_in()` | Add a specific UTxO as input |
| `.tx_out()` | Add an output |
| `.change_address()` | Set the change address |
| `.select_utxos_from()` | Auto-select UTxOs from a pool |
| `.signing_key()` | Add a signing key |
| `.required_signer_hash()` | Add a required signer |
| `.invalid_before()` | Set validity start slot |
| `.invalid_hereafter()` | Set validity end slot |
| `.metadata_value()` | Attach transaction metadata |
| `.complete_signing()` | Sign and return the final tx hex |

## Chapters

- [Simple Transactions](./tx-builder/simple.md) — Send lovelace, lock funds, delegate stake
- [Plutus Script Transactions](./tx-builder/plutus.md) — Unlock funds from scripts, handle datums and redeemers
- [Minting](./tx-builder/minting.md) — Mint and burn native tokens with Plutus scripts
- [Staking & Governance](./tx-builder/staking.md) — Stake registration, delegation, withdrawals, governance
