use pallas::ledger::primitives::conway::Tx;
use whisky_common::{MintItem, MintParameter, SimpleScriptMint, WError};

use crate::{
    tx_parser::context::{ParserContext, RedeemerIndex, Script},
    wrapper::witness_set::redeemer::RedeemerTag,
};

pub fn extract_mints(pallas_tx: &Tx, context: &ParserContext) -> Result<Vec<MintItem>, WError> {
    let mut mints_vec: Vec<MintItem> = Vec::new();
    let pallas_mints = &pallas_tx.transaction_body.mint;
    if let Some(mints) = pallas_mints {
        for (index, (policy_id, asset_map)) in mints.iter().enumerate() {
            let policy_id_hex = policy_id.to_string();
            let script = context.script_witnesses.scripts.get(&policy_id_hex);
            match script {
                Some(script_source) => {
                    match script_source {
                        Script::ProvidedNative(provided_simple_script_source) => {
                            for (asset_name, amount) in asset_map {
                                let asset_name_hex = asset_name.to_string();
                                mints_vec.push(MintItem::SimpleScriptMint(SimpleScriptMint {
                                mint: MintParameter {
                                    policy_id: policy_id_hex.clone(),
                                    asset_name: asset_name_hex,
                                    amount: i64::try_from(amount).unwrap() as i128,
                                },
                                script_source: Some(whisky_common::SimpleScriptSource::ProvidedSimpleScriptSource(provided_simple_script_source.clone())),
                            }))
                            }
                        }
                        Script::ProvidedPlutus(provided_script_source) => {
                            for (asset_name, amount) in asset_map {
                                let asset_name_hex = asset_name.to_string();
                                let redeemer =
                                    context.script_witnesses.redeemers.get(&RedeemerIndex {
                                        tag: RedeemerTag::Mint,
                                        index: index.try_into().unwrap(),
                                    });
                                mints_vec.push(MintItem::ScriptMint(whisky_common::ScriptMint {
                                    mint: MintParameter {
                                        policy_id: policy_id_hex.clone(),
                                        asset_name: asset_name_hex,
                                        amount: i64::try_from(amount).unwrap() as i128,
                                    },
                                    redeemer: redeemer.cloned(),
                                    script_source: Some(
                                        whisky_common::ScriptSource::ProvidedScriptSource(
                                            provided_script_source.clone(),
                                        ),
                                    ),
                                }))
                            }
                        }
                        Script::ReferencedNative(inline_simple_script_source) => {
                            for (asset_name, amount) in asset_map {
                                let asset_name_hex = asset_name.to_string();
                                mints_vec.push(MintItem::SimpleScriptMint(SimpleScriptMint {
                                    mint: MintParameter {
                                        policy_id: policy_id_hex.clone(),
                                        asset_name: asset_name_hex,
                                        amount: i64::try_from(amount).unwrap() as i128,
                                    },
                                    script_source: Some(
                                        whisky_common::SimpleScriptSource::InlineSimpleScriptSource(
                                            inline_simple_script_source.clone(),
                                        ),
                                    ),
                                }))
                            }
                        }
                        Script::ReferencedPlutus(inline_script_source) => {
                            for (asset_name, amount) in asset_map {
                                let asset_name_hex = asset_name.to_string();
                                let redeemer =
                                    context.script_witnesses.redeemers.get(&RedeemerIndex {
                                        tag: RedeemerTag::Mint,
                                        index: index.try_into().unwrap(),
                                    });
                                mints_vec.push(MintItem::ScriptMint(whisky_common::ScriptMint {
                                    mint: MintParameter {
                                        policy_id: policy_id_hex.clone(),
                                        asset_name: asset_name_hex,
                                        amount: i64::try_from(amount).unwrap() as i128,
                                    },
                                    redeemer: redeemer.cloned(),
                                    script_source: Some(
                                        whisky_common::ScriptSource::InlineScriptSource(
                                            inline_script_source.clone(),
                                        ),
                                    ),
                                }))
                            }
                        }
                    }
                }

                None => {
                    return Err(WError::new(
                        "WhiskyPallas - Extracting mints:",
                        &format!("No script found for policy ID: {}", policy_id_hex),
                    ));
                } // for (asset_name, amount) in asset_map {}
            }
        }
    }
    Ok(mints_vec)
}
