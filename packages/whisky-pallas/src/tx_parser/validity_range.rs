use pallas::ledger::primitives::conway::Tx;
use whisky_common::{ValidityRange, WError};

pub fn extract_validity_range(pallas_tx: &Tx) -> Result<ValidityRange, WError> {
    let body = &pallas_tx.transaction_body;
    Ok(ValidityRange {
        invalid_before: body.validity_interval_start,
        invalid_hereafter: body.ttl,
    })
}
