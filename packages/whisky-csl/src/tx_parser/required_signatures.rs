use std::collections::HashSet;

use crate::{blake2b256, csl};
use whisky_common::WError;

use super::TxParser;

impl TxParser {
    pub fn required_signatures(&mut self) -> Result<Vec<String>, WError> {
        let mut required_signer_hashes = vec![];
        let required_signers_key_hashes = self
            .csl_tx_body
            .required_signers()
            .unwrap_or(csl::Ed25519KeyHashes::new());
        for i in 0..required_signers_key_hashes.len() {
            let signer = required_signers_key_hashes.get(i);
            required_signer_hashes.push(signer.to_hex())
        }
        self.tx_body.required_signatures = required_signer_hashes.clone();
        Ok(required_signer_hashes)
    }

    pub fn check_all_required_signers(&self) -> bool {
        let signers = &self.tx_body.required_signatures;
        let mut signer_set: HashSet<String> = HashSet::new();
        let fixed_tx = csl::FixedTransaction::from_hex(&self.tx_hex).unwrap();
        for signer in signers {
            signer_set.insert(signer.clone());
        }
        // for i in 0..signers.len() {
        //     signer_set.insert(signers.get(i));
        // }
        let csl_vkeys = self
            .csl_witness_set
            .vkeys()
            .unwrap_or(csl::Vkeywitnesses::new());
        for i in 0..csl_vkeys.len() {
            let vkey_witness = csl_vkeys.get(i);
            let pub_key = vkey_witness.vkey().public_key();
            if !pub_key.verify(&blake2b256(&fixed_tx.raw_body()), &vkey_witness.signature()) {
                return false;
            } else {
                signer_set.remove(&pub_key.hash().to_hex());
            };
        }
        signer_set.is_empty()
    }
}
