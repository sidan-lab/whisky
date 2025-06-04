use cardano_serialization_lib as csl;
use whisky_common::{TxIn, WError};

use super::{utxo_converter::utxo_to_tx_in, CSLParser};

impl CSLParser {
    pub fn get_inputs(&self) -> &Vec<TxIn> {
        &self.tx_body.inputs
    }

    pub fn extract_all_required_inputs(tx_hex: &str) -> Result<Vec<String>, WError> {
        let csl_tx = csl::FixedTransaction::from_hex(tx_hex).map_err(|e| {
            WError::new(
                "CSLParser - extract_all_required_inputs",
                &format!("Failed to parse transaction hex: {:?}", e),
            )
        })?;
        let inputs = csl_tx.body().inputs();
        let mut required_inputs = Vec::new();
        for input in inputs.into_iter() {
            required_inputs.push(format!(
                "{}#{}",
                input.transaction_id().to_hex(),
                input.index()
            ));
        }
        let collateral_inputs = csl_tx.body().collateral();
        if let Some(collateral_inputs) = collateral_inputs {
            for input in collateral_inputs.into_iter() {
                required_inputs.push(format!(
                    "{}#{}",
                    input.transaction_id().to_hex(),
                    input.index()
                ));
            }
        }
        let reference_inputs = csl_tx.body().reference_inputs();
        if let Some(reference_inputs) = reference_inputs {
            for input in reference_inputs.into_iter() {
                required_inputs.push(format!(
                    "{}#{}",
                    input.transaction_id().to_hex(),
                    input.index()
                ));
            }
        }
        Ok(required_inputs)
    }

    pub(super) fn extract_inputs(&mut self) -> Result<(), WError> {
        let inputs = self.csl_tx_body.inputs();
        for (index, input) in inputs.into_iter().enumerate() {
            let tx_in = utxo_to_tx_in(&input, &self.context, index)?;
            self.tx_body.inputs.push(tx_in);
        }
        Ok(())
    }
}
