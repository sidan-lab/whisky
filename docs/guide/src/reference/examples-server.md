# Examples Server

The `whisky-examples` crate provides runnable transaction examples and an HTTP server that exposes them as API endpoints.

## Running the Server

```sh
cargo run --package whisky-examples
```

The server starts on `http://127.0.0.1:8080` with CORS enabled.

## Available Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/send_lovelace` | POST | Send ADA to a recipient |
| `/lock_fund` | POST | Lock funds at a script address with datum |
| `/unlock_fund` | POST | Unlock funds from a Plutus script |
| `/mint_tokens` | POST | Mint tokens with a Plutus minting policy |

## Example: Send Lovelace

```sh
curl -X POST http://127.0.0.1:8080/send_lovelace \
  -H "Content-Type: application/json" \
  -d '{
    "recipientAddress": "addr_test1...",
    "myAddress": "addr_test1...",
    "inputs": [
      {
        "input": {
          "txHash": "abcdef...",
          "outputIndex": 0
        },
        "output": {
          "address": "addr_test1...",
          "amount": [{"unit": "lovelace", "quantity": "10000000"}]
        }
      }
    ]
  }'
```

Response:

```json
{
  "txHex": "84a400..."
}
```

## Example Functions

The transaction functions are in `packages/whisky-examples/src/tx/`:

| File | Function | Type |
|------|----------|------|
| `send_lovelace.rs` | `send_lovelace` | Sync |
| `lock_fund.rs` | `lock_fund` | Sync |
| `unlock_fund.rs` | `unlock_fund` | Async |
| `mint_tokens.rs` | `mint_tokens` | Async |
| `delegate_stake.rs` | `delegate_stake` | Sync |
| `complex_transaction.rs` | `complex_transaction` | Async |
| `collateral_return.rs` | `collateral_return` | Async |

These examples serve as both documentation and testable reference implementations. See the [Transaction Builder](../guides/tx-builder.md) guide for detailed explanations of each pattern.
