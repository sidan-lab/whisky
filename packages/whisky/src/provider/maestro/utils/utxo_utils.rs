use whisky_common::{
    models::{Asset, UTxO, UtxoInput, UtxoOutput},
    WError,
};
use whisky_csl::{
    apply_double_cbor_encoding,
    csl::{self, JsError, NativeScript, PlutusScript, ScriptRef},
};

use crate::provider::maestro::models::utxo::Utxo;

#[derive(Debug, Clone)]
pub enum Script {
    Plutus(PlutusScript),
    Native(NativeScript),
}

pub fn to_utxo(utxo: &Utxo) -> UTxO {
    UTxO {
        input: UtxoInput {
            output_index: utxo.index as u32,
            tx_hash: utxo.tx_hash.clone(),
        },
        output: UtxoOutput {
            address: utxo.address.clone(),
            amount: utxo
                .assets
                .iter()
                .map(|asset| Asset::new(asset.unit.clone(), asset.amount.to_string()))
                .collect(),
            data_hash: utxo.datum.as_ref().and_then(|datum| {
                datum
                    .get("hash")
                    .and_then(|hash| hash.as_str().map(|s| s.to_string()))
            }),
            plutus_data: utxo.datum.as_ref().and_then(|datum| {
                datum
                    .get("bytes")
                    .and_then(|hash| hash.as_str().map(|s| s.to_string()))
            }),
            script_ref: resolve_script(utxo).unwrap(),
            script_hash: utxo
                .reference_script
                .as_ref()
                .map(|script| script.hash.clone()),
        },
    }
}

pub fn resolve_script(utxo: &Utxo) -> Result<Option<String>, JsError> {
    if let Some(ref_script) = &utxo.reference_script {
        match ref_script.r#type.as_str() {
            "native" => {
                let script: NativeScript =
                    NativeScript::from_json(&serde_json::json!(&ref_script.json).to_string())?;
                let script_ref = to_script_ref(&Script::Native(script));
                Ok(Some(script_ref.native_script().unwrap().to_hex()))
            }
            "plutusv1" => {
                let script_hex = &ref_script.bytes;
                let normalized = normalize_plutus_script(script_hex).unwrap();
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v1(),
                )?;
                let script_ref = to_script_ref(&Script::Plutus(script));
                Ok(Some(script_ref.plutus_script().unwrap().to_hex()))
            }
            "plutusv2" => {
                let script_hex = &ref_script.bytes;
                let normalized = normalize_plutus_script(script_hex).unwrap();
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v2(),
                )?;
                let script_ref = to_script_ref(&Script::Plutus(script));
                Ok(Some(script_ref.plutus_script().unwrap().to_hex()))
            }
            _ => Err(JsError::from_str("Unsupported script type")),
        }
    } else {
        Ok(None)
        // TODO: handle none
    }
}

pub fn normalize_plutus_script(script_hex: &str) -> Result<String, WError> {
    apply_double_cbor_encoding(script_hex)
}

pub fn to_script_ref(script: &Script) -> ScriptRef {
    match script {
        Script::Plutus(plutus) => ScriptRef::new_plutus_script(plutus),
        Script::Native(native) => ScriptRef::new_native_script(native),
    }
}
