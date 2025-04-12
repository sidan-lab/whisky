// use async_trait::async_trait;
// use whisky_csl::{calculate_tx_hash, TxParser};

// use uplc::tx::SlotConfig;
// use whisky_common::models::{Network, UTxO};

// use whisky_common::*;

// use super::*;

// use crate::AdditionalUtxo;

// #[async_trait]
// impl Evaluator for BlockfrostProvider {
//     async fn evaluate_tx(
//         &self,
//         tx: &str,
//         _inputs: &[UTxO], // TODO: parse this also into additional_txs
//         additional_txs: &[String],
//         _network: &Network,
//         _slot_config: &SlotConfig,
//     ) -> Result<Vec<Action>, WError> {
//         let tx_out_cbors: Vec<AdditionalUtxo> = additional_txs
//             .iter()
//             .flat_map(|tx| {
//                 let parsed_tx = TxParser::new(tx);
//                 parsed_tx
//                     .unwrap() //TODO: add error handling
//                     .get_tx_outs_cbor()
//                     .iter()
//                     .enumerate() // Add this line to get the index
//                     .map(|(index, txout_cbor)| AdditionalUtxo {
//                         tx_hash: calculate_tx_hash(tx).unwrap(), // TODO: add error handling
//                         index: index as u32,                     // Use the index here
//                         txout_cbor: txout_cbor.to_string(),
//                     })
//                     .collect::<Vec<AdditionalUtxo>>()
//             })
//             .collect();

//         let result: Vec<Action> = self
//             .blockfrost_client
//             .evaluate_tx(tx, tx_out_cbors)
//             .await
//             .map_err(WError::from_err("evaluate_tx"))?
//             .iter()
//             .map(|r: &RedeemerEvaluation| Action {
//                 index: r.redeemer_index as u32,
//                 budget: Budget {
//                     mem: r.ex_units.mem as u64,
//                     steps: r.ex_units.steps as u64,
//                 },
//                 tag: match r.redeemer_tag.as_str() {
//                     "spend" => RedeemerTag::Spend,
//                     "mint" => RedeemerTag::Mint,
//                     "cert" => RedeemerTag::Cert,
//                     "wdrl" => RedeemerTag::Reward,
//                     _ => panic!("Unknown redeemer tag from maestro service"),
//                 },
//             })
//             .collect();
//         Ok(result)
//     }
// }
