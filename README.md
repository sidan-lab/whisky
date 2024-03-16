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

- (Property) `txBuilder`
- `buildTx` - build the transaction to hex in CSL
- `makePlutusScriptSource`
- `addTxIn`
- `addScriptTxIn`
- `addOutput`
- `addCollateral`
- `addReferenceInput`
- `addPlutusMint`
- `addNativeMint`
- `addCostModels`
- `addChange`
- `addValidityRange`
- `addAllRequiredSignatures`
- `addAllMetadata`

### 2. `MeshTxBuilderCore`

#### 2.1 User-facing wrapper APIs

- `txIn`
- `txInScript`
- `txInDatumValue`
- `txInInlineDatumPresent`
- `txInRedeemerValue`
- `txOut`
- `txOutDatumHashValue`
- `txOutInlineDatumValue`
- `txOutReferenceScript`
- `spendingPlutusScriptV2`
- `spendingTxInReference`
- `spendingReferenceTxInInlineDatumPresent`
- `spendingReferenceTxInRedeemerValue`
- `readOnlyTxInReference`
- `mintPlutusScriptV2`
- `mint`
- `mintingScript`
- `mintTxInReference`
- `mintReferenceTxInRedeemerValue`
- `mintRedeemerValue`
- `requiredSignerHash`
- `txInCollateral`
- `changeAddress`
- `invalidBefore`
- `invalidHereafter`
- `metadataValue`
- `protocolParams`
- `signingKey`

#### 2.2 Logic APIs for package integration

- `updateRedeemer` - Update `SPEND` and `MINT` exUnits
- `reset` - reseting the whole `MeshTxBuilderCore`
- `emptyTxBuilderBody` - reseting the body object
- `completeSync` - determine whether using customizedTx, if not queue all last items then serialize the tx
- `completeSigning` - adding signing keys and return `txHex`
- `serializeTxBody` - take the tx object and serilized it to `txHex`
- `queueInput`
- `queueMint`
- `queueAllLastItem`
- `addAllInputs`
- `addAllOutputs`
- `addAllCollaterals`
- `addAllReferenceInputs`
- `addAllMints`
- `castRawDataToJsonString` - turn object to string, keep string as string
- `castDataToPlutusData`

### 3. Utils APIs

- `apply_params_to_script` - handle Aiken parameterized scripts
- `meshToPlutusData`
- `jsonToPlutusData`
- `cborToPlutusData`
- A bunch of other methods that need CSL to serialized / deserialized
