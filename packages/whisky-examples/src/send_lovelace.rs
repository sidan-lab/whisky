use whisky::{
    builder::MeshTxBuilder,
    csl::JsError,
    model::{Asset, UTxO},
};

pub fn send_lovelace(
    recipient_address: &str,
    my_address: &str,
    inputs: &[UTxO],
) -> Result<String, JsError> {
    let mut tx_builder = MeshTxBuilder::new_core();
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

// #[test]
// fn test_send_lovelace() {
//     let recipient_address = "";
// }
