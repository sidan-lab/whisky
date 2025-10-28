use cardano_serialization_lib as csl;
use std::collections::HashSet;

use whisky_common::WError;

use crate::{blake2b256, CSLParser};

impl CSLParser {
    pub fn check_all_required_signers(&mut self) -> Result<bool, WError> {
        self.extract_required_signatures()
            .map_err(WError::from_err(
                "CSLParser - check_all_required_signers - required_signatures",
            ))?;

        let signers = &self.tx_body.required_signatures;
        let mut signer_set: HashSet<String> = HashSet::new();

        let fixed_tx = csl::FixedTransaction::from_hex(&self.tx_hex).map_err(WError::from_err(
            "CSLParser - check_all_required_signers - from_hex",
        ))?;
        for signer in signers {
            signer_set.insert(signer.clone());
        }

        let csl_vkeys = self
            .csl_witness_set
            .vkeys()
            .unwrap_or(csl::Vkeywitnesses::new());

        for i in 0..csl_vkeys.len() {
            let vkey_witness = csl_vkeys.get(i);
            let pub_key = vkey_witness.vkey().public_key();
            if !pub_key.verify(&blake2b256(&fixed_tx.raw_body()), &vkey_witness.signature()) {
                return Ok(false);
            } else {
                signer_set.remove(&pub_key.hash().to_hex());
            };
        }
        Ok(signer_set.is_empty())
    }
}
