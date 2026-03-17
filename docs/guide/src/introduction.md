# Introduction

Whisky is an open-source Cardano Rust SDK built by [SIDAN Lab](https://github.com/sidan-lab). It provides a comprehensive set of tools for building Cardano DApps in Rust, with a chainable builder API inspired by [MeshJS](https://meshjs.dev/).

## Modules

Whisky is organized as a Rust workspace with the following crates:

| Crate | Description |
|-------|-------------|
| **whisky** | The main crate — re-exports everything you need for DApp development |
| **whisky-common** | Shared types, interfaces, and utilities used across all crates |
| **whisky-pallas** | Transaction serializer built on [TxPipe's Pallas](https://github.com/txpipe/pallas) (recommended) |
| **whisky-csl** | Legacy serializer built on `cardano-serialization-lib` |
| **whisky-provider** | Provider integrations for Blockfrost and Maestro |
| **whisky-wallet** | Wallet signing and key management utilities |
| **whisky-macros** | Procedural macros for Plutus data encoding |
| **whisky-js** | WASM bindings for JavaScript/TypeScript usage |

## What You Can Do

With whisky, you can:

- **Build transactions** with a chainable, cardano-cli-like API supporting complex DApp backends
- **Parse and edit transactions** from raw CBOR hex
- **Sign transactions** with key-based signing in Rust
- **Interact with the blockchain** via Maestro and Blockfrost providers
- **Evaluate scripts off-chain** using TxPipe's `uplc` for execution unit estimation
- **Swap serializer backends** between Pallas and CSL via dependency injection

## Guide Overview

This guide walks you through:

1. **[Installation](./getting-started/installation.md)** — Adding whisky to your project
2. **[Quick Start](./getting-started/quickstart.md)** — Building your first transaction
3. **[Transaction Builder](./guides/tx-builder.md)** — Simple sends, Plutus scripts, minting, staking
4. **[Transaction Parser](./guides/tx-parser.md)** — Parsing and editing existing transactions
5. **[Dependency Injection](./guides/dependency-injection.md)** — Pluggable serializers and providers
6. **[Migration: CSL to Pallas](./guides/migration-csl-to-pallas.md)** — Upgrading from the legacy backend

For full API reference, see the [generated Rust docs](../api/whisky/index.html).
