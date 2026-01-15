mod collaterals;
mod context;
mod inputs;
mod outputs;
mod required_signers;

use crate::{
    tx_parser::{
        collaterals::extract_collaterals, context::ParserContext, inputs::extract_inputs,
        outputs::extract_outputs, required_signers::extract_required_signers,
    },
    wrapper::transaction_body::Transaction,
};
use whisky_common::{TxBuilderBody, UTxO, ValidityRange, WError};

pub fn parse(tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<TxBuilderBody, WError> {
    let bytes = hex::decode(tx_hex).map_err(|e| {
        WError::new(
            "WhiskyPallas - parse tx hex:",
            &format!("Hex decode error: {}", e),
        )
    })?;
    let pallas_tx = Transaction::decode_bytes(&bytes)?;
    let mut parser_context = ParserContext::new();
    parser_context.fill_resolved_utxos(&pallas_tx.inner.transaction_body, resolved_utxos)?;
    parser_context
        .collect_script_witnesses_from_tx_witnesses_set(&pallas_tx.inner.transaction_witness_set)?;
    parser_context.collect_script_witnesses_from_tx_body(&pallas_tx.inner.transaction_body)?;

    let inputs = extract_inputs(&pallas_tx.inner, &parser_context)?;
    let outputs = extract_outputs(&pallas_tx.inner)?;
    let collaterals = extract_collaterals(&pallas_tx.inner, &parser_context)?;
    let required_signers = extract_required_signers(&pallas_tx.inner)?;
    Ok(TxBuilderBody {
        inputs,
        outputs,
        collaterals: collaterals,
        required_signatures: required_signers,
        reference_inputs: vec![],
        withdrawals: vec![],
        mints: vec![],
        change_address: "".to_string(),
        change_datum: None,
        metadata: vec![],
        validity_range: ValidityRange {
            invalid_before: None,
            invalid_hereafter: None,
        },
        certificates: vec![],
        votes: vec![],
        signing_key: vec![],
        fee: None,
        network: None,
        total_collateral: None,
        collateral_return_address: None,
    })
}
