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
        // .mint_plutus_script_v1()
        // .mint_plutus_script_v2()
        .mint_plutus_script_v3()
        .mint(
            to_mint_asset.quantity_i128(),
            &to_mint_asset.policy(),
            &to_mint_asset.name(),
        )
        .minting_script(&script.script_cbor)
        // .mint_tx_in_reference(tx_hash, tx_index, script_hash, script_size) // For reference scripts
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
