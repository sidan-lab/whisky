use cardano_serialization_lib as csl;
use whisky_common::WError;

use super::auxiliary_data::proto_to_auxiliary_data;
use super::body::proto_to_transaction_body;
use super::witness_set::proto_to_transaction_witness_set;
use crate::tx_prototype::types::*;

/// Convert TransactionPrototype to CSL Transaction
pub fn proto_to_csl_transaction(tx: &TransactionPrototype) -> Result<csl::Transaction, WError> {
    let body = proto_to_transaction_body(&tx.body)?;
    let witness_set = proto_to_transaction_witness_set(&tx.witness_set)?;
    let auxiliary_data = tx
        .auxiliary_data
        .as_ref()
        .map(|aux| proto_to_auxiliary_data(aux))
        .transpose()?;

    Ok(csl::Transaction::new(&body, &witness_set, auxiliary_data))
}

/// Convert TransactionPrototype to hex string
pub fn proto_to_transaction_hex(tx: &TransactionPrototype) -> Result<String, WError> {
    let csl_tx = proto_to_csl_transaction(tx)?;
    Ok(csl_tx.to_hex())
}

/// Convert TransactionPrototype to CBOR bytes
pub fn proto_to_transaction_bytes(tx: &TransactionPrototype) -> Result<Vec<u8>, WError> {
    let csl_tx = proto_to_csl_transaction(tx)?;
    Ok(csl_tx.to_bytes())
}

impl TransactionPrototype {
    /// Convert this TransactionPrototype to CSL Transaction
    pub fn to_csl(&self) -> Result<csl::Transaction, WError> {
        proto_to_csl_transaction(self)
    }

    /// Convert this TransactionPrototype to hex string
    pub fn to_hex(&self) -> Result<String, WError> {
        proto_to_transaction_hex(self)
    }

    /// Convert this TransactionPrototype to CBOR bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>, WError> {
        proto_to_transaction_bytes(self)
    }
}
