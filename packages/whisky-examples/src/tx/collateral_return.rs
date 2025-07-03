use whisky::*;

pub async fn collateral_return(
    my_address: &str,
    inputs: &[UTxO],
    collateral: &UTxO,
) -> Result<String, WError> {
    let mut tx_builder = TxBuilder::new_core();

    tx_builder
        .change_address(my_address)
        .tx_in_collateral(
            &collateral.input.tx_hash,
            collateral.input.output_index,
            &collateral.output.amount,
            &collateral.output.address,
        )
        .select_utxos_from(inputs, 5000000)
        .set_total_collateral("5000000")
        .complete(None)
        .await?;

    Ok(tx_builder.tx_hex())
}
