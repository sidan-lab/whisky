use whisky_common::{
    models::{Asset, UTxO, UtxoInput, UtxoOutput},
    WError,
};
use whisky_csl::csl::{self, NativeScript, PlutusScript, ScriptRef};

use crate::{
    blockfrost::{
        models::{utxo::BlockfrostUtxo, Script, Type},
        BlockfrostProvider,
    },
    normalize_plutus_script, to_script_ref, ScriptType,
};

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
                let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));
                Ok(result)
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
                let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));

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
                let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));

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
                let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));

                Ok(result)
            }
        }
    }
}
