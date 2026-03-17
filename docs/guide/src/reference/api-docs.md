# API Documentation

Full API reference documentation is auto-generated from Rust doc comments using `cargo doc`.

## Browse the API Docs

**[Open API Reference](../api/whisky/index.html)**

## Key Entry Points

| Crate | Description | Link |
|-------|-------------|------|
| **whisky** | Main crate — re-exports all public APIs | [docs](../api/whisky/index.html) |
| **whisky-common** | Shared types, traits, and utilities | [docs](../api/whisky_common/index.html) |
| **whisky-pallas** | Pallas-based serializer | [docs](../api/whisky_pallas/index.html) |
| **whisky-csl** | Legacy CSL-based serializer | [docs](../api/whisky_csl/index.html) |
| **whisky-provider** | Blockfrost and Maestro integrations | [docs](../api/whisky_provider/index.html) |
| **whisky-wallet** | Wallet signing and key management | [docs](../api/whisky_wallet/index.html) |
| **whisky-macros** | Procedural macros for Plutus data | [docs](../api/whisky_macros/index.html) |

## Building Docs Locally

```sh
# Generate API reference
npm run rust:doc

# Serve locally
npx http-server ./docs
```

The docs are also published automatically to GitHub Pages on every push to the deployment branch.
