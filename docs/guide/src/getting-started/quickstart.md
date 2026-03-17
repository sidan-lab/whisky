# Quick Start

This guide shows you how to build your first Cardano transaction with whisky.

## Your First Transaction

The simplest transaction sends lovelace from one address to another. Here's the complete example:

```rust,ignore
use whisky::*;

pub fn send_lovelace(
    recipient_address: &str,
    my_address: &str,
    inputs: &[UTxO],
) -> Result<String, WError> {
    let mut tx_builder = TxBuilder::new_core();
    tx_builder
        .tx_out(
            recipient_address,
            &[Asset::new_from_str("lovelace", "1000000")],
        )
        .change_address(my_address)
        .select_utxos_from(inputs, 5000000)
        .complete_sync(None)?;

    Ok(tx_builder.tx_hex())
}
```

Let's break this down:

1. **`TxBuilder::new_core()`** — Creates a new transaction builder with the default Pallas serializer
2. **`.tx_out(address, assets)`** — Adds an output sending 1 ADA (1,000,000 lovelace) to the recipient
3. **`.change_address(address)`** — Sets where leftover funds go after paying fees
4. **`.select_utxos_from(inputs, threshold)`** — Automatically selects UTxOs from the provided set to cover outputs + fees. The threshold (5,000,000 lovelace) is extra headroom for fees and min UTxO
5. **`.complete_sync(None)`** — Balances the transaction, calculates fees, and serializes to CBOR
6. **`.tx_hex()`** — Returns the unsigned transaction as a hex-encoded CBOR string

## Sync vs Async

Whisky offers both synchronous and asynchronous completion:

```rust,ignore
// Synchronous — no script evaluation, no provider calls
tx_builder.complete_sync(None)?;

// Asynchronous — evaluates Plutus scripts, can fetch data from providers
tx_builder.complete(None).await?;
```

Use `complete_sync` for simple transactions without scripts. Use `complete` (async) when your transaction includes Plutus scripts that need execution unit evaluation.

## Signing

After building, sign the transaction with a private key:

```rust,ignore
let signed_tx = tx_builder
    .signing_key("your_signing_key_hex")
    .complete_sync(None)?
    .complete_signing()?;
```

The `signed_tx` string is ready for submission to the Cardano network.

## Running the Tests

Whisky includes comprehensive integration tests you can run to see these patterns in action:

```sh
# Run all tests
cargo test

# Run Pallas integration tests specifically
cargo test --package whisky --test pallas_integration_tests

# Run a specific test with output
cargo test --package whisky --test pallas_integration_tests test_simple_spend -- --nocapture
```

## Next Steps

- [Transaction Builder](../guides/tx-builder.md) — Explore all transaction building patterns
- [Transaction Parser](../guides/tx-parser.md) — Parse and edit existing transactions
- [Dependency Injection](../guides/dependency-injection.md) — Customize serializers and providers
