use crate::*;
use async_trait::async_trait;
use uplc::tx::SlotConfig;
use whisky_common::Evaluator;

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
