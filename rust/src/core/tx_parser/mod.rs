use crate::csl;
use crate::model::{
    Asset, Datum, JsVecString, LanguageVersion, MeshTxBuilderBody, Output, ProvidedScriptSource,
    ValidityRange,
};

pub struct MeshTxParser {
    pub tx_hex: String,
    pub tx_fee_lovelace: u64,
    pub tx_body: MeshTxBuilderBody,
}

pub trait MeshTxParserTrait {
    fn new(s: &str) -> Self;
    // TODO: add testing method lists here
}

impl MeshTxParserTrait for MeshTxParser {
    // Constructor method
    fn new(s: &str) -> MeshTxParser {
        // TODO: Deserialized into the tx_body
        let mut tx_body = MeshTxBuilderBody {
            inputs: vec![],
            outputs: vec![],
            collaterals: vec![],
            required_signatures: JsVecString::new(),
            reference_inputs: vec![],
            mints: vec![],
            change_address: "".to_string(),
            change_datum: None,
            metadata: vec![],
            validity_range: ValidityRange {
                invalid_before: None,
                invalid_hereafter: None,
            },
            signing_key: JsVecString::new(),
        };
        let csl_tx = csl::Transaction::from_hex(s).expect("Invalid transaction");
        let csl_tx_body = csl_tx.body();
        for i in 0..csl_tx_body.outputs().len() {
            tx_body
                .outputs
                .push(csl_output_to_mesh_output(csl_tx_body.outputs().get(i)))
        }
        MeshTxParser {
            tx_hex: s.to_string(),
            tx_fee_lovelace: csl::utils::from_bignum(&csl_tx.body().fee()),
            tx_body,
        }
    }
}

fn csl_output_to_mesh_output(output: csl::TransactionOutput) -> Output {
    let mut value: Vec<Asset> = vec![];
    value.push(Asset {
        unit: "lovelace".to_string(),
        quantity: output.amount().coin().to_str(),
    });
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

                    value.push(Asset {
                        unit: concated_name,
                        quantity: asset_quantity.to_str(),
                    })
                }
            }
        }
    }

    // TODO: Handle datum hash case
    let datum: Option<Datum> = output.plutus_data().map(|csl_datum| Datum {
        type_: "Inline".to_string(),
        data: csl_datum
            .to_json(csl::plutus::PlutusDatumSchema::DetailedSchema)
            .unwrap(),
    });

    let reference_script: Option<ProvidedScriptSource> = match output.script_ref() {
        Some(csl_script_ref) => {
            let plutus_script = csl_script_ref.plutus_script().unwrap();
            let language_version = match plutus_script.language_version().kind() {
                csl::plutus::LanguageKind::PlutusV1 => LanguageVersion::V1,
                csl::plutus::LanguageKind::PlutusV2 => LanguageVersion::V2,
            };
            Some(ProvidedScriptSource {
                script_cbor: plutus_script.to_hex(),
                language_version,
            })
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
