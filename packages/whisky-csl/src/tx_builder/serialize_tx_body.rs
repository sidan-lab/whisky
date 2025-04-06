use whisky_common::{TxBuildable, *};

use super::WhiskyCSL;

impl TxBuildable for WhiskyCSL {
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

        self.core.build_tx()
    }
}
