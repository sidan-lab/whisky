use async_trait::async_trait;
use sidan_csl_rs::csl::JsError;
use sidan_csl_rs::model::{Action, Network, UTxO};
use sidan_csl_rs::core::utils::evaluate_tx_scripts;
use crate::service::Evaluator;

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
    ) -> Result<Vec<Action>, JsError> {
        evaluate_tx_scripts(tx_hex, inputs, additional_txs, network)
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
    ) -> Result<Vec<Action>, JsError> {
        self.evaluate_tx_sync(tx_hex, inputs, additional_txs, network)
    }
}