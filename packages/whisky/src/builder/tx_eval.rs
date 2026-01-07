use crate::*;
use async_trait::async_trait;
use uplc::tx::SlotConfig;
use whisky_common::Evaluator;

use super::TxBuilder;

pub trait TxEvaluation {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self;
}

impl TxEvaluation for TxBuilder {
    fn update_redeemer(&mut self, tx_evaluation: Vec<Action>) -> &mut Self {
        let multiplier = self.serializer.tx_evaluation_multiplier_percentage();
        for redeemer_evaluation in tx_evaluation {
            match redeemer_evaluation.tag {
                RedeemerTag::Spend => {
                    let input =
                        &mut self.tx_builder_body.inputs[redeemer_evaluation.index as usize];
                    if let TxIn::ScriptTxIn(ScriptTxIn { script_tx_in, .. }) = input {
                        let redeemer: &mut Redeemer = script_tx_in.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Mint => {
                    let mint_item =
                        &mut self.tx_builder_body.mints[redeemer_evaluation.index as usize];
                    if let MintItem::ScriptMint(mint) = mint_item {
                        let redeemer: &mut Redeemer = mint.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Cert => {
                    let cert_item =
                        &mut self.tx_builder_body.certificates[redeemer_evaluation.index as usize];
                    if let Certificate::ScriptCertificate(cert) = cert_item {
                        let redeemer: &mut Redeemer = cert.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Reward => {
                    let withdrawal_item =
                        &mut self.tx_builder_body.withdrawals[redeemer_evaluation.index as usize];
                    if let Withdrawal::PlutusScriptWithdrawal(withdrawal) = withdrawal_item {
                        let redeemer: &mut Redeemer = withdrawal.redeemer.as_mut().unwrap();
                        redeemer.ex_units.mem = redeemer_evaluation.budget.mem * multiplier / 100;
                        redeemer.ex_units.steps =
                            redeemer_evaluation.budget.steps * multiplier / 100;
                    }
                }
                RedeemerTag::Propose => todo!(),
                RedeemerTag::Vote => todo!(),
            }
        }
        self
    }
}

#[derive(Clone, Debug)]
pub struct OfflineTxEvaluator {}

impl OfflineTxEvaluator {
    pub fn new() -> Self {
        OfflineTxEvaluator {}
    }
}

impl Default for OfflineTxEvaluator {
    fn default() -> Self {
        OfflineTxEvaluator::new()
    }
}

impl OfflineTxEvaluator {
    fn evaluate_tx_sync(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],
        additional_txs: &[String],
        network: &Network,
        slot_config: &SlotConfig,
    ) -> Result<Vec<Action>, WError> {
        consolidate_errors(evaluate_tx_scripts(
            tx_hex,
            inputs,
            additional_txs,
            network,
            slot_config,
        )?)
    }
}

fn consolidate_errors(eval_results: Vec<EvalResult>) -> Result<Vec<Action>, WError> {
    let mut actions = Vec::new();
    let mut errors_texts = Vec::new();
    for eval_result in eval_results {
        match eval_result {
            EvalResult::Success(action) => actions.push(action),
            EvalResult::Error(error) => {
                errors_texts.push(format!("Error at index: [ {} ] - Budget: [ {:?} ] - Tag: [ {:?} ] - Error message: [ {} ] - Logs: [ {:?} ]",
                                          error.index, error.budget, error.tag, error.error_message, error.logs));
            }
        }
    }
    if errors_texts.is_empty() {
        Ok(actions)
    } else {
        Err(WError::new(
            "consolidate_errors",
            &format!("Errors found during evaluation: [ {:?} ]", errors_texts),
        ))
    }
}

#[async_trait]
impl Evaluator for OfflineTxEvaluator {
    async fn evaluate_tx(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],
        additional_txs: &[String],
        network: &Network,
        slot_config: &SlotConfig,
    ) -> Result<Vec<Action>, WError> {
        self.evaluate_tx_sync(tx_hex, inputs, additional_txs, network, slot_config)
    }
}
