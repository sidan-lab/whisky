use pallas::ledger::primitives::{conway::Tx, Fragment};
use whisky_common::{TxParsable, TxTester, WError};

use crate::{tx_parser::parse, WhiskyPallas};

impl TxParsable for WhiskyPallas {
    fn parse(
        &mut self,
        tx_hex: &str,
        resolved_utxos: &[whisky_common::UTxO],
    ) -> Result<(), whisky_common::WError> {
        self.tx_builder_body = parse(tx_hex, resolved_utxos)?;
        Ok(())
    }

    fn get_required_inputs(
        &mut self,
        tx_hex: &str,
    ) -> Result<Vec<whisky_common::UtxoInput>, whisky_common::WError> {
        let tx_bytes = hex::decode(tx_hex)
            .map_err(|_| WError::new("get_required_inputs", "error deserialising tx_hex"))?;
        let pallas_tx = Tx::decode_fragment(&tx_bytes)
            .map_err(|_| WError::new("get_required_inputs", "error decoding tx fragment"))?;
        let mut required_inputs = Vec::new();
        for inputs in pallas_tx.transaction_body.inputs.iter() {
            required_inputs.push(whisky_common::UtxoInput {
                tx_hash: inputs.transaction_id.to_string(),
                output_index: inputs.index as u32,
            });
        }
        Ok(required_inputs)
    }

    fn get_builder_body(&self) -> whisky_common::TxBuilderBody {
        self.tx_builder_body.clone()
    }

    fn get_builder_body_without_change(&self) -> whisky_common::TxBuilderBody {
        let mut tx_body = self.tx_builder_body.clone();
        if !tx_body.outputs.is_empty() {
            tx_body.outputs.pop();
        }
        tx_body
    }

    fn to_tester(&self) -> TxTester {
        TxTester::new(&self.tx_builder_body)
    }
}
