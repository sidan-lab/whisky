use whisky_common::{Datum, WError};

use super::CSLParser;

impl CSLParser {
    pub fn get_change_datum(&self) -> &Option<Datum> {
        &self.tx_body.change_datum
    }

    pub(super) fn extract_change_datum(&mut self) -> Result<(), WError> {
        let outputs = self.csl_tx_body.outputs();
        let outputs_len = outputs.len();
        if outputs_len > 0 {
            let change_output = outputs.get(outputs_len - 1);
            let change_datum = change_output.plutus_data();
            if let Some(change_datum) = change_datum {
                self.tx_body.change_datum = Some(Datum::Inline(change_datum.to_hex()));
            } else if let Some(data_hash) = change_output.data_hash() {
                self.tx_body.change_datum = Some(Datum::Hash(data_hash.to_hex()));
            }
        }
        Ok(())
    }
}
