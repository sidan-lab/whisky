use pallas_codec::utils::NonEmptyKeyValuePairs;
use pallas_primitives::conway::NativeScript;
use pallas_primitives::conway::PlutusV1Script;
use pallas_primitives::conway::PlutusV2Script;
use pallas_primitives::conway::PlutusV3Script;
use pallas_primitives::conway::RedeemerTag as PRedeemerTag;
use std::collections::HashMap;
use uplc::tx::SlotConfig;

use crate::core::constants::get_cost_models_from_network;
use crate::core::tx_parser::MeshTxParser;
use crate::csl::{Address, JsError};
use crate::model::{Action, Asset, Budget, JsVecString, Network, RedeemerTag, UTxO, UtxoOutput};
use crate::wasm::WasmResult;
use crate::*;
use pallas_codec::minicbor::Decoder;
use pallas_codec::utils::{Bytes, CborWrap, PositiveCoin};
use pallas_primitives::{
    conway::{
        AssetName, Coin, CostMdls, DatumOption, PlutusData, PolicyId, PostAlonzoTransactionOutput,
        PseudoScript, ScriptRef, TransactionOutput, Value,
    },
    Fragment,
};
use pallas_traverse::{Era, MultiEraTx};
use uplc::{
    tx::{eval_phase_two, ResolvedInput},
    Hash, TransactionInput,
};

#[wasm_bindgen]
pub fn evaluate_tx_scripts_js(
    tx_hex: String,
    resolved_utxos: JsVecString,
    additional_txs: JsVecString,
    network: String,
) -> WasmResult {
    let mut deserialized_utxos: Vec<UTxO> = Vec::new();
    for utxo_json in resolved_utxos {
        match serde_json::from_str(utxo_json.as_str()) {
            Ok(utxo) => deserialized_utxos.push(utxo),
            Err(e) => {
                return WasmResult::new_error("failure".to_string(), format!("{:?}", e));
            }
        }
    }

    let deserialize_network = match serde_json::from_str(network.as_str()) {
        Ok(network) => network,
        Err(e) => {
            return WasmResult::new_error("failure".to_string(), format!("{:?}", e));
        }
    };

    let eval_result = evaluate_tx_scripts(
        &tx_hex,
        &deserialized_utxos,
        &additional_txs.into_vec(),
        &deserialize_network,
    );

    match eval_result {
        Ok(actions) => {
            let actions_json = serde_json::to_string(&actions).unwrap();
            WasmResult::new("success".to_string(), actions_json)
        }
        Err(e) => WasmResult::new_error("failure".to_string(), format!("{:?}", e)),
    }
}

pub fn evaluate_tx_scripts(
    tx_hex: &str,
    inputs: &[UTxO],
    additional_txs: &[String],
    network: &Network,
) -> Result<Vec<Action>, JsError> {
    let tx_bytes = hex::decode(tx_hex).expect("Invalid tx hex");
    let mtx = MultiEraTx::decode_for_era(Era::Conway, &tx_bytes);
    let tx = match mtx {
        Ok(MultiEraTx::Conway(tx)) => tx.into_owned(),
        _ => return Err(JsError::from_str("Invalid Tx Era")),
    };

    let tx_outs: Vec<UTxO> = additional_txs
        .iter()
        .flat_map(|tx| {
            let parsed_tx = MeshTxParser::new(tx).unwrap();
            println!(
                "txout: {:?}",
                &parsed_tx.get_tx_outs_utxo().unwrap().clone()
            );
            println!("txout_cbor: {:?}", &parsed_tx.get_tx_outs_cbor().clone());
            parsed_tx.get_tx_outs_utxo().unwrap() //TODO: handle error
        })
        .collect();

    // combine inputs and tx_outs
    let all_inputs: Vec<UTxO> = inputs.iter().chain(tx_outs.iter()).cloned().collect();

    eval_phase_two(
        &tx,
        &to_pallas_utxos(&all_inputs)?,
        Some(&get_cost_mdls(network)?),
        None,
        &SlotConfig::default(),
        false,
        |_r| (),
    )
    .map_err(|err| JsError::from_str(&format!("Error occurred during evaluation: {}", err)))
    .map(|reds| {
        reds.into_iter()
            .map(|red| Action {
                index: red.index,
                budget: Budget {
                    mem: red.ex_units.mem,
                    steps: red.ex_units.steps,
                },
                tag: match red.tag {
                    PRedeemerTag::Spend => RedeemerTag::Spend,
                    PRedeemerTag::Mint => RedeemerTag::Mint,
                    PRedeemerTag::Cert => RedeemerTag::Cert,
                    PRedeemerTag::Reward => RedeemerTag::Reward,
                    PRedeemerTag::Vote => RedeemerTag::Vote,
                    PRedeemerTag::Propose => RedeemerTag::Propose,
                },
            })
            .collect()
    })
}

