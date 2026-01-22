mod certificates;
mod collaterals;
mod context;
mod inputs;
mod metadata;
mod mints;
mod outputs;
mod reference_inputs;
mod required_signers;
mod validity_range;
mod votes;
mod withdrawals;

use crate::{
    tx_parser::{
        certificates::extract_certificates, collaterals::extract_collaterals,
        context::ParserContext, inputs::extract_inputs, metadata::extract_metadata,
        mints::extract_mints, outputs::extract_outputs, reference_inputs::extract_reference_inputs,
        required_signers::extract_required_signers, validity_range::extract_validity_range,
        votes::extract_votes, withdrawals::extract_withdrawals,
    },
    wrapper::transaction_body::Transaction,
};
use pallas::ledger::traverse::ComputeHash;
use pallas_crypto::key::ed25519::{PublicKey, Signature};
use whisky_common::{TxBuilderBody, UTxO, WError};

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
    let reference_inputs = extract_reference_inputs(&pallas_tx.inner, &parser_context)?;
    let withdrawals = extract_withdrawals(&pallas_tx.inner, &parser_context)?;
    let mints = extract_mints(&pallas_tx.inner, &parser_context)?;
    let certificates = extract_certificates(&pallas_tx.inner, &parser_context)?;
    let validity_range = extract_validity_range(&pallas_tx.inner)?;
    let metadata = extract_metadata(&pallas_tx.inner)?;
    let votes = extract_votes(&pallas_tx.inner, &parser_context)?;

    let change_output = outputs.last().unwrap();
    Ok(TxBuilderBody {
        inputs,
        outputs: outputs.clone(),
        collaterals,
        required_signatures: required_signers,
        reference_inputs,
        withdrawals,
        mints,
        change_address: change_output.address.clone(),
        change_datum: change_output.datum.clone(),
        metadata,
        validity_range,
        certificates,
        votes,
        signing_key: vec![],
        fee: None, // These fields are expected to be recalculated by the TxBuilder
        network: None,
        total_collateral: None, // These fields are expected to be recalculated by the TxBuilder
        collateral_return_address: None, // These fields are expected to be recalculated by the TxBuilder
    })
}

pub fn check_tx_required_signers(tx_hex: &str) -> Result<bool, WError> {
    let bytes = hex::decode(tx_hex).map_err(|e| {
        WError::new(
            "WhiskyPallas - check tx required signers:",
            &format!("Hex decode error: {}", e),
        )
    })?;
    let pallas_tx = Transaction::decode_bytes(&bytes)?;
    let required_signers = extract_required_signers(&pallas_tx.inner)?;

    if let Some(signatures) = &pallas_tx.inner.transaction_witness_set.vkeywitness {
        Ok(required_signers.iter().all(|signer| {
            signatures.iter().any(|vkey_witness| {
                let vkey_hex = vkey_witness.vkey.to_string();
                let public_key =
                    PublicKey::from(<[u8; 32]>::try_from(vkey_witness.vkey.to_vec()).unwrap());
                public_key.verify(
                    pallas_tx.inner.transaction_body.compute_hash(),
                    &Signature::from(
                        <[u8; 64]>::try_from(vkey_witness.signature.to_vec()).unwrap(),
                    ),
                ) && (vkey_hex == *signer)
            })
        }))
    } else {
        Ok(required_signers.is_empty())
    }
}
