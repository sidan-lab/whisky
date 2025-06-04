mod certificates;
mod change_address;
mod change_datum;
mod collaterals;
mod context;
mod inputs;
mod metadata;
mod mints;
mod outputs;
mod parsable;
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
pub struct CSLParser {
    pub tx_body: TxBuilderBody,
    pub csl_tx_body: csl::TransactionBody,
    pub csl_witness_set: csl::TransactionWitnessSet,
    pub csl_aux_data: Option<csl::AuxiliaryData>,
    pub context: ParserContext,
    pub tx_hash: String,
}

impl CSLParser {
    pub fn new() -> CSLParser {
        CSLParser {
            tx_body: TxBuilderBody::new(),
            csl_tx_body: csl::TransactionBody::new_tx_body(
                &csl::TransactionInputs::new(),
                &csl::TransactionOutputs::new(),
                &csl::Coin::zero(),
            ),
            csl_witness_set: csl::TransactionWitnessSet::new(),
            csl_aux_data: None,
            context: ParserContext::new(),
            tx_hash: "".to_string(),
        }
    }

    pub fn parse(&mut self, tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<&mut Self, WError> {
        let csl_tx = csl::FixedTransaction::from_hex(tx_hex).map_err(|e| {
            WError::new(
                "CSLParser - new",
                &format!("Failed to parse transaction hex: {:?}", e),
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
                    "CSLParser - new - fill_resolved_utxos",
                    &format!("Failed to fill resolved UTxOs: {:?}", e),
                )
            })?;
        context
            .collect_script_witnesses_from_tx_witnesses_set(csl_witness_set.clone())
            .map_err(|e| {
                WError::new(
                    "CSLParser - new - collect_script_witnesses_from_tx_witnesses_set",
                    &format!(
                        "Failed to collect script witnesses from witness set: {:?}",
                        e
                    ),
                )
            })?;
        context
            .collect_script_witnesses_from_tx_body(csl_tx_body.clone())
            .map_err(|e| {
                WError::new(
                    "CSLParser - new - collect_script_witnesses_from_tx_body",
                    &format!("Failed to collect script witnesses from tx body: {:?}", e),
                )
            })?;

        self.tx_body = tx_body;
        self.csl_tx_body = csl_tx_body;
        self.csl_witness_set = csl_witness_set;
        self.csl_aux_data = csl_aux_data;
        self.context = context;
        self.tx_hash = tx_hash;

        self.extract_inputs()
            .map_err(WError::from_err("CSLParser - new - inputs"))?;
        self.extract_outputs()
            .map_err(WError::from_err("CSLParser - new - outputs"))?;
        self.extract_collaterals()
            .map_err(WError::from_err("CSLParser - new - collaterals"))?;
        self.extract_required_signatures()
            .map_err(WError::from_err("CSLParser - new - required_signatures"))?;
        self.extract_reference_inputs()
            .map_err(WError::from_err("CSLParser - new - reference_inputs"))?;
        self.extract_withdrawals()
            .map_err(WError::from_err("CSLParser - new - withdrawals"))?;
        self.extract_mints()
            .map_err(WError::from_err("CSLParser - new - mints"))?;
        self.extract_change_address()
            .map_err(WError::from_err("CSLParser - new - change_address"))?;
        self.extract_change_datum()
            .map_err(WError::from_err("CSLParser - new - change_datum"))?;
        self.extract_metadata()
            .map_err(WError::from_err("CSLParser - new - metadata"))?;
        self.extract_validity_range()
            .map_err(WError::from_err("CSLParser - new - validity_range"))?;
        self.extract_certificates()
            .map_err(WError::from_err("CSLParser - new - certificates"))?;
        self.extract_votes()
            .map_err(WError::from_err("CSLParser - new - votes"))?;

        Ok(self)
    }
}