fn get_cost_mdls(network: &Network) -> Result<CostMdls, JsError> {
    let cost_model_list = get_cost_models_from_network(network);
    if cost_model_list.len() < 3 {
        return Err(JsError::from_str(
            "Cost models have to contain at least PlutusV1, PlutusV2, and PlutusV3 costs",
        ));
    };
    Ok(CostMdls {
        plutus_v1: Some(cost_model_list[0].clone()),
        plutus_v2: Some(cost_model_list[1].clone()),
        plutus_v3: Some(cost_model_list[2].clone()),
    })
}

fn to_pallas_utxos(utxos: &Vec<UTxO>) -> Result<Vec<ResolvedInput>, JsError> {
    let mut resolved_inputs = Vec::new();
    for utxo in utxos {
        let tx_hash: [u8; 32] = hex::decode(&utxo.input.tx_hash)
            .map_err(|err| JsError::from_str(&format!("Invalid tx hash found: {}", err)))?
            .try_into()
            .map_err(|_e| JsError::from_str("Invalid tx hash length found"))?;

        let resolved_input = ResolvedInput {
            input: TransactionInput {
                transaction_id: Hash::from(tx_hash),
                index: utxo.input.output_index.into(),
            },
            output: TransactionOutput::PostAlonzo(PostAlonzoTransactionOutput {
                address: Bytes::from(Address::from_bech32(&utxo.output.address)?.to_bytes()),
                value: to_pallas_value(&utxo.output.amount)?,
                datum_option: to_pallas_datum(&utxo.output)?,
                script_ref: to_pallas_script_ref(&utxo.output)?,
            }),
        };
        resolved_inputs.push(resolved_input);
    }
    Ok(resolved_inputs)
}

fn to_pallas_script_ref(utxo_output: &UtxoOutput) -> Result<Option<CborWrap<ScriptRef>>, JsError> {
    if let Some(script_ref) = &utxo_output.script_ref {
        let script_bytes = hex::decode(script_ref.script_hex.clone())
            .map_err(|err| JsError::from_str(&format!("Invalid script hex found: {}", err)))?;

        let unwrapped_bytes = Decoder::new(&script_bytes)
            .bytes()
            .map_err(|err| JsError::from_str(&format!("Invalid script hex found: {}", err)))?;

        match &script_ref.script_version {
            Some(version) => match version {
                model::LanguageVersion::V1 => Ok(Some(CborWrap(PseudoScript::PlutusV1Script(
                    PlutusV1Script(unwrapped_bytes.to_vec().into()),
                )))),
                model::LanguageVersion::V2 => Ok(Some(CborWrap(PseudoScript::PlutusV2Script(
                    PlutusV2Script(unwrapped_bytes.to_vec().into()),
                )))),
                model::LanguageVersion::V3 => Ok(Some(CborWrap(PseudoScript::PlutusV3Script(
                    PlutusV3Script(unwrapped_bytes.to_vec().into()),
                )))),
            },
            None => Ok(Some(CborWrap(PseudoScript::NativeScript(
                NativeScript::decode_fragment(unwrapped_bytes).map_err(|err| {
                    JsError::from_str(&format!("Invalid native script found: {}", err))
                })?,
            )))),
        }
    } else {
        Ok(None)
    }
}

