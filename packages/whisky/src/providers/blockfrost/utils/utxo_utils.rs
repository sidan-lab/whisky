use whisky_common::{
    models::{Asset, UTxO, UtxoInput, UtxoOutput},
    WError,
};
use whisky_csl::{
    apply_double_cbor_encoding,
    csl::{self, NativeScript, PlutusScript, ScriptRef},
};

use crate::providers::blockfrost::{
    models::{utxo::BlockfrostUtxo, Script, Type},
    BlockfrostProvider,
};

#[derive(Debug, Clone)]
pub enum ScriptType {
    Plutus(PlutusScript),
    Native(NativeScript),
}

impl BlockfrostProvider {
    pub async fn to_utxo(&self, utxo: &BlockfrostUtxo) -> Result<UTxO, WError> {
        let utxo = UTxO {
            input: UtxoInput {
                output_index: utxo.output_index as u32,
                tx_hash: utxo.tx_hash.clone(),
            },
            output: UtxoOutput {
                address: utxo.address.clone(),
                amount: utxo
                    .amount
                    .iter()
                    .map(|asset| Asset::new(asset.unit.clone(), asset.quantity.to_string()))
                    .collect(),
                data_hash: utxo.data_hash.clone(),
                plutus_data: utxo.inline_datum.clone(),
                script_ref: match &utxo.reference_script_hash {
                    Some(s) => self
                        .resolve_script_ref(s)
                        .await
                        .map_err(WError::from_err("resolve_script_ref"))?,
                    None => None,
                },
                script_hash: utxo.reference_script_hash.clone(),
            },
        };
        Ok(utxo)
    }

    pub async fn resolve_script_ref(&self, script_hash: &str) -> Result<Option<String>, WError> {
        let script: Script = self
            .blockfrost_client
            .fetch_specific_script(script_hash)
            .await?;
        match script.r#type {
            Type::Timelock => {
                let script_json = self
                    .blockfrost_client
                    .fetch_native_script_json(script_hash)
                    .await?;
                let script: NativeScript =
                    NativeScript::from_json(&serde_json::json!(script_json).to_string())
                        .map_err(WError::from_err("json to string"))?;
                let script_ref = to_script_ref(&ScriptType::Native(script));
                Ok(Some(
                    script_ref
                        .native_script()
                        .ok_or_else(WError::from_opt("resolve_script_ref", "script_ref"))?
                        .to_hex(),
                ))
            }
            Type::PlutusV1 => {
                let script_cbor = self
                    .blockfrost_client
                    .fetch_plutus_script_cbor(script_hash)
                    .await?;

                let normalized = normalize_plutus_script(&script_cbor)
                    .map_err(WError::from_err("normalize_plutus_script"))?;
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v1(),
                )
                .map_err(WError::from_err("from_hex_with_version"))?;
                let script_ref: ScriptRef = to_script_ref(&ScriptType::Plutus(script));
                let result = Some(
                    script_ref
                        .plutus_script()
                        .ok_or_else(WError::from_opt("resolve_script_ref", "script_ref"))?
                        .to_hex(),
                );

                Ok(result)
            }
            Type::PlutusV2 => {
                let script_cbor = self
                    .blockfrost_client
                    .fetch_plutus_script_cbor(script_hash)
                    .await?;

                let normalized = normalize_plutus_script(&script_cbor)
                    .map_err(WError::from_err("normalize_plutus_script"))?;
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v2(),
                )
                .map_err(WError::from_err("from_hex_with_version"))?;
                let script_ref: ScriptRef = to_script_ref(&ScriptType::Plutus(script));
                let result = Some(
                    script_ref
                        .plutus_script()
                        .ok_or_else(WError::from_opt("resolve_script_ref", "script_ref"))?
                        .to_hex(),
                );

                Ok(result)
            }
            Type::PlutusV3 => {
                let script_cbor = self
                    .blockfrost_client
                    .fetch_plutus_script_cbor(script_hash)
                    .await?;

                let normalized = normalize_plutus_script(&script_cbor)
                    .map_err(WError::from_err("normalize_plutus_script"))?;
                let script: PlutusScript = PlutusScript::from_hex_with_version(
                    &normalized,
                    &csl::Language::new_plutus_v3(),
                )
                .map_err(WError::from_err("from_hex_with_version"))?;
                let script_ref: ScriptRef = to_script_ref(&ScriptType::Plutus(script));
                let result = Some(
                    script_ref
                        .plutus_script()
                        .ok_or_else(WError::from_opt("resolve_script_ref", "script_ref"))?
                        .to_hex(),
                );

                Ok(result)
            }
        }
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
