# Simple Transactions

These examples cover basic transaction patterns that don't involve Plutus scripts.

## Send Lovelace

The most basic transaction: send ADA from one address to another.

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

> **Note**: You don't need to manually add inputs — `select_utxos_from` automatically picks UTxOs to cover the output amount plus the threshold for fees.

## Lock Funds at a Script Address

Send funds to a script address with an inline datum attached:

```rust,ignore
use whisky::*;

pub fn lock_fund(
    script_address: &str,
    datum: &str,
    my_address: &str,
    inputs: &[UTxO],
) -> Result<String, WError> {
    let mut tx_builder = TxBuilder::new_core();
    tx_builder
        .tx_out(script_address, &[])
        .tx_out_inline_datum_value(&WData::JSON(datum.to_string()))
        .change_address(my_address)
        .select_utxos_from(inputs, 5000000)
        .complete_sync(None)?;

    Ok(tx_builder.tx_hex())
}
```

Key points:
- **`tx_out_inline_datum_value`** attaches an inline datum (stored on-chain) to the output
- The datum is provided as a JSON-encoded Plutus data string (e.g., `{"constructor": 0, "fields": []}`)
- Use `tx_out_datum_hash_value` instead if you only want to store the datum hash on-chain

## Delegate Stake

Register a stake key and delegate to a pool in a single transaction:

```rust,ignore
use whisky::*;

pub fn delegate_stake(
    stake_key_hash: &str,
    pool_id: &str, // e.g., "pool1..."
    my_address: &str,
    inputs: &[UTxO],
) -> Result<String, WError> {
    let mut tx_builder = TxBuilder::new_core();
    tx_builder
        .register_stake_certificate(stake_key_hash)
        .delegate_stake_certificate(stake_key_hash, pool_id)
        .change_address(my_address)
        .select_utxos_from(inputs, 5000000)
        .complete_sync(None)?;

    Ok(tx_builder.tx_hex())
}
```

## Validity Ranges and Metadata

You can set time bounds and attach metadata to any transaction:

```rust,ignore
tx_builder
    .tx_out(address, &[Asset::new_from_str("lovelace", "2000000")])
    .invalid_before(100)          // Transaction valid from slot 100
    .invalid_hereafter(200)       // Transaction invalid after slot 200
    .metadata_value("674", "{\"msg\": [\"Hello, Cardano!\"]}")
    .change_address(my_address)
    .complete_sync(None)?;
```

## Signing and Submitting

After building, sign with one or more keys:

```rust,ignore
let signed_tx = tx_builder
    .signing_key("ed25519_sk_hex_key_1")
    .signing_key("ed25519_sk_hex_key_2")  // Multiple signers
    .complete_sync(None)?
    .complete_signing()?;

// signed_tx is ready for submission
```