fn to_pallas_datum(utxo_output: &UtxoOutput) -> Result<Option<DatumOption>, JsError> {
    if let Some(inline_datum) = &utxo_output.plutus_data {
        //hex to bytes
        let plutus_data_bytes = hex::decode(inline_datum)
            .map_err(|err| JsError::from_str(&format!("Invalid plutus data found: {}", err)))?;
        let datum = CborWrap(
            PlutusData::decode_fragment(&plutus_data_bytes)
                .map_err(|_e| JsError::from_str("Invalid plutus data found"))?,
        );
        Ok(Some(DatumOption::Data(datum)))
    } else if let Some(datum_hash) = &utxo_output.data_hash {
        let datum_hash_bytes: [u8; 32] = hex::decode(datum_hash)
            .map_err(|err| JsError::from_str(&format!("Invalid datum hash found: {}", err)))?
            .try_into()
            .map_err(|_e| JsError::from_str("Invalid byte length of datum hash found"))?;
        Ok(Some(DatumOption::Hash(Hash::from(datum_hash_bytes))))
    } else {
        Ok(None)
    }
}

fn to_pallas_value(assets: &Vec<Asset>) -> Result<Value, JsError> {
    if assets.len() == 1 {
        match assets[0].unit().as_str() {
            "lovelace" => Ok(Value::Coin(assets[0].quantity().parse::<u64>().unwrap())),
            _ => Err(JsError::from_str("Invalid value")),
        }
    } else {
        to_pallas_multi_asset_value(assets)
    }
}

