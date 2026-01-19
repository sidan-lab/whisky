use pallas::ledger::primitives::{conway::Tx, TransactionInput};
use whisky_common::{RefTxIn, WError};

use crate::tx_parser::context::ParserContext;

pub fn extract_reference_inputs(
    pallas_tx: &Tx,
    parser_context: &ParserContext,
) -> Result<Vec<RefTxIn>, WError> {
    let mut reference_inputs_vec: Vec<RefTxIn> = Vec::new();
    let reference_inputs = &pallas_tx.transaction_body.reference_inputs;
    match reference_inputs {
        Some(ref_inputs) => {
            for input in ref_inputs.iter() {
                let tx_in = utxo_to_ref_tx_in(input, parser_context)?;
                reference_inputs_vec.push(tx_in);
            }
        }
        None => {}
    }
    Ok(reference_inputs_vec)
}

fn utxo_to_ref_tx_in(
    tx_input: &TransactionInput,
    context: &ParserContext,
) -> Result<RefTxIn, WError> {
    let utxo = context.resolved_utxos.get(tx_input).ok_or_else(|| {
        WError::new(
            "utxo_to_ref_tx_in",
            &format!("Failed to find UTxO for reference input: {:?}", tx_input),
        )
    })?;

    Ok(RefTxIn {
        tx_hash: utxo.input.tx_hash.clone(),
        tx_index: utxo.input.output_index,
        script_size: utxo
            .output
            .script_ref
            .as_ref()
            .map(|script_ref| script_ref.len() / 2),
    })
}
