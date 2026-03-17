# Minting

This chapter covers minting and burning native tokens using Plutus minting policies.

## Mint Tokens

Mint tokens using a Plutus V3 minting policy:

```rust,ignore
use whisky::*;

pub async fn mint_tokens(
    to_mint_asset: &Asset,
    redeemer: &str,
    script: &ProvidedScriptSource,
    my_address: &str,
    inputs: &[UTxO],
    collateral: &UTxO,
) -> Result<String, WError> {
    let mut tx_builder = TxBuilder::new_core();

    tx_builder
        .mint_plutus_script_v3()
        .mint(
            to_mint_asset.quantity_i128(),
            &to_mint_asset.policy(),
            &to_mint_asset.name(),
        )
        .minting_script(&script.script_cbor)
        .mint_redeemer_value(&WRedeemer {
            data: WData::JSON(redeemer.to_string()),
            ex_units: Budget { mem: 0, steps: 0 },
        })
        .change_address(my_address)
        .tx_in_collateral(
            &collateral.input.tx_hash,
            collateral.input.output_index,
            &collateral.output.amount,
            &collateral.output.address,
        )
        .select_utxos_from(inputs, 5000000)
        .complete(None)
        .await?;

    Ok(tx_builder.tx_hex())
}
```

## The Minting Pattern

Every Plutus mint follows this sequence:

1. **Declare the script version**: `.mint_plutus_script_v3()` (or `_v2()`, `_v1()`)
2. **Specify the mint**: `.mint(quantity, policy_id, asset_name)`
3. **Provide the script**: `.minting_script(&script_cbor)` or use a reference script
4. **Provide the redeemer**: `.mint_redeemer_value(&redeemer)`

## Reference Script Minting

Instead of embedding the script, reference one already on-chain:

```rust,ignore
tx_builder
    .mint_plutus_script_v2()
    .mint(1, policy_id, token_name_hex)
    .mint_tx_in_reference(
        "reference_tx_hash",
        0,              // reference tx index
        policy_id,      // script hash to validate against
        100,            // script size in bytes
    )
    .mint_redeemer_value(&WRedeemer {
        data: WData::JSON(r#"{"constructor": 0, "fields": []}"#.to_string()),
        ex_units: Budget { mem: 3386819, steps: 1048170931 },
    })
```

## Burning Tokens

To burn tokens, use a negative quantity:

```rust,ignore
tx_builder
    .mint_plutus_script_v3()
    .mint(-1, policy_id, token_name_hex)  // Negative = burn
    .minting_script(&script.script_cbor)
    .mint_redeemer_value(&redeemer)
```

## Multiple Mints in One Transaction

You can mint tokens from multiple policies in a single transaction by chaining mint blocks:

```rust,ignore
use whisky::*;

tx_builder
    // First mint
    .mint_plutus_script_v2()
    .mint(
        to_mint_asset_1.quantity_i128(),
        &to_mint_asset_1.policy(),
        &to_mint_asset_1.name(),
    )
    .mint_redeemer_value(&WRedeemer {
        data: WData::JSON(redeemer_1.to_string()),
        ex_units: Budget { mem: 0, steps: 0 },
    })
    .minting_script(&script_1.script_cbor)
    // Second mint
    .mint_plutus_script_v2()
    .mint(
        to_mint_asset_2.quantity_i128(),
        &to_mint_asset_2.policy(),
        &to_mint_asset_2.name(),
    )
    .mint_redeemer_value(&WRedeemer {
        data: WData::JSON(redeemer_2.to_string()),
        ex_units: Budget { mem: 0, steps: 0 },
    })
    .minting_script(&script_2.script_cbor)
    .change_address(my_address)
    .tx_in_collateral(/* ... */)
    .select_utxos_from(inputs, 5000000)
    .complete(None)
    .await?;
```

Each mint block starts with `.mint_plutus_script_vN()` and is independent — you can mix V1, V2, and V3 policies in the same transaction.

## Complex Transaction: Spend + Mint

You can combine script spending and minting in one transaction:

```rust,ignore
tx_builder
    // Script spend
    .spending_plutus_script_v2()
    .tx_in(/* script UTxO */)
    .tx_in_inline_datum_present()
    .tx_in_redeemer_value(&spend_redeemer)
    .tx_in_script(&spend_script_cbor)
    // Mint
    .mint_plutus_script_v2()
    .mint(1, policy_id, token_name)
    .mint_redeemer_value(&mint_redeemer)
    .minting_script(&mint_script_cbor)
    // Finalize
    .change_address(my_address)
    .tx_in_collateral(/* ... */)
    .input_for_evaluation(script_utxo)
    .select_utxos_from(inputs, 5000000)
    .complete(None)
    .await?;
```
