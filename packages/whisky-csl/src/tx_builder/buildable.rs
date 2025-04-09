use whisky_common::{TxBuildable, *};

use super::WhiskyCSL;

impl TxBuildable for WhiskyCSL {
    fn reset_builder(&mut self) -> &mut Self {
        self.core.reset_after_build();
        self
    }

    fn set_protocol_params(&mut self, protocol_params: Protocol) -> &mut Self {
        self.core.protocol_params = protocol_params.clone();
        self
    }

    fn set_tx_builder_body(&mut self, tx_builder_body: TxBuilderBody) -> &mut Self {
        self.tx_builder_body = tx_builder_body.clone();
        self
    }

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
    fn serialize_tx_body(&mut self) -> Result<String, WError> {
        if self.tx_builder_body.change_address.is_empty() {
            return Err(WError::new(
                "serialize_tx_body",
                "change address cannot be empty",
            ));
        }
        self.add_all_inputs()?
            .add_all_outputs()?
            .add_all_collaterals()?
            .add_all_reference_inputs()?
            .add_all_withdrawals()?
            .add_all_mints()?
            .add_all_certificates()?
            .add_all_votes()?
            .add_validity_range()?
            .add_all_required_signature()?
            .add_all_metadata()?
            .add_script_hash()?
            .set_fee_if_needed()?
            .add_change_utxo()?;

        self.core.build_tx(true)
    }

    fn unbalanced_serialize_tx_body(&mut self) -> Result<String, WError> {
        if self.tx_builder_body.change_address.is_empty() {
            return Err(WError::new(
                "serialize_tx_body",
                "change address cannot be empty",
            ));
        }
        self.add_all_inputs()?
            .add_all_outputs()?
            .add_all_collaterals()?
            .add_all_reference_inputs()?
            .add_all_withdrawals()?
            .add_all_mints()?
            .add_all_certificates()?
            .add_all_votes()?
            .add_validity_range()?
            .add_all_required_signature()?
            .add_all_metadata()?
            .add_script_hash()?
            .set_fee_if_needed()?;

        self.core.build_tx(false)
    }

    /// ## Transaction building method
    ///
    /// Complete the signing process
    ///
    /// ### Returns
    ///
    /// * `String` - The signed transaction in hex
    fn complete_signing(&mut self) -> Result<String, WError> {
        let signing_keys = self.tx_builder_body.signing_key.clone();
        self.add_all_signing_keys(
            &signing_keys
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )?;
        Ok(self.core.tx_hex.to_string())
    }

    fn tx_hex(&mut self) -> String {
        self.core.tx_hex.clone()
    }
}
