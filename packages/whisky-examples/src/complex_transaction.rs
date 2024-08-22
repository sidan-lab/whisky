use whisky::{
    builder::{MeshTxBuilder, WData, WRedeemer},
    csl::JsError,
    model::{Asset, Budget, ProvidedScriptSource, UTxO},
};

pub struct UnlockUtxo {
    pub script_utxo: UTxO,
    pub redeemer: String,
    pub script: ProvidedScriptSource,
}

pub struct MintToken {
    pub to_mint_asset: Asset,
    pub redeemer: String,
    pub script: ProvidedScriptSource,
}

pub async fn complex_transaction(
    to_unlock: &UnlockUtxo,
    to_mint_1: &MintToken,
    to_mint_2: &MintToken,
    my_address: &str,
    inputs: &[UTxO],
    collateral: &UTxO,
) -> Result<String, JsError> {
    let UnlockUtxo {
        script_utxo,
        redeemer,
        script,
    } = to_unlock;

    let MintToken {
        to_mint_asset: to_mint_asset_1,
        redeemer: redeemer_1,
        script: script_1,
    } = to_mint_1;

    let MintToken {
        to_mint_asset: to_mint_asset_2,
        redeemer: redeemer_2,
        script: script_2,
    } = to_mint_2;

    let mut tx_builder = MeshTxBuilder::new_core();
    tx_builder
        .spending_plutus_script_v2()
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
