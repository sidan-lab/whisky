use std::collections::HashSet;
use whisky_common::WError;

use super::CSLParser;

impl CSLParser {
    pub fn get_required_signatures(&self) -> &Vec<String> {
        &self.tx_body.required_signatures
    }

    pub(super) fn extract_required_signatures(
        &mut self,
    ) -> Result<(), WError> {
        let mut required_signatures = HashSet::new();
        let required_signers = self.csl_tx_body.required_signers();
        if let Some(required_signers) = required_signers {
            for signer in &required_signers {
                required_signatures.insert(signer.to_hex());
            }
        }
        self.tx_body.required_signatures = required_signatures.into_iter().collect();
        Ok(())
    }
}
