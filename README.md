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

### 1. APIs on core CSL logics

| Type               | Name                       | Description                                                                            |
| ------------------ | -------------------------- | -------------------------------------------------------------------------------------- |
| `MeshCSL` Property | `tx_hex`                   | This is the transaction hex, used for storing the transaction in hexadecimal format.   |
| `MeshCSL` Property | `tx_builder`               | This is the transaction builder, used for building transactions.                       |
| `MeshCSL` Property | `tx_inputs_builder`        | This is the transaction inputs builder, used for building the inputs of a transaction. |
| `MeshCSL` Method   | `add_tx_in`                | This method is used to add a transaction input.                                        |
| `MeshCSL` Method   | `add_script_tx_in`         | This method is used to add a script transaction input.                                 |
| `MeshCSL` Method   | `add_output`               | This method is used to add an output.                                                  |
| `MeshCSL` Method   | `add_collateral`           | This method is used to add collateral.                                                 |
| `MeshCSL` Method   | `add_reference_input`      | This method is used to add a reference input.                                          |
| `MeshCSL` Method   | `add_plutus_mint`          | This method is used to add a Plutus mint.                                              |
| `MeshCSL` Method   | `add_native_mint`          | This method is used to add a native mint.                                              |
| `MeshCSL` Method   | `add_invalid_before`       | This method is used to add an invalid before condition.                                |
| `MeshCSL` Method   | `add_invalid_hereafter`    | This method is used to add an invalid hereafter condition.                             |
| `MeshCSL` Method   | `add_change`               | This method is used to add change.                                                     |
| `MeshCSL` Method   | `add_signing_keys`         | This method is used to add signing keys.                                               |
| `MeshCSL` Method   | `add_required_signature`   | This method is used to add a required signature.                                       |
| `MeshCSL` Method   | `add_metadata`             | This method is used to add metadata.                                                   |
| `MeshCSL` Method   | `add_script_hash`          | This method is used to add a script hash.                                              |
| `MeshCSL` Method   | `build_tx`                 | This method is used to build the transaction to hex in CSL.                            |
| Util Function      | `apply_params_to_script`   | To compile Aiken parameterized scripts.                                                |
| Util Function      | `script_to_address`        | To obtain script address with provided hash and stake cred                             |
| Util Function      | `serialize_bech32_address` | To obtain pub key hash, script hash and stake cred from bech32 address                 |
| Util Function      | `get_v2_script_hash`       | To obtain script hash from script cbor                                                 |
| Util Function      | `calculate_tx_hash`        | To calculate the transaction hash from signed or unsigned hex                          |
| Util Function      | `sign_transaction`         | To add private key signature to current tx_hex                                         |
| Util Function      | `meshToPlutusData`         | To be added -To serialize plutus data from mesh data type                              |
| Util Function      | `jsonToPlutusData`         | To be added -To serialize plutus data from json                                        |
| Util Function      | `cborToPlutusData`         | To be added -To serialize plutus data from cbor                                        |
| Util Function      | To be added                | To be added - A bunch of other methods that need CSL to serialized / deserialized      |

### 2. `MeshTxBuilderCore`

#### 2.1 User-facing wrapper APIs

