use whisky_common::{
    models::{Asset, UTxO, UtxoInput, UtxoOutput},
    WError,
};
use whisky_csl::{
    apply_double_cbor_encoding,
    csl::{self, NativeScript, PlutusScript, ScriptRef},
};

use crate::maestro::models::{utxo::Utxo, ScriptVersion};

#[derive(Debug, Clone)]
pub enum ScriptType {
    Plutus(PlutusScript),
    Native(NativeScript),
}

pub fn to_utxo(utxo: &Utxo) -> Result<UTxO, WError> {
    let utxo = UTxO {
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
            script_ref: resolve_script(utxo).map_err(WError::from_err("to_utxo - script_ref"))?,
            script_hash: utxo
                .reference_script
                .as_ref()
                .map(|script| script.hash.clone()),
        },
    };
    Ok(utxo)
}

pub fn resolve_script(utxo: &Utxo) -> Result<Option<String>, WError> {
    if let Some(ref_script) = &utxo.reference_script {
        match ref_script.r#type {
            ScriptVersion::Native => {
                let script: NativeScript =
                    NativeScript::from_json(&serde_json::json!(&ref_script.json).to_string())
                        .map_err(WError::from_err("json to string"))?;
                let script_ref = to_script_ref(&ScriptType::Native(script));
                Ok(Some(
                    script_ref
                        .native_script()
                        .ok_or_else(WError::from_opt("resolve_script", "script_ref"))?
                        .to_hex(),
                ))
            }
            ScriptVersion::Plutusv1 => {
                let script_hex = &ref_script.bytes;
                let normalized = normalize_plutus_script(script_hex)
                    .map_err(WError::from_err("normalize_plutus_script"))?;
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v1(),
                )
                .map_err(WError::from_err("from_hex_with_version"))?;
                let script_ref = to_script_ref(&ScriptType::Plutus(script));
                Ok(Some(
                    script_ref
                        .plutus_script()
                        .ok_or_else(WError::from_opt("resolve_script", "script_ref"))?
                        .to_hex(),
                ))
            }
            ScriptVersion::Plutusv2 => {
                let script_hex = &ref_script.bytes;
                let normalized = normalize_plutus_script(script_hex)
                    .map_err(WError::from_err("normalize_plutus_script"))?;
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v2(),
                )
                .map_err(WError::from_err("from_hex_with_version"))?;
                let script_ref = to_script_ref(&ScriptType::Plutus(script));
                Ok(Some(
                    script_ref
                        .plutus_script()
                        .ok_or_else(WError::from_opt("resolve_script", "script_ref"))?
                        .to_hex(),
                ))
            }
            ScriptVersion::Plutusv3 => {
                let script_hex = &ref_script.bytes;
                let normalized = normalize_plutus_script(script_hex)
                    .map_err(WError::from_err("normalize_plutus_script"))?;
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v3(),
                )
                .map_err(WError::from_err("from_hex_with_version"))?;
                let script_ref = to_script_ref(&ScriptType::Plutus(script));
                Ok(Some(
                    script_ref
                        .plutus_script()
                        .ok_or_else(WError::from_opt("resolve_script", "script_ref"))?
                        .to_hex(),
                ))
            }
        }
    } else {
        Ok(None)
        // TODO: handle none
    }
}

pub fn normalize_plutus_script(script_hex: &str) -> Result<String, WError> {
    apply_double_cbor_encoding(script_hex)
}

pub fn to_script_ref(script: &ScriptType) -> ScriptRef {
    match script {
        ScriptType::Plutus(plutus) => ScriptRef::new_plutus_script(plutus),
        ScriptType::Native(native) => ScriptRef::new_native_script(native),
    }
}
