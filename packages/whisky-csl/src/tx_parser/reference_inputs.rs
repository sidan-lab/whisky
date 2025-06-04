use whisky_common::{RefTxIn, WError};

use super::{utxo_converter::utxo_to_ref_tx_in, CSLParser};

impl CSLParser {
    pub fn get_reference_inputs(&self) -> &Vec<RefTxIn> {
        &self.tx_body.reference_inputs
    }

    pub(super) fn extract_reference_inputs(&mut self) -> Result<(), WError> {
        let ref_inputs = self.csl_tx_body.reference_inputs();
        if let Some(ref_inputs) = ref_inputs {
            for input in &ref_inputs {
                let tx_in = utxo_to_ref_tx_in(&input, &self.context)?;
                self.tx_body.reference_inputs.push(tx_in);
            }
        }
        Ok(())
    }
}
