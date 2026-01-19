use pallas::{
    codec::utils::PositiveCoin,
    ledger::primitives::{
        conway::{Tx, Value},
        Fragment,
    },
};
use whisky_common::{Output, ProvidedScriptSource, ProvidedSimpleScriptSource, WError};

use crate::converter::{bech32_from_bytes, value_to_asset_vec};

pub fn extract_outputs(pallas_tx: &Tx) -> Result<Vec<Output>, WError> {
    let mut outputs_vec: Vec<Output> = Vec::new();
    let outputs = &pallas_tx.transaction_body.outputs;
    for output in outputs.iter() {
        match output {
            pallas::ledger::primitives::babbage::GenTransactionOutput::Legacy(legacy_output) => {
                let coerced_output = match &legacy_output.amount {
                    pallas::ledger::primitives::alonzo::Value::Coin(coin) => {
                        Value::Coin(coin.clone())
                    }
                    pallas::ledger::primitives::alonzo::Value::Multiasset(coin, asset_map) => {
                        let converted_asset_map = asset_map
                            .into_iter()
                            .map(|(policy_id, assets)| {
                                let converted_assets = assets
                                    .into_iter()
                                    .map(|(asset_name, amount)| {
                                        (
                                            asset_name.clone(),
                                            PositiveCoin::try_from(amount.clone()).unwrap(),
                                        )
                                    })
                                    .collect();
                                (policy_id.clone(), converted_assets)
                            })
                            .collect();
                        Value::Multiasset(coin.clone(), converted_asset_map)
                    }
                };
                let tx_out = Output {
                    address: bech32_from_bytes(&legacy_output.address.to_string())?,
                    amount: value_to_asset_vec(&&crate::wrapper::transaction_body::Value {
                        inner: coerced_output,
                    })?,
                    datum: match &legacy_output.datum_hash {
                        Some(datum) => Some(whisky_common::Datum::Hash(datum.to_string())),
                        None => None,
                    },
                    reference_script: None,
                };
                outputs_vec.push(tx_out);
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
