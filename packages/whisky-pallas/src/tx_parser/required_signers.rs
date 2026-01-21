use std::collections::HashSet;

use pallas::ledger::primitives::conway::Tx;
use whisky_common::WError;

pub fn extract_required_signers(pallas_tx: &Tx) -> Result<Vec<String>, WError> {
    let mut required_signers = HashSet::new();
    match &pallas_tx.transaction_body.required_signers {
        Some(signers) => {
            for signer in signers {
                required_signers.insert(signer.to_string());
            }
            Ok(required_signers.into_iter().collect())
        }
        None => Ok(vec![]),
    }
}
