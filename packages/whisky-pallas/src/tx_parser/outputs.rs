use cardano_serialization_lib::{self as csl};
use whisky_common::*;

use super::TxParser;

impl TxParser {
    pub fn outputs(&mut self) -> Result<Vec<Output>, WError> {
        let mut parsed_outputs = vec![];
        for i in 0..self.csl_tx_body.outputs().len() {
            let tx_output = self.csl_tx_body.outputs().get(i);
            parsed_outputs.push(
                csl_output_to_whisky_output(tx_output)
                    .map_err(WError::from_err("TxParser - outputs"))?,
            )
        }
        self.tx_body.outputs = parsed_outputs.clone();
        Ok(parsed_outputs)
    }

    pub fn get_tx_outs_utxo(&self) -> Result<Vec<UTxO>, WError> {
        let tx_outs = self.tx_body.outputs.clone();
        let mut result = vec![];
        tx_outs.iter().enumerate().for_each(|(i, current_tx_out)| {
            let (data_hash, plutus_data) = match current_tx_out.clone().datum {
                Some(Datum::Hash(data)) => {
                    let data_hash = Some(data);
                    (data_hash, None)
                }
                Some(Datum::Inline(data)) => {
                    let datum_cbor =
                        csl::PlutusData::from_json(&data, csl::PlutusDatumSchema::DetailedSchema)
                            .unwrap() // TODO: Handle unwrap
                            .to_hex();
                    let plutus_data = Some(datum_cbor);
                    (None, plutus_data)
                }
                Some(Datum::Embedded(data)) => {
                    let data_hash = Some(data);
                    (data_hash, None)
                }
                None => (None, None),
            };
            let tx_out_utxo: UTxO = UTxO {
                input: UtxoInput {
                    output_index: i as u32,
                    tx_hash: self.tx_hash.clone(),
                },
                output: UtxoOutput {
                    address: current_tx_out.address.clone(),
                    amount: current_tx_out.amount.clone(),
                    data_hash,
                    plutus_data,
                    script_ref: None,
                    script_hash: None,
                },
            };
            result.push(tx_out_utxo);
        });
        Ok(result)
    }

    pub fn get_tx_outs_cbor(&self) -> Vec<String> {
        let tx_outs = self.csl_tx_body.outputs();
        let mut result = vec![];
        for i in 0..tx_outs.len() {
            let tx_out: csl::TransactionOutput = tx_outs.get(i);
            let tx_out_cbor = tx_out.to_hex();
            result.push(tx_out_cbor);
        }
        result
    }
}

fn csl_output_to_whisky_output(output: csl::TransactionOutput) -> Result<Output, WError> {
    let mut value: Vec<Asset> = vec![];
    value.push(Asset::new_from_str(
        "lovelace",
        &output.amount().coin().to_str(),
    ));
    let multi_asset = output.amount().multiasset();

    match multi_asset {
        None => {}
        Some(multi_asset) => {
            for policy_id_index in 0..multi_asset.keys().len() {
                let policy_id = multi_asset.keys().get(policy_id_index);
                let assets = multi_asset.get(&policy_id).unwrap();
                for asset_index in 0..assets.keys().len() {
                    let asset_name = assets.keys().get(asset_index);
                    let asset_quantity = assets.get(&asset_name).unwrap();
                    let concated_name = policy_id.to_hex() + &asset_name.to_string();

                    value.push(Asset::new_from_str(
                        &concated_name,
                        &asset_quantity.to_str(),
                    ))
                }
            }
        }
    }

    // TODO: Handle datum hash case
    let datum: Option<Datum> = output.plutus_data().map(|csl_datum| {
        Datum::Inline(
            csl_datum
                .to_json(csl::PlutusDatumSchema::DetailedSchema)
                .unwrap(),
        )
    });

    let reference_script: Option<OutputScriptSource> = match output.script_ref() {
        Some(csl_script_ref) => {
            let plutus_script = csl_script_ref.plutus_script().unwrap();
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
        }
        None => None,
    };
    Ok(Output {
        address: output.address().to_bech32(None).unwrap(),
        amount: value,
        datum,
        reference_script,
    })
}
