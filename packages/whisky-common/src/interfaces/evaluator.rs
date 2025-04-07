use crate::*;
use async_trait::async_trait;
use uplc::tx::SlotConfig;

#[async_trait]
pub trait Evaluator: Send {
    async fn evaluate_tx(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],
        additional_txs: &[String],
        network: &Network,
        slot_config: &SlotConfig,
    ) -> Result<Vec<Action>, WError>;
}
