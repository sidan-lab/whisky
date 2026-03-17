# Transaction Parser

The transaction parser lets you deserialize a raw CBOR transaction hex back into a `TxBuilderBody`, inspect it, edit it, and rebuild it into a new transaction.

## Parsing a Transaction

Use the `parse` function to deserialize a transaction:

```rust,ignore
use whisky::*;
use whisky_pallas::tx_parser::parse;

let tx_hex = "84a700d90102..."; // Raw transaction CBOR hex
let utxos = vec![/* resolved UTxOs referenced by the transaction */];

let body = parse(tx_hex, &utxos).unwrap();
```

The `parse` function returns a `TxBuilderBody` containing all the transaction's inputs, outputs, mints, certificates, withdrawals, and metadata.

> **Important**: You must provide the resolved UTxOs that the transaction references as inputs. The parser needs these to reconstruct the full input context.

## The Parse-Edit-Rebuild Pattern

A powerful pattern is to parse an existing transaction, modify it, and rebuild:

```rust,ignore
use whisky::*;
use whisky_pallas::tx_parser::parse;

// 1. Parse the original transaction
let utxos = vec![utxo_1, utxo_2, utxo_3];
let tx_hex = "84a700d90102...";
let mut body = parse(tx_hex, &utxos).unwrap();

// 2. Edit the body
body.outputs.pop();           // Remove last output
body.reference_inputs.pop();  // Remove a reference input

// 3. Rebuild with a new TxBuilder
let mut tx_builder = TxBuilder::new_core();
tx_builder.tx_builder_body = body;

// 4. Add new elements and rebalance
let new_tx_hex = tx_builder
    .tx_out(
        "addr_test1zp...",
        &[Asset::new_from_str("lovelace", "5000000")],
    )
    .invalid_before(100)
    .invalid_hereafter(200)
    .required_signer_hash("3f1b5974f4f09f0974be655e4ce94f8a2d087df378b79ef3916c26b2")
    .complete_sync(None)
    .unwrap()
    .tx_hex();
```

The resulting transaction is automatically rebalanced with proper fee calculation and a new change output.

## TxParsable Trait

The parser implements the `TxParsable` trait, which provides:

```rust,ignore
pub trait TxParsable {
    fn parse(&mut self, tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<(), WError>;
    fn get_required_inputs(&mut self, tx_hex: &str) -> Result<Vec<UtxoInput>, WError>;
    fn get_builder_body(&self) -> TxBuilderBody;
    fn get_builder_body_without_change(&self) -> TxBuilderBody;
    fn to_tester(&self) -> TxTester;
}
```

| Method | Purpose |
|--------|---------|
| `parse` | Deserialize tx hex into internal state |
| `get_required_inputs` | Extract input references without full parsing |
| `get_builder_body` | Get the full `TxBuilderBody` from parsed state |
| `get_builder_body_without_change` | Get the body excluding the change output |
| `to_tester` | Convert to a `TxTester` for making assertions |

## Checking Required Signers

You can also inspect a transaction's required signers:

```rust,ignore
use whisky_pallas::tx_parser::check_tx_required_signers;

let signers = check_tx_required_signers(tx_hex);
```

## Transaction Evaluation

For parsed transactions that include Plutus scripts, you can evaluate execution units:

```rust,ignore
use uplc::tx::script_context::SlotConfig;
use whisky_common::{Network, UTxO};
use whisky_pallas::utils::evaluate_tx_scripts;

let result = evaluate_tx_scripts(
    tx_hex,
    &utxos,
    &[],  // additional chained transactions
    &Network::Mainnet,
    &SlotConfig::default(),
);
```

This returns execution units (memory and CPU steps) for each script in the transaction.
