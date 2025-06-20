use whisky_common::{TxBuilderBody, TxParsable, TxTester, UTxO, UtxoInput, WError};

use crate::WhiskyCSL;

use super::CSLParser;

impl TxParsable for WhiskyCSL {
    fn parse(&mut self, tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<(), WError> {
        let mut parser = CSLParser::new();
        parser.parse(tx_hex, resolved_utxos)?;
        self.parser = parser;
        Ok(())
    }

    fn get_required_inputs(&mut self, tx_hex: &str) -> Result<Vec<UtxoInput>, WError> {
        let required_inputs = CSLParser::extract_all_required_utxo_input(tx_hex)
            .map_err(WError::from_err("WhiskyCSL - get_required_inputs"))?;
        Ok(required_inputs)
    }

    fn get_builder_body(&self) -> TxBuilderBody {
        self.parser.tx_body.clone()
    }

    fn get_builder_body_without_change(&self) -> TxBuilderBody {
        let mut tx_body = self.parser.tx_body.clone();
        let outputs = self.parser.csl_tx_body.outputs();
        let outputs_len = outputs.len();
        if outputs_len > 0 {
            tx_body.outputs.pop();
        }
        tx_body
    }

    fn to_tester(&self) -> TxTester {
        TxTester::new(&self.parser.tx_body)
    }
}
