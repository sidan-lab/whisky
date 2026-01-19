use pallas::ledger::primitives::conway::Tx;
use whisky_common::{
    PlutusScriptWithdrawal, PubKeyWithdrawal, ScriptSource, SimpleScriptSource,
    SimpleScriptWithdrawal, WError, Withdrawal,
};

use crate::{
    tx_parser::context::{ParserContext, RedeemerIndex},
    wrapper::{transaction_body::RewardAccount, witness_set::redeemer::RedeemerTag},
};

pub fn extract_withdrawals(
    pallas_tx: &Tx,
    context: &ParserContext,
) -> Result<Vec<Withdrawal>, WError> {
    let mut withdrawals_vec = Vec::new();
    let pallas_withdrawals = &pallas_tx.transaction_body.withdrawals;
    if let Some(withdrawals) = pallas_withdrawals {
        for (index, (reward_address, amount)) in withdrawals.iter().enumerate() {
            let reward_account = RewardAccount::from_bytes(&reward_address.to_vec())?;
            let address = reward_account.to_bech32().map_err(|e| {
                WError::new(
                    "WhiskyPallas - Extracting withdrawals:",
                    &format!("Failed to convert reward address to bech32: {:?}", e),
                )
            })?;
            let coin = amount.clone();
            match reward_account.to_stake_cred().unwrap().inner {
                pallas::ledger::primitives::StakeCredential::ScriptHash(hash) => {
                    if let Some(script) = context.script_witnesses.scripts.get(&hash.to_string()) {
                        match script {
                            super::context::Script::ProvidedNative(
                                provided_simple_script_source,
                            ) => withdrawals_vec.push(Withdrawal::SimpleScriptWithdrawal(
                                SimpleScriptWithdrawal {
                                    address,
                                    coin,
                                    script_source: Some(
                                        SimpleScriptSource::ProvidedSimpleScriptSource(
                                            provided_simple_script_source.clone(),
                                        ),
                                    ),
                                },
                            )),
                            super::context::Script::ProvidedPlutus(provided_script_source) => {
                                let redeemer =
                                    context.script_witnesses.redeemers.get(&RedeemerIndex {
                                        tag: RedeemerTag::Reward,
                                        index: index.try_into().unwrap(),
                                    });
                                withdrawals_vec.push(Withdrawal::PlutusScriptWithdrawal(
                                    PlutusScriptWithdrawal {
                                        address,
                                        coin,
                                        script_source: Some(ScriptSource::ProvidedScriptSource(
                                            provided_script_source.clone(),
                                        )),
                                        redeemer: redeemer.cloned(),
                                    },
                                ));
                            }
                            super::context::Script::ReferencedNative(
                                inline_simple_script_source,
                            ) => {
                                withdrawals_vec.push(Withdrawal::SimpleScriptWithdrawal(
                                    SimpleScriptWithdrawal {
                                        address,
                                        coin,
                                        script_source: Some(
                                            SimpleScriptSource::InlineSimpleScriptSource(
                                                inline_simple_script_source.clone(),
                                            ),
                                        ),
                                    },
                                ));
                            }
                            super::context::Script::ReferencedPlutus(inline_script_source) => {
                                let redeemer =
                                    context.script_witnesses.redeemers.get(&RedeemerIndex {
                                        tag: RedeemerTag::Reward,
                                        index: index.try_into().unwrap(),
                                    });
                                withdrawals_vec.push(Withdrawal::PlutusScriptWithdrawal(
                                    PlutusScriptWithdrawal {
                                        address,
                                        coin,
                                        script_source: Some(ScriptSource::InlineScriptSource(
                                            inline_script_source.clone(),
                                        )),
                                        redeemer: redeemer.cloned(),
                                    },
                                ));
                            }
                        };
                    } else {
                        return Err(WError::new(
                            "WhiskyPallas - Extracting withdrawals:",
                            &format!(
                                "Script for withdrawal address {} not found",
                                reward_account.to_bech32().unwrap()
                            ),
                        ));
                    }
                }
                pallas::ledger::primitives::StakeCredential::AddrKeyhash(_) => withdrawals_vec
                    .push(Withdrawal::PubKeyWithdrawal(PubKeyWithdrawal {
                        address: reward_account.to_bech32().unwrap(),
                        coin: amount.clone(),
                    })),
            }
        }
    }
    Ok(withdrawals_vec)
}
