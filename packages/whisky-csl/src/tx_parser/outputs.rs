use cardano_serialization_lib::{self as csl};
use whisky_common::*;

use super::CSLParser;

impl CSLParser {
    pub fn get_outputs(&self) -> &Vec<Output> {
        &self.tx_body.outputs
    }

    pub fn extract_output_utxos(tx_hex: &str) -> Result<Vec<UTxO>, WError> {
        let tx = csl::FixedTransaction::from_hex(tx_hex).map_err(WError::from_err("extract_output_utxos"))?;
        let outputs = tx.body().outputs();
        let tx_hash = tx.transaction_hash().to_hex();
        let whisky_outputs = csl_outputs_to_outputs(&outputs)?;
        let mut output_utxos = Vec::new();
        for (index, output) in whisky_outputs.iter().enumerate() {
            output_utxos.push(output_to_utxo(output, &tx_hash, index as u32)?);
        }
        Ok(output_utxos)
    }

    pub fn extract_output_cbors(tx_hex: &str) -> Result<Vec<String>, WError> {
        let mut output_cbors = Vec::new();
        let csl_tx = csl::FixedTransaction::from_hex(tx_hex)
        .map_err(WError::from_err("extract_output_cbors"))?;
        let csl_outputs = csl_tx.body().outputs();
        let len = csl_outputs.len();
        for i in 0..len {
            let output = csl_outputs.get(i);
            output_cbors.push(output.to_hex());
        }
        Ok(output_cbors)
    }


    pub(super) fn extract_outputs(&mut self) -> Result<(), WError> {
        let outputs = self.csl_tx_body.outputs();
        self.tx_body.outputs = csl_outputs_to_outputs(&outputs)?;
        Ok(())
    }
}

fn output_to_utxo(output: &Output, tx_hash: &String, index: u32) -> Result<UTxO, WError> {
    let datum = match &output.datum {
        Some(Datum::Inline(s)) => Some(s.clone()),
        Some(Datum::Embedded(s)) => Some(s.clone()),
        _ => None,
    };
    let data_hash = match &output.datum {
        Some(Datum::Hash(s)) => Some(s.clone()),
        _ => None,
    };
    let script_ref = match &output.reference_script {
        Some(script) => Some(output_reference_script_to_script_source(&script)?),
        None => None,
    };
    let script_hash = match &script_ref {
        Some(script) => {
            if script.is_native_script() {
                Some(script.native_script().unwrap().hash().to_hex())
            } else {
                Some(script.plutus_script().unwrap().hash().to_hex())
            }
        },
        None => None,
    };
    Ok(UTxO {
        input: UtxoInput {
            tx_hash: tx_hash.clone(),
            output_index: index as u32,
        },
        output: UtxoOutput {
            address: output.address.clone(),
            amount: output.amount.clone(),
            data_hash,
            plutus_data: datum,
            script_ref: script_ref.map(|script| script.to_hex()),
            script_hash: script_hash,
        },
    })
}

fn output_reference_script_to_script_source(
    output_reference_script: &OutputScriptSource,
) -> Result<csl::ScriptRef, WError> {
    match output_reference_script {
        OutputScriptSource::ProvidedScriptSource(script) => {
            let language_version = match script.language_version {
                LanguageVersion::V1 => csl::Language::new_plutus_v1(),
                LanguageVersion::V2 => csl::Language::new_plutus_v2(),
                LanguageVersion::V3 => csl::Language::new_plutus_v3(),
            };
            let script_ref = csl::ScriptRef::new_plutus_script(
                &csl::PlutusScript::from_hex_with_version(&script.script_cbor, &language_version)
                    .map_err(|e| {
                    WError::new(
                        "output_reference_script_to_script_source",
                        &format!("Failed to convert script to plutus script: {}", e),
                    )
                })?,
            );
            Ok(script_ref)
        }
        OutputScriptSource::ProvidedSimpleScriptSource(script) => {
            let script_ref = csl::ScriptRef::new_native_script(
                &csl::NativeScript::from_hex(&script.script_cbor).map_err(|e| {
                    WError::new(
                        "output_reference_script_to_script_source",
                        &format!("Failed to convert script to native script: {}", e),
                    )
                })?,
            );
            Ok(script_ref)
        }
    }
}

fn csl_outputs_to_outputs(outputs: &csl::TransactionOutputs) -> Result<Vec<Output>, WError> {
    let mut result = Vec::new();

    for i in 0..outputs.len() {
        let output = outputs.get(i);
        let mut value: Vec<Asset> = vec![];

        value.push(Asset::new_from_str(
            "lovelace",
            &output.amount().coin().to_str(),
        ));

        if let Some(multi_asset) = output.amount().multiasset() {
            for policy_id_index in 0..multi_asset.keys().len() {
                let policy_id = multi_asset.keys().get(policy_id_index);
                let assets = multi_asset.get(&policy_id).ok_or_else(|| {
                    WError::new(
                        "csl_outputs_to_outputs",
                        &format!("Failed to get assets for policy ID: {}", policy_id.to_hex()),
                    )
                })?;
                for asset_index in 0..assets.keys().len() {
                    let asset_name = assets.keys().get(asset_index);
                    let asset_quantity = assets.get(&asset_name).ok_or_else(|| {
                        WError::new(
                            "csl_outputs_to_outputs",
                            &format!(
                                "Failed to get quantity for asset: {}",
                                asset_name.to_string()
                            ),
                        )
                    })?;
                    let asset_name_hex = hex::encode(asset_name.name());
                    let concated_name = policy_id.to_hex() + &asset_name_hex;

                    value.push(Asset::new_from_str(
                        &concated_name,
                        &asset_quantity.to_str(),
                    ));
                }
            }
        }

        let datum: Option<Datum> = if let Some(csl_datum) = output.plutus_data() {
            Some(Datum::Inline(csl_datum.to_hex()))
        } else {
            output
                .data_hash()
                .map(|csl_datum_hash| Datum::Hash(csl_datum_hash.to_hex()))
        };

        let reference_script: Option<OutputScriptSource> = match output.script_ref() {
            Some(csl_script_ref) => {
                if let Some(plutus_script) = csl_script_ref.plutus_script() {
                    let language_version = match plutus_script.language_version().kind() {
                        csl::LanguageKind::PlutusV1 => LanguageVersion::V1,
                        csl::LanguageKind::PlutusV2 => LanguageVersion::V2,
                        csl::LanguageKind::PlutusV3 => LanguageVersion::V3,
                    };
                    Some(OutputScriptSource::ProvidedScriptSource(
                        ProvidedScriptSource {
                            script_cbor: plutus_script.to_hex(),
                            language_version,
                        },
                    ))
                } else if let Some(native_script) = csl_script_ref.native_script() {
                    Some(OutputScriptSource::ProvidedSimpleScriptSource(
                        ProvidedSimpleScriptSource {
                            script_cbor: native_script.to_hex(),
                        },
                    ))
                } else {
                    None
                }
            }
            None => None,
        };

        result.push(Output {
            address: output.address().to_bech32(None).map_err(|e| {
                WError::new(
                    "csl_outputs_to_outputs",
                    &format!("Failed to convert address to bech32: {}", e),
                )
            })?,
            amount: value,
            datum,
            reference_script,
        });
    }
    Ok(result)
}
