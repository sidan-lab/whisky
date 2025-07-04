<div align="center">
  <hr />
    <h2 align="center" style="border-bottom: none"><img style="position: relative; top: 0.25rem;" src="https://raw.githubusercontent.com/sidan-lab/brand_assets/main/whisky.png" alt="Whisky" height="30" /> Whisky - Cardano Rust SDK</h2>

[![Licence](https://img.shields.io/github/license/sidan-lab/whisky)](https://github.com/sidan-lab/whisky/blob/master/LICENSE)
[![Test](https://github.com/sidan-lab/whisky/actions/workflows/rust-build-test.yml/badge.svg)](https://github.com/sidan-lab/whisky/actions/workflows/rust-build-test.yml)
[![Publish](https://github.com/sidan-lab/whisky/actions/workflows/publish-packages.yml/badge.svg)](https://github.com/sidan-lab/whisky/actions/workflows/publish-packages.yml)
[![Docs](https://github.com/sidan-lab/whisky/actions/workflows/static.yml/badge.svg?branch=master)](https://github.com/sidan-lab/whisky/actions/workflows/static.yml)

[![Twitter/X](https://img.shields.io/badge/Follow%20us-@sidan__lab-blue?logo=x&style=for-the-badge)](https://x.com/sidan_lab)
[![Crates.io](https://img.shields.io/crates/v/whisky?style=for-the-badge)](https://crates.io/crates/whisky)
[![NPM](https://img.shields.io/npm/v/%40sidan-lab%2Fwhisky-js-nodejs?style=for-the-badge)](https://www.npmjs.com/package/@sidan-lab/whisky-js-nodejs)

  <hr/>
</div>

# whisky

Whisky is an open-source Cardano Rust SDK, containing following modules:

- `whisky` - The core Rust crate supporting Cardano DApp development in Rust.
- `whisky-common` - Serving universal types and utilities.
- `whisky-csl` - The crate to implement most `cardano-serialization-lib` wrapper.
- `whisky-provider` - The crate to connect external services like blockfrost or maestro.
- `whisky-wallet` - The crate to handle wallet signing and provide key encryption utility.
- `whisky-macros` - The crate to provide Rust macros utility.
- `whisky-js` - An point of output for wasm package for `@meshsdk/core-csl`.

With whisky, you can

- Builder transaction with cardano-cli-like APIs, supporting serious DApps’ backend on the Rust codebase.
- Handling transaction signing in Rust
- Interacting with blockchain with provider services like `Maestro` and `Blockfrost`
- Off-node evaluation on transaction execution units, and updating the transaction accordingly with TxPipe's `uplc` integrated.

## Installation

### Rust Library

```sh
cargo add whisky
```

### JS / TS WASM Lib

```sh
# For nodejs package
yarn add @sidan-lab/whisky-js-nodejs
# For browser package
yarn add @sidan-lab/whisky-js-browser
```

## Getting Started

```rust
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

# Contributor Guides

Make sure llvm is installed

## APIs

Please refer to the [hosted documentation](https://sidan-lab.github.io/whisky/whisky/index.html) for the list of endpoints.

![Alt](https://repobeats.axiom.co/api/embed/2e35716a9dd3250972c06ca2b4c7f1846ef7c51e.svg "Repobeats analytics image")
