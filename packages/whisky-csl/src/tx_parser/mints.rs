use super::CSLParser;
use crate::csl;
use crate::tx_parser::context::{ParserContext, RedeemerIndex, Script};
use whisky_common::{
    MintItem, MintParameter, ScriptMint, ScriptSource, SimpleScriptMint, SimpleScriptSource, WError,
};

impl CSLParser {
    pub fn get_mints(&self) -> &Vec<MintItem> {
        &self.tx_body.mints
    }

    pub(super) fn extract_mints(&mut self) -> Result<(), WError> {
        let mints = self.csl_tx_body.mint();
        if mints.is_none() {
            return Ok(());
        }
        let mints = mints.unwrap();
        self.tx_body.mints = csl_mint_to_mint_item(&mints, &self.context)?;
        Ok(())
    }
}

pub fn csl_mint_to_mint_item(
    mint: &csl::Mint,
    context: &ParserContext,
) -> Result<Vec<MintItem>, WError> {
    let mut mint_items = Vec::new();
    let mint_keys = mint.keys();
    let mint_keys_len = mint_keys.len();
    for i in 0..mint_keys_len {
        let policy_id = mint_keys.get(i);
        let policy_id_hex = policy_id.to_hex();
        let multiple_asstes = mint.get(&policy_id).ok_or_else(|| {
            WError::new(
                "csl_mint_to_mint_item",
                &format!("Failed to get assets for policy ID: {}", policy_id_hex),
            )
        })?;
        let assets = multiple_asstes
            .get(multiple_asstes.len() - 1)
            .ok_or_else(|| {
                WError::new(
                    "csl_mint_to_mint_item",
                    &format!("Failed to get assets for policy ID: {}", policy_id_hex),
                )
            })?;
        let assets_len = assets.len();
        let assets_names = assets.keys();

        for j in 0..assets_len {
            let asset_name = assets_names.get(j);
            let asset_name_hex = hex::encode(asset_name.name());
            let amount = assets
                .get(&asset_name)
                .ok_or_else(|| {
                    WError::new(
                        "csl_mint_to_mint_item",
                        &format!("Failed to get amount for asset: {}", asset_name_hex),
                    )
                })?
                .to_str()
                .parse::<i128>()
                .map_err(|e| {
                    WError::new(
                        "csl_mint_to_mint_item",
                        &format!("Failed to parse amount: {}", e),
                    )
                })?;

            // Get the script witness for this policy_id
            let script = context.script_witness.scripts.get(&policy_id);
            match script {
                Some(Script::ProvidedNative(native_script)) => {
                    mint_items.push(MintItem::SimpleScriptMint(SimpleScriptMint {
                        mint: MintParameter {
                            policy_id: policy_id_hex.clone(),
                            asset_name: asset_name_hex.clone(),
                            amount,
                        },
                        script_source: Some(SimpleScriptSource::ProvidedSimpleScriptSource(
                            native_script.clone(),
                        )),
                    }));
                }
                Some(Script::ReferencedNative(inline_script)) => {
                    mint_items.push(MintItem::SimpleScriptMint(SimpleScriptMint {
                        mint: MintParameter {
                            policy_id: policy_id_hex.clone(),
                            asset_name: asset_name_hex.clone(),
                            amount,
                        },
                        script_source: Some(SimpleScriptSource::InlineSimpleScriptSource(
                            inline_script.clone(),
                        )),
                    }));
                }
                Some(Script::ProvidedPlutus(plutus_script)) => {
                    let redeemer = context
                        .script_witness
                        .redeemers
                        .get(&RedeemerIndex::Mint(i))
                        .cloned();
                    mint_items.push(MintItem::ScriptMint(ScriptMint {
                        mint: MintParameter {
                            policy_id: policy_id_hex.clone(),
                            asset_name: asset_name_hex.clone(),
                            amount,
                        },
                        script_source: Some(ScriptSource::ProvidedScriptSource(
                            plutus_script.clone(),
                        )),
                        redeemer,
                    }));
                }
                Some(Script::ReferencedPlutus(inline_script)) => {
                    let redeemer = context
                        .script_witness
                        .redeemers
                        .get(&RedeemerIndex::Mint(i))
                        .cloned();
                    mint_items.push(MintItem::ScriptMint(ScriptMint {
                        mint: MintParameter {
                            policy_id: policy_id_hex.clone(),
                            asset_name: asset_name_hex.clone(),
                            amount,
                        },
                        script_source: Some(ScriptSource::InlineScriptSource(
                            inline_script.clone(),
                        )),
                        redeemer,
                    }));
                }
                None => {
                    mint_items.push(MintItem::SimpleScriptMint(SimpleScriptMint {
                        mint: MintParameter {
                            policy_id: policy_id_hex.clone(),
                            asset_name: asset_name_hex.clone(),
                            amount,
                        },
                        script_source: None,
                    }));
                }
            }
        }
    }
    Ok(mint_items)
}
