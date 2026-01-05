use whisky_common::{TxIn, WError};

use crate::wrapper::transaction_body::TransactionInput;

pub fn convert_inputs(inputs: &Vec<TxIn>) -> Result<Vec<TransactionInput>, WError> {
    inputs
        .into_iter()
        .map(|input| {
            let (tx_hash, tx_index) = match input {
                TxIn::PubKeyTxIn(pub_key_tx_in) => (
                    &pub_key_tx_in.tx_in.tx_hash,
                    pub_key_tx_in.tx_in.tx_index.into(),
                ),
                TxIn::SimpleScriptTxIn(simple_script_tx_in) => (
                    &simple_script_tx_in.tx_in.tx_hash,
                    simple_script_tx_in.tx_in.tx_index.into(),
                ),
                TxIn::ScriptTxIn(script_tx_in) => (
                    &script_tx_in.tx_in.tx_hash,
                    script_tx_in.tx_in.tx_index.into(),
                ),
            };
            TransactionInput::new(tx_hash, tx_index)
        })
        .collect::<Result<Vec<TransactionInput>, WError>>()
}
