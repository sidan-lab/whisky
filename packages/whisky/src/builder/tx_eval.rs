use crate::service::Evaluator;
use async_trait::async_trait;
use uplc::tx::SlotConfig;
use whisky_core::core::serializer::evaluate_tx_scripts;
use whisky_core::csl::WError;
use whisky_core::model::{Action, EvalResult, Network, UTxO};

#[derive(Clone, Debug)]
pub struct MeshTxEvaluator {}

impl MeshTxEvaluator {
    pub fn new() -> Self {
        MeshTxEvaluator {}
    }
}

impl Default for MeshTxEvaluator {
    fn default() -> Self {
        MeshTxEvaluator::new()
    }
}

impl MeshTxEvaluator {
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
        Err(WError::from_str(&format!(
            "Errors found during evaluation: [ {:?} ]",
            errors_texts
        )))
    }
}

#[async_trait]
impl Evaluator for MeshTxEvaluator {
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
