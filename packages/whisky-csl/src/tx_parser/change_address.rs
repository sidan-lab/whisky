use whisky_common::WError;

use super::CSLParser;

impl CSLParser {
    pub fn get_change_address(&self) -> &String {
        &self.tx_body.change_address
    }

    pub(super) fn extract_change_address(&mut self) -> Result<(), WError> {
        let outputs = self.csl_tx_body.outputs();
        let outputs_len = outputs.len();
        if outputs_len > 0 {
            let change_address = outputs
                .get(outputs_len - 1)
                .address()
                .to_bech32(None)
                .map_err(|e| {
                    WError::new(
                        "extract_change_address",
                        &format!("Failed to convert change address to bech32: {:?}", e),
                    )
                })?;
            self.tx_body.change_address = change_address;
        }
        Ok(())
    }
}