| Type                                   | Method                                          | Description                                                                             |
| -------------------------------------- | ----------------------------------------------- | --------------------------------------------------------------------------------------- |
| `MeshTxBuilderCore` User-facing Method | `tx_in`                                         | Sets the input for the transaction.                                                     |
| `MeshTxBuilderCore` User-facing Method | `tx_in_script`                                  | Sets the script for the transaction input.                                              |
| `MeshTxBuilderCore` User-facing Method | `tx_in_datum_value`                             | Sets the input datum for the transaction input.                                         |
| `MeshTxBuilderCore` User-facing Method | `tx_in_inline_datum_present`                    | Indicates that the input UTxO has inlined datum.                                        |
| `MeshTxBuilderCore` User-facing Method | `tx_in_redeemer_value`                          | Sets the redeemer for the reference input to be spent in the same transaction.          |
| `MeshTxBuilderCore` User-facing Method | `tx_out`                                        | Sets the output for the transaction.                                                    |
| `MeshTxBuilderCore` User-facing Method | `tx_out_datum_hash_value`                       | Sets the output datum hash for the transaction.                                         |
| `MeshTxBuilderCore` User-facing Method | `tx_out_inline_datum_value`                     | Sets the output inline datum for the transaction.                                       |
| `MeshTxBuilderCore` User-facing Method | `tx_out_reference_script`                       | Sets the reference script to be attached with the output.                               |
| `MeshTxBuilderCore` User-facing Method | `spending_plutus_script_v2`                     | Indicates that it is currently using V2 Plutus spending scripts.                        |
| `MeshTxBuilderCore` User-facing Method | `spending_tx_in_reference`                      | Sets the reference input where it would also be spent in the transaction.               |
| `MeshTxBuilderCore` User-facing Method | `spending_reference_tx_in_inline_datum_present` | Indicates that the reference input has inline datum.                                    |
| `MeshTxBuilderCore` User-facing Method | `spending_reference_tx_in_redeemer_value`       | Sets the redeemer for the reference input to be spent in the same transaction.          |
| `MeshTxBuilderCore` User-facing Method | `read_only_tx_in_reference`                     | Specifies a read-only reference input.                                                  |
| `MeshTxBuilderCore` User-facing Method | `mint_plutus_script_v2`                         | Indicates that it is currently using V2 Plutus minting scripts.                         |
| `MeshTxBuilderCore` User-facing Method | `mint`                                          | Sets the minting value of the transaction.                                              |
| `MeshTxBuilderCore` User-facing Method | `minting_script`                                | Sets the minting script of the current mint.                                            |
| `MeshTxBuilderCore` User-facing Method | `mint_tx_in_reference`                          | Uses reference script for minting.                                                      |
| `MeshTxBuilderCore` User-facing Method | `mint_redeemer_value`                           | Sets the redeemer for the reference input to be spent in the same transaction.          |
| `MeshTxBuilderCore` User-facing Method | `mint_reference_tx_in_redeemer_value`           | Sets the redeemer for minting.                                                          |
| `MeshTxBuilderCore` User-facing Method | `required_signer_hash`                          | Sets the required signer of the transaction.                                            |
| `MeshTxBuilderCore` User-facing Method | `tx_in_collateral`                              | Sets the collateral UTxO for the transaction.                                           |
| `MeshTxBuilderCore` User-facing Method | `change_address`                                | Configures the address to accept change UTxO.                                           |
| `MeshTxBuilderCore` User-facing Method | `change_output_datum`                           | [To be implemented]                                                                     |
| `MeshTxBuilderCore` User-facing Method | `invalid_before`                                | Sets the transaction valid interval to be valid only after the slot.                    |
| `MeshTxBuilderCore` User-facing Method | `invalid_hereafter`                             | Sets the transaction valid interval to be valid only before the slot.                   |
| `MeshTxBuilderCore` User-facing Method | `metadata_value`                                | Adds metadata to the transaction.                                                       |
| `MeshTxBuilderCore` User-facing Method | `signing_key`                                   | Signs the transaction with the private key.                                             |
| `MeshTxBuilderCore` User-facing Method | `tx_hex`                                        | Obtain the current transaction hex from build                                           |
| `MeshTxBuilderCore` User-facing Method | `reset`                                         | [To be implemented] Resetting the whole MeshTxBuilderCore                               |
| `MeshTxBuilderCore` User-facing Method | `emptyTxBuilderBody`                            | [To be implemented] Resetting the body object                                           |
| `MeshTxBuilderCore` User-facing Method | `complete_sync`                                 | Determine whether using customizedTx, if not queue all last items then serialize the tx |
| `MeshTxBuilderCore` User-facing Method | `complete_signing`                              | Adding signing keys and return txHex                                                    |
| `MeshTxBuilderCore` Internal Method    | `serialize_tx_body`                             | Take the tx object and serialized it to txHex                                           |
| `MeshTxBuilderCore` Internal Method    | `updateRedeemer`                                | [To be implemented] Update SPEND and MINT exUnits                                       |
| `MeshTxBuilderCore` Internal Method    | `queue_input`                                   | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `queue_mint`                                    | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `queue_all_last_item`                           | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `add_all_signing_keys`                          | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `add_all_inputs`                                | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `add_all_outputs`                               | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `add_all_collaterals`                           | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `add_all_reference_inputs`                      | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `add_all_mints`                                 | [To be implemented]                                                                     |
| `MeshTxBuilderCore` Internal Method    | `castRawDataToJsonString`                       | [To be implemented] Turn object to string, keep string as string                        |
| `MeshTxBuilderCore` Internal Method    | `castDataToPlutusData`                          | [To be implemented]                                                                     |
