use crate::*;
use uplc::tx::SlotConfig;
use whisky_common::*;
use whisky_csl::*;

#[wasm_bindgen]
pub fn js_evaluate_tx_scripts(
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
