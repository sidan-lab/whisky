use pallas_codec::utils::NonEmptyKeyValuePairs;
use pallas_primitives::conway::{Redeemer, RedeemerTag as PRedeemerTag};
use std::collections::HashMap;
use uplc::tx::SlotConfig;

use crate::core::constants::get_cost_models_from_network;
use crate::core::tx_parser::MeshTxParser;
use crate::core::utils::phase_two::{eval_phase_two, PhaseTwoEvalResult};
use crate::csl::{Address, JsError};
use crate::model::{
    Action, Asset, Budget, EvalError, EvalResult, JsVecString, Network, RedeemerTag, UTxO,
    UtxoOutput,
};
use crate::wasm::WasmResult;
use crate::*;
use pallas_codec::utils::{Bytes, CborWrap, PositiveCoin};
use pallas_primitives::{
    conway::{
        AssetName, Coin, CostModels, DatumOption, PlutusData, PolicyId,
        PostAlonzoTransactionOutput, ScriptRef, TransactionOutput, Value,
    },
    Fragment,
};
use pallas_traverse::{Era, MultiEraTx};
use uplc::{tx::error::Error as UplcError, tx::ResolvedInput, Hash, TransactionInput};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonSlotConfig {
    slot_length: u32,
    zero_slot: u64,
    zero_time: u64,
}

#[wasm_bindgen]
pub fn evaluate_tx_scripts_js(
    tx_hex: String,
    resolved_utxos: &JsVecString,
    additional_txs: &JsVecString,
    network: String,
    slot_config: String,
) -> WasmResult {
    let mut deserialized_utxos: Vec<UTxO> = Vec::new();
    for utxo_json in resolved_utxos {
        match serde_json::from_str(utxo_json.as_str()) {
            Ok(utxo) => deserialized_utxos.push(utxo),
            Err(e) => {
                return WasmResult::new_error(
                    "failure".to_string(),
                    format!("Error in decoding UTXO: {:?}", e),
                );
            }
        }
    }

    let deserialize_network = match network.try_into() {
        Ok(network) => network,
        Err(e) => {
            return WasmResult::new_error(
                "failure".to_string(),
                format!("Error in decoding network: {:?}", e),
            );
        }
    };

    let deserialized_slot_config: SlotConfig =
        match serde_json::from_str::<JsonSlotConfig>(slot_config.as_str()) {
            Ok(slot_config) => SlotConfig {
                slot_length: slot_config.slot_length,
                zero_slot: slot_config.zero_slot,
                zero_time: slot_config.zero_time,
            },
            Err(e) => {
                return WasmResult::new_error(
                    "failure".to_string(),
                    format!("Error in decoding slot config: {:?}", e),
                );
            }
        };

    let eval_result = evaluate_tx_scripts(
        &tx_hex,
        &deserialized_utxos,
        additional_txs.as_ref_vec(),
        &deserialize_network,
        &deserialized_slot_config,
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
    slot_config: &SlotConfig,
) -> Result<Vec<EvalResult>, JsError> {
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
        slot_config,
    )
    .map_err(|err| JsError::from_str(&format!("Error occurred during evaluation: {}", err)))
    .map(|reds| reds.into_iter().map(map_eval_result).collect())
}

fn map_eval_result(result: PhaseTwoEvalResult) -> EvalResult {
    match result {
        PhaseTwoEvalResult::Success(redeemer) => {
            EvalResult::Success(map_redeemer_to_action(redeemer))
        }
        PhaseTwoEvalResult::Error(redeemer, err) => {
            EvalResult::Error(map_error_to_eval_error(err, redeemer))
        }
    }
}

fn map_error_to_eval_error(err: UplcError, original_redeemer: Redeemer) -> EvalError {
    match err {
        UplcError::Machine(err, budget, logs) => EvalError {
            index: original_redeemer.index,
            budget: Budget {
                mem: budget.mem as u64,
                steps: budget.cpu as u64,
            },
            tag: map_redeemer_tag(&original_redeemer.tag),
            error_message: format!("{}", err),
            logs,
        },
        UplcError::RedeemerError { err, .. } => match *err {
            UplcError::Machine(err, budget, logs) => EvalError {
                index: original_redeemer.index,
                budget: Budget {
                    mem: budget.mem as u64,
                    steps: budget.cpu as u64,
                },
                tag: map_redeemer_tag(&original_redeemer.tag),
                error_message: format!("{}", err),
                logs,
            },
            _ => EvalError {
                index: original_redeemer.index,
                budget: Budget { mem: 0, steps: 0 },
                tag: map_redeemer_tag(&original_redeemer.tag),
                error_message: format!("{}", err),
                logs: vec![],
            },
        },
        _ => EvalError {
            index: original_redeemer.index,
            budget: Budget { mem: 0, steps: 0 },
            tag: map_redeemer_tag(&original_redeemer.tag),
            error_message: format!("{}", err),
            logs: vec![],
        },
    }
}

fn map_redeemer_to_action(redeemer: Redeemer) -> Action {
    Action {
        index: redeemer.index,
        budget: Budget {
            mem: redeemer.ex_units.mem,
            steps: redeemer.ex_units.steps,
        },
        tag: map_redeemer_tag(&redeemer.tag),
    }
}

fn map_redeemer_tag(tag: &PRedeemerTag) -> RedeemerTag {
    match tag {
        PRedeemerTag::Spend => RedeemerTag::Spend,
        PRedeemerTag::Mint => RedeemerTag::Mint,
        PRedeemerTag::Cert => RedeemerTag::Cert,
        PRedeemerTag::Reward => RedeemerTag::Reward,
        PRedeemerTag::Vote => RedeemerTag::Vote,
        PRedeemerTag::Propose => RedeemerTag::Propose,
    }
}

