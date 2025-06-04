use async_trait::async_trait;
use whisky_csl::{calculate_tx_hash, CSLParser};

use uplc::tx::SlotConfig;
use whisky_common::models::{Network, UTxO};

use whisky_common::*;

use super::*;

#[derive(Serialize)]
pub struct AdditionalUtxo {
    pub tx_hash: String,
    pub index: u32,
    pub txout_cbor: String,
}

#[derive(Serialize)]
pub struct EvaluateTx {
    cbor: String,
    additional_utxos: Vec<AdditionalUtxo>,
}

#[async_trait]
impl Evaluator for MaestroProvider {
    async fn evaluate_tx(
        &self,
        tx: &str,
        _inputs: &[UTxO], // TODO: parse this also into additional_txs
        additional_txs: &[String],
        _network: &Network,
        _slot_config: &SlotConfig,
    ) -> Result<Vec<Action>, WError> {
        let tx_out_cbors: Vec<AdditionalUtxo> = additional_txs
            .iter()
            .flat_map(|tx| {
                let tx_hash = calculate_tx_hash(tx).unwrap();
                let utxos = CSLParser::extract_output_cbors(tx).unwrap();
                utxos
                    .into_iter()
                    .enumerate()
                    .map(|(index, txout_cbor)| AdditionalUtxo {
                        tx_hash: tx_hash.clone(),
                        index: index as u32,
                        txout_cbor,
                    })
                    .collect::<Vec<AdditionalUtxo>>()
            })
            .collect();

        let url = "/transactions/evaluate";
        let body = EvaluateTx {
            cbor: tx.to_string(),
            additional_utxos: tx_out_cbors,
        };

        let resp = self
            .maestro_client
            .post(url, &body)
            .await
            .map_err(WError::from_err("Maestro - evaluate_tx"))?;

        let result: Vec<Action> = serde_json::from_str::<Vec<RedeemerEvaluation>>(&resp)
            .map_err(WError::from_err("Maestro - evaluate_tx"))?
            .iter()
            .map(|r: &RedeemerEvaluation| Action {
                index: r.redeemer_index as u32,
                budget: Budget {
                    mem: r.ex_units.mem as u64,
                    steps: r.ex_units.steps as u64,
                },
                tag: match r.redeemer_tag.as_str() {
                    "spend" => RedeemerTag::Spend,
                    "mint" => RedeemerTag::Mint,
                    "cert" => RedeemerTag::Cert,
                    "wdrl" => RedeemerTag::Reward,
                    _ => panic!("Unknown redeemer tag from maestro service"),
                },
            })
            .collect();
        Ok(result)
    }
}
