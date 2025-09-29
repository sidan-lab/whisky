use whisky_common::{PubKeyTxIn, WError};

use super::{utxo_converter::utxo_to_pub_key_tx_in, CSLParser};

impl CSLParser {
    pub fn get_collaterals(&self) -> &Vec<PubKeyTxIn> {
        &self.tx_body.collaterals
    }

    pub(super) fn extract_collaterals(&mut self) -> Result<(), WError> {
        let collateral_inputs = self.csl_tx_body.collateral();
        if let Some(collateral_inputs) = collateral_inputs {
            for input in &collateral_inputs {
                let tx_in = utxo_to_pub_key_tx_in(&input, &self.context)?;
                self.tx_body.collaterals.push(tx_in);
            }
        }
        Ok(())
    }
}
