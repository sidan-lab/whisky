use async_trait::async_trait;
use cardano_serialization_lib::error::JsError;

use crate::model::{Action, UTxO};

#[async_trait]
pub trait IEvaluator: Send {
    async fn evaluate_tx(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],           // Change the type from &Vec<UTxO> to &[UTxO]
        additional_txs: &[String], // Change the type from &Vec<String> to &[String]
    ) -> Result<Vec<Action>, JsError>;

    fn evaluate_tx_sync(
        &self,
        tx_hex: &str,
        inputs: &[UTxO],           // Change the type from &Vec<UTxO> to &[UTxO]
        additional_txs: &[String], // Change the type from &Vec<String> to &[String]
    ) -> Result<Vec<Action>, JsError>;
}