fn to_pallas_multi_asset_value(assets: &Vec<Asset>) -> Result<Value, JsError> {
    let mut coins: Coin = 0;
    let mut asset_mapping: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for asset in assets {
        if asset.unit() == "lovelace" || asset.unit().is_empty() {
            coins = asset.quantity().parse::<u64>().unwrap();
        } else {
            let asset_unit = asset.unit();
            let (policy_id, asset_name) = asset_unit.split_at(56);
            asset_mapping
                .entry(policy_id.to_string())
                .or_default()
                .push((asset_name.to_string(), asset.quantity().clone()))
        }
    }

    let mut multi_asset = Vec::new();
    for (policy_id, asset_list) in &asset_mapping {
        let policy_id_bytes: [u8; 28] = hex::decode(policy_id)
            .map_err(|err| JsError::from_str(&format!("Invalid policy id found: {}", err)))?
            .try_into()
            .map_err(|_e| JsError::from_str("Invalid length policy id found"))?;

        let policy_id = PolicyId::from(policy_id_bytes);
        let mut mapped_assets = Vec::new();
        for asset in asset_list {
            let (asset_name, asset_quantity) = asset;
            let asset_name_bytes =
                AssetName::from(hex::decode(asset_name).map_err(|err| {
                    JsError::from_str(&format!("Invalid asset name found: {}", err))
                })?);
            mapped_assets.push((
                asset_name_bytes,
                PositiveCoin::try_from(asset_quantity.parse::<u64>().unwrap()).unwrap(),
            ));
        }
        multi_asset.push((policy_id, NonEmptyKeyValuePairs::Def(mapped_assets)));
    }
    let pallas_multi_asset = pallas_codec::utils::NonEmptyKeyValuePairs::Def(multi_asset);
    Ok(Value::Multiasset(coins, pallas_multi_asset))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::csl;
    use model::{LanguageVersion, ScriptRef, UtxoInput};
    use pallas_codec::minicbor::Decoder;
    use serde_json::json;

    #[test]
    fn test_eval() {
        let result = evaluate_tx_scripts(
            "84a80082825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9800825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98010d81825820604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad9801128182582004b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa000181a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b00000002529898c810a200581d601fd5bab167338971d92b4d8f0bdf57d889903e6e934e7ea38c7dadf1011b0000000252882db4111a000412f1021a0002b74b0b5820775d0cf3c95993f6210e4410e92f72ebc3942ce9c1433694749aa239e5d13387a200818258201557f444f3ae6e61dfed593ae15ec8dbd57b8138972bf16fde5b4c559f41549b5840729f1f14ef05b7cf9b0d7583e6777674f80ae64a35bbd6820cc3c82ddf0412ca1d751b7d886eece3c6e219e1c5cc9ef3d387a8d2078f47125d54b474fbdfbd0105818400000182190b111a000b5e35f5f6",
          &vec![UTxO {
              input: UtxoInput {
                  tx_hash: "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98".to_string(),
                  output_index: 0
              },
              output: UtxoOutput {
                  address: "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97".to_string(),
                  amount: vec![Asset::new_from_str("lovelace", "986990")],
                  data_hash: None,
                  plutus_data: Some(csl::PlutusData::from_json(&
                    json!({
                        "constructor": 0,
                        "fields": []
                    })
                    .to_string(), csl::PlutusDatumSchema::DetailedSchema).unwrap().to_hex()),
                  script_hash: None,
                  script_ref: None,
              }
          },
          UTxO {
              input: UtxoInput {
                  tx_hash: "604943e070ffbf81cc09bb2942029f5f5361108a3c0b96a7309e6aa70370ad98".to_string(),
                  output_index: 1
              },
              output: UtxoOutput {
                  address: "addr_test1vq0atw43vuecjuwe9dxc7z7l2lvgnyp7d6f5ul4r3376mug8v67h5".to_string(),
                  amount: vec![Asset::new_from_str("lovelace", "9974857893")],
                  data_hash: None,
                  plutus_data: None,
                  script_hash: None,
                  script_ref: None,
              }
          },
          UTxO {
              input: UtxoInput {
                  tx_hash: "04b9070a30bd63abaaf59a3c48a1575c4127bb0edb00ecd5141fd18a85c721aa".to_string(),
                  output_index: 0
              },
              output: UtxoOutput {
                  address: "addr_test1wzlwsgq97vchypqzk8u8lz30w932tvx7akcj7csm02scl7qlghd97".to_string(),
                  amount: vec![Asset::new_from_str("lovelace", "986990")],
                  data_hash: None,
                  plutus_data: None,
                  script_hash: None,
                  script_ref: Some(ScriptRef { script_hex: "5655010000322223253330054a229309b2b1bad0025735".to_string(), script_version: Some(LanguageVersion::V2)})
              }
          }],
          &[],
          &Network::Mainnet
      );

        let redeemers = result.unwrap();
        let mut redeemer_json = serde_json::Map::new();
        for redeemer in redeemers {
            redeemer_json.insert("index".to_string(), redeemer.index.to_string().into());
            let mut ex_unit_json = serde_json::Map::new();
            ex_unit_json.insert("mem".to_string(), redeemer.budget.mem.into());
            ex_unit_json.insert("steps".to_string(), redeemer.budget.steps.into());
            redeemer_json.insert(
                "ex_units".to_string(),
                serde_json::Value::Object(ex_unit_json),
            );
        }
        assert_eq!(
            serde_json::json!({"ex_units":{"mem":2833,"steps":528893},"index":"0"}).to_string(),
            serde_json::json!(redeemer_json).to_string()
        )
    }

    #[test]
    fn test_cbor() {
        let script_bytes = hex::decode("5655010000322223253330054a229309b2b1bad0025735").unwrap();
        let decoded_bytes = Decoder::new(&script_bytes).bytes().unwrap();
        assert_eq!(
            hex::decode("55010000322223253330054a229309b2b1bad0025735").unwrap(),
            decoded_bytes
        );
    }
}
