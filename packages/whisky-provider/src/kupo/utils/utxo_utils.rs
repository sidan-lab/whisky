use whisky_common::{Asset, UTxO, UtxoInput, UtxoOutput, WError};
use whisky_csl::csl::{self, NativeScript, PlutusScript};

use crate::{
    kupo::{
        models::{KupoUtxo, KupoValue, Script, ScriptVersion},
        KupoProvider,
    },
    normalize_plutus_script, to_script_ref, ScriptType,
};

impl KupoProvider {
    pub async fn to_utxo(&self, utxo: &KupoUtxo) -> Result<UTxO, WError> {
        let utxo = UTxO {
            input: UtxoInput {
                output_index: utxo.output_index as u32,
                tx_hash: utxo.tx_hash.clone(),
            },
            output: UtxoOutput {
                address: utxo.address.clone(),
                amount: self.to_amount(&utxo.value)?,
                data_hash: utxo.datum.clone(),
                plutus_data: utxo.datum.clone(),
                script_ref: self.resolve_script(utxo.script.clone())?,
                script_hash: utxo.script_hash.clone(),
            },
        };
        Ok(utxo)
    }

    pub fn to_amount(&self, value: &KupoValue) -> Result<Vec<Asset>, WError> {
        let mut amount = Vec::new();
        amount.push(Asset::new("lovelace".to_string(), value.coins.to_string()));

        for (unit, quantity) in &value.assets {
            let asset = unit.split(".").collect::<Vec<_>>();
            let (policy_id, asset_name) = if asset.len() == 2 {
                (asset[0].to_string(), asset[1].to_string())
            } else {
                (asset[0].to_string(), "".to_string())
            };
            amount.push(Asset::new(
                format!("{}{}", policy_id, asset_name),
                quantity.to_string(),
            ));
        }
        Ok(amount)
    }

    pub fn resolve_script(&self, script: Option<Script>) -> Result<Option<String>, WError> {
        if let Some(ref_script) = script {
            match ref_script.language {
                ScriptVersion::Native => {
                    let script_hex = &ref_script.script;
                    let script: NativeScript = NativeScript::from_hex(script_hex)
                        .map_err(WError::from_err("from_hex_with_native"))?;
                    let script_ref = to_script_ref(&ScriptType::Native(script));
                    let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));
                    Ok(result)
                }
                ScriptVersion::Plutusv1 => {
                    let script_hex = &ref_script.script;
                    let normalized = normalize_plutus_script(script_hex)
                        .map_err(WError::from_err("normalize_plutus_script"))?;
                    let script: PlutusScript = PlutusScript::from_hex_with_version(
                        &normalized,
                        &csl::Language::new_plutus_v1(),
                    )
                    .map_err(WError::from_err("from_hex_with_version"))?;
                    let script_ref = to_script_ref(&ScriptType::Plutus(script));
                    let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));
                    Ok(result)
                }
                ScriptVersion::Plutusv2 => {
                    let script_hex = &ref_script.script;
                    let normalized = normalize_plutus_script(script_hex)
                        .map_err(WError::from_err("normalize_plutus_script"))?;
                    let script: PlutusScript = PlutusScript::from_hex_with_version(
                        &normalized,
                        &csl::Language::new_plutus_v2(),
                    )
                    .map_err(WError::from_err("from_hex_with_version"))?;
                    let script_ref = to_script_ref(&ScriptType::Plutus(script));
                    let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));
                    Ok(result)
                }
                ScriptVersion::Plutusv3 => {
                    let script_hex = &ref_script.script;
                    let normalized = normalize_plutus_script(script_hex)
                        .map_err(WError::from_err("normalize_plutus_script"))?;
                    let script: PlutusScript = PlutusScript::from_hex_with_version(
                        &normalized,
                        &csl::Language::new_plutus_v3(),
                    )
                    .map_err(WError::from_err("from_hex_with_version"))?;
                    let script_ref = to_script_ref(&ScriptType::Plutus(script));
                    let result = Some(hex::encode(script_ref.to_unwrapped_bytes()));
                    Ok(result)
                }
            }
        } else {
            Ok(None)
            // TODO: handle none
        }
    }
}
