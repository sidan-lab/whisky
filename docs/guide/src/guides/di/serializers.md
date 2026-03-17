# Serializer Backends

The `TxBuildable` trait abstracts transaction serialization. Whisky ships with two implementations: **WhiskyPallas** (recommended) and **WhiskyCSL** (legacy).

## TxBuildable Trait

```rust,ignore
pub trait TxBuildable: Debug + Send + Sync {
    fn set_protocol_params(&mut self, protocol_params: Protocol);
    fn set_tx_builder_body(&mut self, tx_builder: TxBuilderBody);
    fn reset_builder(&mut self);

    fn serialize_tx_body(&mut self) -> Result<String, WError>;
    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, WError>;
    fn complete_signing(&mut self) -> Result<String, WError>;
    fn set_tx_hex(&mut self, tx_hex: String);
    fn tx_hex(&mut self) -> String;
    fn tx_evaluation_multiplier_percentage(&self) -> u64;

    fn add_tx_in(&mut self, input: PubKeyTxIn) -> Result<(), WError>;
}
```

The `TxBuilder` calls these methods internally — you interact with the high-level builder API, not the trait directly.

## WhiskyPallas (Recommended)

The Pallas-based serializer is the default and recommended backend. It uses [TxPipe's Pallas](https://github.com/txpipe/pallas) library for CBOR serialization.

```rust,ignore
use whisky_pallas::WhiskyPallas;

// With default protocol parameters
let serializer = WhiskyPallas::new(None);

// With custom protocol parameters
let serializer = WhiskyPallas::new(Some(protocol_params));
```

Use it with `TxBuilder`:

```rust,ignore
use whisky::*;
use whisky_pallas::WhiskyPallas;

let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),
    evaluator: None,
    fetcher: None,
    submitter: None,
    params: None,
});
```

Or simply:

```rust,ignore
let mut tx_builder = TxBuilder::new_core();
// new_core() uses WhiskyPallas by default
```

**Why Pallas?**
- Pure Rust implementation — no C dependencies
- Actively maintained and updated for hard forks
- Better alignment with the Cardano Rust ecosystem

## WhiskyCSL (Legacy)

The CSL-based serializer uses `cardano-serialization-lib`. It's available for backward compatibility.

```rust,ignore
use whisky_csl::WhiskyCSL;

let serializer = WhiskyCSL::new(None);
```

> **Note**: The main `whisky` crate no longer includes CSL by default. To use it, add `whisky-csl` directly:
>
> ```toml
> [dependencies]
> whisky-csl = "1.0.28-beta.1"
> ```

## Implementing a Custom Serializer

You can implement `TxBuildable` for your own serializer:

```rust,ignore
use whisky_common::*;

#[derive(Debug)]
struct MySerializer {
    // your fields
}

impl TxBuildable for MySerializer {
    fn set_protocol_params(&mut self, protocol_params: Protocol) { /* ... */ }
    fn set_tx_builder_body(&mut self, tx_builder: TxBuilderBody) { /* ... */ }
    fn reset_builder(&mut self) { /* ... */ }
    fn serialize_tx_body(&mut self) -> Result<String, WError> { /* ... */ }
    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, WError> { /* ... */ }
    fn complete_signing(&mut self) -> Result<String, WError> { /* ... */ }
    fn set_tx_hex(&mut self, tx_hex: String) { /* ... */ }
    fn tx_hex(&mut self) -> String { /* ... */ }
    fn tx_evaluation_multiplier_percentage(&self) -> u64 { 110 }
    fn add_tx_in(&mut self, input: PubKeyTxIn) -> Result<(), WError> { /* ... */ }
}
```

Then inject it into `TxBuilder`:

```rust,ignore
let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(MySerializer { /* ... */ }),
    evaluator: None,
    fetcher: None,
    submitter: None,
    params: None,
});
```
