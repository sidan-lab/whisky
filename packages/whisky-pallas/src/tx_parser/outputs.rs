use pallas::ledger::primitives::{conway::Tx, Fragment};
use whisky_common::{Output, ProvidedScriptSource, ProvidedSimpleScriptSource, WError};

use crate::converter::{bech32_from_bytes, value_to_asset_vec};

pub fn extract_outputs(pallas_tx: &Tx) -> Result<Vec<Output>, WError> {
    let mut outputs_vec: Vec<Output> = Vec::new();
    let outputs = &pallas_tx.transaction_body.outputs;
    for output in outputs.iter() {
        match output {
            pallas::ledger::primitives::babbage::GenTransactionOutput::Legacy(_legacy_output) => {
                return Err(WError::new(
                    "Whisky Pallas Parser - ",
                    "Legacy outputs are not supported",
                ))
            }
            pallas::ledger::primitives::babbage::GenTransactionOutput::PostAlonzo(
                post_alonzo_output,
            ) => {
                let tx_out: Output = Output {
                    address: bech32_from_bytes(&post_alonzo_output.address.to_string())?,
                    amount: value_to_asset_vec(&&crate::wrapper::transaction_body::Value {
                        inner: post_alonzo_output.value.clone(),
                    })?,
                    datum: match &post_alonzo_output.datum_option {
                        Some(datum) => match datum.clone().unwrap() {
                            pallas::ledger::primitives::conway::DatumOption::Hash(hash) => {
                                Some(whisky_common::Datum::Hash(hash.to_string()))
                            }
                            pallas::ledger::primitives::conway::DatumOption::Data(datum) => {
                                Some(whisky_common::Datum::Inline(hex::encode(datum.raw_cbor())))
                            }
                        },
                        None => None,
                    },
                    reference_script: match &post_alonzo_output.script_ref {
                        Some(script) => Some(match script.clone().unwrap() {
                            pallas::ledger::primitives::conway::ScriptRef::NativeScript(
                                native_script,
                            ) => whisky_common::OutputScriptSource::ProvidedSimpleScriptSource(
                                ProvidedSimpleScriptSource {
                                    script_cbor: hex::encode(
                                        native_script.encode_fragment().unwrap(),
                                    ),
                                },
                            ),
                            pallas::ledger::primitives::conway::ScriptRef::PlutusV1Script(
                                plutus_script,
                            ) => whisky_common::OutputScriptSource::ProvidedScriptSource(
                                ProvidedScriptSource {
                                    script_cbor: hex::encode(
                                        plutus_script.encode_fragment().unwrap(),
                                    ),
                                    language_version: whisky_common::LanguageVersion::V1,
                                },
                            ),
                            pallas::ledger::primitives::conway::ScriptRef::PlutusV2Script(
                                plutus_script,
                            ) => whisky_common::OutputScriptSource::ProvidedScriptSource(
                                ProvidedScriptSource {
                                    script_cbor: hex::encode(
                                        plutus_script.encode_fragment().unwrap(),
                                    ),
                                    language_version: whisky_common::LanguageVersion::V2,
                                },
                            ),
                            pallas::ledger::primitives::conway::ScriptRef::PlutusV3Script(
                                plutus_script,
                            ) => whisky_common::OutputScriptSource::ProvidedScriptSource(
                                ProvidedScriptSource {
                                    script_cbor: hex::encode(
                                        plutus_script.encode_fragment().unwrap(),
                                    ),
                                    language_version: whisky_common::LanguageVersion::V3,
                                },
                            ),
                        }),
                        None => None,
                    },
                };
                outputs_vec.push(tx_out);
            }
        }
    }
    Ok(outputs_vec)
}
