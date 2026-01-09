mod context;

use crate::wrapper::transaction_body::Transaction;
use whisky_common::{TxBuilderBody, UTxO, WError};

pub fn parse(tx_hex: &str, resolved_utxo: &[UTxO]) -> Result<TxBuilderBody, WError> {
    let bytes = hex::decode(tx_hex).map_err(|e| {
        WError::new(
            "WhiskyPallas - parse tx hex:",
            &format!("Hex decode error: {}", e),
        )
    })?;
    let pallas_tx = Transaction::decode_bytes(&bytes);
    Ok(TxBuilderBody::new())
}
