mod certificates;
mod collaterals;
mod context;
mod inputs;
mod metadata;
mod mints;
pub mod outputs;
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
    wrapper::transaction_body::{ScriptRef, ScriptRefKind, Transaction},
};

use pallas::ledger::traverse::ComputeHash;
use pallas_crypto::key::ed25519::{PublicKey, Signature};
use whisky_common::{TxBuilderBody, UTxO, UtxoInput, UtxoOutput, WError};

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

pub fn extract_tx_utxos(tx_hex: &str) -> Result<Vec<whisky_common::UTxO>, WError> {
    let bytes = hex::decode(tx_hex).map_err(|e| {
        WError::new(
            "WhiskyPallas - extract tx outputs:",
            &format!("Hex decode error: {}", e),
        )
    })?;
    let pallas_tx = Transaction::decode_bytes(&bytes)?;
    extract_outputs(&pallas_tx.inner).and_then(|outputs| {
        outputs
            .into_iter()
            .enumerate()
            .map(|(index, output)| -> Result<UTxO, WError> {
                let datum_cbor = match output.datum.clone() {
                    Some(datum) => match datum {
                        whisky_common::Datum::Inline(s) => Some(s),
                        whisky_common::Datum::Hash(_) => None,
                        whisky_common::Datum::Embedded(_) => None,
                    },
                    None => None,
                };
                let datum_hash = match output.datum {
                    Some(datum) => match datum {
                        whisky_common::Datum::Inline(_) => None,
                        whisky_common::Datum::Hash(s) => Some(s),
                        whisky_common::Datum::Embedded(s) => Some(s),
                    },
                    None => None,
                };
                let script_cbor: Option<String> = match output.reference_script {
                    Some(script) => match script {
                        whisky_common::OutputScriptSource::ProvidedSimpleScriptSource(
                            provided_simple_script_source,
                        ) => Some(
                            ScriptRef::new(ScriptRefKind::NativeScript {
                                native_script_hex: provided_simple_script_source.script_cbor,
                            })
                            .unwrap()
                            .encode()?,
                        ),

                        whisky_common::OutputScriptSource::ProvidedScriptSource(
                            provided_script_source,
                        ) => match provided_script_source.language_version {
                            whisky_common::LanguageVersion::V1 => Some(
                                ScriptRef::new(ScriptRefKind::PlutusV1Script {
                                    plutus_v1_script_hex: provided_script_source.script_cbor,
                                })
                                .unwrap()
                                .encode()?,
                            ),
                            whisky_common::LanguageVersion::V2 => Some(
                                ScriptRef::new(ScriptRefKind::PlutusV2Script {
                                    plutus_v2_script_hex: provided_script_source.script_cbor,
                                })
                                .unwrap()
                                .encode()?,
                            ),
                            whisky_common::LanguageVersion::V3 => Some(
                                ScriptRef::new(ScriptRefKind::PlutusV3Script {
                                    plutus_v3_script_hex: provided_script_source.script_cbor,
                                })
                                .unwrap()
                                .encode()?,
                            ),
                        },
                    },
                    None => None,
                };
                Ok(UTxO {
                    input: UtxoInput {
                        tx_hash: pallas_tx.inner.transaction_body.compute_hash().to_string(),
                        output_index: index as u32,
                    },
                    output: UtxoOutput {
                        address: output.address,
                        amount: output.amount,
                        plutus_data: datum_cbor,
                        data_hash: datum_hash,
                        script_ref: script_cbor,
                        script_hash: None,
                    },
                })
            })
            .collect()
    })
}
