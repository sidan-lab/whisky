<div align="center">
  <hr />
    <h2 align="center" style="border-bottom: none"><img style="position: relative; top: 0.25rem;" src="https://raw.githubusercontent.com/sidan-lab/brand_assets/main/whisky.png" alt="Whisky" height="30" /> Whisky - Cardano Rust SDK</h2>

[![Licence](https://img.shields.io/github/license/sidan-lab/whisky)](https://github.com/sidan-lab/whisky/blob/master/LICENSE)
[![Test](https://github.com/sidan-lab/whisky/actions/workflows/rust-build-test.yml/badge.svg)](https://github.com/sidan-lab/whisky/actions/workflows/rust-build-test.yml)
[![Publish](https://github.com/sidan-lab/whisky/actions/workflows/publish-packages.yml/badge.svg)](https://github.com/sidan-lab/whisky/actions/workflows/publish-packages.yml)
[![Docs](https://github.com/sidan-lab/whisky/actions/workflows/static.yml/badge.svg?branch=master)](https://github.com/sidan-lab/whisky/actions/workflows/static.yml)

[![Twitter/X](https://img.shields.io/badge/Follow%20us-@sidan__lab-blue?logo=x&style=for-the-badge)](https://x.com/sidan_lab)
[![Crates.io](https://img.shields.io/crates/v/whisky?style=for-the-badge)](https://crates.io/crates/whisky)
[![NPM](https://img.shields.io/npm/v/%40sidan-lab%2Fsidan-csl-rs-nodejs?style=for-the-badge)](https://www.npmjs.com/package/@sidan-lab/sidan-csl-rs-nodejs)

  <hr/>
</div>

# whisky

This is a library for building off-chain code on Cardano. It is a cardano-cli-like wrapper on cardano-serialization-lib (equivalent to MeshJS’s lower-level APIs), supporting serious DApps’ backend on the Rust codebase. It has an active [F11 proposal](https://cardano.ideascale.com/c/idea/112172) to support the development.

whisky is composed of 2 layers - the root layer sidan-csl-rs and the user-facing layer whisky. sidan-csl-rs composed of the core serialization logic with JSON-to-transaction pattern, compilable to wasm. whisky is the user-facing package that Rust Cardano developers can import directly for use.

## Features

- Same API patterns with [MeshJS](https://meshjs.dev/apis/transaction/builderExample) - lower learning curve for developers.
- Integrated with TxPipe's `uplc` for off-node auto redeemer exUnits updates.
- Full inline documentation hosted at [github](https://sidan-lab.github.io/whisky/whisky/index.html)

## Installation

### Rust Library

```sh
cargo add whisky
```

### JS / TS WASM Lib

```sh
# For nodejs package
yarn add @sidan-lab/sidan-csl-rs-nodejs
# For browser package
yarn add @sidan-lab/sidan-csl-rs-browser
```

## Getting Started

```rust
use whisky::{
    builder::TxBuilder,
    model::{Asset, UTxO},
};

async fn my_first_whisky_tx(
    recipient_address: &str,
    my_address: &str,
    inputs: Vec<UTxO>,
) -> String {
    let mut tx_builder = TxBuilder::new_core();
    tx_builder.tx_out(
        &recipient_address,
        vec![Asset::new_from_str("lovelace", "1000000")],
    )
    .change_address(my_address)
    .select_utxos_from(inputs.clone(), 5000000)
    .complete(None)
    .await;

    tx_builder.tx_hex()
}
```

## APIs

Please refer to the [hosted documentation](https://sidan-lab.github.io/whisky/whisky/index.html) for the list of endpoints.

![Alt](https://repobeats.axiom.co/api/embed/2e35716a9dd3250972c06ca2b4c7f1846ef7c51e.svg "Repobeats analytics image")
