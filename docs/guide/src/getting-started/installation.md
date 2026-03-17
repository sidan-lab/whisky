# Installation

## Rust Library

Add whisky to your project:

```sh
cargo add whisky
```

Or add it directly to your `Cargo.toml`:

```toml
[dependencies]
whisky = "1.0.28-beta.1"
```

### Feature Flags

By default, all features are enabled (`full`). You can selectively enable only what you need:

```toml
# Full (default) — includes wallet + provider
whisky = "1.0.28-beta.1"

# Just common types (minimal, no serializer backends)
whisky = { version = "1.0.28-beta.1", default-features = false }

# Wallet only (signing + key management)
whisky = { version = "1.0.28-beta.1", default-features = false, features = ["wallet"] }

# Provider only (Blockfrost/Maestro integrations)
whisky = { version = "1.0.28-beta.1", default-features = false, features = ["provider"] }
```

| Feature | Includes | Use Case |
|---------|----------|----------|
| `full` (default) | wallet + provider | Full DApp backend |
| `wallet` | Signing, key encryption | Transaction signing only |
| `provider` | Blockfrost, Maestro | Blockchain data fetching |

## JS / TS WASM Library

For JavaScript or TypeScript projects, whisky is available as a WASM package:

```sh
# For Node.js
yarn add @sidan-lab/whisky-js-nodejs

# For browser
yarn add @sidan-lab/whisky-js-browser
```

## Prerequisites

For building from source, make sure LLVM is installed on your system:

- **macOS**: `brew install llvm`
- **Ubuntu/Debian**: `apt install llvm-dev libclang-dev`
- **Windows**: Install via [LLVM releases](https://releases.llvm.org/)
