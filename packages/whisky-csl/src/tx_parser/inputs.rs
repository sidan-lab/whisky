use cardano_serialization_lib as csl;
use whisky_common::{TxIn, UtxoInput, WError};

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

    pub fn extract_all_required_utxo_input(tx_hex: &str) -> Result<Vec<UtxoInput>, WError> {
        let required_inputs = CSLParser::extract_all_required_inputs(tx_hex)?;
        required_inputs
            .iter()
            .map(|input| {
                let parts: Vec<&str> = input.split('#').collect();
                if parts.len() != 2 {
                    return Err(WError::new(
                        "CSLParser - extract_all_required_utxo_input",
                        &format!("Invalid input format: {}", input),
                    ));
                }
                let tx_hash = parts[0].to_string();
                let output_index: u32 = parts[1].parse().map_err(|_| {
                    WError::new(
                        "CSLParser - extract_all_required_utxo_input",
                        "Invalid output index",
                    )
                })?;
                Ok(UtxoInput {
                    tx_hash,
                    output_index,
                })
            })
            .collect()
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
