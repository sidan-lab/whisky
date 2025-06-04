use super::context::ParserContext;
use super::context::RedeemerIndex;
use super::context::Script;
use super::CSLParser;
use cardano_serialization_lib as csl;
use whisky_common::{
    PlutusScriptWithdrawal, PubKeyWithdrawal, ScriptSource, SimpleScriptSource,
    SimpleScriptWithdrawal, WError, Withdrawal,
};

impl CSLParser {
    pub fn get_withdrawals(&self) -> &Vec<Withdrawal> {
        &self.tx_body.withdrawals
    }

    pub(super) fn extract_withdrawals(&mut self) -> Result<(), WError> {
        let withdrawals = self.csl_tx_body.withdrawals();
        if let Some(withdrawals) = withdrawals {
            self.tx_body.withdrawals = csl_withdrawals_to_withdrawals(&withdrawals, &self.context)?;
        }
        Ok(())
    }
}

pub fn csl_withdrawals_to_withdrawals(
    withdrawals: &csl::Withdrawals,
    context: &ParserContext,
) -> Result<Vec<Withdrawal>, WError> {
    let mut result = Vec::new();
    let withdrawals_keys = withdrawals.keys();
    let len = withdrawals_keys.len();
    for i in 0..len {
        let reward_address = withdrawals_keys.get(i);
        let reward_address_bech32 = reward_address.to_address().to_bech32(None).map_err(|e| {
            WError::new(
                "csl_withdrawals_to_withdrawals",
                &format!("Failed to convert address to bech32: {:?}", e),
            )
        })?;
        let withdrawal_amount = withdrawals.get(&reward_address);
        if let Some(withdrawal_amount) = withdrawal_amount {
            let coin = withdrawal_amount.to_str().parse::<u64>().map_err(|e| {
                WError::new(
                    "csl_withdrawals_to_withdrawals",
                    &format!("Failed to parse withdrawal amount: {:?}", e),
                )
            })?;

            let redeemer = context
                .script_witness
                .redeemers
                .get(&RedeemerIndex::Reward(i));

            let script_hash = reward_address.payment_cred().to_scripthash();
            let script = script_hash
                .map(|sh| context.script_witness.scripts.get(&sh))
                .flatten();

            match script {
                Some(script) => match script {
                    Script::ProvidedPlutus(plutus_script) => {
                        result.push(Withdrawal::PlutusScriptWithdrawal(PlutusScriptWithdrawal {
                            address: reward_address_bech32,
                            coin,
                            script_source: Some(ScriptSource::ProvidedScriptSource(
                                plutus_script.clone(),
                            )),
                            redeemer: redeemer.cloned(),
                        }));
                    }
                    Script::ProvidedNative(native_script) => {
                        result.push(Withdrawal::SimpleScriptWithdrawal(SimpleScriptWithdrawal {
                            address: reward_address_bech32,
                            coin,
                            script_source: Some(SimpleScriptSource::ProvidedSimpleScriptSource(
                                native_script.clone(),
                            )),
                        }));
                    }
                    Script::ReferencedNative(inline_simple_script_source) => {
                        result.push(Withdrawal::SimpleScriptWithdrawal(SimpleScriptWithdrawal {
                            address: reward_address_bech32,
                            coin,
                            script_source: Some(SimpleScriptSource::InlineSimpleScriptSource(
                                inline_simple_script_source.clone(),
                            )),
                        }));
                    }
                    Script::ReferencedPlutus(inline_script_source) => {
                        result.push(Withdrawal::PlutusScriptWithdrawal(PlutusScriptWithdrawal {
                            address: reward_address_bech32,
                            coin,
                            script_source: Some(ScriptSource::InlineScriptSource(
                                inline_script_source.clone(),
                            )),
                            redeemer: redeemer.cloned(),
                        }));
                    }
                },
                None => {
                    result.push(Withdrawal::PubKeyWithdrawal(PubKeyWithdrawal {
                        address: reward_address_bech32,
                        coin,
                    }));
                }
            }
        }
    }

    Ok(result)
}
