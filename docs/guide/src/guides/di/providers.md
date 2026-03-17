# Providers

Providers implement the `Fetcher`, `Evaluator`, and `Submitter` traits to connect whisky to the Cardano blockchain.

## Fetcher

The `Fetcher` trait provides blockchain data:

```rust,ignore
#[async_trait]
pub trait Fetcher: Send + Sync {
    async fn fetch_account_info(&self, address: &str) -> Result<AccountInfo, WError>;
    async fn fetch_address_utxos(
        &self,
        address: &str,
        asset: Option<&str>,
    ) -> Result<Vec<UTxO>, WError>;
    async fn fetch_asset_addresses(&self, asset: &str) -> Result<Vec<(String, String)>, WError>;
    async fn fetch_asset_metadata(
        &self,
        asset: &str,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, WError>;
    async fn fetch_block_info(&self, hash: &str) -> Result<BlockInfo, WError>;
    async fn fetch_collection_assets(
        &self,
        policy_id: &str,
        cursor: Option<String>,
    ) -> Result<(Vec<(String, String)>, Option<String>), WError>;
    async fn fetch_protocol_parameters(&self, epoch: Option<u32>) -> Result<Protocol, WError>;
    async fn fetch_tx_info(&self, hash: &str) -> Result<TransactionInfo, WError>;
    async fn fetch_utxos(&self, hash: &str, index: Option<u32>) -> Result<Vec<UTxO>, WError>;
    async fn get(&self, url: &str) -> Result<serde_json::Value, WError>;
}
```

## Evaluator

The `Evaluator` trait runs Plutus scripts off-chain to determine execution units:

```rust,ignore
#[async_trait]
pub trait Evaluator: Send {
    async fn evaluate_tx(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],
        additional_txs: &[String],
        network: &Network,
        slot_config: &SlotConfig,
    ) -> Result<Vec<Action>, WError>;
}
```

By default, `TxBuilder` uses `OfflineTxEvaluator`, which evaluates scripts locally using TxPipe's `uplc` library — no network calls needed.

## Submitter

The `Submitter` trait submits signed transactions:

```rust,ignore
#[async_trait]
pub trait Submitter: Send + Sync {
    async fn submit_tx(&self, tx_hex: &str) -> Result<String, WError>;
}
```

Returns the transaction hash on success.

## Built-in Providers

### Maestro

```rust,ignore
use whisky_provider::MaestroProvider;

let provider = MaestroProvider::new("your_api_key", "preprod"); // or "mainnet"

let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),
    evaluator: None,
    fetcher: Some(Box::new(provider.clone())),
    submitter: Some(Box::new(provider)),
    params: None,
});
```

### Blockfrost

```rust,ignore
use whisky_provider::BlockfrostProvider;

let provider = BlockfrostProvider::new("your_project_id", "preprod"); // or "mainnet"

let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),
    evaluator: None,
    fetcher: Some(Box::new(provider.clone())),
    submitter: Some(Box::new(provider)),
    params: None,
});
```

> **Note**: Using providers requires the `provider` feature flag (enabled by default).

## Wiring It All Together

A fully-wired `TxBuilder` can fetch UTxOs, evaluate scripts, and submit — all in one flow:

```rust,ignore
use whisky::*;
use whisky_pallas::WhiskyPallas;
use whisky_provider::MaestroProvider;

let provider = MaestroProvider::new("api_key", "preprod");

let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),
    evaluator: None,  // Uses OfflineTxEvaluator by default
    fetcher: Some(Box::new(provider.clone())),
    submitter: Some(Box::new(provider)),
    params: None,
});

// Build, evaluate, sign, and submit
tx_builder
    .tx_out(recipient, &[Asset::new_from_str("lovelace", "5000000")])
    .change_address(my_address)
    .select_utxos_from(&utxos, 5000000)
    .signing_key(skey_hex)
    .complete(None)
    .await?
    .complete_signing()?;

// Submit
let tx_hash = tx_builder
    .submit_tx(&tx_builder.tx_hex())
    .await?;
```
