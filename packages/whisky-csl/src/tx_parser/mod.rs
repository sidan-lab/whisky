use cardano_serialization_lib::{self as csl};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use whisky_common::*;

use crate::utils::{blake2b256, calculate_tx_hash};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxParser {
    pub tx_hash: String,
    pub tx_hex: String,
    pub tx_fee_lovelace: u64,
    pub tx_body: TxBuilderBody,
    pub csl_tx_body: csl::TransactionBody,
    pub csl_witness_set: csl::TransactionWitnessSet,
}

impl TxParser {
    // Constructor method
    pub fn new(s: &str) -> Result<TxParser, WError> {
        // TODO: Deserialized into the tx_body
        let mut tx_body = TxBuilderBody {
            inputs: vec![],
            outputs: vec![],
            collaterals: vec![],
            required_signatures: vec![],
            reference_inputs: vec![],
            withdrawals: vec![],
            mints: vec![],
            change_address: "".to_string(),
            change_datum: None,
            certificates: vec![],
            votes: vec![],
            metadata: vec![],
            validity_range: ValidityRange {
                invalid_before: None,
                invalid_hereafter: None,
            },
            signing_key: vec![],
            fee: None,
            network: None,
        };
        let csl_tx = csl::Transaction::from_hex(s).expect("Invalid transaction");
        let csl_tx_body = csl_tx.body();
        let csl_witness_set = csl_tx.witness_set();
        for i in 0..csl_tx_body.outputs().len() {
            let tx_output = csl_tx_body.outputs().get(i);
            tx_body.outputs.push(csl_output_to_mesh_output(tx_output))
        }
        let required_signers_key_hashes = csl_tx_body
            .required_signers()
            .unwrap_or(csl::Ed25519KeyHashes::new());
        for i in 0..required_signers_key_hashes.len() {
            let signer = required_signers_key_hashes.get(i);
            tx_body.required_signatures.push(signer.to_hex())
        }
        let tx_parser = TxParser {
            tx_hash: calculate_tx_hash(s)?,
            tx_hex: s.to_string(),
            tx_fee_lovelace: csl_tx.body().fee().to_str().parse::<u64>().unwrap(),
            tx_body,
            csl_tx_body,
            csl_witness_set,
        };
        Ok(tx_parser)
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

    pub fn check_all_required_signers(&self) -> bool {
        let signers = &self.tx_body.required_signatures;
        let mut signer_set: HashSet<String> = HashSet::new();
        let fixed_tx = csl::FixedTransaction::from_hex(&self.tx_hex).unwrap();
        for signer in signers {
            signer_set.insert(signer.clone());
        }
        // for i in 0..signers.len() {
        //     signer_set.insert(signers.get(i));
        // }
        let csl_vkeys = self
            .csl_witness_set
            .vkeys()
            .unwrap_or(csl::Vkeywitnesses::new());
        for i in 0..csl_vkeys.len() {
            let vkey_witness = csl_vkeys.get(i);
            let pub_key = vkey_witness.vkey().public_key();
            if !pub_key.verify(&blake2b256(&fixed_tx.raw_body()), &vkey_witness.signature()) {
                return false;
            } else {
                signer_set.remove(&pub_key.hash().to_hex());
            };
        }
        signer_set.is_empty()
    }
}

fn csl_output_to_mesh_output(output: csl::TransactionOutput) -> Output {
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
    Output {
        address: output.address().to_bech32(None).unwrap(),
        amount: value,
        datum,
        reference_script,
    }
}
