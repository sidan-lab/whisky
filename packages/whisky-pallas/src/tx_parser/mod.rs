mod certificates;
mod change_address;
mod change_datum;
mod collaterals;
mod inputs;
mod metadata;
mod mints;
mod outputs;
mod reference_inputs;
mod required_signatures;
mod validity_range;
mod votes;
mod withdrawals;

use serde::{Deserialize, Serialize};
use whisky_common::*;

use crate::utils::calculate_tx_hash;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxParser {
    pub tx_hash: String,
    pub tx_hex: String,
    pub tx_fee_lovelace: u64,
    pub tx_body: TxBuilderBody,
    pub csl_tx_body: csl::TransactionBody,
    pub csl_witness_set: csl::TransactionWitnessSet,
}

impl TxParser {
    pub fn new(tx_hex: &str) -> Result<TxParser, WError> {
        let csl_tx = csl::Transaction::from_hex(tx_hex).expect("Invalid transaction");
        let csl_tx_body = csl_tx.body();
        let csl_witness_set = csl_tx.witness_set();

        // get network here

        let mut tx_parser = TxParser {
            tx_hash: calculate_tx_hash(tx_hex)?,
            tx_hex: tx_hex.to_string(),
            tx_fee_lovelace: csl_tx.body().fee().to_str().parse::<u64>().unwrap(),
            tx_body: TxBuilderBody::new(),
            csl_tx_body,
            csl_witness_set,
        };

        tx_parser
            .inputs()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .outputs()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .collaterals()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .required_signatures()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .reference_inputs()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .withdrawals()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .mints()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .change_address()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .change_datum()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .metadata()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .validity_range()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .certificates()
            .map_err(WError::from_err("TxParser - new"))?;
        tx_parser
            .votes()
            .map_err(WError::from_err("TxParser - new"))?;

        Ok(tx_parser)
    }
}
