use pallas::ledger::primitives::Fragment;
use whisky_common::{TxBuilderBody, WError};

use crate::{
    converter::convert_inputs,
    wrapper::{
        transaction_body::{Transaction, TransactionBody},
        witness_set::witness_set::WitnessSet,
    },
};

#[derive(Clone, Debug)]
pub struct CorePallas {
    pub tx_builder_body: TxBuilderBody,
    pub tx_evaluation_multiplier_percentage: u64,
    pub tx_hex: String,
}

impl CorePallas {
    pub fn build_tx(&mut self) -> Result<String, WError> {
        let tx_body = TransactionBody::new(
            convert_inputs(&self.tx_builder_body.inputs)?,
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
            .encode_fragment()
            .map_err(|e| {
                WError::new(
                    "WhiskyPallas - Building transaction:",
                    &format!("Encoding failed at Transaction: {}", e.to_string()),
                )
            })?;
        Ok(hex::encode(transaction_bytes))
    }
}
