mod certificates;
mod change_address;
mod change_datum;
mod collaterals;
mod context;
mod inputs;
mod metadata;
mod mints;
mod outputs;
mod reference_inputs;
mod required_signatures;
mod utxo_converter;
mod validity_range;
mod votes;
mod withdrawals;

use crate::tx_parser::context::ParserContext;
use cardano_serialization_lib::{self as csl};
use serde::{Deserialize, Serialize};
use whisky_common::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxParser {
    pub tx_body: TxBuilderBody,
    pub csl_tx_body: csl::TransactionBody,
    pub csl_witness_set: csl::TransactionWitnessSet,
    pub csl_aux_data: Option<csl::AuxiliaryData>,
    pub context: ParserContext,
    pub tx_hash: String,
}

impl TxParser {
    pub fn new(tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<TxParser, WError> {
        let csl_tx = csl::FixedTransaction::from_hex(tx_hex).map_err(|e| {
            WError::new(
                "TxParser - new",
                &format!("Failed to parse transaction hex: {}", e),
            )
        })?;
        let tx_hash = csl_tx.transaction_hash().to_hex();

        let csl_tx_body = csl_tx.body();
        let csl_witness_set = csl_tx.witness_set();
        let csl_aux_data = csl_tx.auxiliary_data();

        let tx_body = TxBuilderBody::new();

        let mut context = ParserContext::new();
        context
            .fill_resolved_utxos(&csl_tx_body, resolved_utxos)
            .map_err(|e| {
                WError::new(
                    "TxParser - new - fill_resolved_utxos",
                    &format!("Failed to fill resolved UTxOs: {}", e),
                )
            })?;
        context
            .collect_script_witnesses_from_tx_witnesses_set(csl_witness_set.clone())
            .map_err(|e| {
                WError::new(
                    "TxParser - new - collect_script_witnesses_from_tx_witnesses_set",
                    &format!("Failed to collect script witnesses from witness set: {}", e),
                )
            })?;
        context
            .collect_script_witnesses_from_tx_body(csl_tx_body.clone())
            .map_err(|e| {
                WError::new(
                    "TxParser - new - collect_script_witnesses_from_tx_body",
                    &format!("Failed to collect script witnesses from tx body: {}", e),
                )
            })?;

        let mut tx_parser = TxParser {
            tx_body,
            csl_tx_body,
            csl_witness_set,
            csl_aux_data,
            context,
            tx_hash,
        };

        tx_parser
            .extract_inputs()
            .map_err(WError::from_err("TxParser - new - inputs"))?;
        tx_parser
            .extract_outputs()
            .map_err(WError::from_err("TxParser - new - outputs"))?;
        tx_parser
            .extract_collaterals()
            .map_err(WError::from_err("TxParser - new - collaterals"))?;
        tx_parser
            .extract_required_signatures()
            .map_err(WError::from_err("TxParser - new - required_signatures"))?;
        tx_parser
            .extract_reference_inputs()
            .map_err(WError::from_err("TxParser - new - reference_inputs"))?;
        tx_parser
            .extract_withdrawals()
            .map_err(WError::from_err("TxParser - new - withdrawals"))?;
        tx_parser
            .extract_mints()
            .map_err(WError::from_err("TxParser - new - mints"))?;
        tx_parser
            .extract_change_address()
            .map_err(WError::from_err("TxParser - new - change_address"))?;
        tx_parser
            .extract_change_datum()
            .map_err(WError::from_err("TxParser - new - change_datum"))?;
        tx_parser
            .extract_metadata()
            .map_err(WError::from_err("TxParser - new - metadata"))?;
        tx_parser
            .extract_validity_range()
            .map_err(WError::from_err("TxParser - new - validity_range"))?;
        tx_parser
            .extract_certificates()
            .map_err(WError::from_err("TxParser - new - certificates"))?;
        tx_parser
            .extract_votes()
            .map_err(WError::from_err("TxParser - new - votes"))?;

        Ok(tx_parser)
    }

    pub fn get_builder_body(&self) -> &TxBuilderBody {
        &self.tx_body
    }
}
