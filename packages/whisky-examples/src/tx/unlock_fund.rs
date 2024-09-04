use whisky::{
    builder::{MeshTxBuilder, WData, WRedeemer},
    core::utils::deserialize_bech32_address,
    csl::JsError,
    model::{Budget, ProvidedScriptSource, UTxO},
};

pub async fn unlock_fund(
    script_utxo: &UTxO,
    redeemer: &str,
    script: &ProvidedScriptSource,
    my_address: &str,
    inputs: &[UTxO],
    collateral: &UTxO,
) -> Result<String, JsError> {
    let mut tx_builder = MeshTxBuilder::new_core();
    let pub_key_hash = deserialize_bech32_address(my_address).get_pub_key_hash();

    tx_builder
        // .spending_plutus_script_v1()
        .spending_plutus_script_v2()
        // .spending_plutus_script_v3()
        .tx_in(
            &script_utxo.input.tx_hash,
            script_utxo.input.output_index,
            &script_utxo.output.amount,
            &script_utxo.output.address,
        )
        .tx_in_inline_datum_present()
        // .tx_in_datum_value(datum here) or provide datum value
        .tx_in_redeemer_value(&WRedeemer {
            data: WData::JSON(redeemer.to_string()),
            ex_units: Budget { mem: 0, steps: 0 },
        })
        .tx_in_script(&script.script_cbor)
        // .spending_tx_in_reference(tx_hash, tx_index, script_hash, script_size)
        .change_address(my_address)
        .required_signer_hash(&pub_key_hash) // Extra logic impl
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