fn get_cost_mdls(network: &Network) -> Result<CostModels, JsError> {
    let cost_model_list = get_cost_models_from_network(network);
    if cost_model_list.len() < 3 {
        return Err(JsError::from_str(
            "Cost models have to contain at least PlutusV1, PlutusV2, and PlutusV3 costs",
        ));
    };
    Ok(CostModels {
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
                script_ref: to_pallas_script_ref(&utxo.output.script_ref)?,
            }),
        };
        resolved_inputs.push(resolved_input);
    }
    Ok(resolved_inputs)
}

fn to_pallas_script_ref(
    script_ref: &Option<String>,
) -> Result<Option<CborWrap<ScriptRef>>, JsError> {
    if let Some(script_ref) = script_ref {
        let script_bytes = hex::decode(script_ref)
            .map_err(|err| JsError::from_str(&format!("Invalid script hex found: {}", err)))?;

        let pallas_script = ScriptRef::decode_fragment(&script_bytes)
            .map_err(|err| JsError::from_str(&format!("Invalid script found: {}", err)))?;

        Ok(Some(CborWrap(pallas_script)))
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
    use model::UtxoInput;
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
                          script_ref: Some("82025655010000322223253330054a229309b2b1bad0025735".to_string())
                      }
                  }],
            &[],
            &Network::Mainnet,
            &SlotConfig::default()
        );

        let redeemers = result.unwrap();
        let mut redeemer_json = serde_json::Map::new();

        assert_eq!(redeemers.len(), 1);

        let redeemer = match &redeemers[0] {
            EvalResult::Success(redeemer) => Ok(redeemer),
            EvalResult::Error(_) => Err("Unexpected error"),
        }
        .unwrap();

        redeemer_json.insert("index".to_string(), redeemer.index.to_string().into());
        let mut ex_unit_json = serde_json::Map::new();
        ex_unit_json.insert("mem".to_string(), redeemer.budget.mem.into());
        ex_unit_json.insert("steps".to_string(), redeemer.budget.steps.into());
        redeemer_json.insert(
            "ex_units".to_string(),
            serde_json::Value::Object(ex_unit_json),
        );
        assert_eq!(
            serde_json::json!({"ex_units":{"mem":2833,"steps":528893},"index":"0"}).to_string(),
            serde_json::json!(redeemer_json).to_string()
        )
    }

    #[test]
    fn test_eval_2() {
        let result = evaluate_tx_scripts(
            "84a800d90102818258205fefa1a58e20732f7e55972315dda0123b90e5a1d7f086bc37b566f0f9210229000182a300581d71fe23153596f3415c21374454937dd858b1afcce3ee9c7d43fb05a90f01821a00178b44a1581c9e138ea088768af3d2f1216271f2f41172a96b54780225036bc0475ea14001028201d8185859d8799f1b00000193ae9213085820cff20d3f762fecf4f90a186062263bbe07928bff70d50166be88c4c2bffbd10ed8799fd87a9f581cd649c0eb6532b3649e1a2b358fd83ce5778d1d91fdc8d3f6e0539a20ffd87a80ff03ff825839013354e5941d41d08051005f407b50355a3e7ad6bb983016028baae6a1f3f2436fa221ebc2d81481ee126944d7e11256ea793da594e0e033921a00f19ca6021a000eda89031a0860837005a1581df124f5fe1c897b34180e6dcbc57fa7b2435b4d7cf6e8f3773a3edf08250009a1581c9e138ea088768af3d2f1216271f2f41172a96b54780225036bc0475ea140010b582071c6e77365d456e9ed48e7bfbe4c76a5d7f91a999080624f60d40922661c9fc80dd90102818258205fefa1a58e20732f7e55972315dda0123b90e5a1d7f086bc37b566f0f921022900a301d9010281830303878200581cf800e7ea3e1312cccf402b957e568e0d29786efe3f9cb818caadc7e38200581cfcbb77e221db810dfcbb9c9684c9c23e7f4c25868a20ff0701bf619e8200581c2efaf69bb7551328bb8625684bc1cea922ebc5aeca48df09b7b830458200581c0eaabc8286318f13f45d1eb7383e100964a94a01124b184bde55ba9a8200581c804ff6e71914f1ef08e31c86dc3ee7046424b464a6ee1536b38a04e38200581c21c3bd0ab2357a1879a96b948145fee311015430a68617996901b8bb8200581cd0730dfe0cb0140621dcdf751bbea7b1fb03c5eca57779c3fbc07f1706d9010281590b84590b8101000033332323232323232323232322322232232323232322533300f3232323232325333015300b3017375400226464a66602e66ebcdd30009ba633300500248900480084c94ccc060c038c068dd500089919299980d19b8748010c070dd500089919191919299980f980a98109baa00113232533302153330213375e602060486ea80240705288a998112481366f776e5f6f75747075742e61646472657373203d3d20636f6c645f696e74656e745f686f6c645f61646472657373203f2046616c73650014a02a666042a666042008294454cc0892411174746c5f636865636b203f2046616c73650014a02a666042002294454cc089240117617574686f726974795f636865636b203f2046616c73650014a02940528299981019198008009bab302730283028302830283028302830243754602060486ea804c894ccc0980045280992999811992999812180d18131baa00113375e6054604e6ea800406c54cc0952412465787065637420496e6c696e65286372656429203d207769746864726177616c2e31737400163025302900214a22660060060026052002266e1cdd6981318139813981398119baa00500114a06eb4c094c088dd50008a9981024814265787065637420536f6d6528617574686f726974795f6e756d62657229203d206765745f666972737428617574686f726974795f6d61702c2072656465656d6572290016323300100101b22533302400114c0103d87a8000132323253330233375e034604a006266e95200033028375000297ae0133005005002375a604a0046050004604c0026464a66603e602a0022a6604092012a5472616e73616374696f6e2074746c2073686f756c64206861766520616e20757070657220626f756e6400161533301f30140011337126eb4c094c088dd50011bad300e302237540082a66040921365472616e73616374696f6e20757070657220626f756e642073686f756c646e277420626520706f73697469766520696e66696e697479001630203754002601860406ea8c08cc090c080dd51811981218121812181218121812181218101baa300c3020375401ea6660386024603c6ea80044c94ccc08400454cc078070584c8c94ccc08c00454cc080078584c8c94ccc09400454cc088080584c8c94ccc09c00454cc090088584c94ccc0a0c0ac0084c9265333024301a30263754006264a6660520022a6604c0482c26464a6660560022a6605004c2c264a666058605e0042646493192999815181000089929998178008a998160150b0992999818181980109924c64a66605a6046002264a6660640022a6605e05a2c264a666066606c00426493198118008170a998180170b19299999981b80088008a998180170b0a998180170b0a998180170b0a998180170b181a00098181baa0021533302d30220011325333032001153302f02d16132325333034001153303102f161323253330360011533033031161325333037303a002149854cc0d00c858c94cccccc0ec00454cc0d00c85854cc0d00c85854cc0d00c8584dd68008a9981a0190b181c000981c00119299999981c8008a998190180b0a998190180b0a998190180b09bad00115330320301630360013036002325333333037001153303002e16153303002e16153303002e161375a0022a6606005c2c606800260606ea800854cc0b80b058c0b8dd50008a998168158b19299999981a00088008a998168158b0a998168158b0a998168158b0a998168158b181880098169baa0031533302a301f0011533302e302d37540062930a998158148b0a998158148b18159baa0023301c0030271533029027163253333330300011001153302902716153302902716153302902716153302902716302d001302d00232533333302e0011001153302702516153302702516153302702516153302702516302b001302737540062a6604a0462c2a6604a0462c64a6666660580022a6604a0462c2a6604a0462c2a6604a0462c26eb400454cc09408c58c0a4004c0a4008c94cccccc0a8004400454cc08c0845854cc08c0845854cc08c0845854cc08c08458c09c004c09c008c94cccccc0a000454cc08407c5854cc08407c5854cc08407c5854cc08407c584dd7000981280098128011929999998130008a9980f80e8b0a9980f80e8b0a9980f80e8b09bad001153301f01d163023001301f37540022a6603a0362ca66666604600220022a660380342c2a660380342c2a660380342c2a660380342c6040603a6ea800454cc06d2413665787065637420496e6c696e65446174756d286f776e5f6f75747075745f6461746129203d206f776e5f6f75747075742e646174756d00163006301c3754002603c60366ea800454cc065241146f776e206f7574707574206e6f7420666f756e64001632533301c00114c103d87a800013374a90001980e980f000a5eb80c8cc004004dd61802980d9baa3007301b375401444a66603a002297ae013232533301b3375e6e98cc030dd59805980f1baa002488100374c66601200c91100480084cc080008cc0100100044cc010010004c084008c07c00454ccc05ccdd79ba6001374c66600a00491100480045288a503300737566038603a603a603a603a60326ea8c014c064dd500424500375c603660306ea800454cc059240128657870656374204d696e74286f776e5f706f6c6963795f696429203d206374782e707572706f7365001630043017375400c4603460366036002444a66602a6016002297adef6c6013232330010014bd6f7b63011299980e00089980e99bb0375200c6e9800d2f5bded8c0264646464a66603866e400280084cc084cdd81ba900a374c00e00a2a66603866e3c0280084cc084cdd81ba900a374c00e00626604266ec0dd48011ba6001330060060033756603c0066eb8c070008c080008c078004c8cc0040052f5bded8c044a66603600226603866ec0dd48021ba80034bd6f7b630099191919299980d99b90008002133020337606ea4020dd40038028a99980d99b8f008002133020337606ea4020dd400380189981019bb037520046ea0004cc01801800cdd6980e8019bae301b002301f002301d0012301800123017301800122323300100100322533301700114bd6f7b630099191919299980b99b91007002153330173371e00e0042006200a26603866ec0dd48011ba600133006006003375660320066eb8c05c008c06c008c064004526153301049011856616c696461746f722072657475726e65642066616c736500136563300100400b2232533300f3005001132533301400115330110031613253330153018002149854cc04801058c94cccccc06400454cc0480105854cc0480105854cc0480105854cc048010584dd7000980b00098091baa0031533300f3004001132533301400115330110031613253330153018002149854cc04801058c94cccccc06400454cc0480105854cc0480105854cc0480105854cc048010584dd7000980b00098091baa003153301000216301037540046e1d2002370e9000299999980880088008a998050038b0a998050038b0a998050038b0a998050038b1bae00137560029211472656465656d65723a2043726564656e7469616c0049013d657870656374206f776e5f6f75747075745f646174756d3a20436f6c64526566496e74656e74446174756d203d206f776e5f6f75747075745f64617461005734ae7155ceaab9e5573eae815d0aba25748981b0a5d87a9f581c24f5fe1c897b34180e6dcbc57fa7b2435b4d7cf6e8f3773a3edf0825ff03d87a9f581c883af74520be6f27da1ee389aee3c8ce5b11c7fb24d715404caec1c4ff04d87a9f581c059030947d487b5a1230cb9acae8f93fb29cb03d8061f798133d6de3ff05d87a9f581cc844f7034f137e3230540be47a22e3c6693c3bc76f728ada836d1b8fff06d87a9f581c97a5e64f557774e55fda8171feb7d3af93b5567838b3b25ae99990d9ff07004c0129d8799fd87a9f581cfe23153596f3415c21374454937dd858b1afcce3ee9c7d43fb05a90fffd87a80ff004c01165554737974787743327542326c4b654767436b755136000105a182010082d87a9f581c24f5fe1c897b34180e6dcbc57fa7b2435b4d7cf6e8f3773a3edf0825ff821a006acfc01ab2d05e00f5f6",
            &[UTxO {
                input: UtxoInput {
                    tx_hash: "5fefa1a58e20732f7e55972315dda0123b90e5a1d7f086bc37b566f0f9210229".to_string(),
                    output_index: 0
                },
                output: UtxoOutput {
                    address: "addr1qye4fev5r4qapqz3qp05q76sx4dru7kkhwvrq9sz3w4wdg0n7fpklg3pa0pds9ypacfxj3xhuyf9d6ne8kjefc8qxwfqply43d".to_string(),
                    amount: vec![Asset::new_from_str("lovelace", "18350707")],
                    data_hash: None,
                    plutus_data: None,
                    script_hash: None,
                    script_ref: None,
                }
            }],
            &[],
            &Network::Mainnet,
            &SlotConfig::default()
        );
        assert_eq!(
            serde_json::json!([{"success": {"budget":{"mem":184912,"steps":61492185},"index": 0, "tag": "mint"}}]).to_string(),
            serde_json::json!(result.unwrap()).to_string()
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

    #[test]
    fn test_v1_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82015655010000322223253330054a229309b2b1bad0025735".to_string(),
        ))
        .unwrap()
        .unwrap();

        match script_ref.0 {
            ScriptRef::PlutusV1Script(_) => {}
            _ => panic!("Invalid script ref"),
        }
    }

    #[test]
    fn test_v2_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82025655010000322223253330054a229309b2b1bad0025735".to_string(),
        ))
        .unwrap()
        .unwrap();

        match script_ref.0 {
            ScriptRef::PlutusV2Script(_) => {}
            _ => panic!("Invalid script ref"),
        }
    }

    #[test]
    fn test_v3_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82035655010000322223253330054a229309b2b1bad0025735".to_string(),
        ))
        .unwrap()
        .unwrap();

        match script_ref.0 {
            ScriptRef::PlutusV3Script(_) => {}
            _ => panic!("Invalid script ref"),
        }
    }

    #[test]
    fn test_invalid_native_script_ref() {
        let script_ref = to_pallas_script_ref(&Some(
            "82005655010000322223253330054a229309b2b1bad0025735".to_string(),
        ));
        assert!(script_ref.is_err());
    }

    #[test]
    fn test_network_type_decode() {
        let network = Network::Mainnet;
        let network_str = "Mainnet";
        let network_type: Network = network_str.to_string().try_into().unwrap();
        assert_eq!(network, network_type);
    }

    #[test]
    fn test_network_type_decode_error() {
        let network_str = "Invalid";
        let network_type: Result<Network, _> = network_str.to_string().try_into();
        assert!(network_type.is_err());
    }

    #[test]
    fn test_utxo_tx_evaluating() {
        let tx_hex = "84a900818258205de23a200f136e657307bc69173dddaf38b446bd7242a50f5bf255e329018b65030182a300581d70eafce55e4f0e057b495f77d8019577c56ae1fe188dc7e6d63f4f93b801821a001e8480a1581c32b7e3d552b2b18cb9bf1a39e6e1ce75f62c084f2b917a44c071a3bda14001028201d81858b4d8799f582461666461373264392d383039332d343330332d623030652d3233616362323934313432661a06acfc00d8799fd8799f581c5e0abc8c791c220b8c56e729cb77e95c03c7bd27971896dda0ac2368ffd8799fd8799fd8799f581cc8fdacb82c1cec476d444f559c28f4b75ddb6f483fe20427683a661affffffff1b0000019223dc5f75d8799fd8799f58205f759f3527a47632735470586a7ab2fbee4b4aa8b6504d52d52bc62fa8ec961aff00ff01ff825839005e0abc8c791c220b8c56e729cb77e95c03c7bd27971896dda0ac2368c8fdacb82c1cec476d444f559c28f4b75ddb6f483fe20427683a661a1a15a2ae54021a000d5ab0031a0442ee8109a1581c32b7e3d552b2b18cb9bf1a39e6e1ce75f62c084f2b917a44c071a3bda140010b5820c131e4b612c1a50ddcb739f58185148a26c6beaa198d036f5f9e4e8c7d458de20d8382582002345ff40e1b8730434571b8b4749ad084b2cd582aa8997fa7416be8b76c7da80082582002345ff40e1b8730434571b8b4749ad084b2cd582aa8997fa7416be8b76c7da8058258201c4ef054932bafcb4a59810f31fa0ed001d6611066938d1a1aef1d1237c0a441020e81581cc6aa7af71f8ba577246149edf075d2edd9daa63980b7ca176799af6c128382582066b7282bad1aef9ba0a99f06e618d651d232d4788f0d2ee2d22db22a62391033008258205f759f3527a47632735470586a7ab2fbee4b4aa8b6504d52d52bc62fa8ec961a008258201c4ef054932bafcb4a59810f31fa0ed001d6611066938d1a1aef1d1237c0a44100a10581840100d8799fd8799f58205f759f3527a47632735470586a7ab2fbee4b4aa8b6504d52d52bc62fa8ec961aff00ff820101f5f6";
        let utxo_1 = "{\"input\": {\"outputIndex\": 3, \"txHash\": \"5de23a200f136e657307bc69173dddaf38b446bd7242a50f5bf255e329018b65\"}, \"output\": {\"address\": \"addr_test1qrsaqj54nyedfg74tye8743tkrclgnfztj6z937g50q0fwv6vwcuvc9guftgju6xav470f6da9guk8t3nn46wd34z43s99hwxv\", \"amount\": [{\"unit\": \"lovelace\", \"quantity\": \"365858180\"}], \"scriptHash\": null}}";
        let utxo_2 = "{\"input\": {\"outputIndex\": 0, \"txHash\": \"02345ff40e1b8730434571b8b4749ad084b2cd582aa8997fa7416be8b76c7da8\"}, \"output\": {\"address\": \"addr_test1qrsaqj54nyedfg74tye8743tkrclgnfztj6z937g50q0fwv6vwcuvc9guftgju6xav470f6da9guk8t3nn46wd34z43s99hwxv\", \"amount\": [{\"unit\": \"lovelace\", \"quantity\": \"5000000\"}], \"scriptHash\": null}}";
        let utxo_3 = "{\"input\": {\"outputIndex\": 5, \"txHash\": \"02345ff40e1b8730434571b8b4749ad084b2cd582aa8997fa7416be8b76c7da8\"}, \"output\": {\"address\": \"addr_test1qrsaqj54nyedfg74tye8743tkrclgnfztj6z937g50q0fwv6vwcuvc9guftgju6xav470f6da9guk8t3nn46wd34z43s99hwxv\", \"amount\": [{\"unit\": \"lovelace\", \"quantity\": \"5000000\"}], \"scriptHash\": null}}";
        let utxo_4 = "{\"input\": {\"outputIndex\": 0, \"txHash\": \"1c4ef054932bafcb4a59810f31fa0ed001d6611066938d1a1aef1d1237c0a441\"}, \"output\": {\"address\": \"addr_test1qrsaqj54nyedfg74tye8743tkrclgnfztj6z937g50q0fwv6vwcuvc9guftgju6xav470f6da9guk8t3nn46wd34z43s99hwxv\", \"amount\": [{\"unit\": \"lovelace\", \"quantity\": \"123000000\"}], \"scriptHash\": null}}";
        let utxo_5 = "{\"input\": {\"outputIndex\": 2, \"txHash\": \"1c4ef054932bafcb4a59810f31fa0ed001d6611066938d1a1aef1d1237c0a441\"}, \"output\": {\"address\": \"addr_test1qrsaqj54nyedfg74tye8743tkrclgnfztj6z937g50q0fwv6vwcuvc9guftgju6xav470f6da9guk8t3nn46wd34z43s99hwxv\", \"amount\": [{\"unit\": \"lovelace\", \"quantity\": \"5000000\"}], \"scriptHash\": null}}";
        let utxo_6 = "{\"input\": {\"outputIndex\": 0, \"txHash\": \"66b7282bad1aef9ba0a99f06e618d651d232d4788f0d2ee2d22db22a62391033\"}, \"output\": {\"address\": \"addr_test1qp0q40yv0ywzyzuv2mnjnjmha9wq83aay7t339ka5zkzx6xglkktstqua3rk63z02kwz3a9hthdk7jplugzzw6p6vcdqa39gds\", \"amount\": [{\"unit\": \"lovelace\", \"quantity\": \"26000000\"}], \"scriptHash\": \"32b7e3d552b2b18cb9bf1a39e6e1ce75f62c084f2b917a44c071a3bd\",  \"scriptRef\": \"8202590eca590ec701000033232323232323223223232322533300832323232323232323232323232323232323232323232323232323232323232323232323232323232325333031302d30323754002264a66606460540022a666064666018910103313333000063758601660686ea8c080c0d0dd50158a9998191998050031bac301130343754604060686ea80acdd61810181a1baa30203034375405626666444464646464a666074606c60766ea80044c8c8c8c8c8c8c94ccc10402854ccc10402454ccc10401854ccc10400c54ccc10400840045280a5014a0294052819192999821181f0008b0a999821181d000899b89375a608e60886ea8008dd6982398241824182418221baa0061630423754002605c60846ea8c114c118c108dd50059980f99baf301a30413754006605a60826ea8059240132496e74656e7420646174756d2073686f756c6420636f6e7461696e20616c6c206d61746368696e6720726566732075736564003301e3375e605860806ea8010c084c100dd50092493f496e74656e7420746f6b656e2073686f756c64206f6e6c7920657665722062652073656e7420746f20696e74656e74207370656e64696e672073637269707400533303d3039303e37540022646464646464646464646464a666098609e0042646493182280218178038b18268009826801182580098258011bad3049001304900230470013047002375a608a002608a0046086002607e6ea800458cc0640092401264f7574707574206d75737420636f6e7461696e20736f6d6520696e6c696e656420646174756d003301b3024301c323300100137566036607c6ea8008894ccc10000452f5c0264666444646600200200644a66608c0022006264660906e9ccc120dd4803198241822800998241823000a5eb80cc00c00cc128008c120004dd7181f8009bab304000133003003304400230420014901224f6e6c792061646120616c6c6f776564207769746820696e74656e7420746f6b656e00303f303c37540022ca6660780062980103d87a80001302d3303d303e0034bd701980c1818181380124812e4f6e6c7920612073696e676c65206f7574707574207769746820696e74656e7420746f6b656e20616c6c6f77656400330173375e6e98c04c00cdd31991299981c9818a4000297adef6c6013232330010014bd6f7b63011299981f80089982019bb0375200a6e9800d2f5bded8c0264646464a66608066e400240084cc110cdd81ba9009374c00e00a2a66608066e3c0240084cc110cdd81ba9009374c00e00626608866ec0dd48011ba600133006006003375660820066eb8c0fc008c10c008c104004c8cc0040052f5bded8c044a66607c00226607e66ec0dd4801a610101004bd6f7b630099191919299981f99b90007002133043337606ea401d3010101000051533303f3371e00e00426608666ec0dd4803a61010100003133043337606ea4008dd4000998030030019bad3040003375c607c004608400460800026eb8c094c0e4dd50029bae30163039375400a9212b4f6e6c7920612073696e676c6520696e74656e7420746f6b656e2073686f756c64206265206d696e74656400330280032302f3330153756602c60726ea8004dd71812981c9baa005375c602c60726ea8014c098cc0d8dd48021981b2610140004bd701bac301530343754604060686ea80acdd59806981a1baa302030343754056606e607060706070607060706070607060686ea8c080c0d0dd50158a5014a02a66606466e2000520001533303233300c488103313838000063758601660686ea8c080c0d0dd501589998050031bac301130343754604060686ea80acdd61810181a1baa302030343754056294058dd6981b18199baa00116533303030283301501500113301b0014890014c103d87a8000325333030302830313754002297adef6c6013756606a60646ea8004cc064c02cdd5980518189baa301d3031375405000264a66605e605660606ea80044dd7181a18189baa00116300d3030375404ea66605a6052605c6ea80044c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c8c94ccc108c1140084c8c8c8c8c8c8c926302b00b302a00c302a00d302900e302700f303b010533303f303b3040375402226464a666088608e004264931981b000919181798228011bae3043001163756608a00260826ea80445858c10c004c10c008dd698208009820801181f800981f801181e800981e801181d800981d801181c800981c801181b800981b801181a800981a801181980098179baa00116300f302e3754004a666056604e60586ea80044c8c8c8c8c8c94ccc0d0c0dc0084c8c926301a00430190051630350013035002303300130330023031001302d37540022c6600e601260586ea800524011e436f6c642072656620646174756d206d75737420626520696e6c696e6564003232533302b323300100102a22533303000114a0264a66605c64604e66601a6eacc038c0c4dd5180718189baa005375c603a60626ea8004dd7180718189baa001303300214a2266006006002606600220022ca666054604c60566ea80044c0bcc0b0dd50008b198069bac3008302b3754602e60566ea8088098888c94ccc0b14ccc0b0c090c0b4dd519808001980598171baa0041614a2200229414ccc0acc09cc0b0dd519807800980518169baa0031614a24605a605c605c605c605c605c605c605c605c00244464660020026464a6660586050605a6ea80044c0c4c0b8dd50008992999816981498171baa00113032302f37540022c6603e00491010130003301e0010053756603060586ea8c060c0b0dd5001912999815981398161baa001132323300100100522533303100114a0264a66605e66e3cdd7181a0010020a511330030030013034001375c6060605a6ea80044c8cdc49bad30313032001301b3301e375860620024660080080026eb0c0c0c0b4dd5000918159816181618161816000919198008008011129998150008a5eb7bdb1804c8c8c8c94ccc0accdc8a45000021533302b3371e9101000021003100513302f337606ea4008dd3000998030030019bab302c003375c6054004605c00460580024464a66604c601e604e6ea80044c0acc0a0dd50008b180418139baa002222325333026301e302737540022900009bad302b3028375400264a66604c603c604e6ea8004530103d87a80001330113756605660506ea8004008cc03c00c0088c09cc0a0004894ccc0880085288b18008009129998118008a40002602266004004604c002460466048604800244646600200200644a666046002298103d87a80001323253330223375e602060486ea80080144c058cc0980092f5c0266008008002604e004604a00244a66604000229000098071980100118118009299980d980b980e1baa0011323232325333022302500213232498c94ccc084c0740044c8c94ccc098c0a40084c9263019001163027001302337540062a66604260320022a66604860466ea800c526161630213754004602c0062c604600260460046042002603a6ea80045894ccc068c058c06cdd5000899191919299981098120010a4c2c6eb8c088004c088008dd71810000980e1baa00116232533301a301600113232533301f3022002149858dd71810000980e1baa0021533301a301200113232533301f3022002132498c05c00458c080004c070dd50010a99980d180180089919299980f981100109924c60240022c604000260386ea800854ccc068cdc3a400c00226464a66603e60440042649319299980e980c800899192999811181280109924c602a0022c6046002603e6ea800854ccc074c0540044c8c94ccc088c0940084c9263015001163023001301f37540042a66603a600c002264646464a666048604e00426493180b8018b1bae302500130250023023001301f37540042a66603a66e1d200600113232323253330243027002149858dd7181280098128011bae3023001301f37540042a66603a66e1d200800113232323253330243027002149858dd6981280098128011bae3023001301f37540042a66603a66e1d200a00115333020301f37540042930b0a99980e99b874803000454ccc080c07cdd50010a4c2c2c603a6ea800458c080004c070dd50010b180d1baa001370e900211191980080080191299980e8008a60103d87a8000132323232533301e3372200e0042a66603c66e3c01c0084c048cc088dd3000a5eb80530103d87a80001330060060033756603e0066eb8c074008c084008c07c00488c8cc00400400c894ccc0700045300103d87a8000132323232533301d3372200e0042a66603a66e3c01c0084c044cc084dd4000a5eb80530103d87a8000133006006003375a603c0066eb8c070008c080008c078004c00400488c94ccc058c0480044c8c94ccc06cc07800852616375c603800260306ea800854ccc058c0380044c8c94ccc06cc0780084c926323232533301e3021002132498cc04000c8cc02402400458dd6980f800980f8011bac301d001163758603800260306ea800858c058dd50009180c000980080091299980a8008a4000260066600400460300026e01200222323300100100322533301500114bd7009919299980a180280109980c00119802002000899802002000980c801180b80091191980080080191299980a0008a6103d87a800013232323253330153372200e0042a66602a66e3c01c0084c024cc0640052f5c0298103d87a80001330060060033016003375c60280046030004602c0026e95200022323300100100322533301200114984c8cc00c00cc058008c00cc0500048c94ccc030c0200044c8c94ccc044c0500084c9263005001163012001300e37540042a66601860080022646464646464a66602a60300042930b1bad30160013016002375a602800260280046eb4c048004c038dd50010b18061baa001232533300b30070011323253330103013002149858dd7180880098069baa0021533300b30030011323253330103013002149858dd7180880098069baa00216300b37540026e1d200214984d958c00400c94ccc018c008c01cdd50008991919192999806980800109924ca666014600c60166ea800c4c8c94ccc03cc04800852616375c602000260186ea800c5858dd698070009807001180600098041baa00116370e90001bac0015734aae7555cf2ab9f5740ae855d12601259fd8799f581c55871db8a58f10fddf48b92029827d168271e785646af257de85486c40ffff0001\"}}";
        let utxo_7 = "{\"input\": {\"outputIndex\": 0, \"txHash\": \"5f759f3527a47632735470586a7ab2fbee4b4aa8b6504d52d52bc62fa8ec961a\"}, \"output\": {\"address\": \"addr_test1qp0q40yv0ywzyzuv2mnjnjmha9wq83aay7t339ka5zkzx6xglkktstqua3rk63z02kwz3a9hthdk7jplugzzw6p6vcdqa39gds\", \"amount\": [{\"unit\": \"lovelace\", \"quantity\": \"4000000\"}, {\"unit\": \"55871db8a58f10fddf48b92029827d168271e785646af257de85486c\", \"quantity\": \"1\"}], \"dataHash\": \"a5a21bf7d9119e01f8cf04316dba1d9cdcacd998430728fdb938de8dad4b6c1f\", \"plutusData\": \"d8799fd8799f581c0eb1e4ef980b3c308edd7b3ab64990d5240e5230286038cae5c634cdffd8799f581c1fdf3668220c08618b4f2e5e7cc54f70ca4b11fd2b6a3fbe678235f1ffd8799fd8799fa14130d8799f581cc6aa7af71f8ba577246149edf075d2edd9daa63980b7ca176799af6cffffd8799fd8799f58201c4ef054932bafcb4a59810f31fa0ed001d6611066938d1a1aef1d1237c0a441ff00ffd8799fd87a9f581ceafce55e4f0e057b495f77d8019577c56ae1fe188dc7e6d63f4f93b8ffd87a80ffd8799f581c32b7e3d552b2b18cb9bf1a39e6e1ce75f62c084f2b917a44c071a3bd40ffd8799f581cdbde35dec305604c1c8a596437802fd5e3468c8e92ce1781203e7bb74455534441ffd8799fd87a9f581c10fcea52f80dbc6e499106b02894173a2e60d21c6b3d1fe881d9495dffd87a80ffd8799fd8799f581cc6aa7af71f8ba577246149edf075d2edd9daa63980b7ca176799af6cffd87a80ff1903e801ffff\", \"scriptHash\": null}}";
        let mut resolved_utxos = JsVecString::new();
        resolved_utxos.add(utxo_1.to_string());
        resolved_utxos.add(utxo_2.to_string());
        resolved_utxos.add(utxo_3.to_string());
        resolved_utxos.add(utxo_4.to_string());
        resolved_utxos.add(utxo_5.to_string());
        resolved_utxos.add(utxo_6.to_string());
        resolved_utxos.add(utxo_7.to_string());

        let additional_txs = JsVecString::new();

        let result = evaluate_tx_scripts_js(
            tx_hex.to_string(),
            &resolved_utxos,
            &additional_txs,
            "preprod".to_string(),
            serde_json::to_string(&JsonSlotConfig {
                slot_length: 1000,
                zero_slot: 4492800,
                zero_time: 1596059091000,
            })
            .unwrap(),
        );

        assert_eq!(result.get_status(), "success");

        let results: Vec<EvalResult> = serde_json::from_str(&result.get_data()).unwrap();
        assert_eq!(results.len(), 1);

        let result = &results[0];
        let redeemer = match result {
            EvalResult::Success(redeemer) => redeemer,
            EvalResult::Error(_) => panic!("Unexpected error"),
        };

        assert_eq!(redeemer.budget.mem, 508703);
        assert_eq!(redeemer.budget.steps, 164980381);
        assert_eq!(redeemer.tag, RedeemerTag::Mint);
        assert_eq!(redeemer.index, 0);
    }

    #[test]
    fn test_utxo_tx_evaluating_error() {
        let tx_hex = "84a700d901028182582047ce1b14c0599bb377a3c73c20973e49adbd10e5090129879b068ca0aa4216c2000181825839003f1b5974f4f09f0974be655e4ce94f8a2d087df378b79ef3916c26b2b1f70b573b204c6695b8f66eb6e7c78c55ede9430024ebec6fd5f85d821b0000000252c63160a2581c0f6b02150cbcc7fedafa388abcc41635a9443afb860100099ba40f07a1446d65736801581cf1c9053e4e03414fc37092d0155682f96d20770afc13a07f00f057ffa14001021a000c6b250758207564366f82807a253ef1f25af3f04486ac49ecd7fb631da76a713b32580994d709a1581cf1c9053e4e03414fc37092d0155682f96d20770afc13a07f00f057ffa140010b582001208ac891cd1aefe984b233bb0f9c4ece236b172c279d14d4940a483d68ccb00dd90102818258206213898aa37d5e585721f4ebdd16da2ac6cd9cd0e81318906dfeea3ebdf9f15700a207d901028158a0589e01010032323232323225333002323232323253330073370e900018049baa00113253300949010f5468697320697320612074726163650016375c601660146ea800454cc02124010f5468697320697320612074726163650016300a300b003300900230080023008001300537540022930a99801a491856616c696461746f722072657475726e65642066616c736500136565734ae7155ceaab9e5742ae8905a182010082d8799f446d657368ff821a006acfc01ab2d05e00f5a11902d1a178386631633930353365346530333431346663333730393264303135353638326639366432303737306166633133613037663030663035376666a1646d657368a46b6465736372697074696f6e783254686973204e465420776173206d696e746564206279204d657368202868747470733a2f2f6d6573686a732e6465762f292e65696d6167657835697066733a2f2f516d527a6963705265757477436b4d36616f74754b6a4572464355443231334470775071364279757a4d4a617561696d656469615479706569696d6167652f6a7067646e616d656a4d65736820546f6b656e";
        let utxos = vec![
            UTxO {
                input: UtxoInput {
                    tx_hash: "47ce1b14c0599bb377a3c73c20973e49adbd10e5090129879b068ca0aa4216c2".to_string(),
                    output_index: 0
                },
                output: UtxoOutput {
                    address: "addr_test1qql3kkt57ncf7zt5hej4un8ff79z6zra7dut08hnj9kzdv437u94wweqf3nftw8kd6mw03uv2hk7jscqyn47cm74lpwsju87pd".to_string(),
                    amount: vec![Asset::new_from_str("lovelace", "9979468933"), Asset::new_from_str("0f6b02150cbcc7fedafa388abcc41635a9443afb860100099ba40f076d657368", "1")],
                    data_hash: None,
                    plutus_data: None,
                    script_hash: None,
                    script_ref: None,
                }
            },
            UTxO {
                input: UtxoInput {
                    tx_hash: "6213898aa37d5e585721f4ebdd16da2ac6cd9cd0e81318906dfeea3ebdf9f157".to_string(),
                    output_index: 0
                },
                output: UtxoOutput {
                    address: "addr_test1qql3kkt57ncf7zt5hej4un8ff79z6zra7dut08hnj9kzdv437u94wweqf3nftw8kd6mw03uv2hk7jscqyn47cm74lpwsju87pd".to_string(),
                    amount: vec![Asset::new_from_str("lovelace", "20000000")],
                    data_hash: None,
                    plutus_data: None,
                    script_hash: None,
                    script_ref: None,
                }
                }
            ];

        let mut resolved_utxos = JsVecString::new();
        for utxo in utxos {
            let utxo_str = serde_json::to_string(&utxo).unwrap();
            resolved_utxos.add(utxo_str);
        }

        let additional_txs = JsVecString::new();

        let result = evaluate_tx_scripts_js(
            tx_hex.to_string(),
            &resolved_utxos,
            &additional_txs,
            "preprod".to_string(),
            serde_json::to_string(&JsonSlotConfig {
                slot_length: 1000,
                zero_slot: 0,
                zero_time: 1666656000000,
            })
            .unwrap(),
        );

        assert_eq!(result.get_status(), "success");
        println!("{}", result.get_data());

        let results: Vec<EvalResult> = serde_json::from_str(&result.get_data()).unwrap();
        assert_eq!(results.len(), 1);

        let result = &results[0];
        let error_result = match result {
            EvalResult::Success(_) => panic!("Unexpected error"),
            EvalResult::Error(error) => error,
        };

        assert_eq!(error_result.budget.mem, 550);
        assert_eq!(error_result.budget.steps, 1203691);
        assert_eq!(error_result.tag, RedeemerTag::Mint);
        assert_eq!(error_result.index, 0);
        assert_eq!(
            error_result.error_message,
            "the validator crashed / exited prematurely"
        );
        assert_eq!(error_result.logs, ["This is a trace"]);
    }

    #[test]
    fn config_test() {
        println!(
            "{:?}",
            serde_json::to_string(&JsonSlotConfig {
                slot_length: 1000,
                zero_slot: 0,
                zero_time: 1666656000000,
            })
            .unwrap()
        );
    }
}
