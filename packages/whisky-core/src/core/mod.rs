mod tx_builder_core;
pub use tx_builder_core::*;

/// ## Transaction building method
///
/// Serialize the transaction body
///
/// ### Arguments
///
/// * `tx_builder_body` - The transaction builder body information
/// * `params` - Optional protocol parameters, default as Cardano mainnet configuration
///
/// ### Returns
///
/// * `String` - the built transaction hex
pub fn serialize_tx_body(
    tx_builder_body: TxBuilderCore,
    params: Option<Protocol>,
) -> Result<String, WError> {
    if tx_builder_body.change_address.is_empty() {
        return Err(WError::from_str("change address cannot be empty"));
    }
    let mut whisky_csl = WhiskyCSL::new(params);

    TxBuilderCore::add_all_inputs(&mut whisky_csl, tx_builder_body.inputs.clone())?;
    TxBuilderCore::add_all_outputs(&mut whisky_csl, tx_builder_body.outputs.clone())?;
    TxBuilderCore::add_all_collaterals(&mut whisky_csl, tx_builder_body.collaterals.clone())?;
    TxBuilderCore::add_all_reference_inputs(
        &mut whisky_csl,
        tx_builder_body.reference_inputs.clone(),
    )?;
    TxBuilderCore::add_all_withdrawals(&mut whisky_csl, tx_builder_body.withdrawals.clone())?;
    TxBuilderCore::add_all_mints(&mut whisky_csl, tx_builder_body.mints.clone())?;
    TxBuilderCore::add_all_certificates(&mut whisky_csl, tx_builder_body.certificates.clone())?;
    TxBuilderCore::add_all_votes(&mut whisky_csl, tx_builder_body.votes.clone())?;
    TxBuilderCore::add_validity_range(&mut whisky_csl, tx_builder_body.validity_range.clone());
    TxBuilderCore::add_all_required_signature(
        &mut whisky_csl,
        &tx_builder_body
            .required_signatures
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>(),
    )?;
    TxBuilderCore::add_all_metadata(&mut whisky_csl, tx_builder_body.metadata.clone())?;

    match tx_builder_body.network {
        Some(current_network) => whisky_csl.add_script_hash(current_network)?,
        None => whisky_csl.add_script_hash(Network::Mainnet)?,
    };
    // if self.tx_builder_body.change_address != "" {
    //     let collateral_inputs = self.tx_builder_body.collaterals.clone();
    //     let collateral_vec: Vec<u64> = collateral_inputs
    //         .into_iter()
    //         .map(|pub_key_tx_in| {
    //             let assets = pub_key_tx_in.tx_in.amount.unwrap();
    //             let lovelace = assets
    //                 .into_iter()
    //                 .find(|asset| asset.unit == "lovelace")
    //                 .unwrap();
    //             lovelace.quantity.parse::<u64>().unwrap()
    //         })
    //         .collect();
    //     let total_collateral: u64 = collateral_vec.into_iter().sum();

    //     let collateral_estimate: u64 = (150
    //         * self
    //             .tx_builder
    //             .min_fee()
    //             .unwrap()
    //             .checked_add(&to_bignum(10000))
    //             .unwrap()
    //             .to_string()
    //             .parse::<u64>()
    //             .unwrap())
    //         / 100;

    //     let mut collateral_return_needed = false;
    // if (total_collateral - collateral_estimate) > 0 {
    // let collateral_estimate_output = csl::TransactionOutput::new(
    //     &csl::address::Address::from_bech32(&self.tx_builder_body.change_address)
    //         .unwrap(),
    //     &csl::utils::Value::new(&to_bignum(collateral_estimate)),
    // );

    // let min_ada = csl::utils::min_ada_for_output(
    //     &collateral_estimate_output,
    //     &csl::DataCost::new_coins_per_byte(&to_bignum(4310)),
    // )
    // .unwrap()
    // .to_string()
    // .parse::<u64>()
    // .unwrap();

    // if total_collateral - collateral_estimate > min_ada {
    //     self.tx_builder
    //         .set_collateral_return(&csl::TransactionOutput::new(
    //             &csl::address::Address::from_bech32(
    //                 &self.tx_builder_body.change_address,
    //             )
    //             .unwrap(),
    //             &csl::utils::Value::new(&to_bignum(total_collateral)),
    //         ));

    //     self.tx_builder
    //         .set_total_collateral(&to_bignum(total_collateral));

    //     collateral_return_needed = true;
    // }
    // }
    // self.add_change(self.tx_builder_body.change_address.clone());
    // if collateral_return_needed {
    //     self.add_collateral_return(self.tx_builder_body.change_address.clone());
    // }
    // }
    if tx_builder_body.fee.is_some() {
        TxBuilderCore::set_fee(&mut whisky_csl, tx_builder_body.fee.unwrap());
    }
    whisky_csl.add_change(
        tx_builder_body.change_address.clone(),
        tx_builder_body.change_datum.clone(),
    )?;
    whisky_csl.build_tx()
}
