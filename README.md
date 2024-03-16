# sidan-csl-rs

This is a library for building off-chain code on Cardano. It is a cardano-cli like wrapper on cardano-serialization-lib (equivalent on MeshJS’s lower level APIs), supporting serious DApps’ backend on rust codebase. It has an active [F11 proposal](https://cardano.ideascale.com/c/idea/112172) for supporting the development.

## Installation

### Rust Library

```sh
cargo add sidan-csl-rs
```

### JS / TS WASM Lib

- To be added

## APIs

The APIs of `sidan-csl-rs` consists of 3 parts:

### 1. APIs on core CSL logic as `MeshCSL`

- (Property) `tx_hex`
- (Property) `tx_builder`
- (Property) `tx_inputs_builder`
- `add_tx_in`
- `add_script_tx_in`
- `add_output`
- `add_collateral`
- `add_reference_input`
- `add_plutus_mint`
- `add_native_mint`
- `add_invalid_before`
- `add_invalid_hereafter`
- `add_change`
- `add_signing_keys`
- `add_required_signature`
- `add_metadata`
- `add_script_hash`
- `build_tx` - build the transaction to hex in CSL

### 2. `MeshTxBuilderCore`

#### 2.1 User-facing wrapper APIs

- `tx_in`
- `tx_in_script`
- `tx_in_datum_value`
- `tx_in_inline_datum_present`
- `tx_in_redeemer_value`
- `tx_out`
- `tx_out_datum_hash_value`
- `tx_out_inline_datum_value`
- `tx_out_reference_script`
- `spending_plutus_script_v2`
- `spending_tx_in_reference`
- `spending_reference_tx_in_inline_datum_present`
- `spending_reference_tx_in_redeemer_value`
- `read_only_tx_in_reference`
- `mint_plutus_script_v2`
- `mint`
- `minting_script`
- `mint_tx_in_reference`
- `mint_redeemer_value`
- `mint_reference_tx_in_redeemer_value`
- `required_signer_hash`
- `tx_in_collateral`
- `change_address`
- `change_output_datum`
- `invalid_before`
- `invalid_hereafter`
- `metadata_value`
- `signing_key`
- [To be implemented] `protocolParams`

#### 2.2 Logic APIs for package integration

- [To be implemented] `updateRedeemer` - Update `SPEND` and `MINT` exUnits
- [To be implemented] `reset` - reseting the whole `MeshTxBuilderCore`
- [To be implemented] `emptyTxBuilderBody` - reseting the body object
- `complete_sync` - determine whether using customizedTx, if not queue all last items then serialize the tx
- `complete_signing` - adding signing keys and return `txHex`
- `serialize_tx_body` - take the tx object and serilized it to `txHex`
- `queue_input`
- `queue_mint`
- `queue_all_last_item`
- `add_all_signing_keys`
- `add_all_inputs`
- `add_all_outputs`
- `add_all_collaterals`
- `add_all_reference_inputs`
- `add_all_mints`
- [To be implemented] `castRawDataToJsonString` - turn object to string, keep string as string
- [To be implemented] `castDataToPlutusData`

### 3. Utils APIs

- `apply_params_to_script` - handle Aiken parameterized scripts
- [To be implemented] `meshToPlutusData`
- [To be implemented] `jsonToPlutusData`
- [To be implemented] `cborToPlutusData`
- A bunch of other methods that need CSL to serialized / deserialized
