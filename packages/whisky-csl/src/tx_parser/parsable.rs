use whisky_common::{TxBuilderBody, TxParsable, TxTester, UTxO, WError};

use crate::WhiskyCSL;

use super::CSLParser;

impl TxParsable for WhiskyCSL {
    fn parse(&mut self, tx_hex: &str, resolved_utxos: &[UTxO]) -> Result<(), WError> {
        let mut parser = CSLParser::new();
        parser.parse(tx_hex, resolved_utxos)?;
        self.parser = parser;
        Ok(())
    }

    fn get_builder_body(&self) -> TxBuilderBody {
        self.tx_builder_body.clone()
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
