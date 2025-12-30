use pallas::ledger::primitives::Fragment;
use whisky_common::TxBuilderBody;

use crate::wrapper::{
    transaction_body::{Transaction, TransactionBody},
    witness_set::witness_set::WitnessSet,
};

pub fn from_tx_builder_body(tx_builder_body: TxBuilderBody) -> Result<String, String> {
    let tx_body = TransactionBody::new(
        vec![],
        vec![],
        0,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )?;
    let witness_set = WitnessSet::new(None, None, None, None, None, None, None, None)?;
    let transaction_bytes = Transaction::new(tx_body, witness_set, true, None)?
        .inner
        .encode_fragment();
    match transaction_bytes {
        Ok(bytes) => Ok(hex::encode(bytes)),
        Err(e) => Err(format!("Encoding failed at Transaction: {}", e.to_string())),
    }
}

#[test]
fn test_from_tx_builder_body() {
    let tx_builder_body = TxBuilderBody::new();

    let result = from_tx_builder_body(tx_builder_body);
    assert!(result.is_ok());
    println!("Serialized transaction hex: {}", result.unwrap());
}
