use whisky::{
    builder::{MeshTxBuilder, WData},
    csl::JsError,
    model::{Asset, UTxO},
};

pub fn lock_fund(
    script_address: &str,
    datum: &str,
    my_address: &str,
    inputs: &[UTxO],
) -> Result<String, JsError> {
    let mut mesh = MeshTxBuilder::new_core();
    mesh.tx_out(
        script_address,
        &[Asset::new_from_str("lovelace", "1000000")],
    )
    .tx_out_inline_datum_value(&WData::JSON(datum.to_string())) // JSON string datum
    // .tx_out_datum_hash_value(WData::JSON(datum.to_string())) // Datum hash
    .change_address(my_address)
    .select_utxos_from(inputs, 5000000)
    .complete_sync(None)?;

    Ok(mesh.tx_hex())
}
