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

## End-to-End WhiskyPallas Tx Build and Unit Test Run

Whisky supports multiple serializer backends. The `WhiskyPallas` serializer is a pure-Rust implementation using [TxPipe's Pallas](https://github.com/txpipe/pallas) library, providing an alternative to the CSL (cardano-serialization-lib) backend.

### Choosing a Serializer

You can choose between `WhiskyPallas` and `WhiskyCSL` when creating a `TxBuilder`:

```rust
use whisky::*;
use whisky_pallas::WhiskyPallas;

// Using WhiskyPallas serializer
let mut tx_builder = TxBuilder::new(TxBuilderParam {
    serializer: Box::new(WhiskyPallas::new(None)),
    evaluator: None,
    fetcher: None,
    submitter: None,
    params: None,
});

// Or using the default CSL serializer
let mut tx_builder_csl = TxBuilder::new_core();
```

### End-to-End Transaction Building with WhiskyPallas

#### Simple Spend Transaction

```rust
use whisky::*;
use whisky_pallas::WhiskyPallas;

fn build_simple_spend() -> String {
    let mut tx_builder = TxBuilder::new(TxBuilderParam {
        serializer: Box::new(WhiskyPallas::new(None)),
        evaluator: None,
        fetcher: None,
        submitter: None,
        params: None,
    });

    let signed_tx = tx_builder
        .tx_in(
            "2cb57168ee66b68bd04a0d595060b546edf30c04ae1031b883c9ac797967dd85",
            3,
            &[Asset::new_from_str("lovelace", "9891607895")],
            "addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh",
        )
        .change_address("addr_test1vru4e2un2tq50q4rv6qzk7t8w34gjdtw3y2uzuqxzj0ldrqqactxh")
        .signing_key("your_signing_key_hex")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();

    signed_tx
}
```

#### Complex Plutus Transaction with Minting and Script Reference

```rust
use serde_json::{json, to_string};
use whisky::*;
use whisky_common::data::*;
use whisky_pallas::WhiskyPallas;

fn build_complex_plutus_tx() -> String {
    let mut tx_builder = TxBuilder::new(TxBuilderParam {
        serializer: Box::new(WhiskyPallas::new(None)),
        evaluator: None,
        fetcher: None,
        submitter: None,
        params: None,
    });

    let policy_id = "baefdc6c5b191be372a794cd8d40d839ec0dbdd3c28957267dc81700";
    let token_name_hex = "6d65736874657374696e67342e6164610a";

    tx_builder
        // Add input UTxO
        .tx_in(
            "fc1c806abc9981f4bee2ce259f61578c3341012f3d04f22e82e7e40c7e7e3c3c",
            3,
            &[Asset::new_from_str("lovelace", "9692479606")],
            "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
        )
        // Mint tokens using Plutus V2 script reference
        .mint_plutus_script_v2()
        .mint(1, policy_id, token_name_hex)
        .mint_tx_in_reference(
            "63210437b543c8a11afbbc6765aa205eb2733cb74e2805afd4c1c8cb72bd8e22",
            0,
            policy_id,
            100, // script size
        )
        .mint_redeemer_value(&WRedeemer {
            data: WData::JSON(
                to_string(&json!({
                    "constructor": 0,
                    "fields": []
                }))
                .unwrap(),
            ),
            ex_units: Budget {
                mem: 3386819,
                steps: 1048170931,
            },
        })
        // Add output with minted token
        .tx_out(
            "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
            &[
                Asset::new_from_str("lovelace", "2000000"),
                Asset::new(policy_id.to_string() + token_name_hex, "1".to_string()),
            ],
        )
        // Add collateral
        .tx_in_collateral(
            "3fbdf2b0b4213855dd9b87f7c94a50cf352ba6edfdded85ecb22cf9ceb75f814",
            6,
            &[Asset::new_from_str("lovelace", "10000000")],
            "addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x",
        )
        .change_address("addr_test1vpw22xesfv0hnkfw4k5vtrz386tfgkxu6f7wfadug7prl7s6gt89x")
        .complete_sync(None)
        .unwrap();

    tx_builder.tx_hex()
}
```

#### Staking Withdrawal Transaction

```rust
use whisky::*;
use whisky_pallas::WhiskyPallas;

fn build_withdrawal_tx() -> String {
    let mut tx_builder = TxBuilder::new(TxBuilderParam {
        serializer: Box::new(WhiskyPallas::new(None)),
        evaluator: None,
        fetcher: None,
        submitter: None,
        params: None,
    });

    let signed_tx = tx_builder
        .tx_in(
            "fbd3e8091c9f0c5fb446be9e58d9235f548546a5a7d5f60ee56e389344db9c5e",
            0,
            &[Asset::new_from_str("lovelace", "9496607660")],
            "addr_test1qpjfsrjdr8kk5ffj4jnw02ht3y3td0y0zkcm52rc6w7z7flmy7vplnvz6a7dncss4q5quqwt48tv9dewuvdxqssur9jqc4x459",
        )
        .change_address("addr_test1qpjfsrjdr8kk5ffj4jnw02ht3y3td0y0zkcm52rc6w7z7flmy7vplnvz6a7dncss4q5quqwt48tv9dewuvdxqssur9jqc4x459")
        .withdrawal("stake_test1uraj0xqlekpdwlxeugg2s2qwq896n4kzkuhwxxnqggwpjeqe9s9k2", 0)
        .required_signer_hash("fb27981fcd82d77cd9e210a8280e01cba9d6c2b72ee31a60421c1964")
        .signing_key("your_signing_key_hex")
        .complete_sync(None)
        .unwrap()
        .complete_signing()
        .unwrap();

    signed_tx
}
```

### Running Unit Tests

The whisky crate includes comprehensive integration tests for both WhiskyPallas and WhiskyCSL serializers. These tests verify transaction building for various scenarios including simple spends, complex Plutus transactions, minting, withdrawals, and governance actions.

```sh
# Run all tests
cargo test

# Run only WhiskyPallas integration tests
cargo test --package whisky --test pallas_integration_tests

# Run only WhiskyCSL integration tests
cargo test --package whisky --test csl_integration_tests

# Run a specific test with output
cargo test --package whisky --test pallas_integration_tests test_simple_spend -- --nocapture
```

The integration tests cover:
- **Common transactions**: Simple spend, withdrawals, stake registration/deregistration
- **Complex Plutus transactions**: Minting with script references, spending from script addresses, inline datums
- **Governance transactions**: DRep registration, vote delegation, voting
- **Edge cases**: Multiple collateral inputs, custom protocol parameters, embedded datums

# Contributor Guides

Make sure llvm is installed

# Maintainers List

- Hinson Wong (Github Handle: HinsonSIDAN) - Maintainer
- Tsz Wai (Github Handle: twwu123) - Maintainer
- Ken Lau (Github Handle: kenlau666) - Maintainer
- Anson Chui (Github Handle: AnsonSIDAN) - Project Manager

## APIs

Please refer to the [hosted documentation](https://sidan-lab.github.io/whisky/whisky/index.html) for the list of endpoints.

![Alt](https://repobeats.axiom.co/api/embed/2e35716a9dd3250972c06ca2b4c7f1846ef7c51e.svg "Repobeats analytics image")

# Community Channel Invite

Please join SIDAN Lab's discord server for regular update using https://discord.gg/prJvB6b6p4.

Please view SIDAN Lab's dedicated channel for any SIDAN - Whisky V2 - Cardano Rust SDK with Pallas Catalyst Project Update in https://discord.com/channels/1166784293805228061/1441817320245952532.
