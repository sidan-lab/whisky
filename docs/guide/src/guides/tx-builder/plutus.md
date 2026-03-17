# Plutus Script Transactions

This chapter covers spending from Plutus script addresses — unlocking funds guarded by validators.

## Unlock Funds from a Script

To spend a UTxO locked at a Plutus script address, you must provide the script, datum, and redeemer:

```rust,ignore
use whisky::*;

pub async fn unlock_fund(
    script_utxo: &UTxO,
    redeemer: &str,
    script: &ProvidedScriptSource,
    my_address: &str,
    inputs: &[UTxO],
    collateral: &UTxO,
) -> Result<String, WError> {
    let mut tx_builder = TxBuilder::new_core();
    let pub_key_hash = deserialize_address(my_address)?.pub_key_hash;

    tx_builder
        .spending_plutus_script_v3()
        .tx_in(
            &script_utxo.input.tx_hash,
            script_utxo.input.output_index,
            &script_utxo.output.amount,
            &script_utxo.output.address,
        )
        .tx_in_inline_datum_present()
        .tx_in_redeemer_value(&WRedeemer {
            data: WData::JSON(redeemer.to_string()),
            ex_units: Budget { mem: 0, steps: 0 },
        })
        .tx_in_script(&script.script_cbor)
        .change_address(my_address)
        .required_signer_hash(&pub_key_hash)
        .tx_in_collateral(
            &collateral.input.tx_hash,
            collateral.input.output_index,
            &collateral.output.amount,
            &collateral.output.address,
        )
        .input_for_evaluation(script_utxo)
        .select_utxos_from(inputs, 5000000)
        .complete(None)
        .await?;

    Ok(tx_builder.tx_hex())
}
```

## The Script Spending Pattern

Every Plutus spend follows the same sequence:

1. **Declare the script version**: `.spending_plutus_script_v3()` (or `_v2()`, `_v1()`)
2. **Add the script input**: `.tx_in(hash, index, amount, address)`
3. **Provide the datum**: `.tx_in_inline_datum_present()` or `.tx_in_datum_value(&datum)`
4. **Provide the redeemer**: `.tx_in_redeemer_value(&redeemer)`
5. **Provide the script**: `.tx_in_script(&script_cbor)` or use a reference script

## Datum Handling

Whisky supports two datum modes:

```rust,ignore
// The UTxO already has an inline datum — just declare it's present
.tx_in_inline_datum_present()

// Provide the datum value explicitly (e.g., when only the datum hash is on-chain)
.tx_in_datum_value(&WData::JSON(datum_json.to_string()))
```

## Redeemer Values

Redeemers are Plutus data values passed to the validator. Set execution units to `0` and let the evaluator calculate them:

```rust,ignore
.tx_in_redeemer_value(&WRedeemer {
    data: WData::JSON(r#"{"constructor": 0, "fields": []}"#.to_string()),
    ex_units: Budget { mem: 0, steps: 0 },
})
```

When you call `.complete(None).await`, the built-in offline evaluator runs the script and fills in the actual execution units.

## Script Sources

You can provide the script in two ways:

```rust,ignore
// Embed the script CBOR directly in the transaction
.tx_in_script(&script_cbor)

// Reference a script already on-chain (more efficient — saves tx size)
.spending_tx_in_reference(tx_hash, tx_index, script_hash, script_size)
```

## Collateral

Plutus transactions require collateral — a UTxO that gets consumed if the script fails:

```rust,ignore
.tx_in_collateral(
    &collateral.input.tx_hash,
    collateral.input.output_index,
    &collateral.output.amount,
    &collateral.output.address,
)
```

You can also set a specific total collateral amount:

```rust,ignore
.tx_in_collateral(tx_hash, tx_index, amount, address)
.set_total_collateral("5000000")
```

## Input for Evaluation

When using offline evaluation, provide the script UTxO context so the evaluator can resolve inputs:

```rust,ignore
.input_for_evaluation(script_utxo)
```

This is required for the offline evaluator to correctly simulate script execution.
